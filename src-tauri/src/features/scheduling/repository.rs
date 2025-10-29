// Scheduling repository (User Story 6)
// T160-T164: Database operations for schedules

use std::str::FromStr;
use std::sync::Arc;

use rusqlite::params;

use crate::db::Database;

use super::models::{
    AssessmentSchedule, CreateScheduleRequest, ScheduleFrequency, SchedulingError,
    UpdateScheduleRequest,
};

pub struct SchedulingRepository {
    db: Arc<Database>,
}

impl SchedulingRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// T160: Create a new assessment schedule
    pub fn create_schedule(
        &self,
        request: &CreateScheduleRequest,
    ) -> Result<AssessmentSchedule, SchedulingError> {
        let conn = self.db.get_connection();
        let mut conn = conn.lock();

        // Use IMMEDIATE transaction to acquire write lock immediately
        // This ensures atomicity across all operations (check, insert, fetch)
        let tx = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(SchedulingError::Database)?;

        // Verify assessment type exists
        let assessment_type_exists: bool = tx
            .query_row(
                "SELECT COUNT(*) FROM assessment_types WHERE id = ?",
                params![request.assessment_type_id],
                |row| row.get::<_, i32>(0).map(|count| count > 0),
            )
            .unwrap_or(false);

        if !assessment_type_exists {
            return Err(SchedulingError::NotFound(request.assessment_type_id));
        }

        // Insert schedule
        tx.execute(
            "INSERT INTO assessment_schedules
             (assessment_type_id, frequency, time_of_day, day_of_week, day_of_month, enabled, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            params![
                request.assessment_type_id,
                request.frequency.as_str(),
                request.time_of_day,
                request.day_of_week,
                request.day_of_month,
            ],
        )?;

        let schedule_id = tx.last_insert_rowid() as i32;

        // Fetch created schedule within transaction
        let schedule = tx
            .query_row(
                "SELECT s.id, s.assessment_type_id, a.code, a.name, s.frequency, s.time_of_day,
                        s.day_of_week, s.day_of_month, s.enabled, s.last_triggered_at,
                        s.created_at, s.updated_at
                 FROM assessment_schedules s
                 JOIN assessment_types a ON s.assessment_type_id = a.id
                 WHERE s.id = ?",
                params![schedule_id],
                |row| self.map_schedule_row(row),
            )
            .map_err(|_| SchedulingError::NotFound(schedule_id))?;

        // Commit transaction (auto-rollback on drop if not committed or on panic)
        tx.commit().map_err(SchedulingError::Database)?;

        Ok(schedule)
    }

    /// T161: Update an existing schedule
    pub fn update_schedule(
        &self,
        id: i32,
        request: &UpdateScheduleRequest,
    ) -> Result<AssessmentSchedule, SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Build dynamic update query using safe predefined clauses
        // Each clause is a constant string to prevent SQL injection
        const FREQUENCY_CLAUSE: &str = "frequency = ?";
        const TIME_CLAUSE: &str = "time_of_day = ?";
        const DAY_OF_WEEK_CLAUSE: &str = "day_of_week = ?";
        const DAY_OF_MONTH_CLAUSE: &str = "day_of_month = ?";
        const ENABLED_CLAUSE: &str = "enabled = ?";
        const UPDATED_AT_CLAUSE: &str = "updated_at = CURRENT_TIMESTAMP";

        let mut clauses = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref freq) = request.frequency {
            clauses.push(FREQUENCY_CLAUSE);
            params_vec.push(Box::new(freq.as_str().to_string()));
        }
        if let Some(ref time) = request.time_of_day {
            clauses.push(TIME_CLAUSE);
            params_vec.push(Box::new(time.clone()));
        }
        if let Some(day) = request.day_of_week {
            clauses.push(DAY_OF_WEEK_CLAUSE);
            params_vec.push(Box::new(day));
        }
        if let Some(day) = request.day_of_month {
            clauses.push(DAY_OF_MONTH_CLAUSE);
            params_vec.push(Box::new(day));
        }
        if let Some(enabled) = request.enabled {
            clauses.push(ENABLED_CLAUSE);
            params_vec.push(Box::new(enabled));
        }

        if clauses.is_empty() {
            // Nothing to update, just return current schedule
            return self.get_schedule_with_conn(&conn, id);
        }

        clauses.push(UPDATED_AT_CLAUSE);

        // Build query from safe constant clauses only
        let mut query = String::from("UPDATE assessment_schedules SET ");
        query.push_str(&clauses.join(", "));
        query.push_str(" WHERE id = ?");

        params_vec.push(Box::new(id));

        // Convert to slice of references
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|b| b.as_ref()).collect();

        let rows_affected = conn.execute(&query, params_refs.as_slice())?;

        if rows_affected == 0 {
            return Err(SchedulingError::NotFound(id));
        }

        self.get_schedule_with_conn(&conn, id)
    }

    /// T162: Delete a schedule
    pub fn delete_schedule(&self, id: i32) -> Result<(), SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let rows_affected =
            conn.execute("DELETE FROM assessment_schedules WHERE id = ?", params![id])?;

        if rows_affected == 0 {
            return Err(SchedulingError::NotFound(id));
        }

        Ok(())
    }

    /// T163: Get all schedules for the user
    pub fn get_schedules(
        &self,
        enabled_only: bool,
    ) -> Result<Vec<AssessmentSchedule>, SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();
        self.get_schedules_with_conn(&conn, enabled_only)
    }

    /// Get a single schedule by ID
    pub fn get_schedule(&self, id: i32) -> Result<AssessmentSchedule, SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        self.get_schedule_with_conn(&conn, id)
    }

    /// T164: Get schedules that are due for triggering
    pub fn get_due_schedules(&self) -> Result<Vec<AssessmentSchedule>, SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();
        self.get_due_schedules_with_conn(&conn)
    }

    /// Mark a schedule as triggered
    pub fn mark_triggered(&self, id: i32) -> Result<(), SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let rows_affected = conn.execute(
            "UPDATE assessment_schedules SET last_triggered_at = CURRENT_TIMESTAMP WHERE id = ?",
            params![id],
        )?;

        if rows_affected == 0 {
            return Err(SchedulingError::NotFound(id));
        }

        Ok(())
    }

    /// Mark multiple schedules as triggered in a single transaction
    /// More efficient than calling mark_triggered in a loop
    pub fn mark_multiple_triggered(&self, schedule_ids: &[i32]) -> Result<(), SchedulingError> {
        if schedule_ids.is_empty() {
            return Ok(());
        }

        let conn = self.db.get_connection();
        let mut conn = conn.lock();

        // Use IMMEDIATE transaction for batch updates
        let tx = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(SchedulingError::Database)?;

        // Use prepared statement caching for efficiency
        {
            let mut stmt = tx
                .prepare_cached(
                    "UPDATE assessment_schedules SET last_triggered_at = CURRENT_TIMESTAMP WHERE id = ?",
                )
                .map_err(SchedulingError::Database)?;

            for id in schedule_ids {
                stmt.execute(params![id])?;
            }
        } // Drop stmt before committing

        // Commit transaction (auto-rollback on drop if not committed)
        tx.commit().map_err(SchedulingError::Database)?;

        Ok(())
    }

    // Helper methods

    /// Get schedules that are due for triggering based on frequency and last trigger time.
    ///
    /// This method implements frequency-aware logic to prevent schedules from triggering
    /// too frequently. It uses SQLite's julianday function for accurate day calculations
    /// across all frequency types.
    ///
    /// # Frequency Logic
    ///
    /// - **Daily**: Triggers if last_triggered_at is NULL or on a different date than today
    ///   - Example: Schedule at 09:00, last triggered 2024-10-28 08:00 → triggers again on 2024-10-29
    ///
    /// - **Weekly**: Triggers if ≥7 days have elapsed since last_triggered_at
    ///   - Example: Last triggered 2024-10-22 → triggers again on/after 2024-10-29
    ///
    /// - **Biweekly**: Triggers if ≥14 days have elapsed since last_triggered_at
    ///   - Example: Last triggered 2024-10-15 → triggers again on/after 2024-10-29
    ///
    /// - **Monthly**: Triggers if DATE(last_triggered_at, '+1 month') ≤ DATE('now')
    ///   - Example: Last triggered 2024-09-28 → triggers again on/after 2024-10-28
    ///
    /// # Time-of-Day Filtering
    ///
    /// Only returns schedules where `time_of_day <= current_time` to prevent premature triggers.
    /// For example, if current time is 14:30, schedules set for 09:00 and 14:00 are returned,
    /// but schedules set for 15:00 are not.
    ///
    /// # Examples
    ///
    /// ```text
    /// Current date/time: 2024-10-29 14:30
    ///
    /// Schedule A (Daily, 09:00, last_triggered: 2024-10-28 09:05):
    ///   → RETURNED (different date, time passed)
    ///
    /// Schedule B (Weekly, 14:00, last_triggered: 2024-10-22 14:05):
    ///   → RETURNED (≥7 days elapsed, time passed)
    ///
    /// Schedule C (Daily, 15:00, last_triggered: 2024-10-28 15:05):
    ///   → NOT RETURNED (time hasn't arrived yet)
    ///
    /// Schedule D (Weekly, 09:00, last_triggered: 2024-10-27 09:05):
    ///   → NOT RETURNED (only 2 days elapsed, need 7)
    /// ```
    fn get_due_schedules_with_conn(
        &self,
        conn: &rusqlite::Connection,
    ) -> Result<Vec<AssessmentSchedule>, SchedulingError> {
        // Get current time - use SQLite's built-in datetime for consistency
        let now = chrono::Utc::now();
        let current_time = now.format("%H:%M").to_string();

        // Frequency-aware query to prevent schedules from triggering too frequently
        // Uses julianday for day calculations to handle all frequency types correctly
        let query = "SELECT s.id, s.assessment_type_id, a.code, a.name, s.frequency, s.time_of_day,
                            s.day_of_week, s.day_of_month, s.enabled, s.last_triggered_at,
                            s.created_at, s.updated_at
                     FROM assessment_schedules s
                     JOIN assessment_types a ON s.assessment_type_id = a.id
                     WHERE s.enabled = 1
                       AND s.time_of_day <= ?
                       AND (
                         s.last_triggered_at IS NULL OR
                         (s.frequency = 'daily' AND DATE(s.last_triggered_at) < DATE('now')) OR
                         (s.frequency = 'weekly' AND julianday('now') - julianday(s.last_triggered_at) >= 7) OR
                         (s.frequency = 'biweekly' AND julianday('now') - julianday(s.last_triggered_at) >= 14) OR
                         (s.frequency = 'monthly' AND DATE(s.last_triggered_at, '+1 month') <= DATE('now'))
                       )
                     ORDER BY s.time_of_day ASC";

        let mut stmt = conn.prepare(query)?;
        let schedules = stmt
            .query_map(params![current_time], |row| self.map_schedule_row(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(schedules)
    }

    fn get_schedules_with_conn(
        &self,
        conn: &rusqlite::Connection,
        enabled_only: bool,
    ) -> Result<Vec<AssessmentSchedule>, SchedulingError> {
        let query = if enabled_only {
            "SELECT s.id, s.assessment_type_id, a.code, a.name, s.frequency, s.time_of_day,
                    s.day_of_week, s.day_of_month, s.enabled, s.last_triggered_at,
                    s.created_at, s.updated_at
             FROM assessment_schedules s
             JOIN assessment_types a ON s.assessment_type_id = a.id
             WHERE s.enabled = 1
             ORDER BY s.created_at DESC"
        } else {
            "SELECT s.id, s.assessment_type_id, a.code, a.name, s.frequency, s.time_of_day,
                    s.day_of_week, s.day_of_month, s.enabled, s.last_triggered_at,
                    s.created_at, s.updated_at
             FROM assessment_schedules s
             JOIN assessment_types a ON s.assessment_type_id = a.id
             ORDER BY s.created_at DESC"
        };

        let mut stmt = conn.prepare(query)?;
        let schedules = stmt
            .query_map([], |row| self.map_schedule_row(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(schedules)
    }

    fn get_schedule_with_conn(
        &self,
        conn: &rusqlite::Connection,
        id: i32,
    ) -> Result<AssessmentSchedule, SchedulingError> {
        conn.query_row(
            "SELECT s.id, s.assessment_type_id, a.code, a.name, s.frequency, s.time_of_day,
                    s.day_of_week, s.day_of_month, s.enabled, s.last_triggered_at,
                    s.created_at, s.updated_at
             FROM assessment_schedules s
             JOIN assessment_types a ON s.assessment_type_id = a.id
             WHERE s.id = ?",
            params![id],
            |row| self.map_schedule_row(row),
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => SchedulingError::NotFound(id),
            other => SchedulingError::Database(other),
        })
    }

    fn map_schedule_row(&self, row: &rusqlite::Row) -> rusqlite::Result<AssessmentSchedule> {
        let frequency_str: String = row.get(4)?;
        let frequency = ScheduleFrequency::from_str(&frequency_str)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        Ok(AssessmentSchedule {
            id: row.get(0)?,
            assessment_type_id: row.get(1)?,
            assessment_type_code: row.get(2)?,
            assessment_type_name: row.get(3)?,
            frequency,
            time_of_day: row.get(5)?,
            day_of_week: row.get(6)?,
            day_of_month: row.get(7)?,
            enabled: row.get(8)?,
            last_triggered_at: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    }
}

// Trait implementation for testing with mocks
use super::repository_trait::SchedulingRepositoryTrait;

impl SchedulingRepositoryTrait for SchedulingRepository {
    fn create_schedule(
        &self,
        request: CreateScheduleRequest,
    ) -> Result<AssessmentSchedule, SchedulingError> {
        self.create_schedule(&request)
    }

    fn update_schedule(
        &self,
        id: i32,
        request: UpdateScheduleRequest,
    ) -> Result<AssessmentSchedule, SchedulingError> {
        self.update_schedule(id, &request)
    }

    fn delete_schedule(&self, id: i32) -> Result<(), SchedulingError> {
        self.delete_schedule(id)
    }
}
