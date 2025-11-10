//! Utility functions for common operations across the codebase

/// Sanitizes optional text by trimming whitespace and converting empty strings to None.
///
/// This function takes an optional string, trims leading/trailing whitespace,
/// and returns None if the resulting string is empty.
///
/// # Examples
///
/// ```
/// use tauri_sveltekit_modern_lib::utils::sanitize_optional_text;
///
/// assert_eq!(sanitize_optional_text(Some("  hello  ".to_string())), Some("hello".to_string()));
/// assert_eq!(sanitize_optional_text(Some("   ".to_string())), None);
/// assert_eq!(sanitize_optional_text(None), None);
/// ```
///
/// # Use Cases
///
/// This function is commonly used to sanitize user input fields like notes, descriptions,
/// and other optional text fields before storing them in the database.
pub fn sanitize_optional_text(text: Option<String>) -> Option<String> {
    text.as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_optional_text_trims_whitespace() {
        let input = Some("  hello world  ".to_string());
        let result = sanitize_optional_text(input);
        assert_eq!(result, Some("hello world".to_string()));
    }

    #[test]
    fn test_sanitize_optional_text_whitespace_only_becomes_none() {
        let input = Some("     ".to_string());
        let result = sanitize_optional_text(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sanitize_optional_text_empty_string_becomes_none() {
        let input = Some("".to_string());
        let result = sanitize_optional_text(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sanitize_optional_text_none_remains_none() {
        let input: Option<String> = None;
        let result = sanitize_optional_text(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sanitize_optional_text_preserves_internal_whitespace() {
        let input = Some("  hello   world  ".to_string());
        let result = sanitize_optional_text(input);
        assert_eq!(result, Some("hello   world".to_string()));
    }

    #[test]
    fn test_sanitize_optional_text_with_newlines() {
        let input = Some("  hello\nworld  ".to_string());
        let result = sanitize_optional_text(input);
        assert_eq!(result, Some("hello\nworld".to_string()));
    }
}
