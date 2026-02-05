// Mood commands - Write operations (Tauri commands)
// T080-T084: Mood command implementation

use super::models::*;
use super::repository::MoodRepository;
use super::repository_trait::MoodRepositoryTrait;
use crate::{
    errors::{ErrorType, ToCommandError},
    AppState, CommandError,
};
use tauri::State;
use tracing::error;
use validator::Validate;

// T080: log_mood command
#[tauri::command]
#[specta::specta]
pub async fn log_mood(
    request: LogMoodRequest,
    state: State<'_, AppState>,
) -> Result<MoodCheckin, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = MoodRepository::new(state.db.clone());
    log_mood_impl(&repo, &request).map_err(|e| {
        error!(
            "log_mood error: {} (rating: {}, activities: {}, has_notes: {})",
            e,
            request.mood_rating,
            request.activity_ids.len(),
            request.notes.is_some()
        );
        e.to_command_error()
    })
}

/// Business logic for logging mood - uses trait bound for testability
fn log_mood_impl(
    repo: &impl MoodRepositoryTrait,
    request: &LogMoodRequest,
) -> Result<MoodCheckin, MoodError> {
    repo.create_mood_checkin(
        request.mood_rating,
        request.activity_ids.clone(),
        request.notes.clone(),
    )
}

// T106: create_activity command
#[tauri::command]
#[specta::specta]
pub async fn create_activity(
    request: CreateActivityRequest,
    state: State<'_, AppState>,
) -> Result<Activity, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = MoodRepository::new(state.db.clone());
    create_activity_impl(&repo, &request).map_err(|e| {
        error!(
            "create_activity error: {} (name: '{}', has_color: {}, has_icon: {})",
            e,
            request.name,
            request.color.is_some(),
            request.icon.is_some()
        );
        e.to_command_error()
    })
}

/// Business logic for creating activity - uses trait bound for testability
fn create_activity_impl(
    repo: &impl MoodRepositoryTrait,
    request: &CreateActivityRequest,
) -> Result<Activity, MoodError> {
    repo.create_activity(
        request.name.clone(),
        request.color.as_ref().map(|c| c.value().to_string()),
        request.icon.clone(),
        request.group_id,
    )
}

// T107: update_activity command
#[tauri::command]
#[specta::specta]
pub async fn update_activity(
    id: i32,
    request: UpdateActivityRequest,
    state: State<'_, AppState>,
) -> Result<Activity, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = MoodRepository::new(state.db.clone());
    update_activity_impl(&repo, id, &request).map_err(|e| {
        error!(
            "update_activity error: {} (id: {}, name: {:?}, has_color: {}, has_icon: {})",
            e,
            id,
            request.name,
            request.color.is_some(),
            request.icon.is_some()
        );
        e.to_command_error()
    })
}

/// Business logic for updating activity - uses trait bound for testability
fn update_activity_impl(
    repo: &impl MoodRepositoryTrait,
    id: i32,
    request: &UpdateActivityRequest,
) -> Result<Activity, MoodError> {
    repo.update_activity(
        id,
        request.name.clone(),
        request.color.as_ref().map(|c| c.value().to_string()),
        request.icon.clone(),
    )
}

// T108: delete_activity command
#[tauri::command]
#[specta::specta]
pub async fn delete_activity(id: i32, state: State<'_, AppState>) -> Result<(), CommandError> {
    let repo = MoodRepository::new(state.db.clone());
    delete_activity_impl(&repo, id).map_err(|e| {
        error!("delete_activity error: {} (id: {})", e, id);
        e.to_command_error()
    })
}

/// Business logic for deleting activity - uses trait bound for testability
fn delete_activity_impl(repo: &impl MoodRepositoryTrait, id: i32) -> Result<(), MoodError> {
    repo.delete_activity(id)
}

// T093c: delete_mood_checkin command (cascade deletion)
#[tauri::command]
#[specta::specta]
pub async fn delete_mood_checkin(id: i32, state: State<'_, AppState>) -> Result<(), CommandError> {
    let repo = MoodRepository::new(state.db.clone());
    delete_mood_checkin_impl(&repo, id).map_err(|e| {
        error!("delete_mood_checkin error: {} (id: {})", e, id);
        e.to_command_error()
    })
}

/// Business logic for deleting mood check-in - uses trait bound for testability
fn delete_mood_checkin_impl(repo: &impl MoodRepositoryTrait, id: i32) -> Result<(), MoodError> {
    repo.delete_mood_checkin(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::mood::{repository_trait::MockMoodRepositoryTrait, MoodRepositoryTrait};
    use crate::types::activity::HexColor;
    use validator::Validate;

    // ========================================================================
    // Unit Tests: Command Validation (rating 1-5, string lengths)
    // ========================================================================

    #[test]
    fn test_log_mood_request_validation_invalid_rating_too_low() {
        let request = LogMoodRequest {
            mood_rating: 0,
            activity_ids: vec![],
            notes: None,
        };

        let validation = request.validate();
        assert!(validation.is_err());
        let errors = validation.unwrap_err();
        assert!(errors.field_errors().contains_key("mood_rating"));
    }

    #[test]
    fn test_log_mood_request_validation_invalid_rating_too_high() {
        let request = LogMoodRequest {
            mood_rating: 8, // Above maximum of 7
            activity_ids: vec![],
            notes: None,
        };

        let validation = request.validate();
        assert!(validation.is_err());
    }

    #[test]
    fn test_log_mood_request_validation_notes_too_long() {
        let long_notes = "a".repeat(5001);
        let request = LogMoodRequest {
            mood_rating: 3,
            activity_ids: vec![],
            notes: Some(long_notes),
        };

        let validation = request.validate();
        assert!(validation.is_err());
        let errors = validation.unwrap_err();
        assert!(errors.field_errors().contains_key("notes"));
    }

    #[test]
    fn test_log_mood_request_validation_valid() {
        let request = LogMoodRequest {
            mood_rating: 3,
            activity_ids: vec![1, 2],
            notes: Some("Feeling okay today".to_string()),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_activity_request_validation_empty_name() {
        let request = CreateActivityRequest {
            name: "".to_string(),
            color: None,
            icon: None,
            group_id: 1,
        };

        let validation = request.validate();
        assert!(validation.is_err());
    }

    #[test]
    fn test_create_activity_request_validation_whitespace_only_name() {
        let request = CreateActivityRequest {
            name: "   ".to_string(),
            color: None,
            icon: None,
            group_id: 1,
        };

        let validation = request.validate();
        assert!(validation.is_err());
    }

    #[test]
    fn test_create_activity_request_validation_name_too_long() {
        let request = CreateActivityRequest {
            name: "a".repeat(101),
            color: None,
            icon: None,
            group_id: 1,
        };

        let validation = request.validate();
        assert!(validation.is_err());
    }

    // Note: Invalid color tests moved to types/activity.rs (HexColor newtype tests)
    // HexColor validates on construction, so invalid colors can't be part of CreateActivityRequest

    #[test]
    fn test_create_activity_request_validation_icon_too_long() {
        let request = CreateActivityRequest {
            name: "Exercise".to_string(),
            color: None,
            icon: Some("🎉".repeat(21)), // > 20 chars
            group_id: 1,
        };

        let validation = request.validate();
        assert!(validation.is_err());
    }

    #[test]
    fn test_create_activity_request_validation_valid() {
        let request = CreateActivityRequest {
            name: "Exercise".to_string(),
            color: Some(HexColor::new("#4CAF50").unwrap()),
            icon: Some("🏃".to_string()),
            group_id: 1,
        };

        assert!(request.validate().is_ok());
    }

    // ========================================================================
    // Unit Tests: Error Message Formatting
    // ========================================================================

    /// Helper to create a command-like function that uses the trait
    fn log_mood_with_trait(
        repo: &dyn MoodRepositoryTrait,
        request: LogMoodRequest,
    ) -> Result<MoodCheckin, String> {
        repo.create_mood_checkin(request.mood_rating, request.activity_ids, request.notes)
            .map_err(|e| format!("Failed to log mood: {}", e))
    }

    #[test]
    fn test_error_message_formatting_activity_not_found() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_create_mood_checkin()
            .returning(|_, _, _| Err(MoodError::ActivityNotFound(999)));

        let request = LogMoodRequest {
            mood_rating: 4,
            activity_ids: vec![999],
            notes: None,
        };

        let result = log_mood_with_trait(&mock_repo, request);

        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("Activity not found: 999"));
    }

    #[test]
    fn test_error_message_formatting_database_error() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_create_mood_checkin()
            .returning(|_, _, _| Err(MoodError::Database(rusqlite::Error::InvalidQuery)));

        let request = LogMoodRequest {
            mood_rating: 4,
            activity_ids: vec![],
            notes: None,
        };

        let result = log_mood_with_trait(&mock_repo, request);

        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("Database error"));
    }

    // ========================================================================
    // Unit Tests: Error Propagation and Conversion
    // ========================================================================

    #[test]
    fn test_error_propagation_returns_string_error() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_create_mood_checkin()
            .returning(|_, _, _| Err(MoodError::InvalidRating(10)));

        let request = LogMoodRequest {
            mood_rating: 10,
            activity_ids: vec![],
            notes: None,
        };

        let result = log_mood_with_trait(&mock_repo, request);

        // Should convert MoodError to String
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid mood rating"));
    }

    /// Helper for delete_activity command logic
    fn delete_activity_with_trait(repo: &dyn MoodRepositoryTrait, id: i32) -> Result<(), String> {
        repo.delete_activity(id)
            .map_err(|e| format!("Failed to delete activity: {}", e))
    }

    #[test]
    fn test_delete_activity_not_found_error_conversion() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_delete_activity()
            .with(mockall::predicate::eq(999))
            .returning(|_| Err(MoodError::ActivityNotFound(999)));

        let result = delete_activity_with_trait(&mock_repo, 999);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Activity not found"));
    }

    #[test]
    fn test_delete_activity_success() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_delete_activity()
            .with(mockall::predicate::eq(123))
            .returning(|_| Ok(()));

        let result = delete_activity_with_trait(&mock_repo, 123);

        assert!(result.is_ok());
    }

    // ========================================================================
    // Unit Tests: Conditional Logic in Commands
    // ========================================================================

    /// Helper for create_activity command logic
    fn create_activity_with_trait(
        repo: &dyn MoodRepositoryTrait,
        request: CreateActivityRequest,
    ) -> Result<Activity, String> {
        repo.create_activity(
            request.name,
            request.color.as_ref().map(|c| c.value().to_string()),
            request.icon,
            request.group_id,
        )
        .map_err(|e| format!("Failed to create activity: {}", e))
    }

    #[test]
    fn test_create_activity_with_all_fields() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_create_activity()
            .withf(|name, color, icon, _group_id| {
                name == "Exercise"
                    && color.as_deref() == Some("#4CAF50")
                    && icon.as_deref() == Some("🏃")
            })
            .returning(|name, color, icon, _group_id| {
                Ok(Activity {
                    id: 1,
                    group_id: 1,
                    name,
                    color: color.map(|c| HexColor::new(c).unwrap()),
                    icon,
                    created_at: "2025-01-01T00:00:00Z".to_string(),
                    deleted_at: None,
                })
            });

        let request = CreateActivityRequest {
            name: "Exercise".to_string(),
            color: Some(HexColor::new("#4CAF50").unwrap()),
            icon: Some("🏃".to_string()),
            group_id: 1,
        };

        let result = create_activity_with_trait(&mock_repo, request);

        assert!(result.is_ok());
        let activity = result.unwrap();
        assert_eq!(activity.name, "Exercise");
        assert_eq!(activity.color, Some(HexColor::new("#4CAF50").unwrap()));
    }

    #[test]
    fn test_create_activity_minimal_fields() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_create_activity()
            .withf(|name, color, icon, _group_id| {
                name == "Meditation" && color.is_none() && icon.is_none()
            })
            .returning(|name, _, _, _group_id| {
                Ok(Activity {
                    id: 2,
                    group_id: 1,
                    name,
                    color: None,
                    icon: None,
                    created_at: "2025-01-01T00:00:00Z".to_string(),
                    deleted_at: None,
                })
            });

        let request = CreateActivityRequest {
            name: "Meditation".to_string(),
            color: None,
            icon: None,
            group_id: 1,
        };

        let result = create_activity_with_trait(&mock_repo, request);

        assert!(result.is_ok());
        let activity = result.unwrap();
        assert_eq!(activity.name, "Meditation");
        assert!(activity.color.is_none());
    }

    /// Helper for update_activity command logic
    fn update_activity_with_trait(
        repo: &dyn MoodRepositoryTrait,
        id: i32,
        request: UpdateActivityRequest,
    ) -> Result<Activity, String> {
        repo.update_activity(
            id,
            request.name,
            request.color.as_ref().map(|c| c.value().to_string()),
            request.icon,
        )
        .map_err(|e| format!("Failed to update activity: {}", e))
    }

    #[test]
    fn test_update_activity_partial_fields() {
        let mut mock_repo = MockMoodRepositoryTrait::new();

        mock_repo
            .expect_update_activity()
            .withf(|id, name, color, icon| {
                *id == 5
                    && name.as_deref() == Some("Updated Name")
                    && color.is_none()
                    && icon.is_none()
            })
            .returning(|id, name, _, _| {
                Ok(Activity {
                    id,
                    group_id: 1,
                    name: name.unwrap(),
                    color: Some(HexColor::new("#FF0000").unwrap()),
                    icon: Some("⭐".to_string()),
                    created_at: "2025-01-01T00:00:00Z".to_string(),
                    deleted_at: None,
                })
            });

        let request = UpdateActivityRequest {
            name: Some("Updated Name".to_string()),
            color: None,
            icon: None,
        };

        let result = update_activity_with_trait(&mock_repo, 5, request);

        assert!(result.is_ok());
        let activity = result.unwrap();
        assert_eq!(activity.name, "Updated Name");
    }

    // ========================================================================
    // Unit Tests: Input Sanitization (via validator)
    // ========================================================================

    #[test]
    fn test_input_sanitization_trims_activity_name() {
        // Validator should trim whitespace
        let request = CreateActivityRequest {
            name: "  Exercise  ".to_string(),
            color: None,
            icon: None,
            group_id: 1,
        };

        // Validation should pass (name will be trimmed by custom validator)
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_input_sanitization_rejects_only_whitespace() {
        let request = CreateActivityRequest {
            name: "    ".to_string(),
            color: None,
            icon: None,
            group_id: 1,
        };

        // Should fail - empty after trim
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_input_sanitization_hex_color_case_insensitive() {
        // Both uppercase and lowercase should be valid
        let request1 = CreateActivityRequest {
            name: "Test".to_string(),
            color: Some(HexColor::new("#FF0000").unwrap()),
            icon: None,
            group_id: 1,
        };
        assert!(request1.validate().is_ok());

        let request2 = CreateActivityRequest {
            name: "Test".to_string(),
            color: Some(HexColor::new("#ff0000").unwrap()),
            icon: None,
            group_id: 1,
        };
        assert!(request2.validate().is_ok());
    }
}
