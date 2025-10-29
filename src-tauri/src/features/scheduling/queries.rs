// Scheduling queries (User Story 6)
// T168-T169: Tauri queries for schedule retrieval

use tauri::State;

use crate::{AppState, CommandError};

use super::models::AssessmentSchedule;
use super::repository::SchedulingRepository;

/// T168: Get all schedules (optionally filtered to enabled only)
#[tauri::command]
#[specta::specta]
pub fn get_schedules(
    enabled_only: bool,
    state: State<AppState>,
) -> Result<Vec<AssessmentSchedule>, CommandError> {
    let repo = SchedulingRepository::new(state.db.clone());

    repo.get_schedules(enabled_only)
        .map_err(|e| e.to_command_error())
}

/// T169: Get a single schedule by ID
#[tauri::command]
#[specta::specta]
pub fn get_schedule(id: i32, state: State<AppState>) -> Result<AssessmentSchedule, CommandError> {
    let repo = SchedulingRepository::new(state.db.clone());

    repo.get_schedule(id)
        .map_err(|e| e.to_command_error())
}
