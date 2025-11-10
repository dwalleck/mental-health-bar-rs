// Property-based tests using quickcheck
// These tests verify that validation and scoring functions maintain invariants
// across a wide range of randomly generated inputs.

use quickcheck::{quickcheck, TestResult};
use tauri_sveltekit_modern_lib::features::assessments::models::*;
use tauri_sveltekit_modern_lib::features::mood::models::*;

// ============================================================================
// Mood Validation Property Tests
// ============================================================================

/// Property: validate_mood_rating accepts exactly ratings 1-5, rejects all others
#[test]
fn prop_mood_rating_bounds() {
    fn test(rating: i32) -> bool {
        let result = validate_mood_rating(rating);
        if (1..=5).contains(&rating) {
            result.is_ok()
        } else {
            result.is_err()
        }
    }
    quickcheck(test as fn(i32) -> bool);
}

/// Property: validate_notes accepts strings up to 5000 chars, rejects longer
#[test]
fn prop_notes_length() {
    fn test(length: u16) -> TestResult {
        // Generate string of specific length
        let notes = "a".repeat(length as usize);
        let result = validate_notes(&notes);

        if length <= 5000 {
            TestResult::from_bool(result.is_ok())
        } else {
            TestResult::from_bool(result.is_err())
        }
    }
    quickcheck(test as fn(u16) -> TestResult);
}

/// Property: validate_activity_name trims whitespace and validates length
#[test]
fn prop_activity_name_trimming() {
    fn test(leading: u8, trailing: u8, content: String) -> TestResult {
        // Use chars().count() to check character count, not byte count
        if content.trim().is_empty() || content.trim().chars().count() > 50 {
            return TestResult::discard();
        }

        let spaces_before = " ".repeat(leading as usize % 10);
        let spaces_after = " ".repeat(trailing as usize % 10);
        let padded = format!("{}{}{}", spaces_before, content, spaces_after);

        let result = validate_activity_name(&padded);

        // Should succeed and return trimmed version
        TestResult::from_bool(result.is_ok() && result.unwrap() == content.trim())
    }
    quickcheck(test as fn(u8, u8, String) -> TestResult);
}

/// Property: validate_color accepts only valid hex color formats
#[test]
fn prop_hex_color_format() {
    fn test(r: u8, g: u8, b: u8) -> bool {
        // Generate valid hex color
        let color = format!("#{:02X}{:02X}{:02X}", r, g, b);
        validate_color(&color).is_ok()
    }
    quickcheck(test as fn(u8, u8, u8) -> bool);
}

/// Property: validate_color rejects strings without # prefix
#[test]
fn prop_hex_color_requires_hash() {
    fn test(s: String) -> TestResult {
        if s.is_empty() || s.starts_with('#') {
            return TestResult::discard();
        }
        TestResult::from_bool(validate_color(&s).is_err())
    }
    quickcheck(test as fn(String) -> TestResult);
}

/// Property: validate_icon rejects strings longer than 20 characters
#[test]
fn prop_icon_length() {
    fn test(length: u8) -> TestResult {
        let icon = "🎉".repeat(length as usize);
        let result = validate_icon(&icon);

        // Use chars().count() to check character count, not byte count
        TestResult::from_bool(if icon.chars().count() <= 20 {
            result.is_ok()
        } else {
            result.is_err()
        })
    }
    quickcheck(test as fn(u8) -> TestResult);
}

// ============================================================================
// Assessment Scoring Property Tests
// ============================================================================

/// Property: PHQ-9 score is always between 0-27 for valid inputs
#[test]
fn prop_phq9_score_range() {
    fn test(responses: Vec<u8>) -> TestResult {
        // Only test valid PHQ-9 responses (9 questions, 0-3 each)
        if responses.len() != 9 {
            return TestResult::discard();
        }

        let valid_responses: Vec<i32> = responses
            .iter()
            .map(|&r| (r % 4) as i32) // Constrain to 0-3
            .collect();

        let score = calculate_phq9_score(&valid_responses);

        TestResult::from_bool(
            score.is_ok() && {
                let s = score.unwrap();
                s >= 0 && s <= 27
            },
        )
    }
    quickcheck(test as fn(Vec<u8>) -> TestResult);
}

/// Property: PHQ-9 score equals sum of responses
#[test]
fn prop_phq9_score_is_sum() {
    fn test(responses: Vec<u8>) -> TestResult {
        if responses.len() != 9 {
            return TestResult::discard();
        }

        let valid_responses: Vec<i32> = responses.iter().map(|&r| (r % 4) as i32).collect();

        let expected_sum: i32 = valid_responses.iter().sum();
        let score = calculate_phq9_score(&valid_responses);

        TestResult::from_bool(score.is_ok() && score.unwrap() == expected_sum)
    }
    quickcheck(test as fn(Vec<u8>) -> TestResult);
}

/// Property: PHQ-9 rejects wrong number of responses
#[test]
fn prop_phq9_requires_9_responses() {
    fn test(count: u8) -> TestResult {
        let count = count as usize;
        if count == 9 {
            return TestResult::discard();
        }

        let responses: Vec<i32> = (0..count).map(|_| 0).collect();
        let result = calculate_phq9_score(&responses);

        TestResult::from_bool(result.is_err())
    }
    quickcheck(test as fn(u8) -> TestResult);
}

/// Property: PHQ-9 rejects out-of-range responses
#[test]
fn prop_phq9_rejects_invalid_responses() {
    fn test(invalid_value: i8) -> TestResult {
        // Only test values outside 0-3 range
        if (0..=3).contains(&invalid_value) {
            return TestResult::discard();
        }

        let mut responses = vec![1, 2, 0, 1, 2, 1, 0, 2, 1]; // Valid base
        responses[4] = invalid_value as i32; // Insert invalid value

        let result = calculate_phq9_score(&responses);
        TestResult::from_bool(result.is_err())
    }
    quickcheck(test as fn(i8) -> TestResult);
}

/// Property: GAD-7 score is always between 0-21 for valid inputs
#[test]
fn prop_gad7_score_range() {
    fn test(responses: Vec<u8>) -> TestResult {
        if responses.len() != 7 {
            return TestResult::discard();
        }

        let valid_responses: Vec<i32> = responses.iter().map(|&r| (r % 4) as i32).collect();

        let score = calculate_gad7_score(&valid_responses);

        TestResult::from_bool(
            score.is_ok() && {
                let s = score.unwrap();
                s >= 0 && s <= 21
            },
        )
    }
    quickcheck(test as fn(Vec<u8>) -> TestResult);
}

/// Property: GAD-7 score equals sum of responses
#[test]
fn prop_gad7_score_is_sum() {
    fn test(responses: Vec<u8>) -> TestResult {
        if responses.len() != 7 {
            return TestResult::discard();
        }

        let valid_responses: Vec<i32> = responses.iter().map(|&r| (r % 4) as i32).collect();

        let expected_sum: i32 = valid_responses.iter().sum();
        let score = calculate_gad7_score(&valid_responses);

        TestResult::from_bool(score.is_ok() && score.unwrap() == expected_sum)
    }
    quickcheck(test as fn(Vec<u8>) -> TestResult);
}

// ============================================================================
// Edge Case Property Tests
// ============================================================================

/// Property: All validators handle empty strings correctly
#[test]
fn prop_validators_handle_empty_strings() {
    // Empty activity name should fail
    assert!(validate_activity_name("").is_err());
    assert!(validate_activity_name("   ").is_err());

    // Empty notes should succeed (it's optional)
    assert!(validate_notes("").is_ok());

    // Empty color should fail
    assert!(validate_color("").is_err());

    // Empty icon should succeed (length 0 < 20)
    assert!(validate_icon("").is_ok());
}

/// Property: Validators are idempotent (calling twice gives same result)
#[test]
fn prop_validators_are_idempotent() {
    fn test(rating: i32) -> bool {
        let result1 = validate_mood_rating(rating);
        let result2 = validate_mood_rating(rating);
        result1.is_ok() == result2.is_ok()
    }
    quickcheck(test as fn(i32) -> bool);
}
