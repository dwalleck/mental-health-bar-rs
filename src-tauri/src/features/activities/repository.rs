// Activity repository - Data access layer for activity groups, activities, logs, and goals
//
// ## Concurrency Safety
// This repository uses parking_lot::Mutex to protect database connections. Unlike std::sync::Mutex,
// parking_lot does not implement lock poisoning - it simply blocks until the lock is available.
//
// This design choice is intentional for this application:
// - Tauri applications are single-threaded for UI operations
// - parking_lot provides better performance and simpler error handling
// - If a panic occurs while holding the lock, the application should be restarted
//
// Database safety: SQLite transactions are ACID-compliant, so the database file itself
// cannot be corrupted by panics. However, in-memory state may be inconsistent after a panic,
// requiring application restart for full recovery.

use super::models::*;
use crate::db::Database;
use crate::types::activity::GoalType;
use rusqlite::OptionalExtension;
use std::sync::Arc;
use tracing::info;

pub struct ActivityRepository {
    db: Arc<Database>,
}

impl ActivityRepository {
    /// Creates a new ActivityRepository instance.
    ///
    /// # Arguments
    /// * `db` - Shared reference to the database connection
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Creates a new activity group.
    ///
    /// # Arguments
    /// * `name` - Group name (1-100 characters, validated at request layer)
    /// * `description` - Optional description (max 500 characters)
    ///
    /// # Returns
    /// * `Ok(ActivityGroup)` - The created activity group
    /// * `Err(ActivityError)` - If validation fails or database error occurs
    ///
    /// # Errors
    /// * `EmptyGroupName` - If name is empty
    /// * `GroupNameTooLong` - If name exceeds 100 characters
    /// * `Database` - On database errors
    pub fn create_activity_group(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<ActivityGroup, ActivityError> {
        // Validate name (use char count for UTF-8 correctness)
        let name_char_count = name.chars().count();
        if name.is_empty() {
            return Err(ActivityError::EmptyGroupName);
        }
        if name_char_count > 100 {
            return Err(ActivityError::GroupNameTooLong(name_char_count));
        }
        if let Some(desc) = description {
            let desc_char_count = desc.chars().count();
            if desc_char_count > 500 {
                return Err(ActivityError::DescriptionTooLong(desc_char_count));
            }
        }

        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Insert activity group using RETURNING to get created values
        let (id, created_at): (i32, String) = conn.query_row(
            "INSERT INTO activity_groups (name, description) VALUES (?, ?) RETURNING id, CAST(created_at AS VARCHAR)",
            rusqlite::params![name, description],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        info!(
            group_id = id,
            group_name = name,
            has_description = description.is_some(),
            "Created activity group"
        );

        Ok(ActivityGroup {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            created_at,
            deleted_at: None,
        })
    }

    /// Updates an existing activity group.
    ///
    /// # Arguments
    /// * `id` - Activity group ID to update
    /// * `name` - Optional new name (1-100 characters)
    /// * `description` - Optional new description (max 500 characters)
    ///
    /// # Returns
    /// * `Ok(ActivityGroup)` - The updated activity group
    /// * `Err(ActivityError)` - If validation fails, group not found, or database error
    ///
    /// # Errors
    /// * `GroupNotFound` - If activity group with given ID doesn't exist
    /// * `EmptyGroupName` - If name is empty
    /// * `GroupNameTooLong` - If name exceeds 100 characters
    /// * `Database` - On database errors
    pub fn update_activity_group(
        &self,
        id: i32,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<ActivityGroup, ActivityError> {
        // Validate name if provided (use char count for UTF-8 correctness)
        if let Some(n) = name {
            let name_char_count = n.chars().count();
            if n.is_empty() {
                return Err(ActivityError::EmptyGroupName);
            }
            if name_char_count > 100 {
                return Err(ActivityError::GroupNameTooLong(name_char_count));
            }
        }
        if let Some(desc) = description {
            let desc_char_count = desc.chars().count();
            if desc_char_count > 500 {
                return Err(ActivityError::DescriptionTooLong(desc_char_count));
            }
        }

        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Check if group exists and is not deleted
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM activity_groups WHERE id = ? AND deleted_at IS NULL",
                rusqlite::params![id],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);

        if !exists {
            return Err(ActivityError::GroupNotFound(id));
        }

        // Build UPDATE query based on what's being updated
        // Handle name and description separately to avoid lifetime issues
        match (name, description) {
            (Some(n), Some(d)) => {
                conn.execute(
                    "UPDATE activity_groups SET name = ?, description = ? WHERE id = ?",
                    rusqlite::params![n, d, id],
                )?;
            }
            (Some(n), None) => {
                conn.execute(
                    "UPDATE activity_groups SET name = ? WHERE id = ?",
                    rusqlite::params![n, id],
                )?;
            }
            (None, Some(d)) => {
                conn.execute(
                    "UPDATE activity_groups SET description = ? WHERE id = ?",
                    rusqlite::params![d, id],
                )?;
            }
            (None, None) => {
                // Nothing to update, fetch and return existing group
                return self.get_activity_group_by_id_with_conn(&conn, id);
            }
        }

        info!(
            group_id = id,
            updated_name = name.is_some(),
            updated_description = description.is_some(),
            "Updated activity group"
        );

        // Fetch and return the updated group
        self.get_activity_group_by_id_with_conn(&conn, id)
    }

    /// Soft deletes an activity group (sets deleted_at timestamp).
    /// CASCADE behavior will delete all associated activities.
    ///
    /// # Arguments
    /// * `id` - Activity group ID to delete
    ///
    /// # Returns
    /// * `Ok(())` - If deletion succeeds
    /// * `Err(ActivityError)` - If group not found or database error
    ///
    /// # Errors
    /// * `GroupNotFound` - If activity group with given ID doesn't exist or already deleted
    /// * `Database` - On database errors
    pub fn delete_activity_group(&self, id: i32) -> Result<(), ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Check if group exists and is not already deleted
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM activity_groups WHERE id = ? AND deleted_at IS NULL",
                rusqlite::params![id],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);

        if !exists {
            return Err(ActivityError::GroupNotFound(id));
        }

        // Soft delete the group
        conn.execute(
            "UPDATE activity_groups SET deleted_at = datetime('now') WHERE id = ?",
            rusqlite::params![id],
        )?;

        info!(group_id = id, "Soft deleted activity group");

        Ok(())
    }

    /// Gets all non-deleted activity groups.
    ///
    /// # Returns
    /// * `Ok(Vec<ActivityGroup>)` - List of all active activity groups
    /// * `Err(ActivityError)` - On database error
    pub fn get_activity_groups(&self) -> Result<Vec<ActivityGroup>, ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, name, description, CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR)
             FROM activity_groups
             WHERE deleted_at IS NULL
             ORDER BY name ASC",
        )?;

        let groups = stmt
            .query_map([], |row| {
                Ok(ActivityGroup {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    deleted_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(groups)
    }

    /// Helper method to get an activity group by ID using an existing connection.
    /// Follows the `_with_conn` pattern to avoid deadlocks.
    ///
    /// # Arguments
    /// * `conn` - Database connection reference
    /// * `id` - Activity group ID
    ///
    /// # Returns
    /// * `Ok(ActivityGroup)` - The activity group
    /// * `Err(ActivityError)` - If group not found or database error
    fn get_activity_group_by_id_with_conn(
        &self,
        conn: &rusqlite::Connection,
        id: i32,
    ) -> Result<ActivityGroup, ActivityError> {
        conn.query_row(
            "SELECT id, name, description, CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR)
             FROM activity_groups
             WHERE id = ? AND deleted_at IS NULL",
            rusqlite::params![id],
            |row| {
                Ok(ActivityGroup {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    deleted_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => ActivityError::GroupNotFound(id),
            e => ActivityError::Database(e),
        })
    }

    /// Logs an activity occurrence.
    ///
    /// # Arguments
    /// * `activity_id` - ID of the activity being logged
    /// * `logged_at` - When the activity occurred (ISO 8601 timestamp)
    /// * `notes` - Optional notes (max 500 characters)
    ///
    /// # Returns
    /// * `Ok(ActivityLog)` - The created activity log
    /// * `Err(ActivityError)` - If validation fails or database error
    ///
    /// # Errors
    /// * `ActivityNotFound` - If activity with given ID doesn't exist
    /// * `NotesLengthExceeded` - If notes exceed 500 characters
    /// * `Database` - On database errors
    pub fn log_activity(
        &self,
        activity_id: i32,
        logged_at: &str,
        notes: Option<&str>,
    ) -> Result<ActivityLog, ActivityError> {
        // Trim notes and convert empty string to None
        let notes = notes.map(|n| n.trim()).filter(|n| !n.is_empty());

        // Validate notes length (use char count for UTF-8 correctness)
        if let Some(n) = notes {
            let notes_char_count = n.chars().count();
            if notes_char_count > 500 {
                return Err(ActivityError::NotesLengthExceeded(notes_char_count));
            }
        }

        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Verify activity exists and is not deleted
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM activities WHERE id = ? AND deleted_at IS NULL",
                rusqlite::params![activity_id],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);

        if !exists {
            return Err(ActivityError::ActivityNotFound(activity_id));
        }

        // Insert activity log using RETURNING
        let (id, created_at): (i32, String) = conn.query_row(
            "INSERT INTO activity_logs (activity_id, logged_at, notes) VALUES (?, ?, ?)
             RETURNING id, CAST(created_at AS VARCHAR)",
            rusqlite::params![activity_id, logged_at, notes],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        info!(
            log_id = id,
            activity_id = activity_id,
            has_notes = notes.is_some(),
            "Logged activity"
        );

        Ok(ActivityLog {
            id,
            activity_id,
            logged_at: logged_at.to_string(),
            created_at,
            notes: notes.map(|s| s.to_string()),
            deleted_at: None,
        })
    }

    /// Updates notes for an existing activity log.
    ///
    /// # Arguments
    /// * `id` - Activity log ID to update
    /// * `notes` - Optional notes (max 500 characters, trimmed; empty/whitespace -> NULL)
    ///
    /// # Returns
    /// * `Ok(ActivityLog)` - The updated activity log
    /// * `Err(ActivityError)` - If validation fails, log not found, or database error
    pub fn update_activity_log_notes(
        &self,
        id: i32,
        notes: Option<String>,
    ) -> Result<ActivityLog, ActivityError> {
        // Normalize notes: trim and convert empty to None
        let notes = notes
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty());

        // Validate notes length (UTF-8 safe)
        if let Some(ref n) = notes {
            let notes_char_count = n.chars().count();
            if notes_char_count > 500 {
                return Err(ActivityError::NotesLengthExceeded(notes_char_count));
            }
        }

        let conn = self.db.get_connection();
        let mut conn = conn.lock();

        // Ensure log exists and is not soft-deleted
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM activity_logs WHERE id = ? AND deleted_at IS NULL",
                rusqlite::params![id],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);

        if !exists {
            return Err(ActivityError::LogNotFound(id));
        }

        // Use RAII transaction for UPDATE + SELECT atomicity
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        // Update notes
        let rows_affected = tx.execute(
            "UPDATE activity_logs SET notes = ? WHERE id = ? AND deleted_at IS NULL",
            rusqlite::params![notes, id],
        )?;

        if rows_affected == 0 {
            return Err(ActivityError::LogNotFound(id));
        }

        // Return updated log
        let log = tx.query_row(
            "SELECT id, activity_id, CAST(logged_at AS VARCHAR), CAST(created_at AS VARCHAR),
                    notes, CAST(deleted_at AS VARCHAR)
             FROM activity_logs
             WHERE id = ?",
            rusqlite::params![id],
            |row| {
                Ok(ActivityLog {
                    id: row.get(0)?,
                    activity_id: row.get(1)?,
                    logged_at: row.get(2)?,
                    created_at: row.get(3)?,
                    notes: row.get(4)?,
                    deleted_at: row.get(5)?,
                })
            },
        )?;

        tx.commit()?;
        Ok(log)
    }

    /// Gets activity logs with optional date filtering.
    ///
    /// # Arguments
    /// * `activity_id` - Optional activity ID to filter by
    /// * `start_date` - Optional start date for filtering (ISO 8601)
    /// * `end_date` - Optional end date for filtering (ISO 8601)
    ///
    /// # Returns
    /// * `Ok(Vec<ActivityLog>)` - List of activity logs, ordered by logged_at DESC
    /// * `Err(ActivityError)` - On database error
    pub fn get_activity_logs(
        &self,
        activity_id: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Vec<ActivityLog>, ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Build query dynamically based on filters
        let mut query = String::from(
            "SELECT id, activity_id, CAST(logged_at AS VARCHAR), CAST(created_at AS VARCHAR), notes, CAST(deleted_at AS VARCHAR)
             FROM activity_logs
             WHERE deleted_at IS NULL"
        );
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(aid) = activity_id {
            query.push_str(" AND activity_id = ?");
            params.push(Box::new(aid));
        }
        if let Some(start) = start_date {
            query.push_str(" AND DATE(logged_at) >= DATE(?)");
            params.push(Box::new(start.to_string()));
        }
        if let Some(end) = end_date {
            query.push_str(" AND DATE(logged_at) <= DATE(?)");
            params.push(Box::new(end.to_string()));
        }

        query.push_str(" ORDER BY logged_at DESC");

        let mut stmt = conn.prepare(&query)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let logs = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(ActivityLog {
                    id: row.get(0)?,
                    activity_id: row.get(1)?,
                    logged_at: row.get(2)?,
                    created_at: row.get(3)?,
                    notes: row.get(4)?,
                    deleted_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(logs)
    }

    /// Creates a new activity with required group assignment.
    ///
    /// # Arguments
    /// * `group_id` - Activity group ID (required, NOT NULL)
    /// * `name` - Activity name (1-50 characters)
    /// * `color` - Optional color hex code
    /// * `icon` - Optional icon name (1-20 characters)
    ///
    /// # Returns
    /// * `Ok(Activity)` - The created activity
    /// * `Err(ActivityError)` - If validation fails or database error
    ///
    /// # Errors
    /// * `EmptyActivityName` - If name is empty
    /// * `ActivityNameTooLong` - If name exceeds 50 characters
    /// * `ActivityIconTooLong` - If icon exceeds 20 characters
    /// * `GroupNotFound` - If group_id doesn't exist
    /// * `Database` - On database errors
    pub fn create_activity(
        &self,
        group_id: i32,
        name: &str,
        color: Option<&str>,
        icon: Option<&str>,
    ) -> Result<Activity, ActivityError> {
        // Trim input before validation
        let name = name.trim();
        let icon = icon.map(|i| i.trim()).filter(|i| !i.is_empty());

        // Validate name (use char count for UTF-8 correctness)
        if name.is_empty() {
            return Err(ActivityError::EmptyActivityName);
        }
        let name_char_count = name.chars().count();
        if name_char_count > 50 {
            return Err(ActivityError::ActivityNameTooLong(name_char_count));
        }

        // Validate icon if provided
        if let Some(ic) = icon {
            let icon_char_count = ic.chars().count();
            if icon_char_count > 20 {
                return Err(ActivityError::ActivityIconTooLong(icon_char_count));
            }
        }

        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Verify group exists and is not deleted
        let group_exists: bool = conn
            .query_row(
                "SELECT 1 FROM activity_groups WHERE id = ? AND deleted_at IS NULL",
                rusqlite::params![group_id],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);

        if !group_exists {
            return Err(ActivityError::GroupNotFound(group_id));
        }

        // Insert activity using RETURNING
        let (id, created_at): (i32, String) = conn.query_row(
            "INSERT INTO activities (group_id, name, color, icon) VALUES (?, ?, ?, ?)
             RETURNING id, CAST(created_at AS VARCHAR)",
            rusqlite::params![group_id, name, color, icon],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        info!(
            activity_id = id,
            activity_name = name,
            group_id = group_id,
            has_icon = icon.is_some(),
            "Created activity"
        );

        Ok(Activity {
            id,
            group_id,
            name: name.to_string(),
            color: color.map(|s| s.to_string()),
            icon: icon.map(|s| s.to_string()),
            created_at,
            deleted_at: None,
        })
    }

    /// Gets all activities for a specific group.
    ///
    /// # Arguments
    /// * `group_id` - Activity group ID to filter by
    ///
    /// # Returns
    /// * `Ok(Vec<Activity>)` - List of activities in the group, ordered by name
    /// * `Err(ActivityError)` - On database error
    pub fn get_activities_by_group(&self, group_id: i32) -> Result<Vec<Activity>, ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, group_id, name, color, icon, CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR)
             FROM activities
             WHERE group_id = ? AND deleted_at IS NULL
             ORDER BY name ASC"
        )?;

        let activities = stmt
            .query_map([group_id], |row| {
                Ok(Activity {
                    id: row.get(0)?,
                    group_id: row.get(1)?,
                    name: row.get(2)?,
                    color: row.get(3)?,
                    icon: row.get(4)?,
                    created_at: row.get(5)?,
                    deleted_at: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(activities)
    }

    // ========================================
    // Activity Goals
    // ========================================

    /// Sets an activity goal for tracking completion or improvement.
    ///
    /// # Arguments
    /// * `activity_id` - Optional activity ID (mutually exclusive with group_id)
    /// * `group_id` - Optional group ID (mutually exclusive with activity_id)
    /// * `goal_type` - GoalType::DaysPerPeriod or GoalType::PercentImprovement
    /// * `target_value` - Target days or percentage (must be positive)
    /// * `period_days` - Time period in days (must be positive)
    ///
    /// # Returns
    /// * `Ok(ActivityGoal)` - The created goal
    /// * `Err(ActivityError)` - If validation fails or database error occurs
    ///
    /// # Errors
    /// * `InvalidGoalTarget` - If both activity_id and group_id are provided
    /// * `MissingGoalTarget` - If neither activity_id nor group_id are provided
    /// * `InvalidTargetValue` - If target_value <= 0
    /// * `InvalidPeriodDays` - If period_days <= 0
    pub fn set_activity_goal(
        &self,
        activity_id: Option<i32>,
        group_id: Option<i32>,
        goal_type: GoalType,
        target_value: i32,
        period_days: i32,
    ) -> Result<ActivityGoal, ActivityError> {
        // Validate goal target exclusivity (XOR logic)
        match (&activity_id, &group_id) {
            (Some(_), Some(_)) => return Err(ActivityError::InvalidGoalTarget),
            (None, None) => return Err(ActivityError::MissingGoalTarget),
            _ => {} // Valid: exactly one is Some
        }

        // Validate target_value
        if target_value <= 0 {
            return Err(ActivityError::InvalidTargetValue(target_value));
        }

        // Validate period_days
        if period_days <= 0 {
            return Err(ActivityError::InvalidPeriodDays(period_days));
        }

        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Insert goal using RETURNING to get created values
        let (id, created_at): (i32, String) = conn.query_row(
            "INSERT INTO activity_goals (activity_id, group_id, goal_type, target_value, period_days)
             VALUES (?, ?, ?, ?, ?)
             RETURNING id, CAST(created_at AS VARCHAR)",
            rusqlite::params![activity_id, group_id, goal_type.as_str(), target_value, period_days],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        info!(
            goal_id = id,
            ?activity_id,
            ?group_id,
            goal_type = goal_type.as_str(),
            target_value,
            period_days,
            "Created activity goal"
        );

        Ok(ActivityGoal {
            id,
            activity_id,
            group_id,
            goal_type,
            target_value,
            period_days,
            created_at,
            deleted_at: None,
        })
    }

    /// Gets all active goals for an activity or group.
    ///
    /// # Arguments
    /// * `activity_id` - Optional activity ID to filter by
    /// * `group_id` - Optional group ID to filter by
    ///
    /// # Returns
    /// * `Ok(Vec<ActivityGoal>)` - List of goals (excludes soft-deleted)
    ///
    /// # Notes
    /// If both parameters are None, returns all active goals
    pub fn get_activity_goals(
        &self,
        activity_id: Option<i32>,
        group_id: Option<i32>,
    ) -> Result<Vec<ActivityGoal>, ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let mut query = String::from(
            "SELECT id, activity_id, group_id, goal_type, target_value, period_days,
                    CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR)
             FROM activity_goals
             WHERE deleted_at IS NULL",
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(aid) = activity_id {
            query.push_str(" AND activity_id = ?");
            params.push(Box::new(aid));
        }

        if let Some(gid) = group_id {
            query.push_str(" AND group_id = ?");
            params.push(Box::new(gid));
        }

        query.push_str(" ORDER BY created_at DESC");

        let mut stmt = conn.prepare(&query)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let goals = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(ActivityGoal {
                    id: row.get(0)?,
                    activity_id: row.get(1)?,
                    group_id: row.get(2)?,
                    goal_type: row.get(3)?,
                    target_value: row.get(4)?,
                    period_days: row.get(5)?,
                    created_at: row.get(6)?,
                    deleted_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(goals)
    }

    /// Updates an existing activity goal's target and period.
    ///
    /// # Arguments
    /// * `goal_id` - Goal ID to update
    /// * `target_value` - New target value (must be positive)
    /// * `period_days` - New period in days (must be positive)
    ///
    /// # Returns
    /// * `Ok(ActivityGoal)` - The updated goal
    /// * `Err(ActivityError)` - If goal not found or validation fails
    pub fn update_activity_goal(
        &self,
        goal_id: i32,
        target_value: i32,
        period_days: i32,
    ) -> Result<ActivityGoal, ActivityError> {
        // Validate inputs
        if target_value <= 0 {
            return Err(ActivityError::InvalidTargetValue(target_value));
        }
        if period_days <= 0 {
            return Err(ActivityError::InvalidPeriodDays(period_days));
        }

        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Update the goal
        let rows_affected = conn.execute(
            "UPDATE activity_goals
             SET target_value = ?, period_days = ?
             WHERE id = ? AND deleted_at IS NULL",
            rusqlite::params![target_value, period_days, goal_id],
        )?;

        if rows_affected == 0 {
            return Err(ActivityError::GoalNotFound(goal_id));
        }

        // Fetch and return the updated goal
        let goal = conn.query_row(
            "SELECT id, activity_id, group_id, goal_type, target_value, period_days,
                    CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR)
             FROM activity_goals
             WHERE id = ?",
            rusqlite::params![goal_id],
            |row| {
                Ok(ActivityGoal {
                    id: row.get(0)?,
                    activity_id: row.get(1)?,
                    group_id: row.get(2)?,
                    goal_type: row.get(3)?,
                    target_value: row.get(4)?,
                    period_days: row.get(5)?,
                    created_at: row.get(6)?,
                    deleted_at: row.get(7)?,
                })
            },
        )?;

        info!(goal_id, target_value, period_days, "Updated activity goal");

        Ok(goal)
    }

    /// Soft-deletes an activity goal.
    ///
    /// # Arguments
    /// * `goal_id` - Goal ID to delete
    ///
    /// # Returns
    /// * `Ok(())` - Goal successfully deleted
    /// * `Err(ActivityError::GoalNotFound)` - If goal doesn't exist
    pub fn delete_activity_goal(&self, goal_id: i32) -> Result<(), ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        let rows_affected = conn.execute(
            "UPDATE activity_goals
             SET deleted_at = datetime('now')
             WHERE id = ? AND deleted_at IS NULL",
            rusqlite::params![goal_id],
        )?;

        if rows_affected == 0 {
            return Err(ActivityError::GoalNotFound(goal_id));
        }

        info!(goal_id, "Deleted activity goal");

        Ok(())
    }

    // ========================================
    // Reporting Queries
    // ========================================

    /// Calculates activity frequency (days per week).
    ///
    /// # Arguments
    /// * `activity_id` - Activity ID to analyze
    /// * `start_date` - Period start (ISO 8601)
    /// * `end_date` - Period end (ISO 8601)
    ///
    /// # Returns
    /// * `Ok(ActivityFrequency)` - Frequency report
    pub fn get_activity_frequency(
        &self,
        activity_id: i32,
        start_date: &str,
        end_date: &str,
    ) -> Result<ActivityFrequency, ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Count unique days and total logs
        let (unique_days, total_logs): (i32, i32) = conn.query_row(
            "SELECT
                COUNT(DISTINCT DATE(logged_at)) as unique_days,
                COUNT(*) as total_logs
             FROM activity_logs
             WHERE activity_id = ?
               AND logged_at >= ?
               AND logged_at <= ?
               AND deleted_at IS NULL",
            rusqlite::params![activity_id, start_date, end_date],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        // Calculate period duration in days
        let period_duration_days: f64 = conn.query_row(
            "SELECT CAST((julianday(?) - julianday(?)) AS REAL)",
            rusqlite::params![end_date, start_date],
            |row| row.get(0),
        )?;

        // Calculate days per week
        let num_weeks = period_duration_days / 7.0;
        let days_per_week = if num_weeks > 0.0 {
            (unique_days as f64) / num_weeks
        } else {
            0.0
        };

        Ok(ActivityFrequency {
            activity_id,
            unique_days,
            total_logs,
            days_per_week,
            period_start: start_date.to_string(),
            period_end: end_date.to_string(),
        })
    }

    /// Calculates activity trend (comparison with previous period).
    ///
    /// # Arguments
    /// * `activity_id` - Activity ID to analyze
    /// * `period_days` - Period length in days (e.g., 7 for weekly)
    /// * `current_time` - End of current period (ISO 8601)
    ///
    /// # Returns
    /// * `Ok(ActivityTrend)` - Trend analysis
    pub fn get_activity_trend(
        &self,
        activity_id: i32,
        period_days: i32,
        current_time: &str,
    ) -> Result<ActivityTrend, ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Calculate period boundaries using SQLite date functions
        let (current_start, previous_start): (String, String) = conn.query_row(
            "SELECT
                datetime(?, '-' || ? || ' days') as current_start,
                datetime(?, '-' || (? * 2) || ' days') as previous_start",
            rusqlite::params![current_time, period_days, current_time, period_days],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        // Count unique days in current period
        let current_period_days: i32 = conn.query_row(
            "SELECT COUNT(DISTINCT DATE(logged_at))
             FROM activity_logs
             WHERE activity_id = ?
               AND logged_at >= ?
               AND logged_at <= ?
               AND deleted_at IS NULL",
            rusqlite::params![activity_id, &current_start, current_time],
            |row| row.get(0),
        )?;

        // Count unique days in previous period
        let previous_period_days: i32 = conn.query_row(
            "SELECT COUNT(DISTINCT DATE(logged_at))
             FROM activity_logs
             WHERE activity_id = ?
               AND logged_at >= ?
               AND logged_at < ?
               AND deleted_at IS NULL",
            rusqlite::params![activity_id, &previous_start, &current_start],
            |row| row.get(0),
        )?;

        // Calculate change
        let change_days = current_period_days - previous_period_days;

        // Calculate percentage change (handle division by zero)
        let change_percentage = if previous_period_days == 0 {
            if current_period_days > 0 {
                100.0 // Improvement from nothing
            } else {
                0.0 // No change (both zero)
            }
        } else {
            ((change_days as f64) / (previous_period_days as f64)) * 100.0
        };

        // Determine trend (threshold: ±10%)
        let trend = if change_percentage > 10.0 {
            Trend::Improving
        } else if change_percentage < -10.0 {
            Trend::Declining
        } else {
            Trend::Stable
        };

        Ok(ActivityTrend {
            activity_id,
            current_period_days,
            previous_period_days,
            change_days,
            change_percentage,
            trend,
        })
    }

    /// Checks progress toward a goal.
    ///
    /// # Arguments
    /// * `goal_id` - Goal ID to check
    /// * `current_time` - Time to measure progress at (ISO 8601)
    ///
    /// # Returns
    /// * `Ok(GoalProgress)` - Progress report
    /// * `Err(ActivityError::GoalNotFound)` - If goal doesn't exist
    pub fn check_goal_progress(
        &self,
        goal_id: i32,
        current_time: &str,
    ) -> Result<GoalProgress, ActivityError> {
        let conn = self.db.get_connection();
        let conn = conn.lock();

        // Fetch goal details
        let goal: ActivityGoal = conn
            .query_row(
                "SELECT id, activity_id, group_id, goal_type, target_value, period_days,
                        CAST(created_at AS VARCHAR), CAST(deleted_at AS VARCHAR)
                 FROM activity_goals
                 WHERE id = ? AND deleted_at IS NULL",
                rusqlite::params![goal_id],
                |row| {
                    Ok(ActivityGoal {
                        id: row.get(0)?,
                        activity_id: row.get(1)?,
                        group_id: row.get(2)?,
                        goal_type: row.get(3)?,
                        target_value: row.get(4)?,
                        period_days: row.get(5)?,
                        created_at: row.get(6)?,
                        deleted_at: row.get(7)?,
                    })
                },
            )
            .optional()?
            .ok_or(ActivityError::GoalNotFound(goal_id))?;

        // Calculate period boundaries
        let period_start: String = conn.query_row(
            "SELECT datetime(?, '-' || ? || ' days')",
            rusqlite::params![current_time, goal.period_days],
            |row| row.get(0),
        )?;

        // Determine which activities to count
        let activity_ids: Vec<i32> = if let Some(aid) = goal.activity_id {
            vec![aid]
        } else if let Some(gid) = goal.group_id {
            // Get all activities in the group
            let mut stmt = conn
                .prepare("SELECT id FROM activities WHERE group_id = ? AND deleted_at IS NULL")?;
            let rows = stmt.query_map(rusqlite::params![gid], |row| row.get(0))?;
            rows.collect::<Result<Vec<_>, _>>()?
        } else {
            return Err(ActivityError::MissingGoalTarget);
        };

        // Calculate current_value based on goal_type
        let current_value = match goal.goal_type {
            GoalType::DaysPerPeriod => self.calculate_unique_days_for_activities_with_conn(
                &conn,
                &activity_ids,
                &period_start,
                current_time,
            )?,
            GoalType::PercentImprovement => {
                // Calculate previous period boundaries
                let previous_period_start: String = conn.query_row(
                    "SELECT datetime(?, '-' || (? * 2) || ' days')",
                    rusqlite::params![current_time, goal.period_days],
                    |row| row.get(0),
                )?;

                // Get unique days for previous and current periods
                let previous_days = self.calculate_unique_days_for_activities_with_conn(
                    &conn,
                    &activity_ids,
                    &previous_period_start,
                    &period_start,
                )?;
                let current_days = self.calculate_unique_days_for_activities_with_conn(
                    &conn,
                    &activity_ids,
                    &period_start,
                    current_time,
                )?;

                // Calculate improvement percentage
                if previous_days == 0 {
                    if current_days > 0 {
                        100 // 100% improvement from zero
                    } else {
                        0 // No improvement
                    }
                } else {
                    let improvement =
                        ((current_days - previous_days) as f64 / previous_days as f64) * 100.0;
                    improvement.round() as i32
                }
            }
        };

        // Calculate progress percentage
        let percentage = (current_value as f64 / goal.target_value as f64) * 100.0;
        let is_achieved = percentage >= 100.0;

        Ok(GoalProgress {
            goal_id,
            current_value,
            target_value: goal.target_value,
            percentage,
            is_achieved,
            period_start,
            period_end: current_time.to_string(),
        })
    }

    /// Helper: Counts unique days with activity logs for given activities.
    ///
    /// # Notes
    /// This is a `_with_conn` helper - it accepts a connection reference
    /// and does NOT acquire locks. Only use from public methods that already hold the lock.
    fn calculate_unique_days_for_activities_with_conn(
        &self,
        conn: &rusqlite::Connection,
        activity_ids: &[i32],
        start_date: &str,
        end_date: &str,
    ) -> Result<i32, ActivityError> {
        if activity_ids.is_empty() {
            return Ok(0);
        }

        // Build placeholders for IN clause
        let placeholders = activity_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            "SELECT COUNT(DISTINCT DATE(logged_at))
             FROM activity_logs
             WHERE activity_id IN ({})
               AND logged_at >= ?
               AND logged_at <= ?
               AND deleted_at IS NULL",
            placeholders
        );

        let mut stmt = conn.prepare(&query)?;

        // Build params: activity_ids + start_date + end_date
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        for aid in activity_ids {
            params.push(Box::new(*aid));
        }
        params.push(Box::new(start_date.to_string()));
        params.push(Box::new(end_date.to_string()));

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let unique_days: i32 = stmt.query_row(params_refs.as_slice(), |row| row.get(0))?;

        Ok(unique_days)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use std::sync::Arc;
    use tempfile::TempDir;

    fn setup_test_repo() -> (ActivityRepository, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db = Arc::new(
            Database::new(temp_dir.path().to_path_buf()).expect("Failed to create database"),
        );
        (ActivityRepository::new(db), temp_dir)
    }

    #[test]
    fn test_create_activity_group() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", Some("Physical activities"))
            .expect("Failed to create activity group");

        assert_eq!(group.name, "Exercise");
        assert_eq!(group.description, Some("Physical activities".to_string()));
        assert!(group.id > 0);
        assert!(group.deleted_at.is_none());
    }

    #[test]
    fn test_create_activity_group_empty_name() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.create_activity_group("", None);

        assert!(matches!(result, Err(ActivityError::EmptyGroupName)));
    }

    #[test]
    fn test_create_activity_group_name_too_long() {
        let (repo, _temp_dir) = setup_test_repo();

        let long_name = "a".repeat(101);
        let result = repo.create_activity_group(&long_name, None);

        assert!(matches!(result, Err(ActivityError::GroupNameTooLong(101))));
    }

    #[test]
    fn test_update_activity_group() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", Some("Physical activities"))
            .expect("Failed to create activity group");

        let updated = repo
            .update_activity_group(group.id, Some("Workout"), Some("All workouts"))
            .expect("Failed to update activity group");

        assert_eq!(updated.id, group.id);
        assert_eq!(updated.name, "Workout");
        assert_eq!(updated.description, Some("All workouts".to_string()));
    }

    #[test]
    fn test_update_activity_group_not_found() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.update_activity_group(999, Some("Test"), None);

        assert!(matches!(result, Err(ActivityError::GroupNotFound(999))));
    }

    #[test]
    fn test_delete_activity_group() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create activity group");

        repo.delete_activity_group(group.id)
            .expect("Failed to delete activity group");

        // Verify it's no longer in the list
        let groups = repo.get_activity_groups().expect("Failed to get groups");
        assert!(!groups.iter().any(|g| g.id == group.id));
    }

    #[test]
    fn test_delete_activity_group_not_found() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.delete_activity_group(999);

        assert!(matches!(result, Err(ActivityError::GroupNotFound(999))));
    }

    #[test]
    fn test_get_activity_groups() {
        let (repo, _temp_dir) = setup_test_repo();

        repo.create_activity_group("Exercise", None)
            .expect("Failed to create group");
        repo.create_activity_group("Social", None)
            .expect("Failed to create group");
        repo.create_activity_group("Work", None)
            .expect("Failed to create group");

        let groups = repo.get_activity_groups().expect("Failed to get groups");

        assert_eq!(groups.len(), 3);
        // Should be sorted by name
        assert_eq!(groups[0].name, "Exercise");
        assert_eq!(groups[1].name, "Social");
        assert_eq!(groups[2].name, "Work");
    }

    #[test]
    fn test_get_activity_groups_excludes_deleted() {
        let (repo, _temp_dir) = setup_test_repo();

        let group1 = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        repo.create_activity_group("Social", None)
            .expect("Failed to create group");

        // Delete first group
        repo.delete_activity_group(group1.id)
            .expect("Failed to delete group");

        let groups = repo.get_activity_groups().expect("Failed to get groups");

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "Social");
    }

    #[test]
    fn test_cascading_deletes() {
        // This test verifies CASCADE delete behavior defined in migration 003_activity_groups.sql
        // Full cascade testing requires Activity creation methods (not yet implemented)
        // For now, verify database schema has CASCADE constraints
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        // Verify the CASCADE constraint exists in the schema
        let conn = repo.db.get_connection();
        let conn = conn.lock();

        // Query sqlite_master to verify FOREIGN KEY with CASCADE is defined
        let has_cascade: bool = conn
            .query_row(
                "SELECT sql FROM sqlite_master WHERE type='table' AND name='activities'",
                [],
                |row| {
                    let sql: String = row.get(0)?;
                    Ok(sql.contains("ON DELETE CASCADE"))
                },
            )
            .unwrap_or(false);

        assert!(
            has_cascade,
            "activities table should have ON DELETE CASCADE constraint"
        );

        // Soft delete the group (actual CASCADE testing requires Activity methods)
        drop(conn); // Release lock before calling repository method
        repo.delete_activity_group(group.id)
            .expect("Failed to delete group");

        // Verify group is soft deleted
        let groups = repo.get_activity_groups().expect("Failed to get groups");
        assert!(!groups.iter().any(|g| g.id == group.id));

        // NOTE: Full CASCADE delete testing will be added when Activity CRUD methods
        // are implemented (tasks 1.21-1.29). This will include:
        // 1. Creating activities linked to this group
        // 2. Deleting the group
        // 3. Verifying all linked activities are CASCADE deleted
    }

    #[test]
    fn test_create_activity_group_description_too_long() {
        let (repo, _temp_dir) = setup_test_repo();

        let long_description = "a".repeat(501);
        let result = repo.create_activity_group("Exercise", Some(&long_description));

        assert!(matches!(
            result,
            Err(ActivityError::DescriptionTooLong(501))
        ));
    }

    #[test]
    fn test_update_activity_group_partial_name_only() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", Some("Original description"))
            .expect("Failed to create group");

        let updated = repo
            .update_activity_group(group.id, Some("Updated Name"), None)
            .expect("Failed to update");

        assert_eq!(updated.name, "Updated Name");
        assert_eq!(
            updated.description,
            Some("Original description".to_string())
        );
    }

    #[test]
    fn test_update_activity_group_partial_description_only() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", Some("Original"))
            .expect("Failed to create group");

        let updated = repo
            .update_activity_group(group.id, None, Some("Updated description"))
            .expect("Failed to update");

        assert_eq!(updated.name, "Exercise");
        assert_eq!(updated.description, Some("Updated description".to_string()));
    }

    #[test]
    fn test_create_activity_group_utf8_characters() {
        let (repo, _temp_dir) = setup_test_repo();

        // Test with emojis and multi-byte characters
        let name_with_emoji = "Exercise 💪";
        let description_with_unicode = "健康とフィットネス"; // Japanese characters

        let group = repo
            .create_activity_group(name_with_emoji, Some(description_with_unicode))
            .expect("Failed to create group with UTF-8");

        assert_eq!(group.name, name_with_emoji);
        assert_eq!(
            group.description,
            Some(description_with_unicode.to_string())
        );
    }

    #[test]
    fn test_create_activity_group_utf8_length_validation() {
        let (repo, _temp_dir) = setup_test_repo();

        // Create a name with exactly 100 emoji characters (which are 4 bytes each)
        // This tests that we're counting characters, not bytes
        let emoji_name = "😀".repeat(100);
        let result = repo.create_activity_group(&emoji_name, None);
        assert!(result.is_ok(), "Should accept 100 emoji characters");

        // 101 emoji characters should fail
        let emoji_name_too_long = "😀".repeat(101);
        let result = repo.create_activity_group(&emoji_name_too_long, None);
        assert!(
            matches!(result, Err(ActivityError::GroupNameTooLong(101))),
            "Should reject 101 emoji characters"
        );
    }

    #[test]
    fn test_update_activity_group_description_too_long() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let long_description = "a".repeat(501);
        let result = repo.update_activity_group(group.id, None, Some(&long_description));

        assert!(matches!(
            result,
            Err(ActivityError::DescriptionTooLong(501))
        ));
    }

    // Activity and ActivityLog Tests

    #[test]
    fn test_create_activity_with_group_id() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let activity = repo
            .create_activity(group.id, "Running", Some("#FF0000"), Some("fire"))
            .expect("Failed to create activity");

        assert_eq!(activity.name, "Running");
        assert_eq!(activity.group_id, group.id);
        assert_eq!(activity.color, Some("#FF0000".to_string()));
        assert_eq!(activity.icon, Some("fire".to_string()));
        assert!(activity.id > 0);
    }

    #[test]
    fn test_create_activity_empty_name() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let result = repo.create_activity(group.id, "", None, None);

        assert!(matches!(result, Err(ActivityError::EmptyActivityName)));
    }

    #[test]
    fn test_create_activity_name_too_long() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let long_name = "a".repeat(51);
        let result = repo.create_activity(group.id, &long_name, None, None);

        assert!(matches!(
            result,
            Err(ActivityError::ActivityNameTooLong(51))
        ));
    }

    #[test]
    fn test_create_activity_icon_too_long() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let long_icon = "a".repeat(21);
        let result = repo.create_activity(group.id, "Running", None, Some(&long_icon));

        assert!(matches!(
            result,
            Err(ActivityError::ActivityIconTooLong(21))
        ));
    }

    #[test]
    fn test_create_activity_group_not_found() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.create_activity(999, "Running", None, None);

        assert!(matches!(result, Err(ActivityError::GroupNotFound(999))));
    }

    #[test]
    fn test_get_activities_by_group() {
        let (repo, _temp_dir) = setup_test_repo();

        let group1 = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let group2 = repo
            .create_activity_group("Social", None)
            .expect("Failed to create group");

        repo.create_activity(group1.id, "Running", None, None)
            .expect("Failed to create activity");
        repo.create_activity(group1.id, "Swimming", None, None)
            .expect("Failed to create activity");
        repo.create_activity(group2.id, "Party", None, None)
            .expect("Failed to create activity");

        let group1_activities = repo
            .get_activities_by_group(group1.id)
            .expect("Failed to get activities");

        assert_eq!(group1_activities.len(), 2);
        assert_eq!(group1_activities[0].name, "Running");
        assert_eq!(group1_activities[1].name, "Swimming");
    }

    #[test]
    fn test_log_activity() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let logged_at = "2025-01-15T10:00:00Z";
        let log = repo
            .log_activity(activity.id, logged_at, Some("Felt great!"))
            .expect("Failed to log activity");

        assert_eq!(log.activity_id, activity.id);
        assert_eq!(log.logged_at, logged_at);
        assert_eq!(log.notes, Some("Felt great!".to_string()));
        assert!(log.id > 0);
    }

    #[test]
    fn test_log_activity_notes_too_long() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let long_notes = "a".repeat(501);
        let result = repo.log_activity(activity.id, "2025-01-15T10:00:00Z", Some(&long_notes));

        assert!(matches!(
            result,
            Err(ActivityError::NotesLengthExceeded(501))
        ));
    }

    #[test]
    fn test_log_activity_not_found() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.log_activity(999, "2025-01-15T10:00:00Z", None);

        assert!(matches!(result, Err(ActivityError::ActivityNotFound(999))));
    }

    #[test]
    fn test_get_activity_logs_no_filter() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        repo.log_activity(activity.id, "2025-01-15T10:00:00Z", Some("Morning run"))
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-16T10:00:00Z", Some("Evening run"))
            .expect("Failed to log");

        let logs = repo
            .get_activity_logs(None, None, None)
            .expect("Failed to get logs");

        assert_eq!(logs.len(), 2);
        // Should be ordered by logged_at DESC
        assert_eq!(logs[0].logged_at, "2025-01-16T10:00:00Z");
        assert_eq!(logs[1].logged_at, "2025-01-15T10:00:00Z");
    }

    #[test]
    fn test_get_activity_logs_with_activity_filter() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity1 = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");
        let activity2 = repo
            .create_activity(group.id, "Swimming", None, None)
            .expect("Failed to create activity");

        repo.log_activity(activity1.id, "2025-01-15T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity2.id, "2025-01-15T11:00:00Z", None)
            .expect("Failed to log");

        let logs = repo
            .get_activity_logs(Some(activity1.id), None, None)
            .expect("Failed to get logs");

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].activity_id, activity1.id);
    }

    #[test]
    fn test_get_activity_logs_with_date_filter() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        repo.log_activity(activity.id, "2025-01-15T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-17T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-20T10:00:00Z", None)
            .expect("Failed to log");

        let logs = repo
            .get_activity_logs(None, Some("2025-01-16"), Some("2025-01-19"))
            .expect("Failed to get logs");

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].logged_at, "2025-01-17T10:00:00Z");
    }

    #[test]
    fn test_activity_utf8_characters() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let name_with_emoji = "Running 🏃";
        let icon_with_emoji = "🏃";

        let activity = repo
            .create_activity(group.id, name_with_emoji, None, Some(icon_with_emoji))
            .expect("Failed to create activity with UTF-8");

        assert_eq!(activity.name, name_with_emoji);
        assert_eq!(activity.icon, Some(icon_with_emoji.to_string()));
    }

    #[test]
    fn test_activity_name_utf8_length_validation() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        // 50 emoji characters (4 bytes each) should be accepted
        let emoji_name = "😀".repeat(50);
        let result = repo.create_activity(group.id, &emoji_name, None, None);
        assert!(result.is_ok(), "Should accept 50 emoji characters");

        // 51 emoji characters should fail
        let emoji_name_too_long = "😀".repeat(51);
        let result = repo.create_activity(group.id, &emoji_name_too_long, None, None);
        assert!(
            matches!(result, Err(ActivityError::ActivityNameTooLong(51))),
            "Should reject 51 emoji characters"
        );
    }

    #[test]
    fn test_create_activity_trims_whitespace() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        // Test name trimming
        let activity = repo
            .create_activity(group.id, "  Running  ", None, None)
            .expect("Failed to create activity");
        assert_eq!(activity.name, "Running", "Name should be trimmed");

        // Test icon trimming
        let activity = repo
            .create_activity(group.id, "Cycling", None, Some("  🚴  "))
            .expect("Failed to create activity");
        assert_eq!(
            activity.icon,
            Some("🚴".to_string()),
            "Icon should be trimmed"
        );
    }

    #[test]
    fn test_create_activity_whitespace_only_name_fails() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        // Whitespace-only name should fail after trimming
        let result = repo.create_activity(group.id, "     ", None, None);
        assert!(
            matches!(result, Err(ActivityError::EmptyActivityName)),
            "Whitespace-only name should be rejected"
        );
    }

    #[test]
    fn test_create_activity_whitespace_only_icon_becomes_none() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        // Whitespace-only icon should become None after trimming
        let activity = repo
            .create_activity(group.id, "Running", None, Some("     "))
            .expect("Failed to create activity");
        assert_eq!(
            activity.icon, None,
            "Whitespace-only icon should become None"
        );
    }

    #[test]
    fn test_log_activity_trims_notes() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Test notes trimming
        let log = repo
            .log_activity(
                activity.id,
                "2025-01-15T10:00:00Z",
                Some("  Great workout!  "),
            )
            .expect("Failed to log activity");

        assert_eq!(
            log.notes,
            Some("Great workout!".to_string()),
            "Notes should be trimmed"
        );
    }

    #[test]
    fn test_log_activity_whitespace_only_notes_becomes_none() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Whitespace-only notes should become None after trimming
        let log = repo
            .log_activity(activity.id, "2025-01-15T10:00:00Z", Some("     "))
            .expect("Failed to log activity");

        assert_eq!(log.notes, None, "Whitespace-only notes should become None");
    }

    // ========================================
    // Activity Goals Tests
    // ========================================

    #[test]
    fn test_set_activity_goal_for_activity() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Set activity-level goal: 3 days per week
        let goal = repo
            .set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 3, 7)
            .expect("Failed to set activity goal");

        assert_eq!(goal.activity_id, Some(activity.id));
        assert_eq!(goal.group_id, None);
        assert_eq!(goal.goal_type, GoalType::DaysPerPeriod);
        assert_eq!(goal.target_value, 3);
        assert_eq!(goal.period_days, 7);
        assert!(goal.id > 0);
        assert!(goal.deleted_at.is_none());
    }

    #[test]
    fn test_set_activity_goal_for_group() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Social", None)
            .expect("Failed to create group");

        // Set group-level goal: 5 days per 14-day period
        let goal = repo
            .set_activity_goal(None, Some(group.id), GoalType::DaysPerPeriod, 5, 14)
            .expect("Failed to set group goal");

        assert_eq!(goal.activity_id, None);
        assert_eq!(goal.group_id, Some(group.id));
        assert_eq!(goal.goal_type, GoalType::DaysPerPeriod);
        assert_eq!(goal.target_value, 5);
        assert_eq!(goal.period_days, 14);
    }

    #[test]
    fn test_set_activity_goal_percent_improvement_type() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Meditation", None, None)
            .expect("Failed to create activity");

        // Set percent improvement goal: 20% improvement over 30 days
        let goal = repo
            .set_activity_goal(
                Some(activity.id),
                None,
                GoalType::PercentImprovement,
                20,
                30,
            )
            .expect("Failed to set percent improvement goal");

        assert_eq!(goal.goal_type, GoalType::PercentImprovement);
        assert_eq!(goal.target_value, 20);
        assert_eq!(goal.period_days, 30);
    }

    #[test]
    fn test_set_activity_goal_both_targets_rejected() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Cannot set goal for both activity AND group
        let result = repo.set_activity_goal(
            Some(activity.id),
            Some(group.id),
            GoalType::DaysPerPeriod,
            3,
            7,
        );

        assert!(matches!(result, Err(ActivityError::InvalidGoalTarget)));
    }

    #[test]
    fn test_set_activity_goal_no_targets_rejected() {
        let (repo, _temp_dir) = setup_test_repo();

        // Must set goal for either activity OR group
        let result = repo.set_activity_goal(None, None, GoalType::DaysPerPeriod, 3, 7);

        assert!(matches!(result, Err(ActivityError::MissingGoalTarget)));
    }

    #[test]
    fn test_set_activity_goal_negative_target_rejected() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let result =
            repo.set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, -5, 7);

        assert!(matches!(result, Err(ActivityError::InvalidTargetValue(-5))));
    }

    #[test]
    fn test_set_activity_goal_zero_period_rejected() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let result = repo.set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 3, 0);

        assert!(matches!(result, Err(ActivityError::InvalidPeriodDays(0))));
    }

    #[test]
    fn test_get_activity_goals_by_activity() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let goal1 = repo
            .set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 3, 7)
            .expect("Failed to set goal 1");
        let goal2 = repo
            .set_activity_goal(
                Some(activity.id),
                None,
                GoalType::PercentImprovement,
                20,
                30,
            )
            .expect("Failed to set goal 2");

        let goals = repo
            .get_activity_goals(Some(activity.id), None)
            .expect("Failed to get activity goals");

        assert_eq!(goals.len(), 2);
        assert!(goals.iter().any(|g| g.id == goal1.id));
        assert!(goals.iter().any(|g| g.id == goal2.id));
    }

    #[test]
    fn test_get_activity_goals_by_group() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Social", None)
            .expect("Failed to create group");

        let goal = repo
            .set_activity_goal(None, Some(group.id), GoalType::DaysPerPeriod, 5, 14)
            .expect("Failed to set group goal");

        let goals = repo
            .get_activity_goals(None, Some(group.id))
            .expect("Failed to get group goals");

        assert_eq!(goals.len(), 1);
        assert_eq!(goals[0].id, goal.id);
        assert_eq!(goals[0].group_id, Some(group.id));
    }

    #[test]
    fn test_get_activity_goals_excludes_deleted() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let goal = repo
            .set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 3, 7)
            .expect("Failed to set goal");

        // Delete the goal
        repo.delete_activity_goal(goal.id)
            .expect("Failed to delete goal");

        // Should not appear in results
        let goals = repo
            .get_activity_goals(Some(activity.id), None)
            .expect("Failed to get goals");

        assert_eq!(goals.len(), 0);
    }

    #[test]
    fn test_update_activity_goal() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let goal = repo
            .set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 3, 7)
            .expect("Failed to set goal");

        // Update target to 5 days per 14-day period
        let updated = repo
            .update_activity_goal(goal.id, 5, 14)
            .expect("Failed to update goal");

        assert_eq!(updated.id, goal.id);
        assert_eq!(updated.target_value, 5);
        assert_eq!(updated.period_days, 14);
        assert_eq!(updated.goal_type, GoalType::DaysPerPeriod); // Unchanged
    }

    #[test]
    fn test_update_activity_goal_not_found() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.update_activity_goal(999, 5, 14);

        assert!(matches!(result, Err(ActivityError::GoalNotFound(999))));
    }

    #[test]
    fn test_delete_activity_goal() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let goal = repo
            .set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 3, 7)
            .expect("Failed to set goal");

        repo.delete_activity_goal(goal.id)
            .expect("Failed to delete goal");

        // Verify it's soft-deleted (not in active list)
        let goals = repo
            .get_activity_goals(Some(activity.id), None)
            .expect("Failed to get goals");
        assert!(!goals.iter().any(|g| g.id == goal.id));
    }

    #[test]
    fn test_delete_activity_goal_not_found() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.delete_activity_goal(999);

        assert!(matches!(result, Err(ActivityError::GoalNotFound(999))));
    }

    // ========================================
    // Activity Frequency Tests
    // ========================================

    #[test]
    fn test_get_activity_frequency_basic() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Log activity on 4 different days within a 14-day period
        repo.log_activity(activity.id, "2025-01-01T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-02T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-02T15:00:00Z", None)
            .expect("Failed to log"); // Same day - should count as 1
        repo.log_activity(activity.id, "2025-01-04T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-08T10:00:00Z", None)
            .expect("Failed to log");

        let frequency = repo
            .get_activity_frequency(activity.id, "2025-01-01T00:00:00Z", "2025-01-14T23:59:59Z")
            .expect("Failed to get frequency");

        assert_eq!(frequency.activity_id, activity.id);
        assert_eq!(frequency.unique_days, 4);
        assert_eq!(frequency.total_logs, 5);
        // 14 days = 2 weeks, 4 unique days / 2 weeks = 2.0 days/week
        assert!((frequency.days_per_week - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_get_activity_frequency_no_logs() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let frequency = repo
            .get_activity_frequency(activity.id, "2025-01-01T00:00:00Z", "2025-01-14T23:59:59Z")
            .expect("Failed to get frequency");

        assert_eq!(frequency.unique_days, 0);
        assert_eq!(frequency.total_logs, 0);
        assert_eq!(frequency.days_per_week, 0.0);
    }

    #[test]
    fn test_get_activity_frequency_partial_week() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // 3 days logged within 3.5 day period (0.5 weeks)
        repo.log_activity(activity.id, "2025-01-01T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-02T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-03T10:00:00Z", None)
            .expect("Failed to log");

        let frequency = repo
            .get_activity_frequency(activity.id, "2025-01-01T00:00:00Z", "2025-01-04T12:00:00Z")
            .expect("Failed to get frequency");

        assert_eq!(frequency.unique_days, 3);
        // 3.5 days = 0.5 weeks, 3 days / 0.5 weeks = 6 days/week
        assert!((frequency.days_per_week - 6.0).abs() < 0.1);
    }

    // ========================================
    // Activity Trend Tests
    // ========================================

    #[test]
    fn test_get_activity_trend_improving() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Previous period (Jan 1-7): 3 unique days
        repo.log_activity(activity.id, "2025-01-01T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-03T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-05T10:00:00Z", None)
            .expect("Failed to log");

        // Current period (Jan 8-14): 5 unique days
        repo.log_activity(activity.id, "2025-01-08T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-09T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-10T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-12T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-14T10:00:00Z", None)
            .expect("Failed to log");

        let trend = repo
            .get_activity_trend(activity.id, 7, "2025-01-15T00:00:00Z")
            .expect("Failed to get trend");

        assert_eq!(trend.activity_id, activity.id);
        assert_eq!(trend.current_period_days, 5);
        assert_eq!(trend.previous_period_days, 3);
        assert_eq!(trend.change_days, 2);
        // (5 - 3) / 3 * 100 = 66.67%
        assert!((trend.change_percentage - 66.67).abs() < 0.1);
        assert_eq!(trend.trend, Trend::Improving);
    }

    #[test]
    fn test_get_activity_trend_declining() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Previous period: 5 days
        repo.log_activity(activity.id, "2025-01-01T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-02T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-03T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-04T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-05T10:00:00Z", None)
            .expect("Failed to log");

        // Current period: 2 days
        repo.log_activity(activity.id, "2025-01-08T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-10T10:00:00Z", None)
            .expect("Failed to log");

        let trend = repo
            .get_activity_trend(activity.id, 7, "2025-01-15T00:00:00Z")
            .expect("Failed to get trend");

        assert_eq!(trend.previous_period_days, 5);
        assert_eq!(trend.current_period_days, 2);
        assert_eq!(trend.change_days, -3);
        // (2 - 5) / 5 * 100 = -60%
        assert!((trend.change_percentage + 60.0).abs() < 0.1);
        assert_eq!(trend.trend, Trend::Declining);
    }

    #[test]
    fn test_get_activity_trend_stable() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Previous period: 4 days
        repo.log_activity(activity.id, "2025-01-01T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-02T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-03T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-04T10:00:00Z", None)
            .expect("Failed to log");

        // Current period: 4 days
        repo.log_activity(activity.id, "2025-01-08T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-09T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-10T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-11T10:00:00Z", None)
            .expect("Failed to log");

        let trend = repo
            .get_activity_trend(activity.id, 7, "2025-01-15T00:00:00Z")
            .expect("Failed to get trend");

        assert_eq!(trend.previous_period_days, 4);
        assert_eq!(trend.current_period_days, 4);
        assert_eq!(trend.change_days, 0);
        assert_eq!(trend.change_percentage, 0.0);
        assert_eq!(trend.trend, Trend::Stable);
    }

    #[test]
    fn test_get_activity_trend_from_zero() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Previous period: 0 days
        // Current period: 3 days
        repo.log_activity(activity.id, "2025-01-08T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-10T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-12T10:00:00Z", None)
            .expect("Failed to log");

        let trend = repo
            .get_activity_trend(activity.id, 7, "2025-01-15T00:00:00Z")
            .expect("Failed to get trend");

        assert_eq!(trend.previous_period_days, 0);
        assert_eq!(trend.current_period_days, 3);
        // From 0 to 3 = 100% improvement (per algorithm spec)
        assert_eq!(trend.change_percentage, 100.0);
        assert_eq!(trend.trend, Trend::Improving);
    }

    // ========================================
    // Goal Progress Tests
    // ========================================

    #[test]
    fn test_check_goal_progress_days_per_period() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        // Set goal: 5 days per 7-day period
        let goal = repo
            .set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 5, 7)
            .expect("Failed to set goal");

        // Log activity on 4 unique days in last 7 days
        repo.log_activity(activity.id, "2025-01-09T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-10T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-10T15:00:00Z", None)
            .expect("Failed to log"); // Same day
        repo.log_activity(activity.id, "2025-01-12T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-14T10:00:00Z", None)
            .expect("Failed to log");

        let progress = repo
            .check_goal_progress(goal.id, "2025-01-15T00:00:00Z")
            .expect("Failed to check goal progress");

        assert_eq!(progress.goal_id, goal.id);
        assert_eq!(progress.current_value, 4); // 4 unique days
        assert_eq!(progress.target_value, 5);
        assert_eq!(progress.percentage, 80.0); // 4/5 * 100
        assert!(!progress.is_achieved);
    }

    #[test]
    fn test_check_goal_progress_days_per_period_achieved() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Running", None, None)
            .expect("Failed to create activity");

        let goal = repo
            .set_activity_goal(Some(activity.id), None, GoalType::DaysPerPeriod, 3, 7)
            .expect("Failed to set goal");

        // Log activity on 3 days - exactly meets goal
        repo.log_activity(activity.id, "2025-01-09T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-11T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-14T10:00:00Z", None)
            .expect("Failed to log");

        let progress = repo
            .check_goal_progress(goal.id, "2025-01-15T00:00:00Z")
            .expect("Failed to check goal progress");

        assert_eq!(progress.current_value, 3);
        assert_eq!(progress.target_value, 3);
        assert_eq!(progress.percentage, 100.0);
        assert!(progress.is_achieved);
    }

    #[test]
    fn test_check_goal_progress_percent_improvement() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");
        let activity = repo
            .create_activity(group.id, "Meditation", None, None)
            .expect("Failed to create activity");

        // Goal: 20% improvement over 7-day baseline
        let goal = repo
            .set_activity_goal(Some(activity.id), None, GoalType::PercentImprovement, 20, 7)
            .expect("Failed to set goal");

        // Previous period (Jan 1-7): 3 unique days
        repo.log_activity(activity.id, "2025-01-01T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-03T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-05T10:00:00Z", None)
            .expect("Failed to log");

        // Current period (Jan 8-14): 4 unique days
        repo.log_activity(activity.id, "2025-01-08T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-10T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-12T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity.id, "2025-01-14T10:00:00Z", None)
            .expect("Failed to log");

        let progress = repo
            .check_goal_progress(goal.id, "2025-01-15T00:00:00Z")
            .expect("Failed to check goal progress");

        assert_eq!(progress.goal_id, goal.id);
        // Improvement: (4 - 3) / 3 * 100 = 33.33%
        assert_eq!(progress.current_value, 33);
        assert_eq!(progress.target_value, 20);
        // 33 / 20 * 100 = 165%
        assert!((progress.percentage - 165.0).abs() < 1.0);
        assert!(progress.is_achieved);
    }

    #[test]
    fn test_check_goal_progress_group_goal() {
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Social", None)
            .expect("Failed to create group");
        let activity1 = repo
            .create_activity(group.id, "Coffee with friend", None, None)
            .expect("Failed to create activity 1");
        let activity2 = repo
            .create_activity(group.id, "Phone call", None, None)
            .expect("Failed to create activity 2");

        // Group goal: any social activity 4 days per week
        let goal = repo
            .set_activity_goal(None, Some(group.id), GoalType::DaysPerPeriod, 4, 7)
            .expect("Failed to set group goal");

        // Log different activities in the group
        repo.log_activity(activity1.id, "2025-01-09T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity1.id, "2025-01-10T10:00:00Z", None)
            .expect("Failed to log");
        repo.log_activity(activity2.id, "2025-01-12T10:00:00Z", None)
            .expect("Failed to log");

        let progress = repo
            .check_goal_progress(goal.id, "2025-01-15T00:00:00Z")
            .expect("Failed to check goal progress");

        assert_eq!(progress.current_value, 3); // 3 unique days across group
        assert_eq!(progress.target_value, 4);
        assert_eq!(progress.percentage, 75.0);
        assert!(!progress.is_achieved);
    }

    #[test]
    fn test_check_goal_progress_not_found() {
        let (repo, _temp_dir) = setup_test_repo();

        let result = repo.check_goal_progress(999, "2025-01-15T00:00:00Z");

        assert!(matches!(result, Err(ActivityError::GoalNotFound(999))));
    }
}
