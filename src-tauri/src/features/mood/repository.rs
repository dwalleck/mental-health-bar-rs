// Mood repository - Data access layer for mood check-ins and activities
// T073-T079: Mood repository implementation
//
// ## Lock Poisoning
// This repository uses a Mutex to protect database connections. Lock poisoning occurs when
// a thread panics while holding the lock, leaving the Mutex in a "poisoned" state.
//
// In a single-threaded Tauri application, lock poisoning should never occur under normal
// circumstances. If it does occur, it indicates a serious bug (panic in database code) that
// has likely left the database in an inconsistent state.
//
// The fail-fast approach (returning MoodError::LockPoisoned) is intentional:
// - It surfaces the critical error to the UI layer
// - Prevents continuing with potentially corrupted data
// - The application should be restarted to recover
//
// Recovery: The database file itself is not corrupted (SQLite is ACID-compliant), but
// the in-memory state may be inconsistent. Restarting the application will recover.

use super::models::*;
use crate::db::Database;
use std::sync::Arc;
use tracing::info;

/// Maximum number of records that can be retrieved in a single query
const MAX_QUERY_LIMIT: i32 = 1000;

/// Minimum number of check-ins required to establish activity-mood correlation
const MIN_CORRELATION_SAMPLE_SIZE: i32 = 3;

/// Type alias for activity INSERT RETURNING query result
/// Tuple: (id, name, color, icon, created_at)
type ActivityInsertResult =
    Result<(i32, String, Option<String>, Option<String>, String), rusqlite::Error>;

pub struct MoodRepository {
    db: Arc<Database>,
}

impl MoodRepository {
    /// Creates a new MoodRepository instance.
    ///
    /// # Arguments
    /// * `db` - Shared reference to the database connection
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Creates a new mood check-in with optional activities and notes.
    ///
    /// # Arguments
    /// * `mood_rating` - Mood rating from 1 (worst) to 5 (best)
    /// * `activity_ids` - List of activity IDs to associate with this check-in
    /// * `notes` - Optional text notes (max 5000 characters)
    ///
    /// # Returns
    /// * `Ok(MoodCheckin)` - The created check-in with all associated activities
    /// * `Err(MoodError)` - If validation fails or database error occurs
    ///
    /// # Errors
    /// * `InvalidRating` - If mood_rating is not between 1-5
    /// * `NotesLengthExceeded` - If notes exceed 5000 characters
    /// * `ActivityNotFound` - If any activity_id doesn't exist
    /// * `Database` - On database errors
    // T075: create_mood_checkin method
    pub fn create_mood_checkin(
        &self,
        mood_rating: i32,
        activity_ids: Vec<i32>,
        notes: Option<&str>,
    ) -> Result<MoodCheckin, MoodError> {
        // Validate inputs
        validate_mood_rating(mood_rating)?;
        if let Some(n) = notes {
            validate_notes(n)?;
        }

        let conn = self.db.get_connection();
        let mut conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        // ✅ RAII transaction - automatic rollback on drop if not committed
        let tx = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(MoodError::Database)?;

        // Insert mood check-in and get the ID using RETURNING
        let (mood_checkin_id, created_at): (i32, String) = tx.query_row(
            "INSERT INTO mood_checkins (mood_rating, notes) VALUES (?, ?) RETURNING id, CAST(created_at AS VARCHAR)",
            rusqlite::params![mood_rating, notes],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        info!("Created mood check-in with ID: {}", mood_checkin_id);

        // Link activities
        for activity_id in &activity_ids {
            // Verify activity exists
            let activity_exists: bool = tx
                .query_row(
                    "SELECT COUNT(*) > 0 FROM activities WHERE id = ?",
                    [activity_id],
                    |row| row.get(0),
                )
                .map_err(MoodError::Database)?;

            if !activity_exists {
                return Err(MoodError::ActivityNotFound(*activity_id));
            }

            // Insert into junction table (handles duplicates with UNIQUE constraint)
            let result = tx.execute(
                "INSERT INTO mood_checkin_activities (mood_checkin_id, activity_id) VALUES (?, ?)",
                rusqlite::params![mood_checkin_id, activity_id],
            );

            // Ignore duplicate errors (unique constraint violation), propagate all others
            if let Err(e) = result {
                match e {
                    rusqlite::Error::SqliteFailure(err, _) => {
                        if err.code != rusqlite::ErrorCode::ConstraintViolation {
                            return Err(MoodError::Database(e));
                        }
                        // Silently ignore constraint violations (duplicate activity_id)
                    }
                    _ => return Err(MoodError::Database(e)),
                }
            }
        }

        // Fetch activities for this check-in (need to get underlying connection)
        let activities = self.get_activities_for_checkin_with_conn(&tx, mood_checkin_id)?;

        // Build the created mood check-in
        let mood_checkin = MoodCheckin {
            id: mood_checkin_id,
            mood_rating,
            notes: notes.map(|s| s.to_string()),
            activities,
            created_at,
        };

        // Commit transaction - automatic rollback via Drop on error/panic
        tx.commit().map_err(MoodError::Database)?;

        Ok(mood_checkin)
    }

    /// Retrieves mood check-in history with optional date filtering and limit.
    ///
    /// # Arguments
    /// * `from_date` - Optional ISO 8601 date string to filter check-ins after this date
    /// * `to_date` - Optional ISO 8601 date string to filter check-ins before this date
    /// * `limit` - Optional limit on number of results (max 1000, defaults to all)
    ///
    /// # Returns
    /// * `Ok(Vec<MoodCheckin>)` - List of mood check-ins ordered by created_at DESC
    /// * `Err(MoodError)` - On database errors
    // T076: get_mood_history query
    pub fn get_mood_history(
        &self,
        from_date: Option<String>,
        to_date: Option<String>,
        limit: Option<i32>,
    ) -> Result<Vec<MoodCheckin>, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        let mut query = String::from("SELECT id, mood_rating, notes, CAST(created_at AS VARCHAR) FROM mood_checkins WHERE 1=1");
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(ref from) = from_date {
            query.push_str(" AND created_at >= ?");
            params.push(from);
        }
        if let Some(ref to) = to_date {
            query.push_str(" AND created_at <= ?");
            params.push(to);
        }

        query.push_str(" ORDER BY created_at DESC");

        // Apply limit with bounds checking using parameterized query
        let safe_limit = limit.map(|lim| lim.clamp(1, MAX_QUERY_LIMIT));
        if let Some(ref lim) = safe_limit {
            query.push_str(" LIMIT ?");
            params.push(lim);
        }

        let mut stmt = conn.prepare(&query)?;

        let mood_rows = stmt.query_map(params.as_slice(), |row| {
            Ok((
                row.get::<_, i32>(0)?,            // id
                row.get::<_, i32>(1)?,            // mood_rating
                row.get::<_, Option<String>>(2)?, // notes
                row.get::<_, String>(3)?,         // created_at
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
    pub fn get_mood_checkin(&self, id: i32) -> Result<MoodCheckin, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        let mood_result = conn.query_row(
            "SELECT id, mood_rating, notes, CAST(created_at AS VARCHAR) FROM mood_checkins WHERE id = ?",
            [id],
            |row| {
                Ok((
                    row.get::<_, i32>(0)?,
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
            Err(rusqlite::Error::QueryReturnedNoRows) => Err(MoodError::MoodCheckinNotFound(id)),
            Err(e) => Err(MoodError::Database(e)),
        }
    }

    // Helper method to get activities for a mood check-in
    // Accepts connection reference to avoid deadlock when called from already-locked context
    fn get_activities_for_checkin_with_conn(
        &self,
        conn: &rusqlite::Connection,
        mood_checkin_id: i32,
    ) -> Result<Vec<Activity>, MoodError> {
        // ✅ Use prepare_cached for performance (called in loops)
        let mut stmt = conn.prepare_cached(
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

    /// Computes mood statistics including averages, distribution, and activity correlations.
    ///
    /// # Arguments
    /// * `from_date` - Optional ISO 8601 date string to filter stats after this date
    /// * `to_date` - Optional ISO 8601 date string to filter stats before this date
    ///
    /// # Returns
    /// * `Ok(MoodStats)` - Statistics including average mood, total count, mood distribution, and activity correlations
    /// * `Err(MoodError)` - On database errors
    // T078: get_mood_stats query
    pub fn get_mood_stats(
        &self,
        from_date: Option<String>,
        to_date: Option<String>,
    ) -> Result<MoodStats, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        let mut query =
            String::from("SELECT AVG(mood_rating), COUNT(*) FROM mood_checkins WHERE 1=1");
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(ref from) = from_date {
            query.push_str(" AND created_at >= ?");
            params.push(from);
        }
        if let Some(ref to) = to_date {
            query.push_str(" AND created_at <= ?");
            params.push(to);
        }

        let mut stmt = conn.prepare(&query)?;

        let (average_mood, total_checkins) = stmt.query_row(params.as_slice(), |row| {
            Ok((
                row.get::<_, Option<f64>>(0)?.unwrap_or(0.0), // AVG returns NULL when no rows
                row.get::<_, i32>(1)?,                        // COUNT never returns NULL
            ))
        })?;

        // Get mood distribution
        let mut mood_distribution = std::collections::HashMap::new();
        let mut query2 = String::from("SELECT mood_rating, COUNT(*) FROM mood_checkins WHERE 1=1");
        let mut params2: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(ref from) = from_date {
            query2.push_str(" AND created_at >= ?");
            params2.push(from);
        }
        if let Some(ref to) = to_date {
            query2.push_str(" AND created_at <= ?");
            params2.push(to);
        }

        query2.push_str(" GROUP BY mood_rating");

        let mut stmt2 = conn.prepare(&query2)?;

        let dist_rows = stmt2.query_map(params2.as_slice(), |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
        })?;

        for dist_result in dist_rows {
            let (rating, count) = dist_result?;
            mood_distribution.insert(rating, count);
        }

        // Get activity correlations (pass conn to avoid deadlock)
        let activity_correlations =
            self.get_activity_correlations_with_conn(&conn, from_date, to_date)?;

        Ok(MoodStats {
            average_mood,
            total_checkins,
            mood_distribution,
            activity_correlations,
        })
    }

    // Helper for activity correlations
    fn get_activity_correlations_with_conn(
        &self,
        conn: &rusqlite::Connection,
        from_date: Option<String>,
        to_date: Option<String>,
    ) -> Result<Vec<ActivityCorrelation>, MoodError> {
        let mut query = String::from(
            "SELECT a.id, a.name, a.color, a.icon, CAST(a.created_at AS VARCHAR), CAST(a.deleted_at AS VARCHAR),
                    AVG(mc.mood_rating) as avg_mood, COUNT(mc.id) as checkin_count
             FROM activities a
             JOIN mood_checkin_activities mca ON a.id = mca.activity_id
             JOIN mood_checkins mc ON mca.mood_checkin_id = mc.id
             WHERE 1=1",
        );
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(ref from) = from_date {
            query.push_str(" AND mc.created_at >= ?");
            params.push(from);
        }
        if let Some(ref to) = to_date {
            query.push_str(" AND mc.created_at <= ?");
            params.push(to);
        }

        query.push_str(" GROUP BY a.id, a.name, a.color, a.icon, a.created_at, a.deleted_at");
        query.push_str(" HAVING COUNT(mc.id) >= ?");
        params.push(&MIN_CORRELATION_SAMPLE_SIZE);
        query.push_str(" ORDER BY avg_mood DESC");

        let mut stmt = conn.prepare(&query)?;

        let corr_rows = stmt.query_map(params.as_slice(), |row| {
            Ok((
                Activity {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    icon: row.get(3)?,
                    created_at: row.get(4)?,
                    deleted_at: row.get(5)?,
                },
                row.get::<_, f64>(6)?, // avg_mood
                row.get::<_, i32>(7)?, // checkin_count
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
    /// Creates a new activity for mood tracking.
    ///
    /// # Arguments
    /// * `name` - Activity name (1-100 characters, trimmed)
    /// * `color` - Optional hex color code (#RGB, #RRGGBB, or #RRGGBBAA)
    /// * `icon` - Optional emoji or icon string (max 20 characters)
    ///
    /// # Returns
    /// * `Ok(Activity)` - The created activity
    /// * `Err(MoodError)` - If validation fails or database error occurs
    ///
    /// # Errors
    /// * `EmptyActivityName` - If name is empty after trimming
    /// * `ActivityNameTooLong` - If name exceeds 100 characters
    /// * `DuplicateActivityName` - If an active activity with this name already exists
    /// * `InvalidColorFormat` - If color doesn't match hex format
    /// * `ActivityIconTooLong` - If icon exceeds 20 characters
    /// * `Database` - On database errors
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

        // Validate icon if provided
        if let Some(i) = icon {
            validate_icon(i)?;
        }

        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        // Insert activity and get all fields using RETURNING
        // The partial unique index will enforce uniqueness atomically
        let result: ActivityInsertResult = conn.query_row(
            "INSERT INTO activities (name, color, icon) VALUES (?, ?, ?) RETURNING id, name, color, icon, CAST(created_at AS VARCHAR)",
            rusqlite::params![trimmed_name, color, icon],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        );

        // Handle constraint violation with proper error
        let (id, name, color_value, icon_value, created_at) = match result {
            Ok(data) => data,
            Err(rusqlite::Error::SqliteFailure(err, _)) => {
                if err.code == rusqlite::ErrorCode::ConstraintViolation {
                    return Err(MoodError::DuplicateActivityName(trimmed_name));
                }
                return Err(MoodError::Database(rusqlite::Error::SqliteFailure(
                    err, None,
                )));
            }
            Err(e) => return Err(MoodError::Database(e)),
        };

        info!("Created activity with ID: {}", id);

        // Build and return the created activity
        Ok(Activity {
            id,
            name,
            color: color_value,
            icon: icon_value,
            created_at,
            deleted_at: None,
        })
    }

    // Get a single activity by ID
    pub fn get_activity(&self, id: i32) -> Result<Activity, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

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
    /// Updates an existing activity's properties.
    ///
    /// # Arguments
    /// * `id` - Activity ID to update
    /// * `name` - Optional new name (1-100 characters, trimmed)
    /// * `color` - Optional new hex color code
    /// * `icon` - Optional new emoji or icon string (max 20 characters)
    ///
    /// # Returns
    /// * `Ok(Activity)` - The updated activity
    /// * `Err(MoodError)` - If validation fails or database error occurs
    ///
    /// # Errors
    /// * `ActivityNotFound` - If activity with given ID doesn't exist
    /// * `DuplicateActivityName` - If new name conflicts with another active activity
    /// * `InvalidColorFormat` - If color doesn't match hex format
    /// * `ActivityIconTooLong` - If icon exceeds 20 characters
    /// * `Database` - On database errors
    pub fn update_activity(
        &self,
        id: i32,
        name: Option<&str>,
        color: Option<&str>,
        icon: Option<&str>,
    ) -> Result<Activity, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        // Verify activity exists
        let activity_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM activities WHERE id = ?",
                [id],
                |row| row.get(0),
            )
            .map_err(MoodError::Database)?;

        if !activity_exists {
            return Err(MoodError::ActivityNotFound(id));
        }

        // Validate and update name if provided
        if let Some(n) = name {
            let trimmed_name = validate_activity_name(n)?;

            // Update name atomically - the partial unique index will enforce uniqueness
            let result = conn.execute(
                "UPDATE activities SET name = ? WHERE id = ?",
                rusqlite::params![trimmed_name, id],
            );

            // Handle constraint violation with proper error
            match result {
                Ok(_) => {}
                Err(rusqlite::Error::SqliteFailure(err, _)) => {
                    if err.code == rusqlite::ErrorCode::ConstraintViolation {
                        return Err(MoodError::DuplicateActivityName(trimmed_name));
                    }
                    return Err(MoodError::Database(rusqlite::Error::SqliteFailure(
                        err, None,
                    )));
                }
                Err(e) => return Err(MoodError::Database(e)),
            }
        }

        // Update color if provided
        if let Some(c) = color {
            validate_color(c)?;
            conn.execute(
                "UPDATE activities SET color = ? WHERE id = ?",
                rusqlite::params![c, id],
            )?;
        }

        // Update icon if provided
        if let Some(i) = icon {
            validate_icon(i)?;
            conn.execute(
                "UPDATE activities SET icon = ? WHERE id = ?",
                rusqlite::params![i, id],
            )?;
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
    /// Soft-deletes an activity by setting its deleted_at timestamp.
    ///
    /// The activity remains in the database and historical mood check-ins
    /// will still reference it, but it won't appear in active lists.
    ///
    /// # Arguments
    /// * `id` - Activity ID to delete
    ///
    /// # Returns
    /// * `Ok(())` - Activity successfully marked as deleted
    /// * `Err(MoodError)` - If activity not found or database error occurs
    ///
    /// # Errors
    /// * `ActivityNotFound` - If activity with given ID doesn't exist
    /// * `Database` - On database errors
    pub fn delete_activity(&self, id: i32) -> Result<(), MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        // Verify activity exists
        let activity_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM activities WHERE id = ?",
                [id],
                |row| row.get(0),
            )
            .map_err(MoodError::Database)?;

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
    /// Retrieves all activities, optionally including soft-deleted ones.
    ///
    /// # Arguments
    /// * `include_deleted` - If true, includes soft-deleted activities; if false, only active ones
    ///
    /// # Returns
    /// * `Ok(Vec<Activity>)` - List of activities ordered by created_at DESC
    /// * `Err(MoodError)` - On database errors
    pub fn get_activities(&self, include_deleted: bool) -> Result<Vec<Activity>, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

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
    pub fn delete_mood_checkin(&self, id: i32) -> Result<(), MoodError> {
        let conn = self.db.get_connection();
        let mut conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

        // Verify mood check-in exists
        let checkin_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM mood_checkins WHERE id = ?",
                [id],
                |row| row.get(0),
            )
            .map_err(MoodError::Database)?;

        if !checkin_exists {
            return Err(MoodError::MoodCheckinNotFound(id));
        }

        // ✅ RAII transaction - automatic rollback on drop if not committed
        let tx = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(MoodError::Database)?;

        // Delete mood_checkin_activities junction entries
        tx.execute(
            "DELETE FROM mood_checkin_activities WHERE mood_checkin_id = ?",
            [id],
        )?;

        // Delete the mood check-in itself
        tx.execute("DELETE FROM mood_checkins WHERE id = ?", [id])?;

        // Commit transaction - automatic rollback via Drop on error/panic
        tx.commit().map_err(MoodError::Database)?;

        info!("Deleted mood check-in ID: {} with cascaded deletions", id);
        Ok(())
    }
}
