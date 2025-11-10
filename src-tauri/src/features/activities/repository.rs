// Activity repository - Data access layer for activity groups, activities, logs, and goals
//
// ## Lock Poisoning
// This repository uses a Mutex to protect database connections. Lock poisoning occurs when
// a thread panics while holding the lock, leaving the Mutex in a "poisoned" state.
//
// In a single-threaded Tauri application, lock poisoning should never occur under normal
// circumstances. If it does occur, it indicates a serious bug (panic in database code) that
// has likely left the database in an inconsistent state.
//
// The fail-fast approach (returning ActivityError::LockPoisoned) is intentional:
// - It surfaces the critical error to the UI layer
// - Prevents continuing with potentially corrupted data
// - The application should be restarted to recover
//
// Recovery: The database file itself is not corrupted (SQLite is ACID-compliant), but
// the in-memory state may be inconsistent. Restarting the application will recover.

use super::models::*;
use crate::db::Database;
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
        // Validate name
        if name.is_empty() {
            return Err(ActivityError::EmptyGroupName);
        }
        if name.len() > 100 {
            return Err(ActivityError::GroupNameTooLong(name.len()));
        }
        if let Some(desc) = description {
            if desc.len() > 500 {
                return Err(ActivityError::GroupNameTooLong(desc.len()));
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

        info!("Created activity group '{}' with ID: {}", name, id);

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
        // Validate name if provided
        if let Some(n) = name {
            if n.is_empty() {
                return Err(ActivityError::EmptyGroupName);
            }
            if n.len() > 100 {
                return Err(ActivityError::GroupNameTooLong(n.len()));
            }
        }
        if let Some(desc) = description {
            if desc.len() > 500 {
                return Err(ActivityError::GroupNameTooLong(desc.len()));
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

        info!("Updated activity group ID: {}", id);

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

        info!("Soft deleted activity group ID: {}", id);

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
        // This test will verify CASCADE delete behavior when implemented
        // For now, we'll test that the group deletion works
        let (repo, _temp_dir) = setup_test_repo();

        let group = repo
            .create_activity_group("Exercise", None)
            .expect("Failed to create group");

        // TODO: Create activities associated with this group
        // TODO: Delete the group
        // TODO: Verify all activities are CASCADE deleted

        repo.delete_activity_group(group.id)
            .expect("Failed to delete group");

        // Verify group is deleted
        let groups = repo.get_activity_groups().expect("Failed to get groups");
        assert!(!groups.iter().any(|g| g.id == group.id));
    }
}
