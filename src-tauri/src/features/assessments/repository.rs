// Assessment repository - database access layer
use super::models::{AssessmentError, AssessmentResponse, AssessmentType};
use crate::db::Database;
use crate::types::{AssessmentStatus, SeverityLevel};
use crate::utils::sanitize_optional_text;
use crate::MAX_QUERY_LIMIT;
use rusqlite::Row;
use std::sync::Arc;
use tracing::{error, info};

/// Minimum limit for query results
const MIN_QUERY_LIMIT: i32 = 1;

// ============================================================================
// Row Mapping Helpers - Reduce code duplication across query methods
// ============================================================================

/// Maps a database row to AssessmentType.
///
/// Expected column order: id, code, name, description, question_count, min_score, max_score, thresholds
/// Offset parameter allows using this helper when AssessmentType columns start at a different index
/// (e.g., in JOINed queries where response columns come first).
fn map_assessment_type_row(row: &Row, offset: usize) -> rusqlite::Result<AssessmentType> {
    Ok(AssessmentType {
        id: row.get(offset)?,
        code: row.get(offset + 1)?,
        name: row.get(offset + 2)?,
        description: row.get(offset + 3)?,
        question_count: row.get(offset + 4)?,
        min_score: row.get(offset + 5)?,
        max_score: row.get(offset + 6)?,
        thresholds: serde_json::from_str(&row.get::<_, String>(offset + 7)?).map_err(|e| {
            error!("Failed to deserialize assessment type thresholds: {}", e);
            rusqlite::Error::InvalidColumnType(
                offset + 7,
                "thresholds".to_string(),
                rusqlite::types::Type::Text,
            )
        })?,
    })
}

/// Maps a database row to AssessmentResponse with embedded AssessmentType.
///
/// Expected column order:
/// - Response fields (0-7): id, assessment_type_id, responses, total_score, severity_level, completed_at, notes, status
/// - Assessment type fields (8-15): id, code, name, description, question_count, min_score, max_score, thresholds
fn map_assessment_response_row(row: &Row) -> rusqlite::Result<AssessmentResponse> {
    // Parse responses JSON
    let responses_json: String = row.get(2)?;
    let responses: Vec<i32> = serde_json::from_str(&responses_json).map_err(|e| {
        error!("Failed to deserialize assessment responses: {}", e);
        rusqlite::Error::InvalidColumnType(2, "responses".to_string(), rusqlite::types::Type::Text)
    })?;

    Ok(AssessmentResponse {
        id: row.get(0)?,
        assessment_type: map_assessment_type_row(row, 8)?,
        responses,
        total_score: row.get(3)?,
        severity_level: row.get(4)?,
        completed_at: row.get(5)?,
        notes: row.get(6)?,
        status: row.get(7)?,
    })
}

pub struct AssessmentRepository {
    db: Arc<Database>,
}

impl AssessmentRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Save an assessment (completed or draft)
    ///
    /// # Draft Behavior (One-Draft-Per-Type Constraint)
    ///
    /// Only one draft can exist per assessment type at a time. When saving a draft:
    /// - If a draft already exists for this assessment type, it is **updated** (not duplicated)
    /// - If no draft exists, a new draft record is created
    /// - This allows users to resume their most recent draft for each assessment type
    ///
    /// # Completed Behavior
    ///
    /// Completed assessments always create a **new record** to preserve historical data.
    /// Multiple completed assessments can exist for the same assessment type.
    ///
    /// # Arguments
    ///
    /// * `assessment_type_id` - The ID of the assessment type (PHQ-9, GAD-7, etc.)
    /// * `responses` - Array of response values (may contain `UNANSWERED` (-1) for drafts)
    /// * `total_score` - Calculated score (partial for drafts, full for completed)
    /// * `severity_level` - Calculated severity (Unknown for drafts with partial data)
    /// * `notes` - Optional user notes
    /// * `status` - Draft or Completed
    ///
    /// # Returns
    ///
    /// The ID of the saved assessment (same ID if updating existing draft, new ID otherwise)
    pub fn save_assessment(
        &self,
        assessment_type_id: i32,
        responses: &[i32],
        total_score: i32,
        severity_level: SeverityLevel,
        notes: Option<String>,
        status: AssessmentStatus,
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

        let status_str = status.as_str();
        let severity_str = severity_level.as_str();

        let id = if status == AssessmentStatus::Draft {
            // For drafts: use atomic UPSERT to prevent TOCTOU race condition
            // The partial unique index (idx_one_draft_per_type) ensures only one draft per assessment type
            tx.query_row(
                "INSERT INTO assessment_responses (assessment_type_id, responses, total_score, severity_level, notes, status)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT (assessment_type_id) WHERE status = 'draft'
                 DO UPDATE SET
                     responses = excluded.responses,
                     total_score = excluded.total_score,
                     severity_level = excluded.severity_level,
                     notes = excluded.notes,
                     completed_at = datetime('now')
                 RETURNING id",
                rusqlite::params![
                    &assessment_type_id,
                    &responses_json,
                    &total_score,
                    &severity_str,
                    &notes,
                    &status_str,
                ],
                |row| row.get(0),
            )?
        } else {
            // For completed: always insert new record (historical data)
            tx.query_row(
                "INSERT INTO assessment_responses (assessment_type_id, responses, total_score, severity_level, notes, status)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 RETURNING id",
                rusqlite::params![
                    &assessment_type_id,
                    &responses_json,
                    &total_score,
                    &severity_str,
                    &notes,
                    &status_str,
                ],
                |row| row.get(0),
            )?
        };

        // Commit transaction - automatic rollback via Drop on error/panic
        tx.commit().map_err(AssessmentError::Database)?;

        info!(
            assessment_id = id,
            assessment_type_id = assessment_type_id,
            total_score = total_score,
            severity_level = severity_str,
            has_notes = notes.is_some(),
            status = status_str,
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
            .query_map([], |row| map_assessment_type_row(row, 0))?
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
            |row| map_assessment_type_row(row, 0),
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
        // Design choice: Using clamp() for correction rather than validation error
        // This provides better UX (automatically corrects invalid limits) rather than rejecting requests
        let safe_limit;
        if let Some(lim) = limit {
            safe_limit = lim.clamp(MIN_QUERY_LIMIT, MAX_QUERY_LIMIT);
            if safe_limit != lim {
                tracing::warn!(
                    requested_limit = lim,
                    actual_limit = safe_limit,
                    min = MIN_QUERY_LIMIT,
                    max = MAX_QUERY_LIMIT,
                    "Query limit clamped to valid range"
                );
            }
            query.push_str(" LIMIT ?");
            params.push(&safe_limit);
        }

        let mut stmt = conn.prepare(&query)?;

        let responses = stmt
            .query_map(params.as_slice(), map_assessment_response_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(responses)
    }

    /// Get all draft assessments (not completed)
    pub fn get_draft_assessments(&self) -> Result<Vec<AssessmentResponse>, AssessmentError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let mut stmt = conn.prepare(
            "SELECT resp.id, resp.assessment_type_id, resp.responses, resp.total_score, resp.severity_level,
                    strftime('%Y-%m-%d %H:%M:%S', resp.completed_at) as completed_at, resp.notes, resp.status,
                    atype.id, atype.code, atype.name, atype.description, atype.question_count, atype.min_score, atype.max_score, atype.thresholds
             FROM assessment_responses AS resp
             JOIN assessment_types AS atype ON resp.assessment_type_id = atype.id
             WHERE resp.status = 'draft'
             ORDER BY resp.completed_at DESC"
        )?;

        let responses = stmt
            .query_map([], map_assessment_response_row)?
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
            map_assessment_response_row,
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
        severity_level: SeverityLevel,
        notes: Option<String>,
        status: AssessmentStatus,
    ) -> Result<i32, AssessmentError> {
        self.save_assessment(
            assessment_type_id,
            &responses,
            total_score,
            severity_level,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::assessment::{AssessmentStatus, SeverityLevel};
    use tempfile::TempDir;

    fn setup_test_repo() -> (AssessmentRepository, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db = Arc::new(
            Database::new(temp_dir.path().to_path_buf()).expect("Failed to create database"),
        );
        (AssessmentRepository::new(db), temp_dir)
    }

    #[test]
    fn test_save_assessment_as_draft() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get an assessment type to work with
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let phq9 = assessment_types
            .iter()
            .find(|at| at.code == "PHQ9")
            .expect("PHQ9 not found");

        // Save a draft assessment
        let responses = vec![1, 2, 1, 0, 1, 2, 1, 0, 1];
        let total_score = 10;
        let severity_level = SeverityLevel::Mild;
        let notes = Some("Test draft notes".to_string());

        let id = repo
            .save_assessment(
                phq9.id,
                &responses,
                total_score,
                severity_level,
                notes,
                AssessmentStatus::Draft,
            )
            .expect("Failed to save draft assessment");

        // Retrieve and verify
        let saved = repo
            .get_assessment_response(id)
            .expect("Failed to get assessment");

        assert_eq!(saved.status, AssessmentStatus::Draft);
        assert_eq!(saved.total_score, total_score);
        assert_eq!(saved.severity_level, SeverityLevel::Mild);
        assert_eq!(saved.responses, responses);
        assert_eq!(saved.notes, Some("Test draft notes".to_string()));
    }

    #[test]
    fn test_save_assessment_as_completed() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get an assessment type
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let gad7 = assessment_types
            .iter()
            .find(|at| at.code == "GAD7")
            .expect("GAD7 not found");

        // Save a completed assessment
        let responses = vec![2, 2, 3, 2, 1, 2, 3];
        let total_score = 15;
        let severity_level = SeverityLevel::Moderate;

        let id = repo
            .save_assessment(
                gad7.id,
                &responses,
                total_score,
                severity_level,
                None,
                AssessmentStatus::Completed,
            )
            .expect("Failed to save completed assessment");

        // Retrieve and verify
        let saved = repo
            .get_assessment_response(id)
            .expect("Failed to get assessment");

        assert_eq!(saved.status, AssessmentStatus::Completed);
        assert_eq!(saved.total_score, total_score);
        assert_eq!(saved.severity_level, SeverityLevel::Moderate);
        assert_eq!(saved.responses, responses);
        assert_eq!(saved.notes, None);
    }

    #[test]
    fn test_save_draft_twice_updates_same_record() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get an assessment type
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let phq9 = assessment_types
            .iter()
            .find(|at| at.code == "PHQ9")
            .expect("PHQ9 not found");

        // Save first draft
        let id1 = repo
            .save_assessment(
                phq9.id,
                &vec![1, 2, 1, 2, 1, 2, 1, 2, 1],
                13,
                SeverityLevel::Mild,
                Some("First draft".to_string()),
                AssessmentStatus::Draft,
            )
            .expect("Failed to save first draft");

        // Save second draft for same assessment type (should update, not create new)
        let id2 = repo
            .save_assessment(
                phq9.id,
                &vec![3, 3, 3, 3, 3, 3, 3, 3, 3],
                27,
                SeverityLevel::Severe,
                Some("Updated draft".to_string()),
                AssessmentStatus::Draft,
            )
            .expect("Failed to save second draft");

        // IDs should match - same record was updated
        assert_eq!(
            id1, id2,
            "Draft should update existing record, not create new"
        );

        // Verify only one draft exists for this assessment type
        let drafts = repo.get_draft_assessments().expect("Failed to get drafts");
        assert_eq!(drafts.len(), 1, "Should have exactly one draft");

        // Verify the draft has the updated values
        let draft = &drafts[0];
        assert_eq!(draft.id, id1);
        assert_eq!(draft.responses, vec![3, 3, 3, 3, 3, 3, 3, 3, 3]);
        assert_eq!(draft.total_score, 27);
        assert_eq!(draft.severity_level, SeverityLevel::Severe);
        assert_eq!(draft.notes, Some("Updated draft".to_string()));
    }

    #[test]
    fn test_get_draft_assessments_returns_only_drafts() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get assessment types
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let phq9 = assessment_types
            .iter()
            .find(|at| at.code == "PHQ9")
            .expect("PHQ9 not found");
        let gad7 = assessment_types
            .iter()
            .find(|at| at.code == "GAD7")
            .expect("GAD7 not found");

        // Create a mix of draft and completed assessments
        // Draft 1
        repo.save_assessment(
            phq9.id,
            &vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
            9,
            SeverityLevel::Mild,
            Some("Draft 1".to_string()),
            AssessmentStatus::Draft,
        )
        .expect("Failed to save draft 1");

        // Completed 1
        repo.save_assessment(
            phq9.id,
            &vec![2, 2, 2, 2, 2, 2, 2, 2, 2],
            18,
            SeverityLevel::ModeratelySevere,
            Some("Completed 1".to_string()),
            AssessmentStatus::Completed,
        )
        .expect("Failed to save completed 1");

        // Draft 2
        repo.save_assessment(
            gad7.id,
            &vec![1, 1, 1, 1, 1, 1, 1],
            7,
            SeverityLevel::Mild,
            Some("Draft 2".to_string()),
            AssessmentStatus::Draft,
        )
        .expect("Failed to save draft 2");

        // Completed 2
        repo.save_assessment(
            gad7.id,
            &vec![3, 3, 3, 3, 3, 3, 3],
            21,
            SeverityLevel::Severe,
            Some("Completed 2".to_string()),
            AssessmentStatus::Completed,
        )
        .expect("Failed to save completed 2");

        // Draft 3 for PHQ9 - should UPDATE Draft 1 (same assessment type)
        repo.save_assessment(
            phq9.id,
            &vec![0, 0, 1, 1, 0, 1, 0, 0, 1],
            4,
            SeverityLevel::Minimal,
            Some("Draft 3 (updated PHQ9)".to_string()),
            AssessmentStatus::Draft,
        )
        .expect("Failed to save draft 3");

        // Get only drafts
        let drafts = repo.get_draft_assessments().expect("Failed to get drafts");

        // Should have 2 drafts (PHQ9 was updated, not duplicated)
        assert_eq!(
            drafts.len(),
            2,
            "Should return exactly 2 drafts (PHQ9 updated, GAD7 separate)"
        );

        // Verify all returned assessments are drafts
        for draft in &drafts {
            assert_eq!(
                draft.status,
                AssessmentStatus::Draft,
                "All returned assessments should be drafts"
            );
        }

        // Verify notes - Draft 3 should have replaced Draft 1 for PHQ9
        let notes: Vec<Option<String>> = drafts.iter().map(|d| d.notes.clone()).collect();
        assert!(
            notes.contains(&Some("Draft 3 (updated PHQ9)".to_string())),
            "Should have updated PHQ9 draft"
        );
        assert!(
            notes.contains(&Some("Draft 2".to_string())),
            "Should have GAD7 draft"
        );
        assert!(
            !notes.contains(&Some("Draft 1".to_string())),
            "Original PHQ9 draft should be replaced"
        );
        assert!(!notes.contains(&Some("Completed 1".to_string())));
        assert!(!notes.contains(&Some("Completed 2".to_string())));
    }

    #[test]
    fn test_get_draft_assessments_empty_when_no_drafts() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get assessment type
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let phq9 = assessment_types
            .iter()
            .find(|at| at.code == "PHQ9")
            .expect("PHQ9 not found");

        // Create only completed assessments
        repo.save_assessment(
            phq9.id,
            &vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
            9,
            SeverityLevel::Mild,
            None,
            AssessmentStatus::Completed,
        )
        .expect("Failed to save completed assessment");

        // Get drafts should return empty
        let drafts = repo.get_draft_assessments().expect("Failed to get drafts");

        assert_eq!(drafts.len(), 0, "Should return no drafts");
    }

    #[test]
    fn test_get_assessment_history_includes_status() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get assessment type
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let phq9 = assessment_types
            .iter()
            .find(|at| at.code == "PHQ9")
            .expect("PHQ9 not found");

        // Create both draft and completed
        repo.save_assessment(
            phq9.id,
            &vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
            9,
            SeverityLevel::Mild,
            None,
            AssessmentStatus::Draft,
        )
        .expect("Failed to save draft");

        repo.save_assessment(
            phq9.id,
            &vec![2, 2, 2, 2, 2, 2, 2, 2, 2],
            18,
            SeverityLevel::ModeratelySevere,
            None,
            AssessmentStatus::Completed,
        )
        .expect("Failed to save completed");

        // Get history (should include both)
        let history = repo
            .get_assessment_history(Some("PHQ9".to_string()), None, None, None)
            .expect("Failed to get history");

        assert_eq!(history.len(), 2, "History should include both assessments");

        // Verify both statuses are present
        let statuses: Vec<AssessmentStatus> = history.iter().map(|h| h.status).collect();
        assert!(statuses.contains(&AssessmentStatus::Draft));
        assert!(statuses.contains(&AssessmentStatus::Completed));
    }

    #[test]
    fn test_draft_with_partial_responses() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get assessment type
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let phq9 = assessment_types
            .iter()
            .find(|at| at.code == "PHQ9")
            .expect("PHQ9 not found");

        // Save a draft with some unanswered questions (-1 indicates not answered)
        let responses = vec![1, 2, -1, -1, 1, -1, 1, -1, -1];
        let total_score = 5; // Only count answered questions
        let severity_level = SeverityLevel::Minimal;

        let id = repo
            .save_assessment(
                phq9.id,
                &responses,
                total_score,
                severity_level,
                Some("Partially completed".to_string()),
                AssessmentStatus::Draft,
            )
            .expect("Failed to save partial draft");

        // Retrieve and verify
        let saved = repo
            .get_assessment_response(id)
            .expect("Failed to get assessment");

        assert_eq!(saved.status, AssessmentStatus::Draft);
        assert_eq!(saved.responses, responses);
        assert_eq!(
            saved.responses.iter().filter(|&&r| r == -1).count(),
            5,
            "Should have 5 unanswered questions"
        );
        assert_eq!(
            saved.responses.iter().filter(|&&r| r != -1).count(),
            4,
            "Should have 4 answered questions"
        );
    }

    #[test]
    fn test_draft_updates_instead_of_creating_duplicate() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get assessment type
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let phq9 = assessment_types
            .iter()
            .find(|at| at.code == "PHQ9")
            .expect("PHQ9 not found");

        // Save first draft
        let id1 = repo
            .save_assessment(
                phq9.id,
                &vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
                3,
                SeverityLevel::Minimal,
                Some("First save".to_string()),
                AssessmentStatus::Draft,
            )
            .expect("Failed to save first draft");

        // Save second draft for same assessment type - should UPDATE, not INSERT
        let id2 = repo
            .save_assessment(
                phq9.id,
                &vec![2, 2, 2, 1, 1, 1, 0, 0, 0],
                9,
                SeverityLevel::Mild,
                Some("Second save".to_string()),
                AssessmentStatus::Draft,
            )
            .expect("Failed to save second draft");

        // Should be same ID (updated existing draft)
        assert_eq!(id1, id2, "Draft should be updated, not duplicated");

        // Verify updated values
        let saved = repo
            .get_assessment_response(id1)
            .expect("Failed to get assessment");
        assert_eq!(saved.total_score, 9);
        assert_eq!(saved.notes, Some("Second save".to_string()));
        assert_eq!(saved.responses, vec![2, 2, 2, 1, 1, 1, 0, 0, 0]);

        // Verify only one draft exists
        let drafts = repo.get_draft_assessments().expect("Failed to get drafts");
        assert_eq!(
            drafts.len(),
            1,
            "Should have exactly 1 draft (updated, not duplicated)"
        );
    }

    #[test]
    fn test_completed_assessments_always_create_new_records() {
        let (repo, _temp_dir) = setup_test_repo();

        // Get assessment type
        let assessment_types = repo
            .get_assessment_types()
            .expect("Failed to get assessment types");
        let gad7 = assessment_types
            .iter()
            .find(|at| at.code == "GAD7")
            .expect("GAD7 not found");

        // Save first completed assessment
        let id1 = repo
            .save_assessment(
                gad7.id,
                &vec![1, 1, 1, 1, 1, 1, 1],
                7,
                SeverityLevel::Mild,
                Some("First assessment".to_string()),
                AssessmentStatus::Completed,
            )
            .expect("Failed to save first completed");

        // Save second completed assessment for same type - should INSERT new record
        let id2 = repo
            .save_assessment(
                gad7.id,
                &vec![2, 2, 2, 2, 2, 2, 2],
                14,
                SeverityLevel::Moderate,
                Some("Second assessment".to_string()),
                AssessmentStatus::Completed,
            )
            .expect("Failed to save second completed");

        // Should be different IDs (new records for historical data)
        assert_ne!(
            id1, id2,
            "Completed assessments should create separate records"
        );

        // Verify both exist in history
        let history = repo
            .get_assessment_history(Some("GAD7".to_string()), None, None, None)
            .expect("Failed to get history");
        assert_eq!(
            history.len(),
            2,
            "Should have 2 separate completed assessments"
        );
    }
}
