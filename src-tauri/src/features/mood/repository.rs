// Mood repository - Data access layer for mood check-ins and activities
// T073-T079: Mood repository implementation

use crate::db::Database;
use super::models::*;
use std::sync::Arc;
use tracing::{info, error};

pub struct MoodRepository {
    db: Arc<Database>,
}

impl MoodRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // T075: create_mood_checkin method
    pub fn create_mood_checkin(
        &self,
        mood_rating: i32,
        activity_ids: Vec<i64>,
        notes: Option<&str>,
    ) -> Result<MoodCheckin, MoodError> {
        // Validate mood rating
        validate_mood_rating(mood_rating)?;

        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        // Insert mood check-in and get the ID using RETURNING
        let (mood_checkin_id, created_at): (i64, String) = conn.query_row(
            "INSERT INTO mood_checkins (mood_rating, notes) VALUES (?, ?) RETURNING id, CAST(created_at AS VARCHAR)",
            duckdb::params![mood_rating, notes],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        info!("Created mood check-in with ID: {}", mood_checkin_id);

        // Link activities
        for activity_id in &activity_ids {
            // Verify activity exists
            let activity_exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) > 0 FROM activities WHERE id = ?",
                    [activity_id],
                    |row| row.get(0),
                )
                .unwrap_or(false);

            if !activity_exists {
                return Err(MoodError::ActivityNotFound(*activity_id));
            }

            // Insert into junction table (handles duplicates with UNIQUE constraint)
            let result = conn.execute(
                "INSERT INTO mood_checkin_activities (mood_checkin_id, activity_id) VALUES (?, ?)",
                duckdb::params![mood_checkin_id, activity_id],
            );

            // Ignore duplicate errors (unique constraint violation)
            if let Err(e) = result {
                let err_msg = e.to_string();
                if !err_msg.contains("UNIQUE") && !err_msg.contains("duplicate") {
                    return Err(MoodError::Database(e));
                }
            }
        }

        // Fetch activities for this check-in
        let activities = self.get_activities_for_checkin_with_conn(&conn, mood_checkin_id)?;

        // Build and return the created mood check-in
        Ok(MoodCheckin {
            id: mood_checkin_id,
            mood_rating,
            notes: notes.map(|s| s.to_string()),
            activities,
            created_at,
        })
    }

    // T076: get_mood_history query
    pub fn get_mood_history(
        &self,
        from_date: Option<String>,
        to_date: Option<String>,
        limit: Option<i32>,
    ) -> Result<Vec<MoodCheckin>, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        let mut query = String::from("SELECT id, mood_rating, notes, CAST(created_at AS VARCHAR) FROM mood_checkins WHERE 1=1");
        let mut params: Vec<&dyn duckdb::ToSql> = Vec::new();

        if let Some(ref from) = from_date {
            query.push_str(" AND created_at >= ?");
            params.push(from);
        }
        if let Some(ref to) = to_date {
            query.push_str(" AND created_at <= ?");
            params.push(to);
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(lim) = limit {
            query.push_str(&format!(" LIMIT {}", lim));
        }

        let mut stmt = conn.prepare(&query)?;

        let mood_rows = stmt.query_map(params.as_slice(), |row| {
            Ok((
                row.get::<_, i64>(0)?,      // id
                row.get::<_, i32>(1)?,       // mood_rating
                row.get::<_, Option<String>>(2)?, // notes
                row.get::<_, String>(3)?,    // created_at
            ))
        })?;

        let mut moods = Vec::new();
        for mood_result in mood_rows {
            let (id, mood_rating, notes, created_at) = mood_result?;
            let activities = self.get_activities_for_checkin_with_conn(&conn, id)?;

            moods.push(MoodCheckin {
                id,
                mood_rating,
                notes,
                activities,
                created_at,
            });
        }

        Ok(moods)
    }

    // T077: get_mood_checkin query
    pub fn get_mood_checkin(&self, id: i64) -> Result<MoodCheckin, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        let mood_result = conn.query_row(
            "SELECT id, mood_rating, notes, CAST(created_at AS VARCHAR) FROM mood_checkins WHERE id = ?",
            [id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i32>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                ))
            },
        );

        match mood_result {
            Ok((id, mood_rating, notes, created_at)) => {
                let activities = self.get_activities_for_checkin_with_conn(&conn, id)?;
                Ok(MoodCheckin {
                    id,
                    mood_rating,
                    notes,
                    activities,
                    created_at,
                })
            }
            Err(_) => Err(MoodError::MoodCheckinNotFound(id)),
        }
    }

    // Helper method to get activities for a mood check-in
    // Accepts connection reference to avoid deadlock when called from already-locked context
    fn get_activities_for_checkin_with_conn(
        &self,
        conn: &duckdb::Connection,
        mood_checkin_id: i64,
    ) -> Result<Vec<Activity>, MoodError> {
        let mut stmt = conn.prepare(
            "SELECT a.id, a.name, a.color, a.icon, CAST(a.created_at AS VARCHAR), CAST(a.deleted_at AS VARCHAR)
             FROM activities a
             JOIN mood_checkin_activities mca ON a.id = mca.activity_id
             WHERE mca.mood_checkin_id = ?",
        )?;

        let activity_rows = stmt.query_map([mood_checkin_id], |row| {
            Ok(Activity {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                created_at: row.get(4)?,
                deleted_at: row.get(5)?,
            })
        })?;

        let mut activities = Vec::new();
        for activity_result in activity_rows {
            activities.push(activity_result?);
        }

        Ok(activities)
    }

    // T078: get_mood_stats query
    pub fn get_mood_stats(
        &self,
        from_date: Option<String>,
        to_date: Option<String>,
    ) -> Result<MoodStats, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        let mut query = String::from("SELECT AVG(mood_rating), COUNT(*) FROM mood_checkins WHERE 1=1");

        if from_date.is_some() {
            query.push_str(" AND created_at >= ?");
        }
        if to_date.is_some() {
            query.push_str(" AND created_at <= ?");
        }

        let mut stmt = conn.prepare(&query)?;

        let mut param_index = 1;
        if let Some(ref from) = from_date {
            stmt.raw_bind_parameter(param_index, from)?;
            param_index += 1;
        }
        if let Some(ref to) = to_date {
            stmt.raw_bind_parameter(param_index, to)?;
        }

        let (average_mood, total_checkins) = stmt.query_row([], |row| {
            Ok((row.get::<_, f64>(0).unwrap_or(0.0), row.get::<_, i32>(1).unwrap_or(0)))
        })?;

        // Get mood distribution
        let mut mood_distribution = std::collections::HashMap::new();
        let mut query2 = String::from("SELECT mood_rating, COUNT(*) FROM mood_checkins WHERE 1=1");

        if from_date.is_some() {
            query2.push_str(" AND created_at >= ?");
        }
        if to_date.is_some() {
            query2.push_str(" AND created_at <= ?");
        }

        query2.push_str(" GROUP BY mood_rating");

        let mut stmt2 = conn.prepare(&query2)?;

        let mut param_index2 = 1;
        if let Some(ref from) = from_date {
            stmt2.raw_bind_parameter(param_index2, from)?;
            param_index2 += 1;
        }
        if let Some(ref to) = to_date {
            stmt2.raw_bind_parameter(param_index2, to)?;
        }

        let dist_rows = stmt2.query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
        })?;

        for dist_result in dist_rows {
            let (rating, count) = dist_result?;
            mood_distribution.insert(rating, count);
        }

        // Get activity correlations
        let activity_correlations = self.get_activity_correlations(from_date, to_date)?;

        Ok(MoodStats {
            average_mood,
            total_checkins,
            mood_distribution,
            activity_correlations,
        })
    }

    // Helper for activity correlations
    fn get_activity_correlations(
        &self,
        from_date: Option<String>,
        to_date: Option<String>,
    ) -> Result<Vec<ActivityCorrelation>, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        let mut query = String::from(
            "SELECT a.id, a.name, a.color, a.icon, CAST(a.created_at AS VARCHAR), CAST(a.deleted_at AS VARCHAR),
                    AVG(mc.mood_rating) as avg_mood, COUNT(mc.id) as checkin_count
             FROM activities a
             JOIN mood_checkin_activities mca ON a.id = mca.activity_id
             JOIN mood_checkins mc ON mca.mood_checkin_id = mc.id
             WHERE 1=1",
        );

        if from_date.is_some() {
            query.push_str(" AND mc.created_at >= ?");
        }
        if to_date.is_some() {
            query.push_str(" AND mc.created_at <= ?");
        }

        query.push_str(" GROUP BY a.id, a.name, a.color, a.icon, a.created_at, a.deleted_at");
        query.push_str(" HAVING COUNT(mc.id) >= 3"); // Minimum sample size
        query.push_str(" ORDER BY avg_mood DESC");

        let mut stmt = conn.prepare(&query)?;

        let mut param_index = 1;
        if let Some(ref from) = from_date {
            stmt.raw_bind_parameter(param_index, from)?;
            param_index += 1;
        }
        if let Some(ref to) = to_date {
            stmt.raw_bind_parameter(param_index, to)?;
        }

        let corr_rows = stmt.query_map([], |row| {
            Ok((
                Activity {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    icon: row.get(3)?,
                    created_at: row.get(4)?,
                    deleted_at: row.get(5)?,
                },
                row.get::<_, f64>(6)?,  // avg_mood
                row.get::<_, i32>(7)?,   // checkin_count
            ))
        })?;

        let mut correlations = Vec::new();
        for corr_result in corr_rows {
            let (activity, average_mood, checkin_count) = corr_result?;
            correlations.push(ActivityCorrelation {
                activity,
                average_mood,
                checkin_count,
            });
        }

        Ok(correlations)
    }

    // T102: create_activity method
    pub fn create_activity(
        &self,
        name: &str,
        color: Option<&str>,
        icon: Option<&str>,
    ) -> Result<Activity, MoodError> {
        let trimmed_name = validate_activity_name(name)?;

        // Validate color if provided
        if let Some(c) = color {
            validate_color(c)?;
        }

        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        // Check for duplicate name (among non-deleted activities)
        let name_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM activities WHERE name = ? AND deleted_at IS NULL",
                [&trimmed_name],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if name_exists {
            return Err(MoodError::DuplicateActivityName(trimmed_name));
        }

        // Insert activity and get all fields using RETURNING
        let (id, name, color_result, icon_result, created_at): (i64, String, Option<String>, Option<String>, String) = conn.query_row(
            "INSERT INTO activities (name, color, icon) VALUES (?, ?, ?) RETURNING id, name, color, icon, CAST(created_at AS VARCHAR)",
            duckdb::params![trimmed_name, color, icon],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )?;

        info!("Created activity with ID: {}", id);

        // Build and return the created activity
        Ok(Activity {
            id,
            name,
            color: color_result,
            icon: icon_result,
            created_at,
            deleted_at: None,
        })
    }

    // Helper to get a single activity
    fn get_activity(&self, id: i64) -> Result<Activity, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        conn.query_row(
            "SELECT id, name, color, icon, CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR) FROM activities WHERE id = ?",
            [id],
            |row| {
                Ok(Activity {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    icon: row.get(3)?,
                    created_at: row.get(4)?,
                    deleted_at: row.get(5)?,
                })
            },
        )
        .map_err(|_| MoodError::ActivityNotFound(id))
    }

    // T103: update_activity method
    pub fn update_activity(
        &self,
        id: i64,
        name: Option<&str>,
        color: Option<&str>,
        icon: Option<&str>,
    ) -> Result<Activity, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        // Verify activity exists
        let activity_exists: bool = conn
            .query_row("SELECT COUNT(*) > 0 FROM activities WHERE id = ?", [id], |row| row.get(0))
            .unwrap_or(false);

        if !activity_exists {
            return Err(MoodError::ActivityNotFound(id));
        }

        // Validate and update name if provided
        if let Some(n) = name {
            let trimmed_name = validate_activity_name(n)?;

            // Check for duplicate name
            let name_exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) > 0 FROM activities WHERE name = ? AND id != ? AND deleted_at IS NULL",
                    duckdb::params![&trimmed_name, id],
                    |row| row.get(0),
                )
                .unwrap_or(false);

            if name_exists {
                return Err(MoodError::DuplicateActivityName(trimmed_name));
            }

            conn.execute("UPDATE activities SET name = ? WHERE id = ?", duckdb::params![trimmed_name, id])?;
        }

        // Update color if provided
        if let Some(c) = color {
            validate_color(c)?;
            conn.execute("UPDATE activities SET color = ? WHERE id = ?", duckdb::params![c, id])?;
        }

        // Update icon if provided
        if let Some(i) = icon {
            conn.execute("UPDATE activities SET icon = ? WHERE id = ?", duckdb::params![i, id])?;
        }

        info!("Updated activity ID: {}", id);

        // Fetch and return the updated activity within the same lock scope
        conn.query_row(
            "SELECT id, name, color, icon, CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR) FROM activities WHERE id = ?",
            [id],
            |row| {
                Ok(Activity {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    icon: row.get(3)?,
                    created_at: row.get(4)?,
                    deleted_at: row.get(5)?,
                })
            },
        )
        .map_err(|_| MoodError::ActivityNotFound(id))
    }

    // T104: delete_activity method (soft delete)
    pub fn delete_activity(&self, id: i64) -> Result<(), MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        // Verify activity exists
        let activity_exists: bool = conn
            .query_row("SELECT COUNT(*) > 0 FROM activities WHERE id = ?", [id], |row| row.get(0))
            .unwrap_or(false);

        if !activity_exists {
            return Err(MoodError::ActivityNotFound(id));
        }

        // Soft delete (set deleted_at)
        conn.execute(
            "UPDATE activities SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?",
            [id],
        )?;

        info!("Soft-deleted activity ID: {}", id);

        Ok(())
    }

    // T105: get_activities query
    pub fn get_activities(&self, include_deleted: bool) -> Result<Vec<Activity>, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        let query = if include_deleted {
            "SELECT id, name, color, icon, CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR) FROM activities ORDER BY name"
        } else {
            "SELECT id, name, color, icon, CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR) FROM activities WHERE deleted_at IS NULL ORDER BY name"
        };

        let mut stmt = conn.prepare(query)?;

        let activity_rows = stmt.query_map([], |row| {
            Ok(Activity {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                created_at: row.get(4)?,
                deleted_at: row.get(5)?,
            })
        })?;

        let mut activities = Vec::new();
        for activity_result in activity_rows {
            activities.push(activity_result?);
        }

        Ok(activities)
    }

    // T093b: delete_mood_checkin with transactional cascade
    pub fn delete_mood_checkin(&self, id: i64) -> Result<(), MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|e| {
            error!("Failed to acquire database lock: {}", e);
            MoodError::Database(duckdb::Error::InvalidParameterCount(0, 0))
        })?;

        // Verify mood check-in exists
        let checkin_exists: bool = conn
            .query_row("SELECT COUNT(*) > 0 FROM mood_checkins WHERE id = ?", [id], |row| row.get(0))
            .unwrap_or(false);

        if !checkin_exists {
            return Err(MoodError::MoodCheckinNotFound(id));
        }

        // Begin transaction
        conn.execute("BEGIN TRANSACTION", [])?;

        // Try to delete junction table entries first, then the mood check-in
        let delete_result = (|| {
            // Delete mood_checkin_activities junction entries
            conn.execute(
                "DELETE FROM mood_checkin_activities WHERE mood_checkin_id = ?",
                [id],
            )?;

            // Delete the mood check-in itself
            conn.execute("DELETE FROM mood_checkins WHERE id = ?", [id])?;

            Ok::<(), duckdb::Error>(())
        })();

        match delete_result {
            Ok(()) => {
                conn.execute("COMMIT", [])?;
                info!("Deleted mood check-in ID: {} with cascaded deletions", id);
                Ok(())
            }
            Err(e) => {
                // Rollback on error
                if let Err(rollback_err) = conn.execute("ROLLBACK", []) {
                    error!("Failed to rollback transaction: {}", rollback_err);
                }
                Err(MoodError::Database(e))
            }
        }
    }
}
