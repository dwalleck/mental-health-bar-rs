# Visualization Feature API Contract

**Feature**: Data Visualization & Chart Queries
**Commands**: 3 (all queries, no mutations)

## Data Types

### ChartDataPoint
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct ChartDataPoint {
    pub timestamp: String,  // ISO 8601
    pub value: f64,         // Score or rating
    pub label: Option<String>,  // Optional annotation (e.g., severity level)
}
```

### AssessmentChartData
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct AssessmentChartData {
    pub assessment_type: AssessmentType,
    pub data_points: Vec<ChartDataPoint>,
    pub thresholds: Vec<ThresholdLine>,
    pub statistics: ChartStatistics,
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct ThresholdLine {
    pub label: String,      // "Mild", "Moderate", "Severe"
    pub value: f64,         // Threshold score
    pub color: String,      // Hex color for UI
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct ChartStatistics {
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub trend: TrendDirection,  // "improving", "worsening", "stable"
    pub total_assessments: i32,
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TrendDirection {
    Improving,   // Scores decreasing (lower is better for depression/anxiety)
    Worsening,   // Scores increasing
    Stable,      // No significant change
}
```

### MoodChartData
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct MoodChartData {
    pub data_points: Vec<ChartDataPoint>,
    pub activity_breakdown: Vec<ActivityMoodData>,
    pub statistics: MoodStatistics,
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct ActivityMoodData {
    pub activity: Activity,
    pub average_mood: f64,
    pub data_points: Vec<ChartDataPoint>,  // Mood scores when this activity present
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct MoodStatistics {
    pub min: i32,
    pub max: i32,
    pub average: f64,
    pub median: f64,
    pub mode: i32,              // Most common mood rating
    pub total_checkins: i32,
    pub checkins_per_day: f64,  // Average
}
```

### TimeRange
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TimeRange {
    Week,       // Last 7 days
    Month,      // Last 30 days
    Quarter,    // Last 90 days
    Year,       // Last 365 days
    AllTime,    // All data
    Custom,     // Custom date range
}
```

---

## Queries (Read Operations)

### 1. `get_assessment_chart_data`

**Purpose**: Fetch assessment score trends for charting

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_chart_data(
    code: String,
    time_range: TimeRange,
    from_date: Option<String>,  // Required if time_range = Custom
    to_date: Option<String>,
    state: State<'_, AppState>
) -> Result<AssessmentChartData, String>
```

**Parameters**:
- `code`: Assessment type code (e.g., "PHQ9")
- `time_range`: Predefined range or Custom
- `from_date`: Start date for Custom range (ISO 8601)
- `to_date`: End date for Custom range

**Returns**: Chart data with scores, thresholds, and statistics

**Errors**:
- `"Invalid assessment type: {code}"` - Unknown assessment
- `"from_date and to_date required for custom time range"` - Missing dates
- `"Invalid date format: {date}"` - Bad date
- `"No data available for this time range"` - No assessments found
- `"Failed to fetch chart data: {error}"` - Database error

**Example Usage**:
```typescript
const chartData = await invoke<AssessmentChartData>('get_assessment_chart_data', {
  code: 'PHQ9',
  timeRange: 'quarter',
  fromDate: null,
  toDate: null
});

// Use with Chart.js
const ctx = document.getElementById('chart');
new Chart(ctx, {
  type: 'line',
  data: {
    labels: chartData.data_points.map(p => new Date(p.timestamp)),
    datasets: [{
      label: chartData.assessment_type.name,
      data: chartData.data_points.map(p => p.value),
      borderColor: '#2196F3',
    }]
  },
  options: {
    plugins: {
      annotation: {
        annotations: chartData.thresholds.map(t => ({
          type: 'line',
          yMin: t.value,
          yMax: t.value,
          borderColor: t.color,
          label: { content: t.label, enabled: true }
        }))
      }
    }
  }
});
```

**Trend Calculation Logic**:
```
- If latest_score < first_score by >20%: "improving"
- If latest_score > first_score by >20%: "worsening"
- Otherwise: "stable"
```

---

### 2. `get_mood_chart_data`

**Purpose**: Fetch mood check-in trends for charting

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_mood_chart_data(
    time_range: TimeRange,
    from_date: Option<String>,
    to_date: Option<String>,
    group_by_activity: bool,     // If true, include activity breakdown
    state: State<'_, AppState>
) -> Result<MoodChartData, String>
```

**Parameters**:
- `time_range`: Predefined range or Custom
- `from_date`: Start date for Custom range
- `to_date`: End date for Custom range
- `group_by_activity`: If true, calculate per-activity averages

**Returns**: Mood chart data with optional activity breakdown

**Errors**:
- `"from_date and to_date required for custom time range"` - Missing dates
- `"Invalid date format: {date}"` - Bad date
- `"No data available for this time range"` - No check-ins found
- `"Failed to fetch mood chart data: {error}"` - Database error

**Example Usage**:
```typescript
const moodData = await invoke<MoodChartData>('get_mood_chart_data', {
  timeRange: 'month',
  fromDate: null,
  toDate: null,
  groupByActivity: true
});

console.log('Average mood:', moodData.statistics.average.toFixed(2));
console.log('Most common:', moodData.statistics.mode);

// Activity correlations
moodData.activity_breakdown.forEach(ab => {
  console.log(`${ab.activity.name}: avg ${ab.average_mood.toFixed(2)}`);
});
```

---

### 3. `get_comparison_chart_data`

**Purpose**: Compare multiple assessments side-by-side

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_comparison_chart_data(
    codes: Vec<String>,          // ["PHQ9", "GAD7"]
    time_range: TimeRange,
    from_date: Option<String>,
    to_date: Option<String>,
    state: State<'_, AppState>
) -> Result<ComparisonChartData, String>

#[derive(Serialize, Deserialize, specta::Type)]
pub struct ComparisonChartData {
    pub datasets: Vec<AssessmentChartDataset>,
    pub time_range: (String, String),  // (start, end) dates
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct AssessmentChartDataset {
    pub assessment_type: AssessmentType,
    pub data_points: Vec<ChartDataPoint>,
    pub color: String,  // Assigned color for this dataset
}
```

**Parameters**:
- `codes`: List of assessment codes to compare (max 4)
- `time_range`: Predefined range or Custom
- `from_date`: Start date for Custom range
- `to_date`: End date for Custom range

**Returns**: Multiple datasets for comparison chart

**Errors**:
- `"codes cannot be empty"` - No assessment codes provided
- `"Maximum 4 assessments can be compared"` - Too many assessments
- `"Invalid assessment type: {code}"` - Unknown assessment
- `"Failed to fetch comparison data: {error}"` - Database error

**Example Usage**:
```typescript
// Compare depression (PHQ-9) and anxiety (GAD-7) over time
const comparison = await invoke<ComparisonChartData>('get_comparison_chart_data', {
  codes: ['PHQ9', 'GAD7'],
  timeRange: 'quarter',
  fromDate: null,
  toDate: null
});

// Multi-line chart
new Chart(ctx, {
  type: 'line',
  data: {
    datasets: comparison.datasets.map(ds => ({
      label: ds.assessment_type.name,
      data: ds.data_points.map(p => ({ x: new Date(p.timestamp), y: p.value })),
      borderColor: ds.color,
    }))
  },
  options: {
    scales: {
      y: { beginAtZero: true }
    }
  }
});
```

---

## Chart Recommendations

### Assessment Trend Charts
- **Type**: Line chart
- **X-axis**: Time (dates)
- **Y-axis**: Score (0 to max_score)
- **Annotations**: Threshold lines for severity levels
- **Colors**: Blue for line, red/yellow/green for thresholds
- **Interaction**: Tooltip shows exact score and severity on hover

### Mood Trend Charts
- **Type**: Line chart or bar chart
- **X-axis**: Time (dates or hours for intra-day)
- **Y-axis**: Mood rating (1-5)
- **Colors**: Gradient from red (1) to green (5)
- **Interaction**: Tooltip shows activities and notes

### Activity Correlation Charts
- **Type**: Horizontal bar chart
- **X-axis**: Average mood (1-5)
- **Y-axis**: Activity names
- **Colors**: Activity-specific colors from database
- **Sorting**: Descending by average mood (best activities first)

### Comparison Charts
- **Type**: Multi-line chart
- **X-axis**: Time
- **Y-axis**: Normalized scores (0-100% of max_score)
- **Colors**: Distinct colors per assessment (blue, purple, orange, green)
- **Legend**: Show assessment names with colors

---

## Performance Considerations

### Query Optimization
```sql
-- Use date indexes
CREATE INDEX idx_responses_date ON assessment_responses(completed_at);
CREATE INDEX idx_mood_date ON mood_checkins(created_at);

-- Use aggregations in database (not application)
SELECT
    DATE(completed_at) as date,
    AVG(total_score) as avg_score  -- If multiple per day
FROM assessment_responses
WHERE completed_at >= ? AND completed_at <= ?
GROUP BY DATE(completed_at)
ORDER BY date ASC;
```

### Data Point Limits
- **Week**: All points (max ~50 for daily checks)
- **Month**: All points (max ~200)
- **Quarter**: Daily aggregates (90 points)
- **Year**: Weekly aggregates (52 points)
- **All Time**: Monthly aggregates

### Caching Strategy
- Cache chart data in Svelte stores (5-minute TTL)
- Invalidate cache on new assessment/mood submission
- Preload chart data on app launch (background)

---

## Color Palette

### Assessment Thresholds
```rust
pub const THRESHOLD_COLORS: &[(&str, &str)] = &[
    ("minimal", "#4CAF50"),    // Green
    ("mild", "#FFEB3B"),       // Yellow
    ("moderate", "#FF9800"),   // Orange
    ("moderately_severe", "#F44336"),  // Red
    ("severe", "#B71C1C"),     // Dark Red
];
```

### Mood Colors
```rust
pub const MOOD_COLORS: &[&str] = &[
    "",              // Mood 0 (invalid)
    "#F44336",       // Mood 1: Very Bad (Red)
    "#FF9800",       // Mood 2: Bad (Orange)
    "#FFEB3B",       // Mood 3: Neutral (Yellow)
    "#8BC34A",       // Mood 4: Good (Light Green)
    "#4CAF50",       // Mood 5: Very Good (Green)
];
```

### Comparison Datasets
```rust
pub const COMPARISON_COLORS: &[&str] = &[
    "#2196F3",  // Blue
    "#9C27B0",  // Purple
    "#FF5722",  // Orange
    "#4CAF50",  // Green
];
```

---

## Implementation Notes

1. **DuckDB Aggregations**: Use columnar queries for fast time-series aggregations
2. **Date Bucketing**: Group by day/week/month at database level, not in Rust
3. **Interpolation**: No interpolation for missing data (show gaps in chart)
4. **Responsive**: Return data in format ready for Chart.js (minimal frontend processing)
5. **Statistics**: Calculate in database query (COUNT, AVG, MIN, MAX)

---

## Testing

### Unit Tests
```rust
#[test]
fn test_trend_calculation() {
    let first_score = 20;
    let last_score = 10;
    let trend = calculate_trend(first_score, last_score);
    assert_eq!(trend, TrendDirection::Improving);  // 50% reduction
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_chart_data_generation() {
    // Create test data
    for i in 0..10 {
        submit_assessment(phq9_request(score: i * 2)).await;
    }

    let chart_data = get_assessment_chart_data("PHQ9", TimeRange::AllTime).await;
    assert_eq!(chart_data.data_points.len(), 10);
    assert_eq!(chart_data.statistics.min, 0);
    assert_eq!(chart_data.statistics.max, 18);
}
```
