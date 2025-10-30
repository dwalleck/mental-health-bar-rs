// Visualization feature models (User Story 4 & 5)
// T121-T123, T139-T141: Data types for chart visualization

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::{CommandError, ErrorType, ToCommandError};
use crate::features::assessments::models::AssessmentType;
use crate::features::mood::models::Activity;

/// Visualization-specific errors
#[derive(Error, Debug)]
pub enum VisualizationError {
    #[error("No data available for this time range")]
    NoData,

    #[error("Invalid assessment type: {0}")]
    InvalidAssessmentType(String),

    #[error("Failed to calculate statistics: {0}")]
    StatisticsError(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Database lock issue. This request will be retried automatically.")]
    LockPoisoned,

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl ToCommandError for VisualizationError {
    fn to_command_error(&self) -> CommandError {
        match self {
            // Validation errors - not retryable
            VisualizationError::InvalidAssessmentType(code) => {
                CommandError::permanent(self.to_string(), ErrorType::Validation).with_details(
                    serde_json::json!({
                        "field": "assessment_type_code",
                        "value": code
                    }),
                )
            }
            VisualizationError::NoData => {
                CommandError::permanent(self.to_string(), ErrorType::NoData)
            }
            VisualizationError::StatisticsError(_) => {
                CommandError::permanent(self.to_string(), ErrorType::CalculationError)
            }

            // Database errors - use the shared helper
            VisualizationError::Database(e) => CommandError::from_rusqlite_error(e),

            // Lock poisoned - retryable
            VisualizationError::LockPoisoned => CommandError::retryable(
                "Database lock issue. This request will be retried automatically.".to_string(),
                ErrorType::LockPoisoned,
            ),

            // JSON errors - not retryable (internal error)
            VisualizationError::JsonError(_) => {
                CommandError::permanent(self.to_string(), ErrorType::Internal)
            }
        }
    }
}

/// Chart data point for time-series visualization
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct ChartDataPoint {
    pub timestamp: String,     // ISO 8601 format
    pub value: f64,            // Score or rating
    pub label: Option<String>, // Optional annotation (e.g., severity level)
}

/// Assessment chart data with thresholds and statistics
#[derive(Serialize, Deserialize, specta::Type, Debug)]
pub struct AssessmentChartData {
    pub assessment_type: AssessmentType,
    pub data_points: Vec<ChartDataPoint>,
    pub thresholds: Vec<ThresholdLine>,
    pub statistics: ChartStatistics,
}

/// Threshold line for severity level visualization
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct ThresholdLine {
    pub label: String, // "Mild", "Moderate", "Severe"
    pub value: f64,    // Threshold score
    pub color: String, // Hex color for UI
}

/// Chart statistics (min, max, average, trend)
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct ChartStatistics {
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub trend: TrendDirection,
    pub total_assessments: i32,
}

/// Trend direction for assessment scores
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TrendDirection {
    Improving, // Scores decreasing (lower is better)
    Worsening, // Scores increasing
    Stable,    // No significant change
}

/// Mood chart data with activity breakdown
#[derive(Serialize, Deserialize, specta::Type, Debug)]
pub struct MoodChartData {
    pub data_points: Vec<ChartDataPoint>,
    pub activity_breakdown: Vec<ActivityMoodData>,
    pub statistics: MoodStatistics,
}

/// Activity-specific mood data for correlation analysis
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct ActivityMoodData {
    pub activity: Activity,
    pub average_mood: f64,
    pub data_points: Vec<ChartDataPoint>, // Mood scores when this activity present
}

/// Mood statistics (min, max, average, median, mode)
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
pub struct MoodStatistics {
    pub min: i32,
    pub max: i32,
    pub average: f64,
    pub median: f64,
    pub mode: i32, // Most common mood rating
    pub total_checkins: i32,
    pub checkins_per_day: f64, // Average
}

/// Time range for chart data queries
#[derive(Serialize, Deserialize, specta::Type, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TimeRange {
    Week,    // Last 7 days
    Month,   // Last 30 days
    Quarter, // Last 90 days
    Year,    // Last 365 days
    AllTime, // All data
    Custom,  // Custom date range
}

impl TimeRange {
    /// Convert time range to date bounds (from_date, to_date)
    pub fn to_date_range(&self) -> Option<(String, String)> {
        use chrono::{Duration, Utc};

        let now = Utc::now();
        let to_date = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let from_date = match self {
            TimeRange::Week => now.checked_sub_signed(Duration::days(7)),
            TimeRange::Month => now.checked_sub_signed(Duration::days(30)),
            TimeRange::Quarter => now.checked_sub_signed(Duration::days(90)),
            TimeRange::Year => now.checked_sub_signed(Duration::days(365)),
            TimeRange::AllTime => return None, // No date filter
            TimeRange::Custom => return None,  // Dates provided separately
        };

        from_date.map(|from| (from.format("%Y-%m-%d %H:%M:%S").to_string(), to_date))
    }
}

/// Calculate trend direction based on first and last scores
/// T117: Unit test - Trend calculation (improving/worsening/stable)
pub fn calculate_trend(first_score: f64, last_score: f64) -> TrendDirection {
    if first_score == 0.0 {
        return TrendDirection::Stable;
    }

    let change_percent = ((last_score - first_score) / first_score).abs();

    // Lower scores are better for depression/anxiety assessments
    if last_score < first_score && change_percent > 0.20 {
        TrendDirection::Improving
    } else if last_score > first_score && change_percent > 0.20 {
        TrendDirection::Worsening
    } else {
        TrendDirection::Stable
    }
}

/// Calculate mood statistics from mood ratings
pub fn calculate_mood_statistics(ratings: &[i32]) -> Option<MoodStatistics> {
    if ratings.is_empty() {
        return None;
    }

    let min = *ratings.iter().min()?;
    let max = *ratings.iter().max()?;
    let sum: i32 = ratings.iter().sum();
    let average = sum as f64 / ratings.len() as f64;

    // Calculate median
    let mut sorted = ratings.to_vec();
    sorted.sort();
    let median = if sorted.len().is_multiple_of(2) {
        let mid = sorted.len() / 2;
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
    } else {
        sorted[sorted.len() / 2] as f64
    };

    // Calculate mode (most common rating)
    let mut counts = std::collections::HashMap::new();
    for &rating in ratings {
        *counts.entry(rating).or_insert(0) += 1;
    }
    let mode = *counts.iter().max_by_key(|(_, count)| *count)?.0;

    Some(MoodStatistics {
        min,
        max,
        average,
        median,
        mode,
        total_checkins: ratings.len() as i32,
        checkins_per_day: 0.0, // Calculated in repository with date range
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // T117: Unit test - Trend calculation
    #[test]
    fn test_trend_improving() {
        let trend = calculate_trend(20.0, 10.0);
        assert!(matches!(trend, TrendDirection::Improving));
    }

    #[test]
    fn test_trend_worsening() {
        let trend = calculate_trend(10.0, 20.0);
        assert!(matches!(trend, TrendDirection::Worsening));
    }

    #[test]
    fn test_trend_stable() {
        let trend = calculate_trend(10.0, 11.0); // 10% change, below 20% threshold
        assert!(matches!(trend, TrendDirection::Stable));
    }

    #[test]
    fn test_trend_zero_baseline() {
        let trend = calculate_trend(0.0, 10.0);
        assert!(matches!(trend, TrendDirection::Stable));
    }

    #[test]
    fn test_mood_statistics_calculation() {
        let ratings = vec![3, 4, 4, 5, 3, 2, 4];
        let stats = calculate_mood_statistics(&ratings).unwrap();

        assert_eq!(stats.min, 2);
        assert_eq!(stats.max, 5);
        assert_eq!(stats.mode, 4); // Most common
        assert_eq!(stats.total_checkins, 7);
        assert!((stats.average - 3.57).abs() < 0.01);
        assert!((stats.median - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_mood_statistics_empty() {
        let ratings: Vec<i32> = vec![];
        let stats = calculate_mood_statistics(&ratings);
        assert!(stats.is_none());
    }

    #[test]
    fn test_time_range_conversion() {
        let week_range = TimeRange::Week.to_date_range();
        assert!(week_range.is_some());

        let (from, to) = week_range.unwrap();
        assert!(!from.is_empty());
        assert!(!to.is_empty());

        // AllTime should return None (no filter)
        assert!(TimeRange::AllTime.to_date_range().is_none());
        assert!(TimeRange::Custom.to_date_range().is_none());
    }
}
