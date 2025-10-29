// Scheduling commands (User Story 6)
// T165-T167: Tauri commands for schedule management

use tauri::State;

use crate::AppState;

use super::models::{AssessmentSchedule, CreateScheduleRequest, UpdateScheduleRequest};
use super::repository::SchedulingRepository;

/// T165: Create a new assessment schedule
#[tauri::command]
#[specta::specta]
pub fn create_schedule(
    request: CreateScheduleRequest,
    state: State<AppState>,
) -> Result<AssessmentSchedule, String> {
    let repo = SchedulingRepository::new(state.db.clone());

    repo.create_schedule(&request)
        .map_err(|e| format!("Failed to create schedule: {}", e))
}

/// T166: Update an existing schedule
#[tauri::command]
#[specta::specta]
pub fn update_schedule(
    id: i32,
    request: UpdateScheduleRequest,
    state: State<AppState>,
) -> Result<AssessmentSchedule, String> {
    let repo = SchedulingRepository::new(state.db.clone());

    repo.update_schedule(id, &request)
        .map_err(|e| format!("Failed to update schedule: {}", e))
}

/// T167: Delete a schedule
#[tauri::command]
#[specta::specta]
pub fn delete_schedule(id: i32, state: State<AppState>) -> Result<(), String> {
    let repo = SchedulingRepository::new(state.db.clone());

    repo.delete_schedule(id)
        .map_err(|e| format!("Failed to delete schedule: {}", e))
}
