// Scheduling repository (User Story 6)
// T160-T164: Database operations for schedules

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
        // Validate request
        request.validate()?;

        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| SchedulingError::LockPoisoned)?;

        // Verify assessment type exists
        let assessment_type_exists: bool = conn
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
        conn.execute(
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

        let schedule_id = conn.last_insert_rowid() as i32;

        // Return created schedule
        self.get_schedule_with_conn(&conn, schedule_id)
    }

    /// T161: Update an existing schedule
    pub fn update_schedule(
        &self,
        id: i32,
        request: &UpdateScheduleRequest,
    ) -> Result<AssessmentSchedule, SchedulingError> {
        // Validate request
        request.validate()?;

        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| SchedulingError::LockPoisoned)?;

        // Build dynamic update query
        let mut updates = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref freq) = request.frequency {
            updates.push("frequency = ?");
            params_vec.push(Box::new(freq.as_str().to_string()));
        }
        if let Some(ref time) = request.time_of_day {
            updates.push("time_of_day = ?");
            params_vec.push(Box::new(time.clone()));
        }
        if let Some(day) = request.day_of_week {
            updates.push("day_of_week = ?");
            params_vec.push(Box::new(day));
        }
        if let Some(day) = request.day_of_month {
            updates.push("day_of_month = ?");
            params_vec.push(Box::new(day));
        }
        if let Some(enabled) = request.enabled {
            updates.push("enabled = ?");
            params_vec.push(Box::new(enabled));
        }

        if updates.is_empty() {
            // Nothing to update, just return current schedule
            return self.get_schedule_with_conn(&conn, id);
        }

        updates.push("updated_at = CURRENT_TIMESTAMP");

        let query = format!(
            "UPDATE assessment_schedules SET {} WHERE id = ?",
            updates.join(", ")
        );

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
        let conn = conn.lock().map_err(|_| SchedulingError::LockPoisoned)?;

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
        let conn = conn.lock().map_err(|_| SchedulingError::LockPoisoned)?;

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

    /// Get a single schedule by ID
    pub fn get_schedule(&self, id: i32) -> Result<AssessmentSchedule, SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| SchedulingError::LockPoisoned)?;

        self.get_schedule_with_conn(&conn, id)
    }

    /// T164: Get schedules that are due for triggering
    pub fn get_due_schedules(&self) -> Result<Vec<AssessmentSchedule>, SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| SchedulingError::LockPoisoned)?;

        // Get current time components
        let now = chrono::Utc::now();
        let current_time = now.format("%H:%M").to_string();
        let current_date = now.format("%Y-%m-%d").to_string();

        let query = "SELECT s.id, s.assessment_type_id, a.code, a.name, s.frequency, s.time_of_day,
                            s.day_of_week, s.day_of_month, s.enabled, s.last_triggered_at,
                            s.created_at, s.updated_at
                     FROM assessment_schedules s
                     JOIN assessment_types a ON s.assessment_type_id = a.id
                     WHERE s.enabled = 1
                       AND (s.last_triggered_at IS NULL
                            OR DATE(s.last_triggered_at) < ?)
                       AND s.time_of_day <= ?
                     ORDER BY s.time_of_day ASC";

        let mut stmt = conn.prepare(query)?;
        let schedules = stmt
            .query_map(params![current_date, current_time], |row| {
                self.map_schedule_row(row)
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(schedules)
    }

    /// Mark a schedule as triggered
    pub fn mark_triggered(&self, id: i32) -> Result<(), SchedulingError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| SchedulingError::LockPoisoned)?;

        let rows_affected = conn.execute(
            "UPDATE assessment_schedules SET last_triggered_at = CURRENT_TIMESTAMP WHERE id = ?",
            params![id],
        )?;

        if rows_affected == 0 {
            return Err(SchedulingError::NotFound(id));
        }

        Ok(())
    }

    // Helper methods

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
        .map_err(|_| SchedulingError::NotFound(id))
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
