use crate::errors::{CommandError, ErrorType, ToCommandError};
use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;
use validator::Validate;

// Re-export types for backward compatibility and convenience
pub use crate::types::assessment::{AssessmentCode, AssessmentStatus, SeverityLevel};

/// Severity level constants (deprecated - use SeverityLevel enum instead)
#[deprecated(since = "0.2.0", note = "Use SeverityLevel enum instead")]
pub const SEVERITY_MINIMAL: &str = "minimal";
#[deprecated(since = "0.2.0", note = "Use SeverityLevel enum instead")]
pub const SEVERITY_MILD: &str = "mild";
#[deprecated(since = "0.2.0", note = "Use SeverityLevel enum instead")]
pub const SEVERITY_MODERATE: &str = "moderate";
#[deprecated(since = "0.2.0", note = "Use SeverityLevel enum instead")]
pub const SEVERITY_MODERATELY_SEVERE: &str = "moderately_severe";
#[deprecated(since = "0.2.0", note = "Use SeverityLevel enum instead")]
pub const SEVERITY_SEVERE: &str = "severe";
#[deprecated(since = "0.2.0", note = "Use SeverityLevel enum instead")]
pub const SEVERITY_UNKNOWN: &str = "unknown";

/// Assessment status constants (deprecated - use AssessmentStatus enum instead)
#[deprecated(since = "0.2.0", note = "Use AssessmentStatus enum instead")]
pub const STATUS_DRAFT: &str = "draft";
#[deprecated(since = "0.2.0", note = "Use AssessmentStatus enum instead")]
pub const STATUS_COMPLETED: &str = "completed";

/// Assessment error types
#[derive(Error, Debug)]
pub enum AssessmentError {
    #[error("Invalid assessment type: {0}")]
    InvalidType(String),

    #[error("Invalid assessment status: {0}")]
    InvalidStatus(String),

    #[error("Invalid severity level: {0}")]
    InvalidSeverity(String),

    #[error("Incomplete responses: expected {expected}, got {actual}")]
    IncompleteResponses { expected: usize, actual: usize },

    #[error("Invalid response value: {0}")]
    InvalidResponse(String),

    #[error("Assessment not found: {0}")]
    NotFound(i32),

    #[error("Cannot delete: {0}")]
    HasChildren(String),

    #[error("Database lock poisoned. This is a critical error. Please restart the application to recover.")]
    LockPoisoned,

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

impl ToCommandError for AssessmentError {
    fn to_command_error(&self) -> CommandError {
        match self {
            // Validation errors - not retryable
            AssessmentError::InvalidType(code) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation).with_details(
                    serde_json::json!({
                        "field": "assessment_type_code",
                        "value": code
                    }),
                )
            }
            AssessmentError::InvalidStatus(status) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation).with_details(
                    serde_json::json!({
                        "field": "status",
                        "value": status
                    }),
                )
            }
            AssessmentError::InvalidSeverity(severity) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation).with_details(
                    serde_json::json!({
                        "field": "severity_level",
                        "value": severity
                    }),
                )
            }
            AssessmentError::IncompleteResponses { expected, actual } => {
                CommandError::permanent(self.to_string(), ErrorType::Validation).with_details(
                    serde_json::json!({
                        "field": "responses",
                        "expected": expected,
                        "actual": actual
                    }),
                )
            }
            AssessmentError::InvalidResponse(msg) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation).with_details(
                    serde_json::json!({
                        "field": "responses",
                        "details": msg
                    }),
                )
            }
            AssessmentError::Deserialization(msg) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation).with_details(
                    serde_json::json!({
                        "details": msg
                    }),
                )
            }

            // Not found errors - not retryable
            AssessmentError::NotFound(id) => {
                CommandError::permanent(self.to_string(), ErrorType::NotFound).with_details(
                    serde_json::json!({
                        "resource": "assessment",
                        "id": id
                    }),
                )
            }

            // Constraint errors - not retryable
            AssessmentError::HasChildren(msg) => {
                CommandError::permanent(self.to_string(), ErrorType::ConstraintViolation)
                    .with_details(serde_json::json!({
                        "details": msg
                    }))
            }

            // Database lock/transient errors - retryable
            AssessmentError::LockPoisoned => {
                CommandError::retryable(self.to_string(), ErrorType::LockPoisoned)
            }
            AssessmentError::Database(e) => CommandError::from_rusqlite_error(e),
        }
    }
}

/// Assessment type (PHQ-9, GAD-7, CES-D, OASIS)
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AssessmentType {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub question_count: i32,
    pub min_score: i32,
    pub max_score: i32,
    #[serde(skip)]
    #[specta(skip)]
    pub thresholds: serde_json::Value,
}

/// Assessment question
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AssessmentQuestion {
    pub number: i32,
    pub text: String,
    pub options: Vec<String>,
}

/// Custom validator for assessment type code - alphanumeric only
fn validate_assessment_type_code(code: &str) -> Result<(), validator::ValidationError> {
    if !code.chars().all(|c| c.is_alphanumeric()) {
        let mut error = validator::ValidationError::new("alphanumeric");
        error.message = Some(std::borrow::Cow::from(
            "Assessment type code must contain only alphanumeric characters",
        ));
        return Err(error);
    }
    Ok(())
}

/// Custom validator for notes - no control characters except newline, tab, carriage return
fn validate_notes_control_chars(notes: &str) -> Result<(), validator::ValidationError> {
    for ch in notes.chars() {
        if ch.is_control() && ch != '\n' && ch != '\t' && ch != '\r' {
            let mut error = validator::ValidationError::new("control_character");
            error.message = Some(std::borrow::Cow::from(
                format!("Notes contain invalid control character (code {}). Only newlines and tabs are allowed.", ch as u32)
            ));
            return Err(error);
        }
    }
    Ok(())
}

/// Request to submit assessment
#[derive(Debug, Clone, Serialize, Deserialize, Type, Validate)]
pub struct SubmitAssessmentRequest {
    #[validate(length(max = 10), custom(function = "validate_assessment_type_code"))]
    pub assessment_type_code: String,
    pub responses: Vec<i32>,
    #[validate(length(max = 10000), custom(function = "validate_notes_control_chars"))]
    pub notes: Option<String>,
    #[serde(default)]
    pub status: AssessmentStatus,
}

/// Assessment response with calculated score
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AssessmentResponse {
    pub id: i32,
    pub assessment_type: AssessmentType,
    pub responses: Vec<i32>,
    pub total_score: i32,
    pub severity_level: SeverityLevel,
    pub completed_at: String,
    pub notes: Option<String>,
    pub status: AssessmentStatus,
}

/// Calculate PHQ-9 score (0-27)
pub fn calculate_phq9_score(responses: &[i32]) -> Result<i32, AssessmentError> {
    if responses.len() != 9 {
        return Err(AssessmentError::IncompleteResponses {
            expected: 9,
            actual: responses.len(),
        });
    }

    // Validate each response is 0-3
    for (i, &response) in responses.iter().enumerate() {
        if !(0..=3).contains(&response) {
            return Err(AssessmentError::InvalidResponse(format!(
                "Question {} has invalid value: {}. Must be 0-3",
                i + 1,
                response
            )));
        }
    }

    Ok(responses.iter().sum())
}

/// Get PHQ-9 severity level
pub fn get_phq9_severity(score: i32) -> SeverityLevel {
    match score {
        0..=4 => SeverityLevel::Minimal,
        5..=9 => SeverityLevel::Mild,
        10..=14 => SeverityLevel::Moderate,
        15..=19 => SeverityLevel::ModeratelySevere,
        20..=27 => SeverityLevel::Severe,
        _ => SeverityLevel::Unknown,
    }
}

/// Calculate GAD-7 score (0-21)
pub fn calculate_gad7_score(responses: &[i32]) -> Result<i32, AssessmentError> {
    if responses.len() != 7 {
        return Err(AssessmentError::IncompleteResponses {
            expected: 7,
            actual: responses.len(),
        });
    }

    for (i, &response) in responses.iter().enumerate() {
        if !(0..=3).contains(&response) {
            return Err(AssessmentError::InvalidResponse(format!(
                "Question {} has invalid value: {}. Must be 0-3",
                i + 1,
                response
            )));
        }
    }

    Ok(responses.iter().sum())
}

/// Get GAD-7 severity level
pub fn get_gad7_severity(score: i32) -> SeverityLevel {
    match score {
        0..=4 => SeverityLevel::Minimal,
        5..=9 => SeverityLevel::Mild,
        10..=14 => SeverityLevel::Moderate,
        15..=21 => SeverityLevel::Severe,
        _ => SeverityLevel::Unknown,
    }
}

/// Calculate CES-D score (0-60)
pub fn calculate_cesd_score(responses: &[i32]) -> Result<i32, AssessmentError> {
    if responses.len() != 20 {
        return Err(AssessmentError::IncompleteResponses {
            expected: 20,
            actual: responses.len(),
        });
    }

    for (i, &response) in responses.iter().enumerate() {
        if !(0..=3).contains(&response) {
            return Err(AssessmentError::InvalidResponse(format!(
                "Question {} has invalid value: {}. Must be 0-3",
                i + 1,
                response
            )));
        }
    }

    Ok(responses.iter().sum())
}

/// Get CES-D severity level
pub fn get_cesd_severity(score: i32) -> SeverityLevel {
    match score {
        0..=15 => SeverityLevel::Minimal,
        16..=21 => SeverityLevel::Mild,
        22..=36 => SeverityLevel::Moderate,
        37..=60 => SeverityLevel::Severe,
        _ => SeverityLevel::Unknown,
    }
}

/// Calculate OASIS score (0-20)
pub fn calculate_oasis_score(responses: &[i32]) -> Result<i32, AssessmentError> {
    if responses.len() != 5 {
        return Err(AssessmentError::IncompleteResponses {
            expected: 5,
            actual: responses.len(),
        });
    }

    for (i, &response) in responses.iter().enumerate() {
        if !(0..=4).contains(&response) {
            return Err(AssessmentError::InvalidResponse(format!(
                "Question {} has invalid value: {}. Must be 0-4",
                i + 1,
                response
            )));
        }
    }

    Ok(responses.iter().sum())
}

/// Get OASIS severity level
pub fn get_oasis_severity(score: i32) -> SeverityLevel {
    match score {
        0..=7 => SeverityLevel::Minimal,
        8..=14 => SeverityLevel::Moderate,
        15..=20 => SeverityLevel::Severe,
        _ => SeverityLevel::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // T021: PHQ-9 scoring algorithm tests
    #[test]
    fn test_phq9_scoring_all_zeros() {
        let responses = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let score = calculate_phq9_score(&responses).unwrap();
        assert_eq!(score, 0);
        assert_eq!(get_phq9_severity(score), SeverityLevel::Minimal);
    }

    #[test]
    fn test_phq9_scoring_all_threes() {
        let responses = vec![3, 3, 3, 3, 3, 3, 3, 3, 3];
        let score = calculate_phq9_score(&responses).unwrap();
        assert_eq!(score, 27);
        assert_eq!(get_phq9_severity(score), SeverityLevel::Severe);
    }

    #[test]
    fn test_phq9_scoring_mixed() {
        let responses = vec![1, 1, 0, 2, 1, 0, 1, 0, 1];
        let score = calculate_phq9_score(&responses).unwrap();
        assert_eq!(score, 7);
        assert_eq!(get_phq9_severity(score), SeverityLevel::Mild);
    }

    #[test]
    fn test_phq9_incomplete_responses() {
        let responses = vec![1, 2, 3];
        let result = calculate_phq9_score(&responses);
        assert!(result.is_err());
    }

    #[test]
    fn test_phq9_invalid_response_value() {
        let responses = vec![1, 2, 3, 4, 0, 0, 0, 0, 0]; // 4 is invalid
        let result = calculate_phq9_score(&responses);
        assert!(result.is_err());
    }

    // T022: GAD-7 scoring algorithm tests
    #[test]
    fn test_gad7_scoring_all_zeros() {
        let responses = vec![0, 0, 0, 0, 0, 0, 0];
        let score = calculate_gad7_score(&responses).unwrap();
        assert_eq!(score, 0);
        assert_eq!(get_gad7_severity(score), SeverityLevel::Minimal);
    }

    #[test]
    fn test_gad7_scoring_all_threes() {
        let responses = vec![3, 3, 3, 3, 3, 3, 3];
        let score = calculate_gad7_score(&responses).unwrap();
        assert_eq!(score, 21);
        assert_eq!(get_gad7_severity(score), SeverityLevel::Severe);
    }

    #[test]
    fn test_gad7_scoring_moderate() {
        let responses = vec![2, 2, 1, 2, 1, 2, 1];
        let score = calculate_gad7_score(&responses).unwrap();
        assert_eq!(score, 11);
        assert_eq!(get_gad7_severity(score), SeverityLevel::Moderate);
    }

    // T023: CES-D scoring algorithm tests
    #[test]
    fn test_cesd_scoring_all_zeros() {
        let responses = vec![0; 20];
        let score = calculate_cesd_score(&responses).unwrap();
        assert_eq!(score, 0);
        assert_eq!(get_cesd_severity(score), SeverityLevel::Minimal);
    }

    #[test]
    fn test_cesd_scoring_all_threes() {
        let responses = vec![3; 20];
        let score = calculate_cesd_score(&responses).unwrap();
        assert_eq!(score, 60);
        assert_eq!(get_cesd_severity(score), SeverityLevel::Severe);
    }

    #[test]
    fn test_cesd_scoring_mild() {
        let responses = vec![1; 20];
        let score = calculate_cesd_score(&responses).unwrap();
        assert_eq!(score, 20);
        assert_eq!(get_cesd_severity(score), SeverityLevel::Mild);
    }

    // T024: OASIS scoring algorithm tests
    #[test]
    fn test_oasis_scoring_all_zeros() {
        let responses = vec![0, 0, 0, 0, 0];
        let score = calculate_oasis_score(&responses).unwrap();
        assert_eq!(score, 0);
        assert_eq!(get_oasis_severity(score), SeverityLevel::Minimal);
    }

    #[test]
    fn test_oasis_scoring_all_fours() {
        let responses = vec![4, 4, 4, 4, 4];
        let score = calculate_oasis_score(&responses).unwrap();
        assert_eq!(score, 20);
        assert_eq!(get_oasis_severity(score), SeverityLevel::Severe);
    }

    #[test]
    fn test_oasis_scoring_moderate() {
        let responses = vec![2, 2, 2, 2, 2];
        let score = calculate_oasis_score(&responses).unwrap();
        assert_eq!(score, 10);
        assert_eq!(get_oasis_severity(score), SeverityLevel::Moderate);
    }

    // T025: Severity level calculation tests
    #[test]
    fn test_severity_boundaries_phq9() {
        assert_eq!(get_phq9_severity(4), SeverityLevel::Minimal);
        assert_eq!(get_phq9_severity(5), SeverityLevel::Mild);
        assert_eq!(get_phq9_severity(9), SeverityLevel::Mild);
        assert_eq!(get_phq9_severity(10), SeverityLevel::Moderate);
        assert_eq!(get_phq9_severity(14), SeverityLevel::Moderate);
        assert_eq!(get_phq9_severity(15), SeverityLevel::ModeratelySevere);
        assert_eq!(get_phq9_severity(19), SeverityLevel::ModeratelySevere);
        assert_eq!(get_phq9_severity(20), SeverityLevel::Severe);
    }

    // T026: Response validation tests
    #[test]
    fn test_response_count_validation() {
        assert!(calculate_phq9_score(&vec![1; 8]).is_err());
        assert!(calculate_gad7_score(&vec![1; 6]).is_err());
        assert!(calculate_cesd_score(&vec![1; 19]).is_err());
        assert!(calculate_oasis_score(&vec![1; 4]).is_err());
    }

    #[test]
    fn test_response_range_validation() {
        assert!(calculate_phq9_score(&vec![0, 1, 2, 3, 4, 0, 0, 0, 0]).is_err());
        assert!(calculate_gad7_score(&vec![0, 1, 2, 3, -1, 0, 0]).is_err());
        assert!(calculate_oasis_score(&vec![0, 1, 2, 3, 5]).is_err());
    }

    // T027: Deserialization validation tests
    #[test]
    fn test_submit_assessment_request_invalid_status_deserialization() {
        let json =
            r#"{"assessment_type_code":"PHQ9","responses":[0,1,2,0,1,0,1,0,1],"status":"invalid"}"#;
        let result: Result<SubmitAssessmentRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Deserialization should fail for invalid status value"
        );
    }

    #[test]
    fn test_submit_assessment_request_valid_status_deserialization() {
        // Test valid draft status
        let json_draft =
            r#"{"assessment_type_code":"PHQ9","responses":[0,1,2,0,1,0,1,0,1],"status":"draft"}"#;
        let result: Result<SubmitAssessmentRequest, _> = serde_json::from_str(json_draft);
        assert!(
            result.is_ok(),
            "Deserialization should succeed for valid draft status"
        );
        assert_eq!(result.unwrap().status, AssessmentStatus::Draft);

        // Test valid completed status
        let json_completed = r#"{"assessment_type_code":"PHQ9","responses":[0,1,2,0,1,0,1,0,1],"status":"completed"}"#;
        let result: Result<SubmitAssessmentRequest, _> = serde_json::from_str(json_completed);
        assert!(
            result.is_ok(),
            "Deserialization should succeed for valid completed status"
        );
        assert_eq!(result.unwrap().status, AssessmentStatus::Completed);
    }

    #[test]
    fn test_submit_assessment_request_default_status() {
        // Test that missing status defaults to Completed
        let json = r#"{"assessment_type_code":"PHQ9","responses":[0,1,2,0,1,0,1,0,1]}"#;
        let result: Result<SubmitAssessmentRequest, _> = serde_json::from_str(json);
        assert!(
            result.is_ok(),
            "Deserialization should succeed when status is missing"
        );
        assert_eq!(result.unwrap().status, AssessmentStatus::Completed);
    }
}
