// Assessment repository - database access layer
use super::models::{AssessmentError, AssessmentResponse, AssessmentType};
use crate::db::Database;
use crate::utils::sanitize_optional_text;
use crate::MAX_QUERY_LIMIT;
use std::sync::Arc;
use tracing::{error, info};

/// Minimum limit for query results
const MIN_QUERY_LIMIT: i32 = 1;

pub struct AssessmentRepository {
    db: Arc<Database>,
}

impl AssessmentRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Save an assessment (completed or draft)
    pub fn save_assessment(
        &self,
        assessment_type_id: i32,
        responses: &[i32],
        total_score: i32,
        severity_level: &str,
        notes: Option<String>,
        status: &str,
    ) -> Result<i32, AssessmentError> {
        // Sanitize notes (trim and convert empty string to None)
        let notes = sanitize_optional_text(notes);

        let conn = self.db.get_connection();
        let mut conn = conn.lock();

        let responses_json = serde_json::to_string(responses).map_err(|e| {
            AssessmentError::InvalidResponse(format!("Failed to serialize responses: {}", e))
        })?;

        // ✅ RAII transaction - automatic rollback on drop if not committed
        let tx = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(AssessmentError::Database)?;

        let id = tx.query_row(
            "INSERT INTO assessment_responses (assessment_type_id, responses, total_score, severity_level, notes, status)
             VALUES (?, ?, ?, ?, ?, ?)
             RETURNING id",
            [
                &assessment_type_id as &dyn rusqlite::ToSql,
                &responses_json as &dyn rusqlite::ToSql,
                &total_score as &dyn rusqlite::ToSql,
                &severity_level as &dyn rusqlite::ToSql,
                &notes as &dyn rusqlite::ToSql,
                &status as &dyn rusqlite::ToSql,
            ],
            |row| row.get(0),
        )?;

        // Commit transaction - automatic rollback via Drop on error/panic
        tx.commit().map_err(AssessmentError::Database)?;

        info!(
            assessment_id = id,
            assessment_type_id = assessment_type_id,
            total_score = total_score,
            severity_level = severity_level,
            has_notes = notes.is_some(),
            status = status,
            "Saved assessment"
        );

        Ok(id)
    }

    /// Get all assessment types
    pub fn get_assessment_types(&self) -> Result<Vec<AssessmentType>, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, code, name, description, question_count, min_score, max_score, thresholds
             FROM assessment_types
             ORDER BY id",
        )?;

        let types = stmt
            .query_map([], |row| {
                Ok(AssessmentType {
                    id: row.get(0)?,
                    code: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    question_count: row.get(4)?,
                    min_score: row.get(5)?,
                    max_score: row.get(6)?,
                    thresholds: serde_json::from_str(&row.get::<_, String>(7)?).map_err(|e| {
                        error!("Failed to deserialize assessment type thresholds: {}", e);
                        rusqlite::Error::InvalidColumnType(
                            7,
                            "thresholds".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(types)
    }

    /// Get assessment type by code
    pub fn get_assessment_type_by_code(
        &self,
        code: &str,
    ) -> Result<AssessmentType, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let result = conn.query_row(
            "SELECT id, code, name, description, question_count, min_score, max_score, thresholds
             FROM assessment_types
             WHERE code = ?",
            [code],
            |row| {
                Ok(AssessmentType {
                    id: row.get(0)?,
                    code: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    question_count: row.get(4)?,
                    min_score: row.get(5)?,
                    max_score: row.get(6)?,
                    thresholds: serde_json::from_str(&row.get::<_, String>(7)?).map_err(|e| {
                        error!("Failed to deserialize assessment type thresholds: {}", e);
                        rusqlite::Error::InvalidColumnType(
                            7,
                            "thresholds".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })?,
                })
            },
        );

        match result {
            Ok(assessment_type) => Ok(assessment_type),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err(AssessmentError::InvalidType(code.to_string()))
            }
            Err(e) => Err(AssessmentError::Database(e)),
        }
    }

    /// Get assessment history with optional date filtering
    pub fn get_assessment_history(
        &self,
        assessment_type_code: Option<String>,
        from_date: Option<String>,
        to_date: Option<String>,
        limit: Option<i32>,
    ) -> Result<Vec<AssessmentResponse>, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Build date filter using query builder helper
        let (date_filter, date_params) = crate::db::query_builder::DateFilterBuilder::new()
            .with_from_date(from_date.as_deref(), "resp.completed_at")
            .with_to_date(to_date.as_deref(), "resp.completed_at")
            .build();

        // Build assessment type filter
        let type_filter = if assessment_type_code.is_some() {
            " AND atype.code = ?"
        } else {
            ""
        };

        let mut query = format!(
            "SELECT resp.id, resp.assessment_type_id, resp.responses, resp.total_score, resp.severity_level,
                    strftime('%Y-%m-%d %H:%M:%S', resp.completed_at) as completed_at, resp.notes, resp.status,
                    atype.id, atype.code, atype.name, atype.description, atype.question_count, atype.min_score, atype.max_score, atype.thresholds
             FROM assessment_responses AS resp
             JOIN assessment_types AS atype ON resp.assessment_type_id = atype.id
             WHERE 1=1{}{}
             ORDER BY resp.completed_at DESC",
            type_filter, date_filter
        );

        // Build params dynamically: type code + date params
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        if let Some(code) = &assessment_type_code {
            params.push(code);
        }
        // Add date params
        for param in &date_params {
            params.push(param.as_ref());
        }

        // ✅ FIXED: Use parameterized query for LIMIT (prevents SQL injection)
        // Enforce reasonable bounds to prevent excessive queries
        // Design choice: Using clamp() for silent correction rather than validation error
        // This provides better UX (automatically corrects invalid limits) rather than rejecting requests
        let safe_limit;
        if let Some(lim) = limit {
            safe_limit = lim.clamp(MIN_QUERY_LIMIT, MAX_QUERY_LIMIT);
            query.push_str(" LIMIT ?");
            params.push(&safe_limit);
        }

        let mut stmt = conn.prepare(&query)?;

        let responses = stmt
            .query_map(params.as_slice(), |row| {
                let responses_json: String = row.get(2)?;
                let responses: Vec<i32> = serde_json::from_str(&responses_json).map_err(|e| {
                    error!("Failed to deserialize assessment responses: {}", e);
                    rusqlite::Error::InvalidColumnType(
                        2,
                        "responses".to_string(),
                        rusqlite::types::Type::Text,
                    )
                })?;

                Ok(AssessmentResponse {
                    id: row.get(0)?,
                    assessment_type: AssessmentType {
                        id: row.get(8)?,
                        code: row.get(9)?,
                        name: row.get(10)?,
                        description: row.get(11)?,
                        question_count: row.get(12)?,
                        min_score: row.get(13)?,
                        max_score: row.get(14)?,
                        thresholds: serde_json::from_str(&row.get::<_, String>(15)?).map_err(
                            |e| {
                                error!(
                                    "Failed to deserialize thresholds in assessment history: {}",
                                    e
                                );
                                rusqlite::Error::InvalidColumnType(
                                    15,
                                    "thresholds".to_string(),
                                    rusqlite::types::Type::Text,
                                )
                            },
                        )?,
                    },
                    responses,
                    total_score: row.get(3)?,
                    severity_level: row.get(4)?,
                    completed_at: row.get(5)?,
                    notes: row.get(6)?,
                    status: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(responses)
    }

    /// Get a single assessment response by ID
    pub fn get_assessment_response(&self, id: i32) -> Result<AssessmentResponse, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let result = conn.query_row(
            "SELECT resp.id, resp.assessment_type_id, resp.responses, resp.total_score, resp.severity_level,
                    strftime('%Y-%m-%d %H:%M:%S', resp.completed_at) as completed_at, resp.notes, resp.status,
                    atype.id, atype.code, atype.name, atype.description, atype.question_count, atype.min_score, atype.max_score, atype.thresholds
             FROM assessment_responses AS resp
             JOIN assessment_types AS atype ON resp.assessment_type_id = atype.id
             WHERE resp.id = ?",
            [id],
            |row| {
                let responses_json: String = row.get(2)?;
                let responses: Vec<i32> = serde_json::from_str(&responses_json)
                    .map_err(|e| {
                        error!("Failed to deserialize assessment responses: {}", e);
                        rusqlite::Error::InvalidColumnType(
                            2,
                            "responses".to_string(),
                            rusqlite::types::Type::Text
                        )
                    })?;

                Ok(AssessmentResponse {
                    id: row.get(0)?,
                    assessment_type: AssessmentType {
                        id: row.get(8)?,
                        code: row.get(9)?,
                        name: row.get(10)?,
                        description: row.get(11)?,
                        question_count: row.get(12)?,
                        min_score: row.get(13)?,
                        max_score: row.get(14)?,
                        thresholds: serde_json::from_str(&row.get::<_, String>(15)?)
                            .map_err(|e| {
                                error!("Failed to deserialize thresholds in assessment history: {}", e);
                                rusqlite::Error::InvalidColumnType(
                                    15,
                                    "thresholds".to_string(),
                                    rusqlite::types::Type::Text
                                )
                            })?,
                    },
                    responses,
                    total_score: row.get(3)?,
                    severity_level: row.get(4)?,
                    completed_at: row.get(5)?,
                    notes: row.get(6)?,
                    status: row.get(7)?,
                })
            },
        );

        match result {
            Ok(response) => Ok(response),
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(AssessmentError::NotFound(id)),
            Err(e) => Err(AssessmentError::Database(e)),
        }
    }

    /// Delete an assessment response
    pub fn delete_assessment(&self, id: i32) -> Result<(), AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        conn.execute("DELETE FROM assessment_responses WHERE id = ?", [id])?;

        info!(assessment_id = id, "Deleted assessment");

        Ok(())
    }

    /// Count assessment responses for a given assessment type (for defensive deletion)
    pub fn count_assessment_responses(
        &self,
        assessment_type_id: i32,
    ) -> Result<i32, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        self.count_assessment_responses_with_conn(&conn, assessment_type_id)
    }

    /// Helper: Count assessment responses with provided connection
    fn count_assessment_responses_with_conn(
        &self,
        conn: &rusqlite::Connection,
        assessment_type_id: i32,
    ) -> Result<i32, AssessmentError> {
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM assessment_responses WHERE assessment_type_id = ?",
            [assessment_type_id],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Count assessment schedules for a given assessment type (for defensive deletion)
    pub fn count_assessment_schedules(
        &self,
        assessment_type_id: i32,
    ) -> Result<i32, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        self.count_assessment_schedules_with_conn(&conn, assessment_type_id)
    }

    /// Helper: Count assessment schedules with provided connection
    fn count_assessment_schedules_with_conn(
        &self,
        conn: &rusqlite::Connection,
        assessment_type_id: i32,
    ) -> Result<i32, AssessmentError> {
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM assessment_schedules WHERE assessment_type_id = ?",
            [assessment_type_id],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Delete an assessment type with defensive checks (prevents deletion if children exist)
    pub fn delete_assessment_type(&self, id: i32) -> Result<(), AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        self.delete_assessment_type_with_conn(&conn, id)
    }

    /// Helper: Delete assessment type with provided connection
    /// Uses single lock acquisition for atomic operation (prevents race conditions)
    fn delete_assessment_type_with_conn(
        &self,
        conn: &rusqlite::Connection,
        id: i32,
    ) -> Result<(), AssessmentError> {
        // Count child records atomically within same lock
        let response_count = self.count_assessment_responses_with_conn(conn, id)?;
        let schedule_count = self.count_assessment_schedules_with_conn(conn, id)?;

        // Block deletion if children exist
        if response_count > 0 || schedule_count > 0 {
            return Err(AssessmentError::HasChildren(format!(
                "{} assessment response(s) and {} schedule(s) exist. Delete or export data first.",
                response_count, schedule_count
            )));
        }

        // Safe to delete - no children
        conn.execute("DELETE FROM assessment_types WHERE id = ?", [id])?;

        info!(
            assessment_type_id = id,
            had_responses = response_count > 0,
            had_schedules = schedule_count > 0,
            "Deleted assessment type"
        );

        Ok(())
    }
}

// Trait implementation for testing with mocks
use super::repository_trait::AssessmentRepositoryTrait;

impl AssessmentRepositoryTrait for AssessmentRepository {
    fn save_assessment(
        &self,
        assessment_type_id: i32,
        responses: Vec<i32>,
        total_score: i32,
        severity_level: String,
        notes: Option<String>,
        status: &str,
    ) -> Result<i32, AssessmentError> {
        self.save_assessment(
            assessment_type_id,
            &responses,
            total_score,
            &severity_level,
            notes,
            status,
        )
    }

    fn get_assessment_type_by_code(&self, code: String) -> Result<AssessmentType, AssessmentError> {
        self.get_assessment_type_by_code(&code)
    }

    fn get_assessment_response(&self, id: i32) -> Result<AssessmentResponse, AssessmentError> {
        self.get_assessment_response(id)
    }

    fn delete_assessment(&self, id: i32) -> Result<(), AssessmentError> {
        self.delete_assessment(id)
    }

    fn delete_assessment_type(&self, id: i32) -> Result<(), AssessmentError> {
        self.delete_assessment_type(id)
    }
}
