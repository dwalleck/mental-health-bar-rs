use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

/// Severity level constants
pub const SEVERITY_MINIMAL: &str = "minimal";
pub const SEVERITY_MILD: &str = "mild";
pub const SEVERITY_MODERATE: &str = "moderate";
pub const SEVERITY_MODERATELY_SEVERE: &str = "moderately_severe";
pub const SEVERITY_SEVERE: &str = "severe";
pub const SEVERITY_UNKNOWN: &str = "unknown";

/// Assessment error types
#[derive(Error, Debug)]
pub enum AssessmentError {
    #[error("Invalid assessment type: {0}")]
    InvalidType(String),

    #[error("Incomplete responses: expected {expected}, got {actual}")]
    IncompleteResponses { expected: usize, actual: usize },

    #[error("Invalid response value: {0}")]
    InvalidResponse(String),

    #[error("Assessment not found: {0}")]
    NotFound(i32),

    #[error("Database lock poisoned. This is a critical error. Please restart the application to recover.")]
    LockPoisoned,

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Database error: {0}")]
    Database(#[from] duckdb::Error),
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

/// Request to submit assessment
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SubmitAssessmentRequest {
    pub assessment_type_code: String,
    pub responses: Vec<i32>,
    pub notes: Option<String>,
}

/// Assessment response with calculated score
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AssessmentResponse {
    pub id: i32,
    pub assessment_type: AssessmentType,
    pub responses: Vec<i32>,
    pub total_score: i32,
    pub severity_level: String,
    pub completed_at: String,
    pub notes: Option<String>,
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
pub fn get_phq9_severity(score: i32) -> &'static str {
    match score {
        0..=4 => SEVERITY_MINIMAL,
        5..=9 => SEVERITY_MILD,
        10..=14 => SEVERITY_MODERATE,
        15..=19 => SEVERITY_MODERATELY_SEVERE,
        20..=27 => SEVERITY_SEVERE,
        _ => SEVERITY_UNKNOWN,
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
pub fn get_gad7_severity(score: i32) -> &'static str {
    match score {
        0..=4 => SEVERITY_MINIMAL,
        5..=9 => SEVERITY_MILD,
        10..=14 => SEVERITY_MODERATE,
        15..=21 => SEVERITY_SEVERE,
        _ => SEVERITY_UNKNOWN,
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
pub fn get_cesd_severity(score: i32) -> &'static str {
    match score {
        0..=15 => SEVERITY_MINIMAL,
        16..=21 => SEVERITY_MILD,
        22..=36 => SEVERITY_MODERATE,
        37..=60 => SEVERITY_SEVERE,
        _ => SEVERITY_UNKNOWN,
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
pub fn get_oasis_severity(score: i32) -> &'static str {
    match score {
        0..=7 => SEVERITY_MINIMAL,
        8..=14 => SEVERITY_MODERATE,
        15..=20 => SEVERITY_SEVERE,
        _ => SEVERITY_UNKNOWN,
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
        assert_eq!(get_phq9_severity(score), "minimal");
    }

    #[test]
    fn test_phq9_scoring_all_threes() {
        let responses = vec![3, 3, 3, 3, 3, 3, 3, 3, 3];
        let score = calculate_phq9_score(&responses).unwrap();
        assert_eq!(score, 27);
        assert_eq!(get_phq9_severity(score), "severe");
    }

    #[test]
    fn test_phq9_scoring_mixed() {
        let responses = vec![1, 1, 0, 2, 1, 0, 1, 0, 1];
        let score = calculate_phq9_score(&responses).unwrap();
        assert_eq!(score, 7);
        assert_eq!(get_phq9_severity(score), "mild");
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
        assert_eq!(get_gad7_severity(score), "minimal");
    }

    #[test]
    fn test_gad7_scoring_all_threes() {
        let responses = vec![3, 3, 3, 3, 3, 3, 3];
        let score = calculate_gad7_score(&responses).unwrap();
        assert_eq!(score, 21);
        assert_eq!(get_gad7_severity(score), "severe");
    }

    #[test]
    fn test_gad7_scoring_moderate() {
        let responses = vec![2, 2, 1, 2, 1, 2, 1];
        let score = calculate_gad7_score(&responses).unwrap();
        assert_eq!(score, 11);
        assert_eq!(get_gad7_severity(score), "moderate");
    }

    // T023: CES-D scoring algorithm tests
    #[test]
    fn test_cesd_scoring_all_zeros() {
        let responses = vec![0; 20];
        let score = calculate_cesd_score(&responses).unwrap();
        assert_eq!(score, 0);
        assert_eq!(get_cesd_severity(score), "minimal");
    }

    #[test]
    fn test_cesd_scoring_all_threes() {
        let responses = vec![3; 20];
        let score = calculate_cesd_score(&responses).unwrap();
        assert_eq!(score, 60);
        assert_eq!(get_cesd_severity(score), "severe");
    }

    #[test]
    fn test_cesd_scoring_mild() {
        let responses = vec![1; 20];
        let score = calculate_cesd_score(&responses).unwrap();
        assert_eq!(score, 20);
        assert_eq!(get_cesd_severity(score), "mild");
    }

    // T024: OASIS scoring algorithm tests
    #[test]
    fn test_oasis_scoring_all_zeros() {
        let responses = vec![0, 0, 0, 0, 0];
        let score = calculate_oasis_score(&responses).unwrap();
        assert_eq!(score, 0);
        assert_eq!(get_oasis_severity(score), "minimal");
    }

    #[test]
    fn test_oasis_scoring_all_fours() {
        let responses = vec![4, 4, 4, 4, 4];
        let score = calculate_oasis_score(&responses).unwrap();
        assert_eq!(score, 20);
        assert_eq!(get_oasis_severity(score), "severe");
    }

    #[test]
    fn test_oasis_scoring_moderate() {
        let responses = vec![2, 2, 2, 2, 2];
        let score = calculate_oasis_score(&responses).unwrap();
        assert_eq!(score, 10);
        assert_eq!(get_oasis_severity(score), "moderate");
    }

    // T025: Severity level calculation tests
    #[test]
    fn test_severity_boundaries_phq9() {
        assert_eq!(get_phq9_severity(4), "minimal");
        assert_eq!(get_phq9_severity(5), "mild");
        assert_eq!(get_phq9_severity(9), "mild");
        assert_eq!(get_phq9_severity(10), "moderate");
        assert_eq!(get_phq9_severity(14), "moderate");
        assert_eq!(get_phq9_severity(15), "moderately_severe");
        assert_eq!(get_phq9_severity(19), "moderately_severe");
        assert_eq!(get_phq9_severity(20), "severe");
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
}
