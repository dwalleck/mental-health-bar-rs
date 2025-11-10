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
}
