// Assessment type definitions for improved type safety
// Replaces stringly-typed status and severity fields with proper enums

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// Errors for assessment type parsing
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AssessmentTypeError {
    #[error("Invalid assessment status: '{0}'. Must be 'draft' or 'completed'")]
    InvalidStatus(String),

    #[error("Invalid severity level: '{0}'. Must be one of: minimal, mild, moderate, moderately_severe, severe, unknown")]
    InvalidSeverity(String),

    #[error("Invalid assessment code: '{0}'. Must be one of: PHQ9, GAD7, CESD, OASIS")]
    InvalidCode(String),
}

/// Assessment status (draft or completed)
///
/// Replaces string constants `STATUS_DRAFT` and `STATUS_COMPLETED`.
/// Draft assessments have incomplete responses; completed have full responses with scores.
#[derive(
    Serialize, Deserialize, specta::Type, Clone, Copy, Debug, Default, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentStatus {
    Draft,
    #[default]
    Completed,
}

impl AssessmentStatus {
    /// Get the string representation for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            AssessmentStatus::Draft => "draft",
            AssessmentStatus::Completed => "completed",
        }
    }

    /// Check if this is a draft status
    pub fn is_draft(&self) -> bool {
        matches!(self, AssessmentStatus::Draft)
    }

    /// Check if this is a completed status
    pub fn is_completed(&self) -> bool {
        matches!(self, AssessmentStatus::Completed)
    }
}

impl FromStr for AssessmentStatus {
    type Err = AssessmentTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(AssessmentStatus::Draft),
            "completed" => Ok(AssessmentStatus::Completed),
            _ => Err(AssessmentTypeError::InvalidStatus(s.to_string())),
        }
    }
}

impl std::fmt::Display for AssessmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToSql for AssessmentStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for AssessmentStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        AssessmentStatus::from_str(s).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

/// Severity level for assessment scores
///
/// Replaces string constants `SEVERITY_*`.
/// Each assessment type has different score thresholds for these levels.
#[derive(Serialize, Deserialize, specta::Type, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SeverityLevel {
    Minimal,
    Mild,
    Moderate,
    ModeratelySevere,
    Severe,
    Unknown,
}

impl SeverityLevel {
    /// Get the string representation for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            SeverityLevel::Minimal => "minimal",
            SeverityLevel::Mild => "mild",
            SeverityLevel::Moderate => "moderate",
            SeverityLevel::ModeratelySevere => "moderately_severe",
            SeverityLevel::Severe => "severe",
            SeverityLevel::Unknown => "unknown",
        }
    }

    /// Returns ordered severity levels from least to most severe (excluding Unknown)
    pub fn severity_order() -> &'static [SeverityLevel] {
        &[
            SeverityLevel::Minimal,
            SeverityLevel::Mild,
            SeverityLevel::Moderate,
            SeverityLevel::ModeratelySevere,
            SeverityLevel::Severe,
        ]
    }

    /// Compare severity levels (returns None if either is Unknown)
    pub fn compare(&self, other: &SeverityLevel) -> Option<std::cmp::Ordering> {
        if *self == SeverityLevel::Unknown || *other == SeverityLevel::Unknown {
            return None;
        }
        let order = Self::severity_order();
        let self_idx = order.iter().position(|s| s == self)?;
        let other_idx = order.iter().position(|s| s == other)?;
        Some(self_idx.cmp(&other_idx))
    }
}

impl FromStr for SeverityLevel {
    type Err = AssessmentTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minimal" => Ok(SeverityLevel::Minimal),
            "mild" => Ok(SeverityLevel::Mild),
            "moderate" => Ok(SeverityLevel::Moderate),
            "moderately_severe" => Ok(SeverityLevel::ModeratelySevere),
            "severe" => Ok(SeverityLevel::Severe),
            "unknown" => Ok(SeverityLevel::Unknown),
            _ => Err(AssessmentTypeError::InvalidSeverity(s.to_string())),
        }
    }
}

impl std::fmt::Display for SeverityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToSql for SeverityLevel {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for SeverityLevel {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        SeverityLevel::from_str(s).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

/// Assessment code identifying the assessment type
///
/// Each assessment has a specific number of questions and scoring algorithm.
#[derive(Serialize, Deserialize, specta::Type, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssessmentCode {
    /// Patient Health Questionnaire-9 (depression screening)
    /// 9 questions, score 0-27
    #[serde(rename = "PHQ9")]
    Phq9,
    /// Generalized Anxiety Disorder-7 (anxiety screening)
    /// 7 questions, score 0-21
    #[serde(rename = "GAD7")]
    Gad7,
    /// Center for Epidemiologic Studies Depression Scale
    /// 20 questions, score 0-60
    #[serde(rename = "CESD")]
    Cesd,
    /// Overall Anxiety Severity and Impairment Scale
    /// 5 questions, score 0-20
    #[serde(rename = "OASIS")]
    Oasis,
}

impl AssessmentCode {
    /// Get the string representation (uppercase) for database/API
    pub fn as_str(&self) -> &'static str {
        match self {
            AssessmentCode::Phq9 => "PHQ9",
            AssessmentCode::Gad7 => "GAD7",
            AssessmentCode::Cesd => "CESD",
            AssessmentCode::Oasis => "OASIS",
        }
    }

    /// Get the number of questions for this assessment type
    pub fn question_count(&self) -> usize {
        match self {
            AssessmentCode::Phq9 => 9,
            AssessmentCode::Gad7 => 7,
            AssessmentCode::Cesd => 20,
            AssessmentCode::Oasis => 5,
        }
    }

    /// Get the maximum possible score for this assessment type
    pub fn max_score(&self) -> i32 {
        match self {
            AssessmentCode::Phq9 => 27,  // 9 questions × 3 max
            AssessmentCode::Gad7 => 21,  // 7 questions × 3 max
            AssessmentCode::Cesd => 60,  // 20 questions × 3 max
            AssessmentCode::Oasis => 20, // 5 questions × 4 max
        }
    }

    /// Get the maximum value for each question response
    pub fn max_response_value(&self) -> i32 {
        match self {
            AssessmentCode::Phq9 => 3,
            AssessmentCode::Gad7 => 3,
            AssessmentCode::Cesd => 3,
            AssessmentCode::Oasis => 4,
        }
    }

    /// Get all valid assessment codes
    pub fn all() -> &'static [AssessmentCode] {
        &[
            AssessmentCode::Phq9,
            AssessmentCode::Gad7,
            AssessmentCode::Cesd,
            AssessmentCode::Oasis,
        ]
    }
}

impl FromStr for AssessmentCode {
    type Err = AssessmentTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PHQ9" | "PHQ-9" => Ok(AssessmentCode::Phq9),
            "GAD7" | "GAD-7" => Ok(AssessmentCode::Gad7),
            "CESD" | "CES-D" => Ok(AssessmentCode::Cesd),
            "OASIS" => Ok(AssessmentCode::Oasis),
            _ => Err(AssessmentTypeError::InvalidCode(s.to_string())),
        }
    }
}

impl std::fmt::Display for AssessmentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToSql for AssessmentCode {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for AssessmentCode {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        AssessmentCode::from_str(s).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AssessmentStatus tests
    #[test]
    fn test_status_from_str() {
        assert_eq!(
            AssessmentStatus::from_str("draft").unwrap(),
            AssessmentStatus::Draft
        );
        assert_eq!(
            AssessmentStatus::from_str("DRAFT").unwrap(),
            AssessmentStatus::Draft
        );
        assert_eq!(
            AssessmentStatus::from_str("completed").unwrap(),
            AssessmentStatus::Completed
        );
        assert_eq!(
            AssessmentStatus::from_str("COMPLETED").unwrap(),
            AssessmentStatus::Completed
        );
    }

    #[test]
    fn test_status_from_str_invalid() {
        assert!(AssessmentStatus::from_str("pending").is_err());
        assert!(AssessmentStatus::from_str("").is_err());
    }

    #[test]
    fn test_status_as_str() {
        assert_eq!(AssessmentStatus::Draft.as_str(), "draft");
        assert_eq!(AssessmentStatus::Completed.as_str(), "completed");
    }

    #[test]
    fn test_status_default() {
        assert_eq!(AssessmentStatus::default(), AssessmentStatus::Completed);
    }

    #[test]
    fn test_status_helpers() {
        assert!(AssessmentStatus::Draft.is_draft());
        assert!(!AssessmentStatus::Draft.is_completed());
        assert!(AssessmentStatus::Completed.is_completed());
        assert!(!AssessmentStatus::Completed.is_draft());
    }

    // SeverityLevel tests
    #[test]
    fn test_severity_from_str() {
        assert_eq!(
            SeverityLevel::from_str("minimal").unwrap(),
            SeverityLevel::Minimal
        );
        assert_eq!(
            SeverityLevel::from_str("MILD").unwrap(),
            SeverityLevel::Mild
        );
        assert_eq!(
            SeverityLevel::from_str("moderately_severe").unwrap(),
            SeverityLevel::ModeratelySevere
        );
    }

    #[test]
    fn test_severity_from_str_invalid() {
        assert!(SeverityLevel::from_str("critical").is_err());
        assert!(SeverityLevel::from_str("").is_err());
    }

    #[test]
    fn test_severity_as_str() {
        assert_eq!(SeverityLevel::Minimal.as_str(), "minimal");
        assert_eq!(
            SeverityLevel::ModeratelySevere.as_str(),
            "moderately_severe"
        );
    }

    #[test]
    fn test_severity_compare() {
        assert_eq!(
            SeverityLevel::Minimal.compare(&SeverityLevel::Severe),
            Some(std::cmp::Ordering::Less)
        );
        assert_eq!(
            SeverityLevel::Severe.compare(&SeverityLevel::Mild),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            SeverityLevel::Moderate.compare(&SeverityLevel::Moderate),
            Some(std::cmp::Ordering::Equal)
        );
        assert_eq!(SeverityLevel::Unknown.compare(&SeverityLevel::Mild), None);
    }

    // AssessmentCode tests
    #[test]
    fn test_code_from_str() {
        assert_eq!(
            AssessmentCode::from_str("PHQ9").unwrap(),
            AssessmentCode::Phq9
        );
        assert_eq!(
            AssessmentCode::from_str("phq9").unwrap(),
            AssessmentCode::Phq9
        );
        assert_eq!(
            AssessmentCode::from_str("PHQ-9").unwrap(),
            AssessmentCode::Phq9
        );
        assert_eq!(
            AssessmentCode::from_str("gad7").unwrap(),
            AssessmentCode::Gad7
        );
        assert_eq!(
            AssessmentCode::from_str("CESD").unwrap(),
            AssessmentCode::Cesd
        );
        assert_eq!(
            AssessmentCode::from_str("CES-D").unwrap(),
            AssessmentCode::Cesd
        );
        assert_eq!(
            AssessmentCode::from_str("OASIS").unwrap(),
            AssessmentCode::Oasis
        );
    }

    #[test]
    fn test_code_from_str_invalid() {
        assert!(AssessmentCode::from_str("PHQ10").is_err());
        assert!(AssessmentCode::from_str("").is_err());
    }

    #[test]
    fn test_code_as_str() {
        assert_eq!(AssessmentCode::Phq9.as_str(), "PHQ9");
        assert_eq!(AssessmentCode::Gad7.as_str(), "GAD7");
        assert_eq!(AssessmentCode::Cesd.as_str(), "CESD");
        assert_eq!(AssessmentCode::Oasis.as_str(), "OASIS");
    }

    #[test]
    fn test_code_question_count() {
        assert_eq!(AssessmentCode::Phq9.question_count(), 9);
        assert_eq!(AssessmentCode::Gad7.question_count(), 7);
        assert_eq!(AssessmentCode::Cesd.question_count(), 20);
        assert_eq!(AssessmentCode::Oasis.question_count(), 5);
    }

    #[test]
    fn test_code_max_score() {
        assert_eq!(AssessmentCode::Phq9.max_score(), 27);
        assert_eq!(AssessmentCode::Gad7.max_score(), 21);
        assert_eq!(AssessmentCode::Cesd.max_score(), 60);
        assert_eq!(AssessmentCode::Oasis.max_score(), 20);
    }

    #[test]
    fn test_code_serde_roundtrip() {
        let code = AssessmentCode::Phq9;
        let json = serde_json::to_string(&code).unwrap();
        assert_eq!(json, r#""PHQ9""#);
        let parsed: AssessmentCode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, code);
    }

    #[test]
    fn test_status_serde_roundtrip() {
        let status = AssessmentStatus::Draft;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""draft""#);
        let parsed: AssessmentStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, status);
    }

    #[test]
    fn test_severity_serde_roundtrip() {
        let severity = SeverityLevel::ModeratelySevere;
        let json = serde_json::to_string(&severity).unwrap();
        assert_eq!(json, r#""moderately_severe""#);
        let parsed: SeverityLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, severity);
    }
}
