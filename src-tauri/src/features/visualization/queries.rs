// Visualization queries (User Story 4 & 5)
// T126, T144: Tauri commands for chart data retrieval

use tauri::State;

use crate::AppState;

use super::models::*;
use super::repository::VisualizationRepository;

/// T126: Get assessment chart data for visualization
#[tauri::command]
#[specta::specta]
pub fn get_assessment_chart_data(
    code: String,
    time_range: TimeRange,
    from_date: Option<String>,
    to_date: Option<String>,
    state: State<AppState>,
) -> Result<AssessmentChartData, String> {
    let repo = VisualizationRepository::new(state.db.clone());

    // Resolve time range to dates
    let (from, to) = match time_range {
        TimeRange::Custom => {
            // Custom range requires both dates
            if from_date.is_none() || to_date.is_none() {
                return Err("from_date and to_date required for custom time range".to_string());
            }
            (from_date, to_date)
        }
        TimeRange::AllTime => (None, None),
        _ => {
            // Use predefined range
            match time_range.to_date_range() {
                Some((from_str, to_str)) => (Some(from_str), Some(to_str)),
                None => (None, None),
            }
        }
    };

    repo.get_assessment_chart_data(&code, from.as_deref(), to.as_deref())
        .map_err(|e| format!("Failed to fetch chart data: {}", e))
}

/// T144: Get mood chart data for visualization
#[tauri::command]
#[specta::specta]
pub fn get_mood_chart_data(
    time_range: TimeRange,
    from_date: Option<String>,
    to_date: Option<String>,
    group_by_activity: bool,
    state: State<AppState>,
) -> Result<MoodChartData, String> {
    let repo = VisualizationRepository::new(state.db.clone());

    // Resolve time range to dates
    let (from, to) = match time_range {
        TimeRange::Custom => {
            // Custom range requires both dates
            if from_date.is_none() || to_date.is_none() {
                return Err("from_date and to_date required for custom time range".to_string());
            }
            (from_date, to_date)
        }
        TimeRange::AllTime => (None, None),
        _ => {
            // Use predefined range
            match time_range.to_date_range() {
                Some((from_str, to_str)) => (Some(from_str), Some(to_str)),
                None => (None, None),
            }
        }
    };

    repo.get_mood_chart_data(from.as_deref(), to.as_deref(), group_by_activity)
        .map_err(|e| format!("Failed to fetch mood chart data: {}", e))
}
