// Mood commands - Write operations (Tauri commands)
// T080-T084: Mood command implementation

use super::models::*;
use super::repository::MoodRepository;
use crate::AppState;
use anyhow::Context;
use tauri::State;
use tracing::error;

// T080: log_mood command
#[tauri::command]
#[specta::specta]
pub async fn log_mood(
    request: LogMoodRequest,
    state: State<'_, AppState>,
) -> Result<MoodCheckin, String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.create_mood_checkin(
        request.mood_rating,
        request.activity_ids,
        request.notes.as_deref(),
    )
    .context("Failed to log mood")
    .map_err(|e| {
        error!("log_mood error: {}", e);
        e.to_string()
    })
}

// T106: create_activity command
#[tauri::command]
#[specta::specta]
pub async fn create_activity(
    request: CreateActivityRequest,
    state: State<'_, AppState>,
) -> Result<Activity, String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.create_activity(
        &request.name,
        request.color.as_deref(),
        request.icon.as_deref(),
    )
    .context("Failed to create activity")
    .map_err(|e| {
        error!("create_activity error: {}", e);
        e.to_string()
    })
}

// T107: update_activity command
#[tauri::command]
#[specta::specta]
pub async fn update_activity(
    id: i64,
    request: UpdateActivityRequest,
    state: State<'_, AppState>,
) -> Result<Activity, String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.update_activity(
        id,
        request.name.as_deref(),
        request.color.as_deref(),
        request.icon.as_deref(),
    )
    .context("Failed to update activity")
    .map_err(|e| {
        error!("update_activity error: {}", e);
        e.to_string()
    })
}

// T108: delete_activity command
#[tauri::command]
#[specta::specta]
pub async fn delete_activity(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.delete_activity(id)
        .context("Failed to delete activity")
        .map_err(|e| {
            error!("delete_activity error: {}", e);
            e.to_string()
        })
}

// T093c: delete_mood_checkin command (cascade deletion)
#[tauri::command]
#[specta::specta]
pub async fn delete_mood_checkin(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let repo = MoodRepository::new(state.db.clone());

    repo.delete_mood_checkin(id)
        .context("Failed to delete mood check-in")
        .map_err(|e| {
            error!("delete_mood_checkin error: {}", e);
            e.to_string()
        })
}
