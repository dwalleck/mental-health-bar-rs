// Assessment commands (mutations)
use super::models::{UNANSWERED, *};
use super::repository::AssessmentRepository;
use super::repository_trait::AssessmentRepositoryTrait;
use crate::{
    errors::{ErrorType, ToCommandError},
    AppState, CommandError,
};
use tauri::State;
use tracing::error;
use validator::Validate;

/// Submit a completed assessment
#[tauri::command]
#[specta::specta]
pub async fn submit_assessment(
    request: SubmitAssessmentRequest,
    state: State<'_, AppState>,
) -> Result<AssessmentResponse, CommandError> {
    // Validate request
    request.validate().map_err(|e| {
        CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation)
    })?;

    let repo = AssessmentRepository::new(state.db.clone());
    submit_assessment_impl(&repo, &request).map_err(|e| {
        error!(
            "submit_assessment error: {} (type: '{}', responses: {}, has_notes: {})",
            e,
            request.assessment_type_code,
            request.responses.len(),
            request.notes.is_some()
        );
        e.to_command_error()
    })
}

/// Business logic for submitting assessment - uses trait bound for testability
fn submit_assessment_impl(
    repo: &impl AssessmentRepositoryTrait,
    request: &SubmitAssessmentRequest,
) -> Result<AssessmentResponse, AssessmentError> {
    // Get assessment type
    let assessment_type = repo.get_assessment_type_by_code(request.assessment_type_code.clone())?;

    // For completed assessments, validate no unanswered questions
    // Drafts are allowed to have UNANSWERED (-1) values
    if request.status == AssessmentStatus::Completed {
        let unanswered_count = request
            .responses
            .iter()
            .filter(|&&r| r == UNANSWERED)
            .count();
        if unanswered_count > 0 {
            return Err(AssessmentError::UnansweredQuestions {
                count: unanswered_count,
                total: request.responses.len(),
            });
        }
    }

    // Calculate score based on type
    // For drafts: filter out UNANSWERED values before scoring
    // For completed: all values are valid (validated above)
    let valid_responses: Vec<i32> = if request.status == AssessmentStatus::Draft {
        request
            .responses
            .iter()
            .copied()
            .filter(|&r| r != UNANSWERED)
            .collect()
    } else {
        request.responses.clone()
    };

    let (total_score, severity_level) = match assessment_type.code.as_str() {
        "PHQ9" => {
            if request.status == AssessmentStatus::Draft && valid_responses.is_empty() {
                (0, SeverityLevel::Unknown)
            } else if request.status == AssessmentStatus::Draft {
                // For drafts, calculate partial score from answered questions only
                let partial_score: i32 = valid_responses.iter().sum();
                (partial_score, SeverityLevel::Unknown) // Unknown severity for incomplete
            } else {
                let score = calculate_phq9_score(&request.responses)?;
                (score, get_phq9_severity(score))
            }
        }
        "GAD7" => {
            if request.status == AssessmentStatus::Draft && valid_responses.is_empty() {
                (0, SeverityLevel::Unknown)
            } else if request.status == AssessmentStatus::Draft {
                let partial_score: i32 = valid_responses.iter().sum();
                (partial_score, SeverityLevel::Unknown)
            } else {
                let score = calculate_gad7_score(&request.responses)?;
                (score, get_gad7_severity(score))
            }
        }
        "CESD" => {
            if request.status == AssessmentStatus::Draft && valid_responses.is_empty() {
                (0, SeverityLevel::Unknown)
            } else if request.status == AssessmentStatus::Draft {
                let partial_score: i32 = valid_responses.iter().sum();
                (partial_score, SeverityLevel::Unknown)
            } else {
                let score = calculate_cesd_score(&request.responses)?;
                (score, get_cesd_severity(score))
            }
        }
        "OASIS" => {
            if request.status == AssessmentStatus::Draft && valid_responses.is_empty() {
                (0, SeverityLevel::Unknown)
            } else if request.status == AssessmentStatus::Draft {
                let partial_score: i32 = valid_responses.iter().sum();
                (partial_score, SeverityLevel::Unknown)
            } else {
                let score = calculate_oasis_score(&request.responses)?;
                (score, get_oasis_severity(score))
            }
        }
        _ => return Err(AssessmentError::InvalidType(assessment_type.code.clone())),
    };

    // Save to database
    let id = repo.save_assessment(
        assessment_type.id,
        request.responses.clone(),
        total_score,
        severity_level,
        request.notes.clone(),
        request.status,
    )?;

    // Return the complete response
    repo.get_assessment_response(id)
}

/// Delete an assessment response
#[tauri::command]
#[specta::specta]
pub async fn delete_assessment(id: i32, state: State<'_, AppState>) -> Result<(), CommandError> {
    let repo = AssessmentRepository::new(state.db.clone());
    delete_assessment_impl(&repo, id).map_err(|e| {
        error!("delete_assessment error: {} (id: {})", e, id);
        e.to_command_error()
    })
}

/// Business logic for deleting assessment - uses trait bound for testability
fn delete_assessment_impl(
    repo: &impl AssessmentRepositoryTrait,
    id: i32,
) -> Result<(), AssessmentError> {
    repo.delete_assessment(id)
}

/// Delete an assessment type (defensive - prevents deletion if children exist)
#[tauri::command]
#[specta::specta]
pub async fn delete_assessment_type(
    id: i32,
    state: State<'_, AppState>,
) -> Result<(), CommandError> {
    let repo = AssessmentRepository::new(state.db.clone());
    delete_assessment_type_impl(&repo, id).map_err(|e| {
        error!("delete_assessment_type error: {} (id: {})", e, id);
        e.to_command_error()
    })
}

/// Business logic for deleting assessment type - uses trait bound for testability
fn delete_assessment_type_impl(
    repo: &impl AssessmentRepositoryTrait,
    id: i32,
) -> Result<(), AssessmentError> {
    repo.delete_assessment_type(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::assessments::{
        repository_trait::MockAssessmentRepositoryTrait, AssessmentRepositoryTrait,
    };
    use validator::Validate;

    // ========================================================================
    // Unit Tests: Command Validation
    // ========================================================================

    #[test]
    fn test_submit_assessment_request_validation_type_code_too_long() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "A".repeat(11),
            responses: vec![0, 1, 2],
            notes: None,
            status: AssessmentStatus::Completed,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_submit_assessment_request_validation_type_code_alphanumeric_only() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ-9".to_string(), // Has hyphen
            responses: vec![0, 1, 2],
            notes: None,
            status: AssessmentStatus::Completed,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_submit_assessment_request_validation_notes_too_long() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0, 1, 2],
            notes: Some("a".repeat(10001)),
            status: AssessmentStatus::Completed,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_submit_assessment_request_validation_notes_control_chars() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0, 1, 2],
            notes: Some("Test\x00Invalid".to_string()), // Null byte
            status: AssessmentStatus::Completed,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_submit_assessment_request_validation_valid() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0, 1, 2, 1, 0, 1, 2, 1, 0],
            notes: Some("Feeling okay today\nSome notes".to_string()),
            status: AssessmentStatus::Completed,
        };

        assert!(request.validate().is_ok());
    }

    // ========================================================================
    // Unit Tests: Error Message Formatting
    // ========================================================================

    /// Helper function to simulate the command logic
    fn submit_assessment_with_trait(
        repo: &dyn AssessmentRepositoryTrait,
        request: SubmitAssessmentRequest,
    ) -> Result<AssessmentResponse, String> {
        // Get assessment type
        let assessment_type = repo
            .get_assessment_type_by_code(request.assessment_type_code.clone())
            .map_err(|e| format!("Failed to get assessment type: {}", e))?;

        // Calculate score (simplified for testing)
        let total_score = request.responses.iter().sum();
        let severity_level = SeverityLevel::Moderate;

        // Save assessment
        let id = repo
            .save_assessment(
                assessment_type.id,
                request.responses.clone(),
                total_score,
                severity_level,
                request.notes.clone(),
                request.status,
            )
            .map_err(|e| format!("Failed to save assessment: {}", e))?;

        // Get saved response
        repo.get_assessment_response(id)
            .map_err(|e| format!("Failed to retrieve assessment: {}", e))
    }

    #[test]
    fn test_error_message_formatting_invalid_assessment_type() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_get_assessment_type_by_code()
            .returning(|_| Err(AssessmentError::InvalidType("INVALID".to_string())));

        let request = SubmitAssessmentRequest {
            assessment_type_code: "INVALID".to_string(),
            responses: vec![0, 1, 2],
            notes: None,
            status: AssessmentStatus::Completed,
        };

        let result = submit_assessment_with_trait(&mock_repo, request);

        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("Invalid assessment type"));
    }

    #[test]
    fn test_error_message_formatting_database_error() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_get_assessment_type_by_code()
            .returning(|_| {
                Ok(AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                })
            });

        mock_repo
            .expect_save_assessment()
            .returning(|_, _, _, _, _, _| {
                Err(AssessmentError::Database(rusqlite::Error::InvalidQuery))
            });

        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0; 9],
            notes: None,
            status: AssessmentStatus::Completed,
        };

        let result = submit_assessment_with_trait(&mock_repo, request);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Database error"));
    }

    // ========================================================================
    // Unit Tests: Conditional Logic (Different Assessment Types)
    // ========================================================================

    #[test]
    fn test_submit_assessment_phq9_scoring() {
        // PHQ-9: 9 questions, 0-3 each
        let responses = vec![1, 2, 0, 1, 2, 1, 0, 2, 1]; // Sum = 10
        let score = calculate_phq9_score(&responses);

        assert!(score.is_ok());
        assert_eq!(score.unwrap(), 10);
    }

    #[test]
    fn test_submit_assessment_gad7_scoring() {
        // GAD-7: 7 questions, 0-3 each
        let responses = vec![2, 1, 3, 0, 2, 1, 1]; // Sum = 10
        let score = calculate_gad7_score(&responses);

        assert!(score.is_ok());
        assert_eq!(score.unwrap(), 10);
    }

    #[test]
    fn test_submit_assessment_phq9_invalid_response_count() {
        let responses = vec![1, 2, 0]; // Only 3 responses, need 9
        let score = calculate_phq9_score(&responses);

        assert!(score.is_err());
    }

    #[test]
    fn test_submit_assessment_phq9_invalid_response_value() {
        let responses = vec![1, 2, 0, 1, 5, 1, 0, 2, 1]; // 5 is invalid (0-3 only)
        let score = calculate_phq9_score(&responses);

        assert!(score.is_err());
    }

    // ========================================================================
    // Unit Tests: Error Propagation and Conversion
    // ========================================================================

    /// Helper for delete_assessment command logic
    fn delete_assessment_with_trait(
        repo: &dyn AssessmentRepositoryTrait,
        id: i32,
    ) -> Result<(), String> {
        repo.delete_assessment(id)
            .map_err(|e| format!("Failed to delete assessment: {}", e))
    }

    #[test]
    fn test_delete_assessment_not_found() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_delete_assessment()
            .with(mockall::predicate::eq(999))
            .returning(|_| Err(AssessmentError::NotFound(999)));

        let result = delete_assessment_with_trait(&mock_repo, 999);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Assessment not found"));
    }

    #[test]
    fn test_delete_assessment_success() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_delete_assessment()
            .with(mockall::predicate::eq(123))
            .returning(|_| Ok(()));

        let result = delete_assessment_with_trait(&mock_repo, 123);

        assert!(result.is_ok());
    }

    /// Helper for delete_assessment_type command logic
    fn delete_assessment_type_with_trait(
        repo: &dyn AssessmentRepositoryTrait,
        id: i32,
    ) -> Result<(), String> {
        repo.delete_assessment_type(id)
            .map_err(|e| format!("Failed to delete assessment type: {}", e))
    }

    #[test]
    fn test_delete_assessment_type_has_children() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo.expect_delete_assessment_type().returning(|_| {
            Err(AssessmentError::HasChildren(
                "Cannot delete assessment type with existing responses".to_string(),
            ))
        });

        let result = delete_assessment_type_with_trait(&mock_repo, 1);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot delete"));
    }

    #[test]
    fn test_delete_assessment_type_success() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_delete_assessment_type()
            .with(mockall::predicate::eq(5))
            .returning(|_| Ok(()));

        let result = delete_assessment_type_with_trait(&mock_repo, 5);

        assert!(result.is_ok());
    }

    // ========================================================================
    // Unit Tests: Input Sanitization
    // ========================================================================

    #[test]
    fn test_input_sanitization_type_code_uppercase() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0; 9],
            notes: None,
            status: AssessmentStatus::Completed,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_input_sanitization_notes_with_newlines_and_tabs() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0; 9],
            notes: Some("Line 1\nLine 2\tTabbed".to_string()),
            status: AssessmentStatus::Completed,
        };

        // Newlines and tabs should be allowed
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_input_sanitization_empty_notes() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "GAD7".to_string(),
            responses: vec![0; 7],
            notes: Some("".to_string()),
            status: AssessmentStatus::Completed,
        };

        // Empty notes should be valid
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_severity_level_calculation_phq9() {
        // Test severity boundaries
        assert_eq!(get_phq9_severity(0), SeverityLevel::Minimal);
        assert_eq!(get_phq9_severity(4), SeverityLevel::Minimal);
        assert_eq!(get_phq9_severity(5), SeverityLevel::Mild);
        assert_eq!(get_phq9_severity(9), SeverityLevel::Mild);
        assert_eq!(get_phq9_severity(10), SeverityLevel::Moderate);
        assert_eq!(get_phq9_severity(14), SeverityLevel::Moderate);
        assert_eq!(get_phq9_severity(15), SeverityLevel::ModeratelySevere);
        assert_eq!(get_phq9_severity(19), SeverityLevel::ModeratelySevere);
        assert_eq!(get_phq9_severity(20), SeverityLevel::Severe);
        assert_eq!(get_phq9_severity(27), SeverityLevel::Severe);
    }

    #[test]
    fn test_severity_level_calculation_gad7() {
        assert_eq!(get_gad7_severity(0), SeverityLevel::Minimal);
        assert_eq!(get_gad7_severity(4), SeverityLevel::Minimal);
        assert_eq!(get_gad7_severity(5), SeverityLevel::Mild);
        assert_eq!(get_gad7_severity(9), SeverityLevel::Mild);
        assert_eq!(get_gad7_severity(10), SeverityLevel::Moderate);
        assert_eq!(get_gad7_severity(14), SeverityLevel::Moderate);
        assert_eq!(get_gad7_severity(15), SeverityLevel::Severe);
        assert_eq!(get_gad7_severity(21), SeverityLevel::Severe);
    }

    // ========================================================================
    // Unit Tests: Draft Assessment Functionality (FR-009a)
    // ========================================================================

    #[test]
    fn test_submit_assessment_request_validation_status_draft() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0, 1, 2, 1, 0, 1, 2, 1, 0],
            notes: Some("Draft notes".to_string()),
            status: AssessmentStatus::Draft,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_submit_assessment_request_validation_status_completed() {
        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![0, 1, 2, 1, 0, 1, 2, 1, 0],
            notes: Some("Completed notes".to_string()),
            status: AssessmentStatus::Completed,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_submit_assessment_as_draft() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        // Setup mock
        mock_repo
            .expect_get_assessment_type_by_code()
            .returning(|_| {
                Ok(AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                })
            });

        mock_repo
            .expect_save_assessment()
            .returning(|_, _, _, _, _, status| {
                assert_eq!(status, AssessmentStatus::Draft, "Status should be Draft");
                Ok(1) // Return mock ID
            });

        mock_repo.expect_get_assessment_response().returning(|_| {
            Ok(AssessmentResponse {
                id: 1,
                assessment_type: AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                },
                responses: vec![1, 1, 0, 1, 1, 0, 1, 0, 1],
                total_score: 6,
                severity_level: SeverityLevel::Mild,
                completed_at: "2024-01-01 12:00:00".to_string(),
                notes: Some("Draft notes".to_string()),
                status: AssessmentStatus::Draft,
            })
        });

        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![1, 1, 0, 1, 1, 0, 1, 0, 1],
            notes: Some("Draft notes".to_string()),
            status: AssessmentStatus::Draft,
        };

        let result = submit_assessment_with_trait(&mock_repo, request);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, AssessmentStatus::Draft);
        assert_eq!(response.notes, Some("Draft notes".to_string()));
    }

    #[test]
    fn test_submit_assessment_as_completed() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        // Setup mock
        mock_repo
            .expect_get_assessment_type_by_code()
            .returning(|_| {
                Ok(AssessmentType {
                    id: 1,
                    code: "GAD7".to_string(),
                    name: "GAD-7".to_string(),
                    description: None,
                    question_count: 7,
                    min_score: 0,
                    max_score: 21,
                    thresholds: serde_json::json!({}),
                })
            });

        mock_repo
            .expect_save_assessment()
            .returning(|_, _, _, _, _, status| {
                assert_eq!(
                    status,
                    AssessmentStatus::Completed,
                    "Status should be Completed"
                );
                Ok(2) // Return mock ID
            });

        mock_repo.expect_get_assessment_response().returning(|_| {
            Ok(AssessmentResponse {
                id: 2,
                assessment_type: AssessmentType {
                    id: 1,
                    code: "GAD7".to_string(),
                    name: "GAD-7".to_string(),
                    description: None,
                    question_count: 7,
                    min_score: 0,
                    max_score: 21,
                    thresholds: serde_json::json!({}),
                },
                responses: vec![2, 2, 2, 2, 2, 2, 2],
                total_score: 14,
                severity_level: SeverityLevel::Moderate,
                completed_at: "2024-01-01 14:00:00".to_string(),
                notes: None,
                status: AssessmentStatus::Completed,
            })
        });

        let request = SubmitAssessmentRequest {
            assessment_type_code: "GAD7".to_string(),
            responses: vec![2, 2, 2, 2, 2, 2, 2],
            notes: None,
            status: AssessmentStatus::Completed,
        };

        let result = submit_assessment_with_trait(&mock_repo, request);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, AssessmentStatus::Completed);
        assert_eq!(response.total_score, 14);
    }

    #[test]
    fn test_submit_assessment_draft_with_partial_responses() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_get_assessment_type_by_code()
            .returning(|_| {
                Ok(AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                })
            });

        mock_repo
            .expect_save_assessment()
            .returning(|_, responses, _, _, _, status| {
                assert_eq!(status, AssessmentStatus::Draft);
                // Verify partial responses (some -1 values for unanswered)
                assert_eq!(responses.len(), 9);
                assert!(responses.contains(&-1), "Should have unanswered questions");
                Ok(3)
            });

        mock_repo.expect_get_assessment_response().returning(|_| {
            Ok(AssessmentResponse {
                id: 3,
                assessment_type: AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                },
                responses: vec![1, 2, -1, -1, 1, -1, 1, -1, -1], // Partial responses
                total_score: 5,
                severity_level: SeverityLevel::Minimal,
                completed_at: "2024-01-01 10:00:00".to_string(),
                notes: Some("Partially completed".to_string()),
                status: AssessmentStatus::Draft,
            })
        });

        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![1, 2, -1, -1, 1, -1, 1, -1, -1],
            notes: Some("Partially completed".to_string()),
            status: AssessmentStatus::Draft,
        };

        let result = submit_assessment_with_trait(&mock_repo, request);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, AssessmentStatus::Draft);
        assert_eq!(
            response.responses.iter().filter(|&&r| r == -1).count(),
            5,
            "Should have 5 unanswered questions"
        );
    }

    #[test]
    fn test_submit_completed_assessment_rejects_unanswered_questions() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_get_assessment_type_by_code()
            .returning(|_| {
                Ok(AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                })
            });

        // Note: save_assessment should NOT be called because validation fails first
        // No expectation set means test fails if it's called

        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![1, 2, -1, 0, 1, -1, 1, 0, 1], // Has unanswered (-1) values
            notes: None,
            status: AssessmentStatus::Completed, // Completed should reject -1
        };

        let result = submit_assessment_impl(&mock_repo, &request);

        assert!(
            result.is_err(),
            "Should reject completed assessment with unanswered questions"
        );
        match result.unwrap_err() {
            AssessmentError::UnansweredQuestions { count, total } => {
                assert_eq!(count, 2, "Should have 2 unanswered questions");
                assert_eq!(total, 9, "Should have 9 total questions");
            }
            other => panic!("Expected UnansweredQuestions error, got: {:?}", other),
        }
    }

    #[test]
    fn test_submit_completed_assessment_accepts_all_answered() {
        let mut mock_repo = MockAssessmentRepositoryTrait::new();

        mock_repo
            .expect_get_assessment_type_by_code()
            .returning(|_| {
                Ok(AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                })
            });

        mock_repo
            .expect_save_assessment()
            .returning(|_, _, _, _, _, _| Ok(1));

        mock_repo.expect_get_assessment_response().returning(|_| {
            Ok(AssessmentResponse {
                id: 1,
                assessment_type: AssessmentType {
                    id: 1,
                    code: "PHQ9".to_string(),
                    name: "PHQ-9".to_string(),
                    description: None,
                    question_count: 9,
                    min_score: 0,
                    max_score: 27,
                    thresholds: serde_json::json!({}),
                },
                responses: vec![1, 2, 0, 0, 1, 0, 1, 0, 1],
                total_score: 6,
                severity_level: SeverityLevel::Mild,
                completed_at: "2024-01-01 12:00:00".to_string(),
                notes: None,
                status: AssessmentStatus::Completed,
            })
        });

        let request = SubmitAssessmentRequest {
            assessment_type_code: "PHQ9".to_string(),
            responses: vec![1, 2, 0, 0, 1, 0, 1, 0, 1], // All answered (no -1)
            notes: None,
            status: AssessmentStatus::Completed,
        };

        let result = submit_assessment_impl(&mock_repo, &request);

        assert!(
            result.is_ok(),
            "Should accept completed assessment with all questions answered"
        );
    }
}
