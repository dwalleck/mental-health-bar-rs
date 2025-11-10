// Activity queries - Read operations (Tauri commands)
// Week 2: Activity Groups, Goals, and Reporting

use super::models::*;
use super::repository::ActivityRepository;
use crate::{errors::ToCommandError, AppState, CommandError};
use tauri::State;
use tracing::error;

// ========================================
// Activity Group Queries
// ========================================

#[tauri::command]
#[specta::specta]
pub async fn get_activity_groups(
    state: State<'_, AppState>,
) -> Result<Vec<ActivityGroup>, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());

    repo.get_activity_groups().map_err(|e| {
        error!("get_activity_groups error: {}", e);
        e.to_command_error()
    })
}

// ========================================
// Activity Log Queries
// ========================================
// Note: get_activities query is in mood::queries for backward compatibility
// Week 2 adds activity logging and reporting queries

#[tauri::command]
#[specta::specta]
pub async fn get_activity_logs(
    activity_id: Option<i32>,
    start_date: Option<String>,
    end_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ActivityLog>, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());

    repo.get_activity_logs(activity_id, start_date.as_deref(), end_date.as_deref())
        .map_err(|e| {
            error!(
            "get_activity_logs error: {} (activity_id: {:?}, has_start_date: {}, has_end_date: {})",
            e,
            activity_id,
            start_date.is_some(),
            end_date.is_some()
        );
            e.to_command_error()
        })
}

// ========================================
// Activity Goal Queries
// ========================================

#[tauri::command]
#[specta::specta]
pub async fn get_activity_goals(
    activity_id: Option<i32>,
    group_id: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<ActivityGoal>, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());

    repo.get_activity_goals(activity_id, group_id).map_err(|e| {
        error!(
            "get_activity_goals error: {} (activity_id: {:?}, group_id: {:?})",
            e, activity_id, group_id
        );
        e.to_command_error()
    })
}

// ========================================
// Reporting Queries
// ========================================

#[tauri::command]
#[specta::specta]
pub async fn get_activity_frequency(
    activity_id: i32,
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<ActivityFrequency, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());

    repo.get_activity_frequency(activity_id, &start_date, &end_date)
        .map_err(|e| {
            error!(
                "get_activity_frequency error: {} (activity_id: {}, start: {}, end: {})",
                e, activity_id, start_date, end_date
            );
            e.to_command_error()
        })
}

#[tauri::command]
#[specta::specta]
pub async fn get_activity_trend(
    activity_id: i32,
    period_days: i32,
    current_time: String,
    state: State<'_, AppState>,
) -> Result<ActivityTrend, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());

    repo.get_activity_trend(activity_id, period_days, &current_time)
        .map_err(|e| {
            error!(
                "get_activity_trend error: {} (activity_id: {}, period_days: {}, current_time: {})",
                e, activity_id, period_days, current_time
            );
            e.to_command_error()
        })
}

#[tauri::command]
#[specta::specta]
pub async fn check_goal_progress(
    goal_id: i32,
    current_time: String,
    state: State<'_, AppState>,
) -> Result<GoalProgress, CommandError> {
    let repo = ActivityRepository::new(state.db.clone());

    repo.check_goal_progress(goal_id, &current_time)
        .map_err(|e| {
            error!(
                "check_goal_progress error: {} (goal_id: {}, current_time: {})",
                e, goal_id, current_time
            );
            e.to_command_error()
        })
}
