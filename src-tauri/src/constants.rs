// Application-wide validation and configuration constants
//
// Centralized constants ensure consistency across all features and prevent
// configuration drift between different parts of the application.

/// Maximum length for notes fields across all features
///
/// This limit applies to:
/// - Mood check-in notes
/// - Assessment notes
/// - Activity notes (future)
///
/// Rationale: 5,000 characters provides ample space for detailed notes
/// (~800-1000 words) while preventing abuse and keeping database size manageable.
/// This aligns with the frontend validation in mood check-ins.
pub const MAX_NOTES_LENGTH: usize = 5000;

/// Maximum length for assessment type codes (e.g., "PHQ9", "GAD7")
///
/// Assessment type codes are short identifiers used in the API and database.
/// All standard assessment codes are < 10 characters.
pub const MAX_TYPE_CODE_LENGTH: usize = 10;

/// Maximum number of records to return in a single query
///
/// This limit applies to:
/// - get_assessment_history
/// - get_mood_history
/// - get_activities
///
/// Rationale: Prevents excessive memory usage and response times while being
/// large enough for typical UI pagination needs. Clients should implement
/// pagination for larger datasets.
pub const MAX_QUERY_LIMIT: i32 = 1000;

/// Maximum length for activity names
///
/// Activity names should be concise but descriptive. 100 characters provides
/// enough space for multi-word descriptions while preventing abuse.
pub const MAX_ACTIVITY_NAME_LENGTH: usize = 100;

/// Maximum length for activity icons (emoji or short symbol)
///
/// Icons are typically single emoji (1-4 bytes UTF-8) or short symbols.
/// 10 characters allows for complex emoji sequences while preventing abuse.
pub const MAX_ACTIVITY_ICON_LENGTH: usize = 10;

/// Maximum length for activity color hex codes
///
/// Format: #RRGGBB (7 characters) or #RGB (4 characters)
/// Extra space allows for future formats like #RRGGBBAA
pub const MAX_ACTIVITY_COLOR_LENGTH: usize = 10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_notes_length_is_reasonable() {
        // 5000 characters is approximately 800-1000 words
        // This should be sufficient for detailed notes
        assert!(MAX_NOTES_LENGTH >= 1000);
        assert!(MAX_NOTES_LENGTH <= 100_000);
    }

    #[test]
    fn test_max_query_limit_is_reasonable() {
        // Should be large enough for typical UI needs but not excessive
        assert!(MAX_QUERY_LIMIT >= 100);
        assert!(MAX_QUERY_LIMIT <= 10_000);
    }

    #[test]
    fn test_assessment_codes_fit_in_max_length() {
        // Test all known assessment type codes
        let codes = vec!["PHQ9", "GAD7", "CESD", "OASIS"];
        for code in codes {
            assert!(
                code.len() <= MAX_TYPE_CODE_LENGTH,
                "Assessment code '{}' exceeds MAX_TYPE_CODE_LENGTH",
                code
            );
        }
    }

    #[test]
    fn test_activity_constraints_are_reasonable() {
        // Activity name should fit common use cases
        assert!(MAX_ACTIVITY_NAME_LENGTH >= 20);

        // Icon should fit emoji sequences
        assert!(MAX_ACTIVITY_ICON_LENGTH >= 4);

        // Color should fit hex codes
        assert!(MAX_ACTIVITY_COLOR_LENGTH >= 7);
    }
}
