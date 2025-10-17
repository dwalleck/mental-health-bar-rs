// Assessment queries (reads)
use tauri::State;
use crate::AppState;
use super::models::*;
use super::repository::AssessmentRepository;
use super::content;

/// Get all available assessment types
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_types(
    state: State<'_, AppState>,
) -> Result<Vec<AssessmentType>, String> {
    let repo = AssessmentRepository::new(state.db.clone());
    repo.get_assessment_types()
        .map_err(|e| e.to_string())
}

/// Get questions for a specific assessment type
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_questions(
    assessment_type_code: String,
) -> Result<Vec<AssessmentQuestion>, String> {
    let questions = match assessment_type_code.as_str() {
        "PHQ9" => content::get_phq9_questions(),
        "GAD7" => content::get_gad7_questions(),
        "CESD" => content::get_cesd_questions(),
        "OASIS" => content::get_oasis_questions(),
        _ => return Err(format!("Unknown assessment type: {}", assessment_type_code)),
    };

    Ok(questions)
}

/// Get assessment history with optional filtering
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_history(
    assessment_type_code: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<AssessmentResponse>, String> {
    let repo = AssessmentRepository::new(state.db.clone());
    repo.get_assessment_history(assessment_type_code, from_date, to_date, limit)
        .map_err(|e| e.to_string())
}

/// Get a single assessment response by ID
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_response(
    id: i32,
    state: State<'_, AppState>,
) -> Result<AssessmentResponse, String> {
    let repo = AssessmentRepository::new(state.db.clone());
    repo.get_assessment_response(id)
        .map_err(|e| e.to_string())
}

/// Get the most recent assessment for a specific type
#[tauri::command]
#[specta::specta]
pub async fn get_latest_assessment(
    assessment_type_code: String,
    state: State<'_, AppState>,
) -> Result<Option<AssessmentResponse>, String> {
    let repo = AssessmentRepository::new(state.db.clone());
    let history = repo
        .get_assessment_history(Some(assessment_type_code), None, None, Some(1))
        .map_err(|e| e.to_string())?;

    Ok(history.into_iter().next())
}
