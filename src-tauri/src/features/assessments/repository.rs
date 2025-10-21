// Assessment repository - database access layer
use super::models::{AssessmentType, AssessmentResponse, AssessmentError};
use crate::db::Database;
use std::sync::Arc;
use tracing::error;

/// Minimum limit for query results
const MIN_QUERY_LIMIT: i32 = 1;

/// Maximum limit for query results to prevent excessive memory usage
const MAX_QUERY_LIMIT: i32 = 1000;

pub struct AssessmentRepository {
    db: Arc<Database>,
}

impl AssessmentRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Save a completed assessment
    pub fn save_assessment(
        &self,
        assessment_type_id: i32,
        responses: &[i32],
        total_score: i32,
        severity_level: &str,
        notes: Option<String>,
    ) -> Result<i32, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock()
            .map_err(|_| AssessmentError::LockPoisoned)?;

        let responses_json = serde_json::to_string(responses)
            .map_err(|e| AssessmentError::InvalidResponse(format!("Failed to serialize responses: {}", e)))?;

        // Begin transaction for data consistency
        conn.execute("BEGIN TRANSACTION", [])?;

        let result = conn.query_row(
            "INSERT INTO assessment_responses (assessment_type_id, responses, total_score, severity_level, notes)
             VALUES (?, ?, ?, ?, ?)
             RETURNING id",
            [
                &assessment_type_id as &dyn duckdb::ToSql,
                &responses_json as &dyn duckdb::ToSql,
                &total_score as &dyn duckdb::ToSql,
                &severity_level as &dyn duckdb::ToSql,
                &notes as &dyn duckdb::ToSql,
            ],
            |row| row.get(0)
        );

        match result {
            Ok(id) => {
                conn.execute("COMMIT", [])?;
                Ok(id)
            }
            Err(e) => {
                // Rollback on error
                let _ = conn.execute("ROLLBACK", []);
                Err(AssessmentError::Database(e))
            }
        }
    }

    /// Get all assessment types
    pub fn get_assessment_types(&self) -> Result<Vec<AssessmentType>, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock()
            .map_err(|_| AssessmentError::LockPoisoned)?;

        let mut stmt = conn.prepare(
            "SELECT id, code, name, description, question_count, min_score, max_score, thresholds
             FROM assessment_types
             ORDER BY id"
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
                    thresholds: serde_json::from_str(&row.get::<_, String>(7)?)
                        .map_err(|e| {
                            error!("Failed to deserialize assessment type thresholds: {}", e);
                            duckdb::Error::InvalidColumnType(
                                7,
                                "thresholds".to_string(),
                                duckdb::types::Type::Text
                            )
                        })?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(types)
    }

    /// Get assessment type by code
    pub fn get_assessment_type_by_code(&self, code: &str) -> Result<AssessmentType, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock()
            .map_err(|_| AssessmentError::LockPoisoned)?;

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
                    thresholds: serde_json::from_str(&row.get::<_, String>(7)?)
                        .map_err(|e| {
                            error!("Failed to deserialize assessment type thresholds: {}", e);
                            duckdb::Error::InvalidColumnType(
                                7,
                                "thresholds".to_string(),
                                duckdb::types::Type::Text
                            )
                        })?,
                })
            },
        );

        match result {
            Ok(assessment_type) => Ok(assessment_type),
            Err(duckdb::Error::QueryReturnedNoRows) => {
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
        let conn = conn.lock()
            .map_err(|_| AssessmentError::LockPoisoned)?;

        let mut query = String::from(
            "SELECT ar.id, ar.assessment_type_id, ar.responses, ar.total_score, ar.severity_level,
                    ar.completed_at, ar.notes,
                    at.id, at.code, at.name, at.description, at.question_count, at.min_score, at.max_score, at.thresholds
             FROM assessment_responses ar
             JOIN assessment_types at ON ar.assessment_type_id = at.id
             WHERE 1=1"
        );

        if assessment_type_code.is_some() {
            query.push_str(" AND at.code = ?");
        }
        if from_date.is_some() {
            query.push_str(" AND ar.completed_at >= ?");
        }
        if to_date.is_some() {
            query.push_str(" AND ar.completed_at <= ?");
        }

        query.push_str(" ORDER BY ar.completed_at DESC");

        if let Some(lim) = limit {
            // Enforce reasonable bounds to prevent excessive queries
            let safe_limit = lim.clamp(MIN_QUERY_LIMIT, MAX_QUERY_LIMIT);
            query.push_str(&format!(" LIMIT {}", safe_limit));
        }

        let mut stmt = conn.prepare(&query)?;

        // Build params dynamically
        let mut params: Vec<&dyn duckdb::ToSql> = Vec::new();
        if let Some(code) = &assessment_type_code {
            params.push(code);
        }
        if let Some(from) = &from_date {
            params.push(from);
        }
        if let Some(to) = &to_date {
            params.push(to);
        }

        let responses = stmt
            .query_map(params.as_slice(), |row| {
                let responses_json: String = row.get(2)?;
                let responses: Vec<i32> = serde_json::from_str(&responses_json)
                    .map_err(|e| {
                        error!("Failed to deserialize assessment responses: {}", e);
                        duckdb::Error::InvalidColumnType(
                            2,
                            "responses".to_string(),
                            duckdb::types::Type::Text
                        )
                    })?;

                Ok(AssessmentResponse {
                    id: row.get(0)?,
                    assessment_type: AssessmentType {
                        id: row.get(7)?,
                        code: row.get(8)?,
                        name: row.get(9)?,
                        description: row.get(10)?,
                        question_count: row.get(11)?,
                        min_score: row.get(12)?,
                        max_score: row.get(13)?,
                        thresholds: serde_json::from_str(&row.get::<_, String>(14)?)
                            .map_err(|e| {
                                error!("Failed to deserialize thresholds in assessment history: {}", e);
                                duckdb::Error::InvalidColumnType(
                                    14,
                                    "thresholds".to_string(),
                                    duckdb::types::Type::Text
                                )
                            })?,
                    },
                    responses,
                    total_score: row.get(3)?,
                    severity_level: row.get(4)?,
                    completed_at: row.get(5)?,
                    notes: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(responses)
    }

    /// Get a single assessment response by ID
    pub fn get_assessment_response(&self, id: i32) -> Result<AssessmentResponse, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock()
            .map_err(|_| AssessmentError::LockPoisoned)?;

        let result = conn.query_row(
            "SELECT ar.id, ar.assessment_type_id, ar.responses, ar.total_score, ar.severity_level,
                    ar.completed_at, ar.notes,
                    at.id, at.code, at.name, at.description, at.question_count, at.min_score, at.max_score, at.thresholds
             FROM assessment_responses ar
             JOIN assessment_types at ON ar.assessment_type_id = at.id
             WHERE ar.id = ?",
            [id],
            |row| {
                let responses_json: String = row.get(2)?;
                let responses: Vec<i32> = serde_json::from_str(&responses_json)
                    .map_err(|e| {
                        error!("Failed to deserialize assessment responses: {}", e);
                        duckdb::Error::InvalidColumnType(
                            2,
                            "responses".to_string(),
                            duckdb::types::Type::Text
                        )
                    })?;

                Ok(AssessmentResponse {
                    id: row.get(0)?,
                    assessment_type: AssessmentType {
                        id: row.get(7)?,
                        code: row.get(8)?,
                        name: row.get(9)?,
                        description: row.get(10)?,
                        question_count: row.get(11)?,
                        min_score: row.get(12)?,
                        max_score: row.get(13)?,
                        thresholds: serde_json::from_str(&row.get::<_, String>(14)?)
                            .map_err(|e| {
                                error!("Failed to deserialize thresholds in assessment history: {}", e);
                                duckdb::Error::InvalidColumnType(
                                    14,
                                    "thresholds".to_string(),
                                    duckdb::types::Type::Text
                                )
                            })?,
                    },
                    responses,
                    total_score: row.get(3)?,
                    severity_level: row.get(4)?,
                    completed_at: row.get(5)?,
                    notes: row.get(6)?,
                })
            },
        );

        match result {
            Ok(response) => Ok(response),
            Err(duckdb::Error::QueryReturnedNoRows) => {
                Err(AssessmentError::NotFound(id))
            }
            Err(e) => Err(AssessmentError::Database(e)),
        }
    }

    /// Delete an assessment response
    pub fn delete_assessment(&self, id: i32) -> Result<(), AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock()
            .map_err(|_| AssessmentError::LockPoisoned)?;

        conn.execute("DELETE FROM assessment_responses WHERE id = ?", [id])?;

        Ok(())
    }
}
