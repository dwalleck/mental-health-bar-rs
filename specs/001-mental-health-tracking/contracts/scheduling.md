# Scheduling Feature API Contract

**Feature**: Assessment Schedule Management & Notifications
**Commands**: 5 (3 queries, 2 commands)

## Data Types

### AssessmentSchedule
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct AssessmentSchedule {
    pub id: i64,
    pub assessment_type: AssessmentType,
    pub frequency: ScheduleFrequency,
    pub time_of_day: String,         // "HH:MM" format (e.g., "09:00")
    pub day_of_week: Option<i32>,    // 0-6 (0=Sunday)
    pub day_of_month: Option<i32>,   // 1-31
    pub enabled: bool,
    pub last_triggered_at: Option<String>,  // ISO 8601
    pub next_trigger_at: String,     // Calculated next trigger time
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
}
```

### CreateScheduleRequest
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct CreateScheduleRequest {
    pub assessment_type_code: String,
    pub frequency: ScheduleFrequency,
    pub time_of_day: String,         // "HH:MM"
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
}
```

### UpdateScheduleRequest
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct UpdateScheduleRequest {
    pub frequency: Option<ScheduleFrequency>,
    pub time_of_day: Option<String>,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub enabled: Option<bool>,
}
```

---

## Commands (Write Operations)

### 1. `create_schedule`

**Purpose**: Create a recurring assessment reminder schedule

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn create_schedule(
    request: CreateScheduleRequest,
    state: State<'_, AppState>
) -> Result<AssessmentSchedule, String>
```

**Parameters**:
- `request.assessment_type_code`: Assessment type (e.g., "PHQ9")
- `request.frequency`: Daily, Weekly, Biweekly, or Monthly
- `request.time_of_day`: Time in "HH:MM" format (24-hour)
- `request.day_of_week`: Required for Weekly/Biweekly (0=Sunday, 6=Saturday)
- `request.day_of_month`: Required for Monthly (1-31)

**Returns**: Created schedule with calculated next trigger time

**Errors**:
- `"Invalid assessment type: {code}"` - Unknown assessment
- `"Invalid time format: {time}. Expected HH:MM"` - Bad time format
- `"day_of_week required for weekly schedules"` - Missing field
- `"day_of_month required for monthly schedules"` - Missing field
- `"Invalid day_of_week: {day}. Must be 0-6"` - Out of range
- `"Invalid day_of_month: {day}. Must be 1-31"` - Out of range
- `"Schedule already exists for this assessment type"` - Duplicate
- `"Failed to create schedule: {error}"` - Database error

**Validation**:
- time_of_day must match `^([01]\d|2[0-3]):([0-5]\d)$`
- day_of_week: 0-6 (0=Sunday, 1=Monday, ..., 6=Saturday)
- day_of_month: 1-31
- Conditional requirements based on frequency

**Example Usage**:
```typescript
// Weekly PHQ-9 on Mondays at 9 AM
const schedule = await invoke<AssessmentSchedule>('create_schedule', {
  request: {
    assessment_type_code: 'PHQ9',
    frequency: 'weekly',
    time_of_day: '09:00',
    day_of_week: 1,  // Monday
    day_of_month: null
  }
});
```

---

### 2. `update_schedule`

**Purpose**: Update an existing schedule

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn update_schedule(
    id: i64,
    request: UpdateScheduleRequest,
    state: State<'_, AppState>
) -> Result<AssessmentSchedule, String>
```

**Parameters**:
- `id`: Schedule ID
- `request`: Fields to update (all optional)

**Returns**: Updated schedule

**Errors**:
- `"Schedule not found: {id}"` - Unknown ID
- `"Invalid time format: {time}"` - Bad time
- `"Failed to update schedule: {error}"` - Database error

**Example Usage**:
```typescript
// Change time to 10 AM
const updated = await invoke<AssessmentSchedule>('update_schedule', {
  id: 3,
  request: {
    time_of_day: '10:00',
    frequency: null,
    day_of_week: null,
    day_of_month: null,
    enabled: null
  }
});
```

---

## Queries (Read Operations)

### 3. `get_schedules`

**Purpose**: Fetch all schedules (optionally filtered)

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_schedules(
    enabled_only: bool,
    state: State<'_, AppState>
) -> Result<Vec<AssessmentSchedule>, String>
```

**Parameters**:
- `enabled_only`: If true, only return enabled schedules

**Returns**: List of schedules

**Errors**:
- `"Failed to fetch schedules: {error}"` - Database error

**Example Usage**:
```typescript
const schedules = await invoke<AssessmentSchedule[]>('get_schedules', {
  enabledOnly: true
});

schedules.forEach(s => {
  console.log(`${s.assessment_type.name}: ${s.frequency} at ${s.time_of_day}`);
});
```

---

### 4. `get_schedule`

**Purpose**: Fetch a specific schedule by ID

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_schedule(
    id: i64,
    state: State<'_, AppState>
) -> Result<AssessmentSchedule, String>
```

**Parameters**:
- `id`: Schedule ID

**Returns**: Schedule details

**Errors**:
- `"Schedule not found: {id}"` - Unknown ID
- `"Failed to fetch schedule: {error}"` - Database error

**Example Usage**:
```typescript
const schedule = await invoke<AssessmentSchedule>('get_schedule', { id: 2 });
```

---

### 5. `delete_schedule`

**Purpose**: Delete a schedule

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn delete_schedule(
    id: i64,
    state: State<'_, AppState>
) -> Result<(), String>
```

**Parameters**:
- `id`: Schedule ID

**Returns**: Unit type (success)

**Errors**:
- `"Schedule not found: {id}"` - Unknown ID
- `"Failed to delete schedule: {error}"` - Database error

**Example Usage**:
```typescript
await invoke('delete_schedule', { id: 5 });
console.log('Schedule deleted');
```

---

## Background Scheduler

### Internal Function (Not a Tauri Command)

**Purpose**: Background task that checks for due schedules and sends notifications

**Implementation**:
```rust
// Runs every minute via tokio task
pub async fn check_schedules(state: &AppState) {
    let now = Local::now();
    let schedules = get_due_schedules(state).await;

    for schedule in schedules {
        // Send notification
        send_notification(&schedule).await;

        // Update last_triggered_at
        update_last_triggered(schedule.id, now).await;
    }
}

fn get_due_schedules(state: &AppState) -> Vec<AssessmentSchedule> {
    // Query: enabled=true AND (last_triggered_at IS NULL OR last_triggered_at < today)
    // AND current_time >= time_of_day
}

async fn send_notification(schedule: &AssessmentSchedule) {
    use tauri_plugin_notification::NotificationExt;

    app.notification()
        .builder()
        .title("Assessment Reminder")
        .body(&format!("Time to complete your {}", schedule.assessment_type.name))
        .show()
        .unwrap();
}
```

**Notification Behavior**:
- Sent once per day when schedule is due
- Click notification → opens app to assessment page
- Dismissing notification doesn't mark as completed
- Only completing the assessment updates `last_triggered_at`

---

## Schedule Logic

### Trigger Calculation

**Daily**: Trigger every day at `time_of_day`
```
next_trigger = today at time_of_day (or tomorrow if already passed)
```

**Weekly**: Trigger every week on `day_of_week` at `time_of_day`
```
next_trigger = next occurrence of day_of_week at time_of_day
```

**Biweekly**: Trigger every two weeks on `day_of_week` at `time_of_day`
```
next_trigger = next occurrence of day_of_week + 14 days if already triggered this week
```

**Monthly**: Trigger every month on `day_of_month` at `time_of_day`
```
next_trigger = next occurrence of day_of_month at time_of_day
If day_of_month > days_in_month, use last day of month
```

### Example Schedule Patterns

```typescript
// Daily at 9 AM
{
  frequency: 'daily',
  time_of_day: '09:00',
  day_of_week: null,
  day_of_month: null
}

// Weekly on Mondays at 10 AM
{
  frequency: 'weekly',
  time_of_day: '10:00',
  day_of_week: 1,
  day_of_month: null
}

// Biweekly on Thursdays at 2 PM
{
  frequency: 'biweekly',
  time_of_day: '14:00',
  day_of_week: 4,
  day_of_month: null
}

// Monthly on the 15th at 11 AM
{
  frequency: 'monthly',
  time_of_day: '11:00',
  day_of_week: null,
  day_of_month: 15
}
```

---

## Notification Permissions

**Platform Handling**:
- **Linux**: Uses D-Bus notifications (no permission needed)
- **macOS**: Requires notification permission (prompted on first use)
- **Windows**: Uses Windows Notification Center (no permission needed)

**Error Handling**:
- If notification permission denied, log warning but don't fail
- User can still manually check for due assessments in UI

---

## Implementation Notes

1. **Background Task**: Started on app launch via `tauri::async_runtime::spawn`
2. **Frequency**: Check every minute (lightweight query)
3. **Persistence**: Last triggered time stored in database
4. **Timezone**: All times in user's local timezone
5. **Edge Cases**:
   - If day_of_month > days_in_month, use last day (e.g., Feb 31 → Feb 28/29)
   - Biweekly tracks last trigger to ensure 14-day gap
   - If app not running, notifications missed (not retroactive)

---

## Testing

### Unit Tests
```rust
#[test]
fn test_time_validation() {
    assert!(validate_time("09:00").is_ok());
    assert!(validate_time("23:59").is_ok());
    assert!(validate_time("24:00").is_err());
    assert!(validate_time("9:00").is_err());  // Missing leading zero
}

#[test]
fn test_next_trigger_calculation() {
    let schedule = Schedule {
        frequency: ScheduleFrequency::Daily,
        time_of_day: "09:00",
        ...
    };

    let next = calculate_next_trigger(&schedule, Local::now());
    assert_eq!(next.hour(), 9);
    assert_eq!(next.minute(), 0);
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_schedule_notification_flow() {
    let schedule = create_schedule(...).await;

    // Simulate time passing
    advance_time_to(schedule.next_trigger_at);

    let due_schedules = get_due_schedules().await;
    assert_eq!(due_schedules.len(), 1);
    assert_eq!(due_schedules[0].id, schedule.id);
}
```
