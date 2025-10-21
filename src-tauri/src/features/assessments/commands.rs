// Assessment commands (mutations)
use super::models::*;
use super::repository::AssessmentRepository;
use crate::AppState;
use tauri::State;

/// Maximum length for assessment notes field
const MAX_NOTES_LENGTH: usize = 10_000;

/// Submit a completed assessment
#[tauri::command]
#[specta::specta]
pub async fn submit_assessment(
    request: SubmitAssessmentRequest,
    state: State<'_, AppState>,
) -> Result<AssessmentResponse, String> {
    // Validate notes field length
    if let Some(ref notes) = request.notes {
        if notes.len() > MAX_NOTES_LENGTH {
            return Err(format!(
                "Notes exceed maximum length of {} characters",
                MAX_NOTES_LENGTH
            ));
        }
    }

    // Validate assessment type code format (alphanumeric only)
    if !request
        .assessment_type_code
        .chars()
        .all(|c| c.is_alphanumeric())
    {
        return Err("Assessment type code must contain only alphanumeric characters".to_string());
    }

    let repo = AssessmentRepository::new(state.db.clone());

    // Get assessment type
    let assessment_type = repo
        .get_assessment_type_by_code(&request.assessment_type_code)
        .map_err(|e| {
            format!(
                "Failed to retrieve assessment type '{}': {}",
                request.assessment_type_code, e
            )
        })?;

    // Calculate score based on type
    let (total_score, severity_level) = match assessment_type.code.as_str() {
        "PHQ9" => {
            let score = calculate_phq9_score(&request.responses)
                .map_err(|e| format!("Failed to calculate PHQ-9 score: {}", e))?;
            (score, get_phq9_severity(score).to_string())
        }
        "GAD7" => {
            let score = calculate_gad7_score(&request.responses)
                .map_err(|e| format!("Failed to calculate GAD-7 score: {}", e))?;
            (score, get_gad7_severity(score).to_string())
        }
        "CESD" => {
            let score = calculate_cesd_score(&request.responses)
                .map_err(|e| format!("Failed to calculate CES-D score: {}", e))?;
            (score, get_cesd_severity(score).to_string())
        }
        "OASIS" => {
            let score = calculate_oasis_score(&request.responses)
                .map_err(|e| format!("Failed to calculate OASIS score: {}", e))?;
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
        .map_err(|e| format!("Failed to save assessment to database: {}", e))?;

    // Return the complete response
    repo.get_assessment_response(id)
        .map_err(|e| format!("Failed to retrieve saved assessment: {}", e))
}

/// Delete an assessment response
#[tauri::command]
#[specta::specta]
pub async fn delete_assessment(id: i32, state: State<'_, AppState>) -> Result<(), String> {
    let repo = AssessmentRepository::new(state.db.clone());

    repo.delete_assessment(id)
        .map_err(|e| format!("Failed to delete assessment {}: {}", id, e))
}
