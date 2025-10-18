# Assessments Feature API Contract

**Feature**: Mental Health Assessments
**Commands**: 7 (4 queries, 3 commands)

## Data Types

### AssessmentType
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct AssessmentType {
    pub id: i32,
    pub code: String,           // "PHQ9", "GAD7", "CESD", "OASIS"
    pub name: String,           // "Patient Health Questionnaire-9"
    pub description: String,
    pub question_count: i32,
    pub min_score: i32,
    pub max_score: i32,
    pub thresholds: HashMap<String, i32>,  // {"minimal": 4, "mild": 9, ...}
}
```

### AssessmentQuestion
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct AssessmentQuestion {
    pub number: i32,            // 1-based question number
    pub text: String,           // Question text
    pub options: Vec<String>,   // Answer options (e.g., ["Not at all", "Several days", ...])
}
```

### AssessmentResponse
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct AssessmentResponse {
    pub id: i64,
    pub assessment_type: AssessmentType,
    pub responses: Vec<i32>,    // Answer values (e.g., [0, 1, 2, 1, 0, ...])
    pub total_score: i32,
    pub severity_level: String, // "minimal", "mild", "moderate", "severe"
    pub completed_at: String,   // ISO 8601 timestamp
    pub notes: Option<String>,
}
```

### SubmitAssessmentRequest
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct SubmitAssessmentRequest {
    pub assessment_type_code: String,  // "PHQ9", "GAD7", etc.
    pub responses: Vec<i32>,
    pub notes: Option<String>,
}
```

### AssessmentHistory
```rust
#[derive(Serialize, Deserialize, specta::Type)]
pub struct AssessmentHistory {
    pub assessment_type: AssessmentType,
    pub entries: Vec<AssessmentHistoryEntry>,
}

#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct AssessmentHistoryEntry {
    pub id: i64,
    pub total_score: i32,
    pub severity_level: String,
    pub completed_at: String,   // ISO 8601
}
```

---

## Commands (Write Operations)

### 1. `submit_assessment`

**Purpose**: Submit a completed assessment with responses

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn submit_assessment(
    request: SubmitAssessmentRequest,
    state: State<'_, AppState>
) -> Result<AssessmentResponse, String>
```

**Parameters**:
- `request.assessment_type_code`: Assessment type code (e.g., "PHQ9")
- `request.responses`: Array of response values (length must match question_count)
- `request.notes`: Optional user notes

**Returns**: Complete assessment response with calculated score and severity

**Errors**:
- `"Invalid assessment type: {code}"` - Unknown assessment code
- `"Incomplete responses: expected {expected}, got {actual}"` - Wrong number of responses
- `"Invalid response value at position {i}: {value}"` - Response value out of range
- `"Failed to save assessment: {error}"` - Database error

**Validation**:
- Assessment type must exist
- Response count must match `assessment_type.question_count`
- Each response value must be 0-3 (standard Likert scale)
- Total score calculated automatically using assessment-specific algorithm

**Example Usage**:
```typescript
import { invoke } from '@tauri-apps/api/core';

const request = {
  assessment_type_code: 'PHQ9',
  responses: [0, 1, 2, 1, 0, 1, 2, 3, 1],  // 9 responses for PHQ-9
  notes: 'Feeling better this week'
};

try {
  const response = await invoke<AssessmentResponse>('submit_assessment', { request });
  console.log('Score:', response.total_score);
  console.log('Severity:', response.severity_level);
} catch (error) {
  console.error('Submission failed:', error);
}
```

---

## Queries (Read Operations)

### 2. `get_assessment_types`

**Purpose**: Fetch all available assessment types

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_types(
    state: State<'_, AppState>
) -> Result<Vec<AssessmentType>, String>
```

**Parameters**: None

**Returns**: List of all available assessment types (PHQ9, GAD7, CESD, OASIS)

**Errors**:
- `"Failed to fetch assessment types: {error}"` - Database error

**Example Usage**:
```typescript
const assessmentTypes = await invoke<AssessmentType[]>('get_assessment_types');
// Display list: "PHQ-9", "GAD-7", "CES-D", "OASIS"
```

---

### 3. `get_assessment_questions`

**Purpose**: Fetch questions for a specific assessment type

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_questions(
    code: String,
    state: State<'_, AppState>
) -> Result<Vec<AssessmentQuestion>, String>
```

**Parameters**:
- `code`: Assessment type code (e.g., "PHQ9")

**Returns**: List of questions with answer options

**Errors**:
- `"Invalid assessment type: {code}"` - Unknown assessment code
- `"Failed to load questions: {error}"` - Internal error

**Example Usage**:
```typescript
const questions = await invoke<AssessmentQuestion[]>('get_assessment_questions', {
  code: 'PHQ9'
});

questions.forEach((q, i) => {
  console.log(`${q.number}. ${q.text}`);
  q.options.forEach((opt, j) => console.log(`  ${j}: ${opt}`));
});
```

---

### 4. `get_assessment_history`

**Purpose**: Fetch historical assessment responses for a specific type

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_history(
    code: String,
    from_date: Option<String>,  // ISO 8601 date (e.g., "2025-01-01")
    to_date: Option<String>,
    state: State<'_, AppState>
) -> Result<AssessmentHistory, String>
```

**Parameters**:
- `code`: Assessment type code
- `from_date`: Optional start date (inclusive)
- `to_date`: Optional end date (inclusive)

**Returns**: Assessment history with all entries in date range

**Errors**:
- `"Invalid assessment type: {code}"` - Unknown assessment code
- `"Invalid date format: {date}"` - Date must be ISO 8601 (YYYY-MM-DD)
- `"Failed to fetch history: {error}"` - Database error

**Example Usage**:
```typescript
const history = await invoke<AssessmentHistory>('get_assessment_history', {
  code: 'PHQ9',
  fromDate: '2025-01-01',
  toDate: null  // Until today
});

console.log(`${history.entries.length} PHQ-9 assessments`);
history.entries.forEach(entry => {
  console.log(`${entry.completed_at}: Score ${entry.total_score} (${entry.severity_level})`);
});
```

---

### 5. `get_assessment_response`

**Purpose**: Fetch full details of a specific assessment response

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_assessment_response(
    id: i64,
    state: State<'_, AppState>
) -> Result<AssessmentResponse, String>
```

**Parameters**:
- `id`: Assessment response ID

**Returns**: Complete assessment response with all details

**Errors**:
- `"Assessment not found: {id}"` - No assessment with that ID
- `"Failed to fetch assessment: {error}"` - Database error

**Example Usage**:
```typescript
const response = await invoke<AssessmentResponse>('get_assessment_response', {
  id: 123
});

console.log('Responses:', response.responses);  // [0, 1, 2, 1, ...]
console.log('Score:', response.total_score);
```

---

### 6. `get_latest_assessment`

**Purpose**: Fetch the most recent assessment of a specific type

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn get_latest_assessment(
    code: String,
    state: State<'_, AppState>
) -> Result<Option<AssessmentResponse>, String>
```

**Parameters**:
- `code`: Assessment type code

**Returns**: Latest assessment response, or null if none exist

**Errors**:
- `"Invalid assessment type: {code}"` - Unknown assessment code
- `"Failed to fetch latest assessment: {error}"` - Database error

**Example Usage**:
```typescript
const latest = await invoke<AssessmentResponse | null>('get_latest_assessment', {
  code: 'PHQ9'
});

if (latest) {
  console.log('Last score:', latest.total_score);
  console.log('Taken on:', new Date(latest.completed_at).toLocaleDateString());
} else {
  console.log('No PHQ-9 assessments yet');
}
```

---

### 7. `delete_assessment`

**Purpose**: Delete a specific assessment response

**Signature**:
```rust
#[tauri::command]
#[specta::specta]
pub async fn delete_assessment(
    id: i64,
    state: State<'_, AppState>
) -> Result<(), String>
```

**Parameters**:
- `id`: Assessment response ID to delete

**Returns**: Unit type (success)

**Errors**:
- `"Assessment not found: {id}"` - No assessment with that ID
- `"Failed to delete assessment: {error}"` - Database error

**Example Usage**:
```typescript
await invoke('delete_assessment', { id: 123 });
console.log('Assessment deleted');
```

---

## Scoring Algorithms

### PHQ-9 (Patient Health Questionnaire-9)
- **Questions**: 9
- **Scale**: Each question scored 0-3
- **Total Score**: Sum of all responses (0-27)
- **Severity**:
  - 0-4: Minimal
  - 5-9: Mild
  - 10-14: Moderate
  - 15-19: Moderately Severe
  - 20-27: Severe

### GAD-7 (Generalized Anxiety Disorder-7)
- **Questions**: 7
- **Scale**: Each question scored 0-3
- **Total Score**: Sum of all responses (0-21)
- **Severity**:
  - 0-4: Minimal
  - 5-9: Mild
  - 10-14: Moderate
  - 15-21: Severe

### CES-D (Center for Epidemiologic Studies Depression Scale)
- **Questions**: 20
- **Scale**: Each question scored 0-3
- **Total Score**: Sum of all responses (0-60)
- **Severity**:
  - 0-15: Minimal
  - 16-21: Mild
  - 22-36: Moderate
  - 37-60: Severe

### OASIS (Overall Anxiety Severity and Impairment Scale)
- **Questions**: 5
- **Scale**: Each question scored 0-4
- **Total Score**: Sum of all responses (0-20)
- **Severity**:
  - 0-7: Minimal
  - 8-14: Moderate
  - 15-20: Severe

---

## Implementation Notes

1. **Assessment Content**: Questions stored as constants in Rust code (see `src-tauri/src/features/assessments/content.rs`)
2. **Scoring Logic**: Implemented in `src-tauri/src/features/assessments/models.rs`
3. **Validation**: Double-check (database constraints + Rust validation)
4. **Immutability**: Assessment responses cannot be edited after submission (immutable records)
5. **Soft Delete**: `delete_assessment` marks as deleted but retains data for audit trail (optional enhancement)

---

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phq9_scoring() {
        let responses = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let score = calculate_phq9_score(&responses);
        assert_eq!(score, 0);

        let responses = vec![3, 3, 3, 3, 3, 3, 3, 3, 3];
        let score = calculate_phq9_score(&responses);
        assert_eq!(score, 27);
    }

    #[test]
    fn test_severity_level() {
        assert_eq!(get_phq9_severity(4), "minimal");
        assert_eq!(get_phq9_severity(10), "moderate");
        assert_eq!(get_phq9_severity(20), "severe");
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_submit_and_retrieve_assessment() {
    let request = SubmitAssessmentRequest {
        assessment_type_code: "PHQ9".to_string(),
        responses: vec![1, 1, 0, 2, 1, 0, 1, 0, 1],
        notes: Some("Test".to_string()),
    };

    let response = submit_assessment(request, state).await.unwrap();
    assert_eq!(response.total_score, 7);

    let retrieved = get_assessment_response(response.id, state).await.unwrap();
    assert_eq!(retrieved.responses.len(), 9);
}
```
