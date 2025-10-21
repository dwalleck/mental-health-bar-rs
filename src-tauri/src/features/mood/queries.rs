// Mood queries - Read operations (Tauri commands)
// T081-T083: Mood query implementation

use super::models::*;
use super::repository::MoodRepository;
use crate::AppState;
use tauri::State;
use anyhow::Context;
use tracing::error;

// T081: get_mood_history command
#[tauri::command]
#[specta::specta]
pub async fn get_mood_history(
    from_date: Option<String>,
    to_date: Option<String>,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<MoodCheckin>, String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_mood_history(from_date, to_date, limit)
        .context("Failed to get mood history")
        .map_err(|e| {
            error!("get_mood_history error: {}", e);
            e.to_string()
        })
}

// T082: get_mood_checkin command
#[tauri::command]
#[specta::specta]
pub async fn get_mood_checkin(
    id: i64,
    state: State<'_, AppState>,
) -> Result<MoodCheckin, String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_mood_checkin(id)
        .context("Failed to get mood check-in")
        .map_err(|e| {
            error!("get_mood_checkin error: {}", e);
            e.to_string()
        })
}

// T083: get_mood_stats command
#[tauri::command]
#[specta::specta]
pub async fn get_mood_stats(
    from_date: Option<String>,
    to_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<MoodStats, String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_mood_stats(from_date, to_date)
        .context("Failed to get mood stats")
        .map_err(|e| {
            error!("get_mood_stats error: {}", e);
            e.to_string()
        })
}

// T109: get_activities command
#[tauri::command]
#[specta::specta]
pub async fn get_activities(
    include_deleted: bool,
    state: State<'_, AppState>,
) -> Result<Vec<Activity>, String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.get_activities(include_deleted)
        .context("Failed to get activities")
        .map_err(|e| {
            error!("get_activities error: {}", e);
            e.to_string()
        })
}
