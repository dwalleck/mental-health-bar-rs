# Algorithm Specifications

**Last Updated:** 2025-11-07
**Status:** Phase 0 - Specifications
**Related:** data-structures.md, REVISED-plan.md, REVISED-tasks.md

**Applies To Tasks:**
- **Week 2, Tasks 2.8-2.17**: Goal progress, activity frequency, trend analysis
- **Week 4, Tasks 4.1-4.16**: Reporting dashboard calculations
- **Week 5, Tasks 5.1-5.23**: Mood scale calculations (7-point)

---

## Overview

This document specifies all major algorithms in **pseudocode before implementation**. Each algorithm includes:
- Clear inputs and outputs with types
- Step-by-step logic
- Edge case handling
- Validation rules
- Expected behavior with examples

**Principle:** Write the algorithm specification → Write tests from spec → Implement to pass tests (TDD)

---

## Table of Contents

1. [Goal Progress Calculation](#goal-progress-calculation)
2. [Activity Frequency Calculation](#activity-frequency-calculation)
3. [Activity Trend Analysis](#activity-trend-analysis)
4. [Mood Correlation with Activities](#mood-correlation-with-activities)
5. [Report Aggregation](#report-aggregation)
6. [Chart Data Transformation](#chart-data-transformation)

---

## Goal Progress Calculation

**Purpose:** Calculate progress toward an activity or activity group goal

**Complexity:** HIGH (multiple goal types, date ranges, previous period comparison)

---

### Algorithm: `calculate_goal_progress`

**Inputs:**
```rust
goal: ActivityGoal {
  id: i64,
  activity_id: Option<i64>,     // NULL if group goal
  group_id: Option<i64>,        // NULL if activity goal
  goal_type: GoalType,          // 'days_per_period' | 'percent_improvement'
  target_value: i64,            // e.g., 7 (days) or 20 (%)
  period_days: i64              // e.g., 7 (week), 30 (month)
}

activity_logs: Vec<ActivityLog> {
  activity_id: i64,
  logged_at: DateTime<Utc>
}

current_time: DateTime<Utc>
```

**Output:**
```rust
GoalProgress {
  goal_id: i64,
  current_value: i64,       // e.g., 5 days or 25%
  target_value: i64,        // from goal
  percentage: f64,          // (current / target) * 100
  is_achieved: bool,        // percentage >= 100
  period_start: DateTime,
  period_end: DateTime
}
```

---

### Algorithm Steps

```
FUNCTION calculate_goal_progress(goal, activity_logs, current_time):

  # 1. Define current period boundaries
  period_end = current_time
  period_start = current_time - goal.period_days

  # 2. Filter logs to relevant activities
  IF goal.activity_id IS NOT NULL:
    relevant_logs = activity_logs.filter(log => log.activity_id == goal.activity_id)
  ELSE IF goal.group_id IS NOT NULL:
    # Get all activities in group
    group_activities = get_activities_in_group(goal.group_id)
    activity_ids = group_activities.map(a => a.id)
    relevant_logs = activity_logs.filter(log => activity_ids.contains(log.activity_id))
  ELSE:
    RETURN Error("Goal must have either activity_id or group_id")

  # 3. Filter logs to current period
  current_period_logs = relevant_logs.filter(log =>
    log.logged_at >= period_start AND log.logged_at <= period_end
  )

  # 4. Calculate current_value based on goal_type
  IF goal.goal_type == 'days_per_period':
    current_value = calculate_unique_days(current_period_logs)

  ELSE IF goal.goal_type == 'percent_improvement':
    # Define previous period
    previous_period_end = period_start
    previous_period_start = period_start - goal.period_days

    # Filter logs to previous period
    previous_period_logs = relevant_logs.filter(log =>
      log.logged_at >= previous_period_start AND log.logged_at < previous_period_end
    )

    # Calculate frequencies
    previous_value = calculate_unique_days(previous_period_logs)
    current_days = calculate_unique_days(current_period_logs)

    # Calculate improvement percentage
    IF previous_value == 0:
      # Handle division by zero
      IF current_days > 0:
        current_value = 100  # Infinite improvement = 100%
      ELSE:
        current_value = 0    # No improvement
    ELSE:
      improvement = ((current_days - previous_value) / previous_value) * 100
      current_value = round(improvement)

  ELSE:
    RETURN Error("Invalid goal_type")

  # 5. Calculate percentage and achievement
  percentage = (current_value / goal.target_value) * 100
  is_achieved = percentage >= 100

  # 6. Return progress
  RETURN GoalProgress {
    goal_id: goal.id,
    current_value: current_value,
    target_value: goal.target_value,
    percentage: percentage,
    is_achieved: is_achieved,
    period_start: period_start,
    period_end: period_end
  }

END FUNCTION
```

---

### Helper: `calculate_unique_days`

**Purpose:** Count unique days with activity logs

```
FUNCTION calculate_unique_days(logs: Vec<ActivityLog>) -> i64:

  IF logs.is_empty():
    RETURN 0

  # Extract date (ignore time) from each log
  dates = logs.map(log => log.logged_at.date())

  # Get unique dates
  unique_dates = dates.into_set()

  RETURN unique_dates.len()

END FUNCTION
```

---

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| **No logs in period** | `current_value = 0`, `percentage = 0`, `is_achieved = false` |
| **Previous period has 0 logs** | Improvement = 100% if current > 0, else 0% |
| **Goal already exceeded** | `percentage > 100`, `is_achieved = true` |
| **Period crosses month boundary** | Works correctly (period is always N days from current_time) |
| **Multiple logs same day** | Counts as 1 day (unique dates only) |
| **Future logs** | Ignored (filtered to `<= period_end`) |

---

### Examples

**Example 1: Days Per Period Goal**
```
Goal:
  goal_type = 'days_per_period'
  target_value = 5
  period_days = 7

Logs (last 7 days):
  Day 1: Activity X logged
  Day 2: Activity X logged twice
  Day 3: No log
  Day 4: Activity X logged
  Day 5: No log
  Day 6: Activity X logged
  Day 7: No log

Result:
  unique_days = 4 (Day 1, 2, 4, 6)
  current_value = 4
  target_value = 5
  percentage = 80%
  is_achieved = false
```

**Example 2: Percent Improvement Goal**
```
Goal:
  goal_type = 'percent_improvement'
  target_value = 20  (20% improvement)
  period_days = 7

Logs:
  Previous 7 days: 3 unique days
  Current 7 days: 4 unique days

Calculation:
  previous_value = 3
  current_days = 4
  improvement = ((4 - 3) / 3) * 100 = 33.33%
  current_value = 33 (rounded)
  target_value = 20
  percentage = (33 / 20) * 100 = 165%
  is_achieved = true
```

---

## Activity Frequency Calculation

**Purpose:** Calculate how many days per week an activity was performed

**Complexity:** MEDIUM (date range, unique days)

---

### Algorithm: `calculate_activity_frequency`

**Inputs:**
```rust
activity_id: i64
start_date: DateTime<Utc>
end_date: DateTime<Utc>
activity_logs: Vec<ActivityLog>
```

**Output:**
```rust
ActivityFrequency {
  activity_id: i64,
  unique_days: i64,           // Number of days with logs
  total_logs: i64,            // Total number of logs
  days_per_week: f64,         // (unique_days / num_weeks)
  period_start: DateTime,
  period_end: DateTime
}
```

---

### Algorithm Steps

```
FUNCTION calculate_activity_frequency(activity_id, start_date, end_date, logs):

  # 1. Validate date range
  IF end_date < start_date:
    RETURN Error("end_date must be >= start_date")

  # 2. Filter logs to activity and date range
  filtered_logs = logs.filter(log =>
    log.activity_id == activity_id AND
    log.logged_at >= start_date AND
    log.logged_at <= end_date
  )

  # 3. Count unique days
  unique_days = calculate_unique_days(filtered_logs)

  # 4. Count total logs
  total_logs = filtered_logs.len()

  # 5. Calculate days per week
  period_duration_days = (end_date - start_date).days()
  num_weeks = period_duration_days / 7.0

  IF num_weeks == 0:
    days_per_week = 0.0
  ELSE:
    days_per_week = unique_days / num_weeks

  # 6. Return frequency
  RETURN ActivityFrequency {
    activity_id: activity_id,
    unique_days: unique_days,
    total_logs: total_logs,
    days_per_week: days_per_week,
    period_start: start_date,
    period_end: end_date
  }

END FUNCTION
```

---

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| **Period < 1 week** | `days_per_week` may be > 7 (e.g., 14 for 3.5 days in 0.5 weeks) |
| **No logs** | `unique_days = 0`, `days_per_week = 0` |
| **Multiple logs per day** | Counts as 1 day for `unique_days`, all counted in `total_logs` |
| **Partial week** | Uses fractional weeks (e.g., 10 days = 1.43 weeks) |

---

### Examples

**Example: 2-Week Period**
```
Inputs:
  activity_id = 1
  start_date = 2025-01-01
  end_date = 2025-01-14 (14 days)

Logs:
  2025-01-01: 1 log
  2025-01-02: 2 logs
  2025-01-04: 1 log
  2025-01-08: 1 log
  2025-01-10: 1 log
  2025-01-12: 1 log

Result:
  unique_days = 6
  total_logs = 7
  period_duration = 14 days
  num_weeks = 2.0
  days_per_week = 6 / 2.0 = 3.0
```

---

## Activity Trend Analysis

**Purpose:** Calculate percent change from previous period

**Complexity:** MEDIUM (two periods, comparison)

---

### Algorithm: `calculate_activity_trend`

**Inputs:**
```rust
activity_id: i64
period_days: i64              // e.g., 7 (week), 30 (month)
current_time: DateTime<Utc>
activity_logs: Vec<ActivityLog>
```

**Output:**
```rust
ActivityTrend {
  activity_id: i64,
  current_period_days: i64,
  previous_period_days: i64,
  change_days: i64,           // current - previous
  change_percentage: f64,     // ((current - previous) / previous) * 100
  trend: Trend                // 'improving' | 'declining' | 'stable'
}
```

---

### Algorithm Steps

```
FUNCTION calculate_activity_trend(activity_id, period_days, current_time, logs):

  # 1. Define current period
  current_end = current_time
  current_start = current_time - period_days

  # 2. Define previous period (same duration, immediately before current)
  previous_end = current_start
  previous_start = current_start - period_days

  # 3. Filter logs to activity
  activity_logs = logs.filter(log => log.activity_id == activity_id)

  # 4. Get unique days for current period
  current_logs = activity_logs.filter(log =>
    log.logged_at >= current_start AND log.logged_at <= current_end
  )
  current_period_days = calculate_unique_days(current_logs)

  # 5. Get unique days for previous period
  previous_logs = activity_logs.filter(log =>
    log.logged_at >= previous_start AND log.logged_at < previous_end
  )
  previous_period_days = calculate_unique_days(previous_logs)

  # 6. Calculate change
  change_days = current_period_days - previous_period_days

  # 7. Calculate percentage change
  IF previous_period_days == 0:
    IF current_period_days > 0:
      change_percentage = 100.0  # Improvement from nothing
    ELSE:
      change_percentage = 0.0    # No change (both zero)
  ELSE:
    change_percentage = (change_days / previous_period_days) * 100.0

  # 8. Determine trend
  IF change_percentage > 10.0:
    trend = 'improving'
  ELSE IF change_percentage < -10.0:
    trend = 'declining'
  ELSE:
    trend = 'stable'

  # 9. Return trend analysis
  RETURN ActivityTrend {
    activity_id: activity_id,
    current_period_days: current_period_days,
    previous_period_days: previous_period_days,
    change_days: change_days,
    change_percentage: change_percentage,
    trend: trend
  }

END FUNCTION
```

---

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| **Previous period = 0 days** | `change_percentage = 100%` if current > 0, else `0%` |
| **Both periods = 0** | `change_percentage = 0%`, `trend = 'stable'` |
| **Negative change** | `change_percentage < 0`, may be `'declining'` |
| **Small changes** | Within ±10% = `'stable'` (threshold) |

---

### Examples

**Example: Improving Trend**
```
Inputs:
  activity_id = 1
  period_days = 7
  current_time = 2025-01-15

Previous Week (Jan 1-7): 3 days
Current Week (Jan 8-14): 5 days

Result:
  current_period_days = 5
  previous_period_days = 3
  change_days = 2
  change_percentage = (2 / 3) * 100 = 66.67%
  trend = 'improving' (>10%)
```

**Example: Stable Trend**
```
Previous Week: 4 days
Current Week: 4 days

Result:
  change_days = 0
  change_percentage = 0%
  trend = 'stable'
```

---

## Mood Correlation with Activities

**Purpose:** Identify which activities correlate with higher/lower mood

**Complexity:** HIGH (statistical analysis, correlation)

---

### Algorithm: `calculate_mood_activity_correlation`

**Inputs:**
```rust
activity_id: i64
mood_checkins: Vec<MoodCheckin> {
  mood_rating: i64,           // 1-7
  activity_ids: Vec<i64>,
  checked_in_at: DateTime
}
min_samples: i64              // e.g., 5 (minimum checkins for valid correlation)
```

**Output:**
```rust
ActivityCorrelation {
  activity_id: i64,
  avg_mood_with_activity: f64,      // Average mood when activity performed
  avg_mood_without_activity: f64,   // Average mood when NOT performed
  correlation: f64,                 // Difference (with - without)
  sample_size: i64,                 // Number of checkins
  is_significant: bool              // sample_size >= min_samples
}
```

---

### Algorithm Steps

```
FUNCTION calculate_mood_activity_correlation(activity_id, checkins, min_samples):

  # 1. Partition checkins by activity presence
  with_activity = checkins.filter(c => c.activity_ids.contains(activity_id))
  without_activity = checkins.filter(c => !c.activity_ids.contains(activity_id))

  # 2. Calculate average mood with activity
  IF with_activity.is_empty():
    avg_mood_with = 0.0
  ELSE:
    mood_sum_with = with_activity.map(c => c.mood_rating).sum()
    avg_mood_with = mood_sum_with / with_activity.len()

  # 3. Calculate average mood without activity
  IF without_activity.is_empty():
    avg_mood_without = 0.0
  ELSE:
    mood_sum_without = without_activity.map(c => c.mood_rating).sum()
    avg_mood_without = mood_sum_without / without_activity.len()

  # 4. Calculate correlation (simple difference)
  correlation = avg_mood_with - avg_mood_without

  # 5. Determine significance
  total_samples = with_activity.len() + without_activity.len()
  is_significant = with_activity.len() >= min_samples AND
                   without_activity.len() >= min_samples

  # 6. Return correlation
  RETURN ActivityCorrelation {
    activity_id: activity_id,
    avg_mood_with_activity: avg_mood_with,
    avg_mood_without_activity: avg_mood_without,
    correlation: correlation,
    sample_size: total_samples,
    is_significant: is_significant
  }

END FUNCTION
```

---

### Interpretation

| Correlation | Meaning |
|-------------|---------|
| `> 1.0` | **Strong positive** - Activity associated with higher mood |
| `0.5 to 1.0` | **Moderate positive** - Activity somewhat associated with higher mood |
| `-0.5 to 0.5` | **Neutral** - No clear association |
| `-1.0 to -0.5` | **Moderate negative** - Activity associated with lower mood |
| `< -1.0` | **Strong negative** - Activity associated with much lower mood |

---

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| **Activity never performed** | `avg_mood_with = 0`, `correlation = 0`, `is_significant = false` |
| **Activity always performed** | `avg_mood_without = 0`, `correlation = avg_mood_with`, `is_significant = false` |
| **Few samples** | `is_significant = false` (need min_samples in both groups) |

---

### Examples

**Example: Positive Correlation**
```
Activity: Exercise
Checkins:
  With Exercise (5 checkins): [6, 7, 6, 7, 6] → avg = 6.4
  Without Exercise (10 checkins): [3, 4, 4, 3, 5, 4, 3, 4, 5, 4] → avg = 3.9

Result:
  avg_mood_with_activity = 6.4
  avg_mood_without_activity = 3.9
  correlation = 6.4 - 3.9 = 2.5 (strong positive)
  sample_size = 15
  is_significant = true (both groups >= 5)
```

---

## Report Aggregation

**Purpose:** Aggregate activity and mood data for weekly/monthly reports

**Complexity:** MEDIUM (grouping, aggregation)

---

### Algorithm: `generate_weekly_report`

**Inputs:**
```rust
week_start: DateTime<Utc>
week_end: DateTime<Utc>
activity_logs: Vec<ActivityLog>
mood_checkins: Vec<MoodCheckin>
activity_groups: Vec<ActivityGroup>
```

**Output:**
```rust
WeeklyReport {
  period_start: DateTime,
  period_end: DateTime,
  total_logs: i64,
  unique_activities: i64,
  avg_mood: f64,
  top_activities: Vec<(String, i64)>,    // (activity_name, count)
  group_summaries: Vec<GroupSummary>
}

GroupSummary {
  group_name: String,
  activities_logged: i64,
  days_active: i64
}
```

---

### Algorithm Steps

```
FUNCTION generate_weekly_report(week_start, week_end, logs, checkins, groups):

  # 1. Filter data to week
  week_logs = logs.filter(log =>
    log.logged_at >= week_start AND log.logged_at <= week_end
  )
  week_checkins = checkins.filter(c =>
    c.checked_in_at >= week_start AND c.checked_in_at <= week_end
  )

  # 2. Calculate basic metrics
  total_logs = week_logs.len()
  unique_activity_ids = week_logs.map(log => log.activity_id).into_set()
  unique_activities = unique_activity_ids.len()

  # 3. Calculate average mood
  IF week_checkins.is_empty():
    avg_mood = 0.0
  ELSE:
    mood_sum = week_checkins.map(c => c.mood_rating).sum()
    avg_mood = mood_sum / week_checkins.len()

  # 4. Find top activities (by log count)
  activity_counts = week_logs.group_by(log => log.activity_id)
                              .map(group => (group.key, group.count()))
                              .sort_by(|a, b| b.1.cmp(a.1))  // Descending
                              .take(5)

  # Resolve activity names
  top_activities = activity_counts.map(|(id, count)| {
    activity = get_activity_by_id(id)
    (activity.name, count)
  })

  # 5. Calculate group summaries
  group_summaries = groups.map(group => {
    group_activity_ids = get_activities_in_group(group.id).map(a => a.id)

    group_logs = week_logs.filter(log => group_activity_ids.contains(log.activity_id))
    activities_logged = group_logs.map(log => log.activity_id).into_set().len()
    days_active = calculate_unique_days(group_logs)

    GroupSummary {
      group_name: group.name,
      activities_logged: activities_logged,
      days_active: days_active
    }
  })

  # 6. Return report
  RETURN WeeklyReport {
    period_start: week_start,
    period_end: week_end,
    total_logs: total_logs,
    unique_activities: unique_activities,
    avg_mood: avg_mood,
    top_activities: top_activities,
    group_summaries: group_summaries
  }

END FUNCTION
```

---

## Chart Data Transformation

**Purpose:** Transform raw data into Chart.js format

**Complexity:** LOW (data mapping)

---

### Algorithm: `create_mood_trend_chart_data`

**Inputs:**
```rust
mood_checkins: Vec<MoodCheckin>
```

**Output:**
```typescript
ChartData<'line'> {
  labels: string[],           // Dates
  datasets: [{
    label: string,
    data: number[],           // Mood ratings
    borderColor: string,
    backgroundColor: string
  }]
}
```

---

### Algorithm Steps

```
FUNCTION create_mood_trend_chart_data(checkins):

  # 1. Sort checkins by date (oldest first)
  sorted_checkins = checkins.sort_by(c => c.checked_in_at)

  # 2. Extract labels (dates)
  labels = sorted_checkins.map(c => {
    date = c.checked_in_at.date()
    format_date(date, "MMM d")  // e.g., "Jan 15"
  })

  # 3. Extract data (mood ratings)
  data = sorted_checkins.map(c => c.mood_rating)

  # 4. Create Chart.js dataset
  dataset = {
    label: 'Mood Rating',
    data: data,
    borderColor: 'rgb(59, 130, 246)',      // blue-500
    backgroundColor: 'rgba(59, 130, 246, 0.1)',
    tension: 0.3                            // Smooth line
  }

  # 5. Return chart data
  RETURN ChartData {
    labels: labels,
    datasets: [dataset]
  }

END FUNCTION
```

---

### Algorithm: `create_activity_frequency_chart_data`

**Inputs:**
```rust
activity_frequencies: Vec<ActivityFrequency>
```

**Output:**
```typescript
ChartData<'bar'> {
  labels: string[],           // Activity names
  datasets: [{
    label: string,
    data: number[],           // Days per week
    backgroundColor: string
  }]
}
```

---

### Algorithm Steps

```
FUNCTION create_activity_frequency_chart_data(frequencies):

  # 1. Sort by days_per_week (highest first)
  sorted = frequencies.sort_by(f => -f.days_per_week)
                      .take(10)  // Top 10 activities

  # 2. Extract labels (activity names)
  labels = sorted.map(f => {
    activity = get_activity_by_id(f.activity_id)
    activity.name
  })

  # 3. Extract data (days per week)
  data = sorted.map(f => f.days_per_week)

  # 4. Create Chart.js dataset
  dataset = {
    label: 'Days per Week',
    data: data,
    backgroundColor: 'rgb(34, 197, 94)'   // green-500
  }

  # 5. Return chart data
  RETURN ChartData {
    labels: labels,
    datasets: [dataset]
  }

END FUNCTION
```

---

## Implementation Checklist

### Before Week 2

- [ ] Review all algorithm specifications
- [ ] Understand edge cases for goal progress calculation
- [ ] Understand improvement percentage calculation (previous period logic)
- [ ] Write tests based on algorithm examples

### During Week 2

- [ ] Implement `calculate_goal_progress` following spec
- [ ] Implement `calculate_activity_frequency` following spec
- [ ] Implement `calculate_activity_trend` following spec
- [ ] Verify all edge cases handled correctly

### During Week 4

- [ ] Implement `calculate_mood_activity_correlation`
- [ ] Implement `generate_weekly_report`
- [ ] Implement chart data transformations

---

## Testing Strategy

**For each algorithm:**

1. **Write test from spec example** - Use the examples in this doc as test cases
2. **Test happy path** - Normal inputs, expected outputs
3. **Test edge cases** - Empty data, zeros, boundary conditions
4. **Test validation** - Invalid inputs should error
5. **Test performance** - Large datasets (1000+ logs)

**Example Test Structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_progress_days_per_period() {
        // Arrange (from spec example)
        let goal = ActivityGoal {
            goal_type: GoalType::DaysPerPeriod,
            target_value: 5,
            period_days: 7,
            ...
        };
        let logs = vec![
            // Logs from spec example
        ];

        // Act
        let result = calculate_goal_progress(goal, logs, Utc::now());

        // Assert (from spec expected output)
        assert_eq!(result.current_value, 4);
        assert_eq!(result.percentage, 80.0);
        assert_eq!(result.is_achieved, false);
    }

    #[test]
    fn test_goal_progress_no_logs_edge_case() {
        // Test empty logs edge case
        let logs = vec![];
        let result = calculate_goal_progress(goal, logs, Utc::now());
        assert_eq!(result.current_value, 0);
        assert_eq!(result.percentage, 0.0);
    }
}
```

---

## References

- **Data Structures:** `data-structures.md` (GoalProgress, ActivityFrequency types)
- **Implementation Tasks:** `REVISED-tasks.md` (Week 2, Tasks 2.8-2.17)
- **Plan Context:** `REVISED-plan.md` (Week 2: Activity Goals and Reporting Logic)
