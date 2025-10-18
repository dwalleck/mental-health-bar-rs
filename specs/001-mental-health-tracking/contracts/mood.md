# Mood Tracking Feature API Contract

**Feature**: Mood Check-Ins and Activity Management
**Commands**: 8 (4 queries, 4 commands)

## Data Types

### Activity
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,  // Hex color (#RRGGBB)
    pub icon: Option<String>,   // Emoji or icon name
    pub created_at: String,     // ISO 8601
    pub deleted_at: Option<String>,
}
```

### MoodCheckin
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct MoodCheckin {
    pub id: i64,
    pub mood_rating: i32,        // 1-5 (1=Very Bad, 5=Very Good)
    pub notes: Option<String>,
    pub activities: Vec<Activity>,
    pub created_at: String,      // ISO 8601
}
```

### LogMoodRequest
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct LogMoodRequest {
    pub mood_rating: i32,        // Must be 1-5
    pub activity_ids: Vec<i64>,  // Associated activities
    pub notes: Option<String>,
}
```

### CreateActivityRequest
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct CreateActivityRequest {
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}
```

### UpdateActivityRequest
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct UpdateActivityRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}
```

---

## Commands (Write Operations)

### 1. `log_mood`

**Purpose**: Create a mood check-in with optional activities

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn log_mood(
    request: LogMoodRequest,
    state: State<'_, AppState>
) -> Result<MoodCheckin, String>
```

**Parameters**:
- `request.mood_rating`: Mood rating (1-5)
- `request.activity_ids`: List of activity IDs to associate
- `request.notes`: Optional notes

**Returns**: Created mood check-in with activities

**Errors**:
- `"Invalid mood rating: {rating}. Must be 1-5"` - Rating out of range
- `"Activity not found: {id}"` - Unknown activity ID
- `"Failed to log mood: {error}"` - Database error

**Validation**:
- Mood rating must be 1-5
- All activity IDs must exist (can be soft-deleted)
- Notes trimmed, max 1000 characters

**Example Usage**:
```typescript
const request = {
  mood_rating: 4,
  activity_ids: [1, 3],  // Exercise + Social
  notes: 'Great workout with friends'
};

const checkin = await invoke<MoodCheckin>('log_mood', { request });
console.log('Mood logged:', checkin.mood_rating);
```

---

### 2. `create_activity`

**Purpose**: Create a new activity

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn create_activity(
    request: CreateActivityRequest,
    state: State<'_, AppState>
) -> Result<Activity, String>
```

**Parameters**:
- `request.name`: Activity name (required, 1-100 characters)
- `request.color`: Optional hex color
- `request.icon`: Optional emoji/icon

**Returns**: Created activity

**Errors**:
- `"Activity name cannot be empty"` - Empty name
- `"Activity name already exists: {name}"` - Duplicate name
- `"Invalid color format: {color}. Must be #RRGGBB"` - Bad color
- `"Failed to create activity: {error}"` - Database error

**Validation**:
- Name trimmed, 1-100 characters
- Name unique among non-deleted activities
- Color must match regex `^#[0-9A-Fa-f]{6}$`

**Example Usage**:
```typescript
const activity = await invoke<Activity>('create_activity', {
  request: {
    name: 'Reading',
    color: '#FF5733',
    icon: '📚'
  }
});
```

---

### 3. `update_activity`

**Purpose**: Update an existing activity

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn update_activity(
    id: i64,
    request: UpdateActivityRequest,
    state: State<'_, AppState>
) -> Result<Activity, String>
```

**Parameters**:
- `id`: Activity ID
- `request.name`: New name (optional)
- `request.color`: New color (optional)
- `request.icon`: New icon (optional)

**Returns**: Updated activity

**Errors**:
- `"Activity not found: {id}"` - Unknown ID
- `"Activity name already exists: {name}"` - Duplicate name
- `"Invalid color format: {color}"` - Bad color
- `"Failed to update activity: {error}"` - Database error

**Example Usage**:
```typescript
const updated = await invoke<Activity>('update_activity', {
  id: 5,
  request: { name: 'Strength Training', color: null, icon: null }
});
```

---

### 4. `delete_activity`

**Purpose**: Soft-delete an activity (preserves historical data)

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn delete_activity(
    id: i64,
    state: State<'_, AppState>
) -> Result<(), String>
```

**Parameters**:
- `id`: Activity ID

**Returns**: Unit type (success)

**Errors**:
- `"Activity not found: {id}"` - Unknown ID
- `"Failed to delete activity: {error}"` - Database error

**Note**: Soft delete only (sets `deleted_at`). Historical mood check-ins still reference this activity.

**Example Usage**:
```typescript
await invoke('delete_activity', { id: 3 });
console.log('Activity deleted');
```

---

## Queries (Read Operations)

### 5. `get_activities`

**Purpose**: Fetch all active activities

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_activities(
    include_deleted: bool,
    state: State<'_, AppState>
) -> Result<Vec<Activity>, String>
```

**Parameters**:
- `include_deleted`: If true, include soft-deleted activities

**Returns**: List of activities

**Errors**:
- `"Failed to fetch activities: {error}"` - Database error

**Example Usage**:
```typescript
const activities = await invoke<Activity[]>('get_activities', {
  includeDeleted: false
});

activities.forEach(a => {
  console.log(`${a.icon} ${a.name}`);
});
```

---

### 6. `get_mood_history`

**Purpose**: Fetch mood check-ins with date filtering

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_mood_history(
    from_date: Option<String>,  // ISO 8601
    to_date: Option<String>,
    limit: Option<i32>,          // Max results
    state: State<'_, AppState>
) -> Result<Vec<MoodCheckin>, String>
```

**Parameters**:
- `from_date`: Optional start date (inclusive)
- `to_date`: Optional end date (inclusive)
- `limit`: Optional max results (default: 100)

**Returns**: List of mood check-ins with activities

**Errors**:
- `"Invalid date format: {date}"` - Bad date format
- `"Failed to fetch mood history: {error}"` - Database error

**Example Usage**:
```typescript
const history = await invoke<MoodCheckin[]>('get_mood_history', {
  fromDate: '2025-10-01',
  toDate: null,
  limit: 30
});

console.log(`${history.length} mood check-ins`);
```

---

### 7. `get_mood_checkin`

**Purpose**: Fetch a specific mood check-in by ID

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_mood_checkin(
    id: i64,
    state: State<'_, AppState>
) -> Result<MoodCheckin, String>
```

**Parameters**:
- `id`: Mood check-in ID

**Returns**: Mood check-in with activities

**Errors**:
- `"Mood check-in not found: {id}"` - Unknown ID
- `"Failed to fetch mood check-in: {error}"` - Database error

**Example Usage**:
```typescript
const checkin = await invoke<MoodCheckin>('get_mood_checkin', { id: 42 });
console.log('Mood:', checkin.mood_rating);
console.log('Activities:', checkin.activities.map(a => a.name).join(', '));
```

---

### 8. `get_mood_stats`

**Purpose**: Get mood statistics (average, trends, activity correlations)

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_mood_stats(
    from_date: Option<String>,
    to_date: Option<String>,
    state: State<'_, AppState>
) -> Result<MoodStats, String>

#[derive(Serialize, Deserialize, specta::Type)]
pub struct MoodStats {
    pub average_mood: f64,
    pub total_checkins: i32,
    pub mood_distribution: HashMap<i32, i32>,  // {1: 5, 2: 10, 3: 20, ...}
    pub activity_correlations: Vec<ActivityCorrelation>,
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct ActivityCorrelation {
    pub activity: Activity,
    pub average_mood: f64,
    pub checkin_count: i32,
}
```

**Parameters**:
- `from_date`: Optional start date
- `to_date`: Optional end date

**Returns**: Mood statistics and activity correlations

**Errors**:
- `"Invalid date format: {date}"` - Bad date format
- `"Failed to calculate mood stats: {error}"` - Database error

**Example Usage**:
```typescript
const stats = await invoke<MoodStats>('get_mood_stats', {
  fromDate: '2025-01-01',
  toDate: null
});

console.log('Average mood:', stats.average_mood.toFixed(2));
console.log('Top activity:', stats.activity_correlations[0].activity.name);
```

---

## Mood Scale Reference

**1 = Very Bad**
- Feeling very low, negative emotions
- Display: Red color (#F44336)

**2 = Bad**
- Below average, somewhat negative
- Display: Orange color (#FF9800)

**3 = Neutral**
- Neither good nor bad, balanced
- Display: Yellow color (#FFEB3B)

**4 = Good**
- Above average, positive mood
- Display: Light green color (#8BC34A)

**5 = Very Good**
- Feeling great, very positive
- Display: Green color (#4CAF50)

---

## Implementation Notes

1. **Soft Delete**: Activities use `deleted_at` timestamp for soft delete
2. **Historical Integrity**: Deleted activities still appear in past mood check-ins
3. **Activity Limits**: No hard limit on number of activities (reasonable: <50)
4. **Multiple Check-ins**: Users can log multiple moods per day (unlimited)
5. **Timestamps**: Auto-generated on creation, immutable

---

## Testing

### Unit Tests
```rust
#[test]
fn test_mood_rating_validation() {
    assert!(validate_mood_rating(1).is_ok());
    assert!(validate_mood_rating(5).is_ok());
    assert!(validate_mood_rating(0).is_err());
    assert!(validate_mood_rating(6).is_err());
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_log_mood_with_activities() {
    let activity1 = create_activity("Exercise").await;
    let activity2 = create_activity("Social").await;

    let request = LogMoodRequest {
        mood_rating: 4,
        activity_ids: vec![activity1.id, activity2.id],
        notes: Some("Great day".to_string()),
    };

    let checkin = log_mood(request, state).await.unwrap();
    assert_eq!(checkin.activities.len(), 2);
    assert_eq!(checkin.mood_rating, 4);
}
```
