// Visualization repository (User Story 4 & 5)
// T124-T125, T142-T143: Database queries for chart data

use std::sync::Arc;

use crate::db::Database;
use crate::features::assessments::models::AssessmentType;
use crate::features::mood::models::Activity;

use super::models::*;

pub struct VisualizationRepository {
    db: Arc<Database>,
}

impl VisualizationRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// T124: Get assessment chart data with time-series aggregation
    pub fn get_assessment_chart_data(
        &self,
        code: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<AssessmentChartData, VisualizationError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| VisualizationError::LockPoisoned)?;

        // Get assessment type
        let assessment_type = self.get_assessment_type_by_code(&conn, code)?;

        // SECURITY NOTE: Dynamic query building pattern used here
        // This is SAFE because:
        // 1. date_filter variable contains only static SQL strings (no user input)
        // 2. All user-provided values (from_date, to_date) are passed via params vector with `?` placeholders
        // 3. format!() inserts only the static date_filter string, not user data
        // This pattern allows flexible query construction while maintaining 100% parameterization

        // Build date filter
        let (date_filter, params): (String, Vec<Box<dyn rusqlite::ToSql>>) =
            match (from_date, to_date) {
                (Some(from), Some(to)) => (
                    "AND ar.completed_at >= ? AND ar.completed_at <= ?".to_string(),
                    vec![Box::new(from.to_string()), Box::new(to.to_string())],
                ),
                (Some(from), None) => (
                    "AND ar.completed_at >= ?".to_string(),
                    vec![Box::new(from.to_string())],
                ),
                (None, Some(to)) => (
                    "AND ar.completed_at <= ?".to_string(),
                    vec![Box::new(to.to_string())],
                ),
                (None, None) => ("".to_string(), vec![]),
            };

        // Query data points
        let query = format!(
            "SELECT ar.completed_at, ar.total_score, ar.severity_level
             FROM assessment_responses ar
             WHERE ar.assessment_type_id = ?
             {}
             ORDER BY ar.completed_at ASC",
            date_filter
        );

        let mut stmt = conn.prepare(&query)?;

        // Build params for query
        let mut query_params: Vec<&dyn rusqlite::ToSql> = vec![&assessment_type.id];
        for param in &params {
            query_params.push(param.as_ref());
        }

        let data_points: Vec<ChartDataPoint> = stmt
            .query_map(&query_params[..], |row| {
                Ok(ChartDataPoint {
                    timestamp: row.get(0)?,
                    value: row.get::<_, i32>(1)? as f64,
                    label: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        if data_points.is_empty() {
            return Err(VisualizationError::NoData);
        }

        // Get thresholds
        let thresholds = self.get_thresholds(&assessment_type)?;

        // Calculate statistics
        let statistics = self.calculate_statistics(&data_points, &assessment_type)?;

        Ok(AssessmentChartData {
            assessment_type,
            data_points,
            thresholds,
            statistics,
        })
    }

    /// T142: Get mood chart data with activity breakdown
    pub fn get_mood_chart_data(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
        group_by_activity: bool,
    ) -> Result<MoodChartData, VisualizationError> {
        let conn = self.db.get_connection();
        let conn = conn.lock().map_err(|_| VisualizationError::LockPoisoned)?;

        // SECURITY NOTE: Dynamic query building pattern used here
        // This is SAFE because:
        // 1. date_filter variable contains only static SQL strings (no user input)
        // 2. All user-provided values (from_date, to_date) are passed via params vector with `?` placeholders
        // 3. format!() inserts only the static date_filter string, not user data
        // This pattern allows flexible query construction while maintaining 100% parameterization

        // Build date filter
        let (date_filter, params): (String, Vec<Box<dyn rusqlite::ToSql>>) =
            match (from_date, to_date) {
                (Some(from), Some(to)) => (
                    "WHERE mc.created_at >= ? AND mc.created_at <= ?".to_string(),
                    vec![Box::new(from.to_string()), Box::new(to.to_string())],
                ),
                (Some(from), None) => (
                    "WHERE mc.created_at >= ?".to_string(),
                    vec![Box::new(from.to_string())],
                ),
                (None, Some(to)) => (
                    "WHERE mc.created_at <= ?".to_string(),
                    vec![Box::new(to.to_string())],
                ),
                (None, None) => ("".to_string(), vec![]),
            };

        // Query mood data points
        let query = format!(
            "SELECT mc.created_at, mc.mood_rating
             FROM mood_checkins mc
             {}
             ORDER BY mc.created_at ASC",
            date_filter
        );

        let mut stmt = conn.prepare(&query)?;

        let query_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let data_points: Vec<ChartDataPoint> = stmt
            .query_map(&query_params[..], |row| {
                Ok(ChartDataPoint {
                    timestamp: row.get(0)?,
                    value: row.get::<_, i32>(1)? as f64,
                    label: None,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        if data_points.is_empty() {
            return Err(VisualizationError::NoData);
        }

        // Calculate mood statistics
        let ratings: Vec<i32> = data_points.iter().map(|dp| dp.value as i32).collect();
        let mut statistics = calculate_mood_statistics(&ratings).ok_or_else(|| {
            VisualizationError::StatisticsError("No ratings available".to_string())
        })?;

        // Calculate checkins per day
        if let (Some(from), Some(to)) = (from_date, to_date) {
            if let (Ok(from_dt), Ok(to_dt)) = (
                chrono::NaiveDateTime::parse_from_str(from, "%Y-%m-%d %H:%M:%S"),
                chrono::NaiveDateTime::parse_from_str(to, "%Y-%m-%d %H:%M:%S"),
            ) {
                let days = (to_dt - from_dt).num_days().max(1);
                statistics.checkins_per_day = ratings.len() as f64 / days as f64;
            }
        }

        // Get activity breakdown if requested
        let activity_breakdown = if group_by_activity {
            self.get_activity_mood_breakdown(&conn, from_date, to_date)?
        } else {
            vec![]
        };

        Ok(MoodChartData {
            data_points,
            activity_breakdown,
            statistics,
        })
    }

    /// Get activity-specific mood data for correlation analysis
    fn get_activity_mood_breakdown(
        &self,
        conn: &rusqlite::Connection,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<ActivityMoodData>, VisualizationError> {
        // SECURITY NOTE: Dynamic query building pattern used here
        // This is SAFE because:
        // 1. date_filter variable contains only static SQL strings (no user input)
        // 2. All user-provided values (from_date, to_date) are passed via params vector with `?` placeholders
        // 3. format!() inserts only the static date_filter string, not user data
        // This pattern allows flexible query construction while maintaining 100% parameterization
        let (date_filter, params): (String, Vec<Box<dyn rusqlite::ToSql>>) =
            match (from_date, to_date) {
                (Some(from), Some(to)) => (
                    "AND mc.created_at >= ? AND mc.created_at <= ?".to_string(),
                    vec![Box::new(from.to_string()), Box::new(to.to_string())],
                ),
                (Some(from), None) => (
                    "AND mc.created_at >= ?".to_string(),
                    vec![Box::new(from.to_string())],
                ),
                (None, Some(to)) => (
                    "AND mc.created_at <= ?".to_string(),
                    vec![Box::new(to.to_string())],
                ),
                (None, None) => ("".to_string(), vec![]),
            };

        let query = format!(
            "SELECT
                a.id, a.name, a.color, a.icon,
                AVG(mc.mood_rating) as avg_mood,
                COUNT(mc.id) as checkin_count
             FROM activities a
             JOIN mood_checkin_activities mca ON a.id = mca.activity_id
             JOIN mood_checkins mc ON mca.mood_checkin_id = mc.id
             WHERE a.deleted_at IS NULL
             {}
             GROUP BY a.id, a.name, a.color, a.icon
             HAVING checkin_count >= 2
             ORDER BY avg_mood DESC",
            date_filter
        );

        let mut stmt = conn.prepare(&query)?;

        let query_params: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let breakdown: Vec<ActivityMoodData> = stmt
            .query_map(&query_params[..], |row| {
                let activity = Activity {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    icon: row.get(3)?,
                    created_at: String::new(), // Not needed for visualization
                    deleted_at: None,
                };

                Ok(ActivityMoodData {
                    activity,
                    average_mood: row.get(4)?,
                    data_points: vec![], // Populated separately if needed
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(breakdown)
    }

    /// Get assessment type by code
    fn get_assessment_type_by_code(
        &self,
        conn: &rusqlite::Connection,
        code: &str,
    ) -> Result<AssessmentType, VisualizationError> {
        conn.query_row(
            "SELECT id, code, name, description, question_count, min_score, max_score, thresholds
             FROM assessment_types
             WHERE code = ?",
            [code],
            |row| {
                let thresholds_str: String = row.get(7)?;
                Ok(AssessmentType {
                    id: row.get(0)?,
                    code: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    question_count: row.get(4)?,
                    min_score: row.get(5)?,
                    max_score: row.get(6)?,
                    thresholds: serde_json::from_str(&thresholds_str)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                })
            },
        )
        .map_err(|_| VisualizationError::InvalidAssessmentType(code.to_string()))
    }

    /// T125: Calculate chart statistics
    fn calculate_statistics(
        &self,
        data_points: &[ChartDataPoint],
        _assessment_type: &AssessmentType,
    ) -> Result<ChartStatistics, VisualizationError> {
        if data_points.is_empty() {
            return Err(VisualizationError::StatisticsError(
                "No data points available".to_string(),
            ));
        }

        let values: Vec<f64> = data_points.iter().map(|dp| dp.value).collect();

        let min = values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(0.0);
        let max = values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(0.0);
        let sum: f64 = values.iter().sum();
        let average = sum / values.len() as f64;

        // Calculate trend
        let first_score = values.first().copied().unwrap_or(0.0);
        let last_score = values.last().copied().unwrap_or(0.0);
        let trend = calculate_trend(first_score, last_score);

        Ok(ChartStatistics {
            min,
            max,
            average,
            trend,
            total_assessments: data_points.len() as i32,
        })
    }

    /// Get threshold lines for assessment type
    fn get_thresholds(
        &self,
        assessment_type: &AssessmentType,
    ) -> Result<Vec<ThresholdLine>, VisualizationError> {
        // Parse thresholds JSON (already deserialized as Value from database)
        let thresholds_map = assessment_type.thresholds.as_object().ok_or_else(|| {
            VisualizationError::StatisticsError("Thresholds is not a JSON object".to_string())
        })?;

        let mut thresholds = Vec::new();

        // Define colors for severity levels
        let colors = [
            ("minimal", "#4CAF50"),           // Green
            ("mild", "#FFEB3B"),              // Yellow
            ("moderate", "#FF9800"),          // Orange
            ("moderately_severe", "#F44336"), // Red
            ("severe", "#B71C1C"),            // Dark Red
        ];

        for (level, color) in &colors {
            if let Some(value) = thresholds_map.get(*level) {
                if let Some(threshold_value) = value.as_i64() {
                    thresholds.push(ThresholdLine {
                        label: level.replace('_', " ").to_string(),
                        value: threshold_value as f64,
                        color: color.to_string(),
                    });
                }
            }
        }

        Ok(thresholds)
    }
}
