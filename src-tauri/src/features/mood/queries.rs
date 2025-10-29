// Mood queries - Read operations (Tauri commands)
// T081-T083: Mood query implementation

use super::models::*;
use super::repository::MoodRepository;
use crate::{AppState, CommandError};
use tauri::State;
use tracing::error;

// T081: get_mood_history command
#[tauri::command]
#[specta::specta]
pub async fn get_mood_history(
    from_date: Option<String>,
    to_date: Option<String>,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<MoodCheckin>, CommandError> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_mood_history(from_date, to_date, limit)
        .map_err(|e| {
            error!("get_mood_history error: {}", e);
            e.to_command_error()
        })
}

// T082: get_mood_checkin command
#[tauri::command]
#[specta::specta]
pub async fn get_mood_checkin(id: i32, state: State<'_, AppState>) -> Result<MoodCheckin, CommandError> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_mood_checkin(id)
        .map_err(|e| {
            error!("get_mood_checkin error: {}", e);
            e.to_command_error()
        })
}

// T083: get_mood_stats command
#[tauri::command]
#[specta::specta]
pub async fn get_mood_stats(
    from_date: Option<String>,
    to_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<MoodStats, CommandError> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_mood_stats(from_date, to_date)
        .map_err(|e| {
            error!("get_mood_stats error: {}", e);
            e.to_command_error()
        })
}

// T109: get_activities command
#[tauri::command]
#[specta::specta]
pub async fn get_activities(
    include_deleted: bool,
    state: State<'_, AppState>,
) -> Result<Vec<Activity>, CommandError> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_activities(include_deleted)
        .map_err(|e| {
            error!("get_activities error: {}", e);
            e.to_command_error()
        })
}
