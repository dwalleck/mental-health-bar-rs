// Activity commands - Write operations (Tauri commands)
// Week 2: Activity Groups, Goals, and Logging

use super::models::*;
use super::repository::ActivityRepository;
use crate::{
    errors::{ErrorType, ToCommandError},
    AppState, CommandError,
};
use tauri::State;
use tracing::error;
use validator::Validate;

// ========================================
// Activity Group Commands
// ========================================

#[tauri::command]
#[specta::specta]
pub async fn create_activity_group(
    request: CreateActivityGroupRequest,
    state: State<'_, AppState>,
) -> Result<ActivityGroup, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = ActivityRepository::new(state.db.clone());
    repo.create_activity_group(&request.name, request.description.as_deref())
        .map_err(|e| {
            error!(
                "create_activity_group error: {} (name: '{}', has_description: {})",
                e,
                request.name,
                request.description.is_some()
            );
            e.to_command_error()
        })
}

#[tauri::command]
#[specta::specta]
pub async fn update_activity_group(
    id: i32,
    request: UpdateActivityGroupRequest,
    state: State<'_, AppState>,
) -> Result<ActivityGroup, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = ActivityRepository::new(state.db.clone());
    repo.update_activity_group(id, request.name.as_deref(), request.description.as_deref())
        .map_err(|e| {
            error!(
                "update_activity_group error: {} (id: {}, has_name: {}, has_description: {})",
                e,
                id,
                request.name.is_some(),
                request.description.is_some()
            );
            e.to_command_error()
        })
}

#[tauri::command]
#[specta::specta]
pub async fn delete_activity_group(
    id: i32,
    state: State<'_, AppState>,
) -> Result<(), CommandError> {
    let repo = ActivityRepository::new(state.db.clone());
    repo.delete_activity_group(id).map_err(|e| {
        error!("delete_activity_group error: {} (id: {})", e, id);
        e.to_command_error()
    })
}

// ========================================
// Activity Logging Commands
// ========================================
// Note: Basic activity CRUD (create/update/delete) commands are in mood::commands
// for backward compatibility. Week 2 adds logging and goals.

#[tauri::command]
#[specta::specta]
pub async fn log_activity(
    request: LogActivityRequest,
    state: State<'_, AppState>,
) -> Result<ActivityLog, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = ActivityRepository::new(state.db.clone());

    // Use provided timestamp or default to now
    let logged_at = request
        .logged_at
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

    repo.log_activity(request.activity_id, &logged_at, request.notes.as_deref())
        .map_err(|e| {
            error!(
                "log_activity error: {} (activity_id: {}, has_notes: {})",
                e,
                request.activity_id,
                request.notes.is_some()
            );
            e.to_command_error()
        })
}

/// Update notes for an existing activity log.
///
/// Frontend usage:
/// - Allows editing/clearing notes from ActivityLogHistory
/// - Enforces same constraints as log_activity (<= 500 chars, trimmed)
#[tauri::command]
#[specta::specta]
pub async fn update_activity_log(
    id: i32,
    notes: Option<String>,
    state: State<'_, AppState>,
) -> Result<ActivityLog, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());

    // Capture whether notes is present before moving it
    let has_notes = notes
        .as_ref()
        .map(|n| !n.trim().is_empty())
        .unwrap_or(false);

    repo.update_activity_log_notes(id, notes).map_err(|e| {
        error!(
            "update_activity_log error: {} (id: {}, has_notes: {})",
            e, id, has_notes
        );
        e.to_command_error()
    })
}

// ========================================
// Activity Goal Commands
// ========================================

#[tauri::command]
#[specta::specta]
pub async fn set_activity_goal(
    request: SetActivityGoalRequest,
    state: State<'_, AppState>,
) -> Result<ActivityGoal, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = ActivityRepository::new(state.db.clone());
    repo.set_activity_goal(
        request.activity_id,
        request.group_id,
        request.goal_type,
        request.target_value,
        request.period_days,
    )
    .map_err(|e| {
        error!(
            "set_activity_goal error: {} (activity_id: {:?}, group_id: {:?}, goal_type: '{}', target: {}, period: {})",
            e,
            request.activity_id,
            request.group_id,
            request.goal_type,
            request.target_value,
            request.period_days
        );
        e.to_command_error()
    })
}

#[tauri::command]
#[specta::specta]
pub async fn update_activity_goal(
    goal_id: i32,
    target_value: i32,
    period_days: i32,
    state: State<'_, AppState>,
) -> Result<ActivityGoal, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());
    repo.update_activity_goal(goal_id, target_value, period_days)
        .map_err(|e| {
            error!(
                "update_activity_goal error: {} (goal_id: {}, target: {}, period: {})",
                e, goal_id, target_value, period_days
            );
            e.to_command_error()
        })
}

#[tauri::command]
#[specta::specta]
pub async fn delete_activity_goal(
    goal_id: i32,
    state: State<'_, AppState>,
) -> Result<(), CommandError> {
    let repo = ActivityRepository::new(state.db.clone());
    repo.delete_activity_goal(goal_id).map_err(|e| {
        error!("delete_activity_goal error: {} (goal_id: {})", e, goal_id);
        e.to_command_error()
    })
}
