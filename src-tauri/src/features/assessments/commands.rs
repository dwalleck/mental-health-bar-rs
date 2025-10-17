// Assessment commands (mutations)
use tauri::State;
use crate::AppState;
use super::models::*;
use super::repository::AssessmentRepository;
use super::content;

/// Submit a completed assessment
#[tauri::command]
#[specta::specta]
pub async fn submit_assessment(
    request: SubmitAssessmentRequest,
    state: State<'_, AppState>,
) -> Result<AssessmentResponse, String> {
    let repo = AssessmentRepository::new(state.db.clone());

    // Get assessment type
    let assessment_type = repo
        .get_assessment_type_by_code(&request.assessment_type_code)
        .map_err(|e| e.to_string())?;

    // Calculate score based on type
    let (total_score, severity_level) = match assessment_type.code.as_str() {
        "PHQ9" => {
            let score = calculate_phq9_score(&request.responses)
                .map_err(|e| e.to_string())?;
            (score, get_phq9_severity(score).to_string())
        }
        "GAD7" => {
            let score = calculate_gad7_score(&request.responses)
                .map_err(|e| e.to_string())?;
            (score, get_gad7_severity(score).to_string())
        }
        "CESD" => {
            let score = calculate_cesd_score(&request.responses)
                .map_err(|e| e.to_string())?;
            (score, get_cesd_severity(score).to_string())
        }
        "OASIS" => {
            let score = calculate_oasis_score(&request.responses)
                .map_err(|e| e.to_string())?;
            (score, get_oasis_severity(score).to_string())
        }
        _ => return Err(format!("Unknown assessment type: {}", assessment_type.code)),
    };

    // Save to database
    let id = repo
        .save_assessment(
            assessment_type.id,
            &request.responses,
            total_score,
            &severity_level,
            request.notes,
        )
        .map_err(|e| e.to_string())?;

    // Return the complete response
    repo.get_assessment_response(id)
        .map_err(|e| e.to_string())
}

/// Delete an assessment response
#[tauri::command]
#[specta::specta]
pub async fn delete_assessment(
    id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let conn = state.db.get_connection();
    let conn = conn.lock().unwrap();

    conn.execute("DELETE FROM assessment_responses WHERE id = ?", [id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
