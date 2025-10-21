# Data Model

**Feature**: Mental Health Assessment and Tracking Application
**Date**: 2025-10-15
**Database**: DuckDB (embedded SQL)

## Overview

This document defines the data model for the mental health tracking application. The model supports four clinical assessments (PHQ-9, GAD-7, CES-D, OASIS), daily mood check-ins with activity tracking, assessment scheduling, and user configuration.

## Design Principles

1. **Normalized**: Minimize data duplication (3NF where practical)
2. **Temporal**: All records include timestamps for time-series analysis
3. **Extensible**: New assessment types can be added without schema changes
4. **Privacy-First**: No PII beyond local user preferences
5. **Performance**: Indexed for common query patterns (time-series aggregations)

## Entity Relationship Diagram

```
┌─────────────┐
│   User      │
│   Config    │
└──────┬──────┘
       │
       │ (implicit: single user)
       │
       ├───────────────┬───────────────┬────────────────┐
       │               │               │                │
       ▼               ▼               ▼                ▼
┌─────────────┐ ┌─────────────┐ ┌──────────────┐ ┌────────────┐
│ Assessments │ │  Mood       │ │  Activities  │ │  Schedules │
│  Responses  │ │  Check-Ins  │ │              │ │            │
└─────────────┘ └──────┬──────┘ └──────┬───────┘ └────────────┘
                       │               │
                       │       ┌───────┴────────┐
                       │       │                │
                       └───────┤  Mood Activity │
                               │   Junction     │
                               └────────────────┘
```

## Tables

### 1. `assessment_types`

**Purpose**: Define available assessment types (PHQ-9, GAD-7, CES-D, OASIS)

**Schema**:
```sql
CREATE TABLE assessment_types (
    id INTEGER PRIMARY KEY,
    code VARCHAR(10) NOT NULL UNIQUE,  -- 'PHQ9', 'GAD7', 'CESD', 'OASIS'
    name VARCHAR(100) NOT NULL,         -- 'Patient Health Questionnaire-9'
    description TEXT,
    question_count INTEGER NOT NULL,
    min_score INTEGER NOT NULL,
    max_score INTEGER NOT NULL,
    thresholds JSON NOT NULL,           -- {"minimal": 5, "mild": 10, "moderate": 15, "severe": 20}
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

**Validation Rules**:
- `code` must be uppercase, 3-10 characters
- `question_count` must be > 0
- `min_score` < `max_score`
- `thresholds` JSON must contain severity level boundaries

**Sample Data**:
```sql
INSERT INTO assessment_types (code, name, description, question_count, min_score, max_score, thresholds) VALUES
('PHQ9', 'Patient Health Questionnaire-9', 'Depression screening tool', 9, 0, 27,
 '{"minimal": 4, "mild": 9, "moderate": 14, "moderately_severe": 19, "severe": 27}'),
('GAD7', 'Generalized Anxiety Disorder-7', 'Anxiety screening tool', 7, 0, 21,
 '{"minimal": 4, "mild": 9, "moderate": 14, "severe": 21}'),
('CESD', 'Center for Epidemiologic Studies Depression Scale', 'Depression assessment', 20, 0, 60,
 '{"minimal": 15, "mild": 21, "moderate": 36, "severe": 60}'),
('OASIS', 'Overall Anxiety Severity and Impairment Scale', 'Anxiety assessment', 5, 0, 20,
 '{"minimal": 7, "moderate": 14, "severe": 20}');
```

---

### 2. `assessment_responses`

**Purpose**: Store completed assessment instances with responses and calculated scores

**Schema**:
```sql
CREATE TABLE assessment_responses (
    id INTEGER PRIMARY KEY,
    assessment_type_id INTEGER NOT NULL,
    responses JSON NOT NULL,              -- [0, 1, 2, 1, 0, 1, 2, 3, 1] for PHQ-9
    total_score INTEGER NOT NULL,
    severity_level VARCHAR(50),           -- 'minimal', 'mild', 'moderate', 'severe'
    completed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes TEXT,                            -- Optional user notes
    FOREIGN KEY (assessment_type_id) REFERENCES assessment_types(id)
);

CREATE INDEX idx_assessment_responses_completed_at ON assessment_responses(completed_at);
CREATE INDEX idx_assessment_responses_type_date ON assessment_responses(assessment_type_id, completed_at);
```

**Validation Rules**:
- `responses` JSON array length must match `assessment_types.question_count`
- Each response value must be within valid range for that question (typically 0-3)
- `total_score` must be between `assessment_types.min_score` and `max_score`
- `severity_level` must match one of the keys in `assessment_types.thresholds`

**State Transitions**: None (immutable after creation)

**Indexes**:
- `completed_at`: For time-series queries (chart data)
- `(assessment_type_id, completed_at)`: For per-assessment history queries

---

### 3. `activities`

**Purpose**: User-defined activity categories for mood tracking

**Schema**:
```sql
CREATE TABLE activities (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7),                     -- Hex color for UI display (e.g., '#FF5733')
    icon VARCHAR(50),                     -- Optional icon name/emoji
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP                  -- Soft delete for data integrity
);

CREATE INDEX idx_activities_deleted_at ON activities(deleted_at);
```

**Validation Rules**:
- `name` must be 1-100 characters, non-empty after trim
- `name` must be unique among non-deleted activities
- `color` must be valid hex color format (#RRGGBB)

**State Transitions**:
- `active` (deleted_at IS NULL) → `deleted` (deleted_at IS NOT NULL)
- No hard deletes (preserve historical data integrity)

**Sample Data**:
```sql
INSERT INTO activities (name, color, icon) VALUES
('Exercise', '#4CAF50', '🏃'),
('Meditation', '#9C27B0', '🧘'),
('Social', '#2196F3', '👥'),
('Work', '#FF9800', '💼'),
('Sleep', '#3F51B5', '😴'),
('Hobby', '#E91E63', '🎨');
```

---

### 4. `mood_checkins`

**Purpose**: Daily mood logging with 1-5 rating scale

**Schema**:
```sql
CREATE TABLE mood_checkins (
    id INTEGER PRIMARY KEY,
    mood_rating INTEGER NOT NULL CHECK (mood_rating BETWEEN 1 AND 5),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_mood_checkins_created_at ON mood_checkins(created_at);
CREATE INDEX idx_mood_checkins_date ON mood_checkins(DATE(created_at));
```

**Validation Rules**:
- `mood_rating` must be 1-5 (1=Very Bad, 2=Bad, 3=Neutral, 4=Good, 5=Very Good)
- `created_at` automatically set to current timestamp

**State Transitions**: None (immutable after creation)

**Indexes**:
- `created_at`: For time-series queries
- `DATE(created_at)`: For daily aggregations (e.g., average mood per day)

---

### 5. `mood_checkin_activities`

**Purpose**: Many-to-many junction table linking mood check-ins to activities

**Schema**:
```sql
CREATE TABLE mood_checkin_activities (
    id INTEGER PRIMARY KEY,
    mood_checkin_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    FOREIGN KEY (mood_checkin_id) REFERENCES mood_checkins(id) ON DELETE CASCADE,
    FOREIGN KEY (activity_id) REFERENCES activities(id),
    UNIQUE(mood_checkin_id, activity_id)
);

CREATE INDEX idx_mood_checkin_activities_checkin ON mood_checkin_activities(mood_checkin_id);
CREATE INDEX idx_mood_checkin_activities_activity ON mood_checkin_activities(activity_id);
```

**Validation Rules**:
- `mood_checkin_id` must reference existing mood check-in
- `activity_id` must reference existing activity (even if soft-deleted)
- No duplicate (mood_checkin_id, activity_id) pairs

**State Transitions**: None

**Indexes**:
- `mood_checkin_id`: For fetching activities per check-in
- `activity_id`: For analyzing mood by activity type

---

### 6. `assessment_schedules`

**Purpose**: Configure recurring assessment reminders

**Schema**:
```sql
CREATE TABLE assessment_schedules (
    id INTEGER PRIMARY KEY,
    assessment_type_id INTEGER NOT NULL,
    frequency VARCHAR(20) NOT NULL CHECK (frequency IN ('daily', 'weekly', 'biweekly', 'monthly')),
    time_of_day TIME NOT NULL,            -- HH:MM format (e.g., '09:00')
    day_of_week INTEGER,                   -- 0-6 for weekly (0=Sunday, 6=Saturday)
    day_of_month INTEGER,                  -- 1-31 for monthly
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    last_triggered_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (assessment_type_id) REFERENCES assessment_types(id)
);

CREATE INDEX idx_assessment_schedules_enabled ON assessment_schedules(enabled);
CREATE INDEX idx_assessment_schedules_next ON assessment_schedules(enabled, last_triggered_at);
```

**Validation Rules**:
- `frequency` must be one of: 'daily', 'weekly', 'biweekly', 'monthly'
- `time_of_day` must be valid 24-hour time (00:00 - 23:59)
- `day_of_week` required if frequency is 'weekly' or 'biweekly', must be 0-6
- `day_of_month` required if frequency is 'monthly', must be 1-31
- Only one schedule per (assessment_type_id, frequency) combination

**State Transitions**:
- `enabled=true` → `enabled=false` (pause schedule)
- `enabled=false` → `enabled=true` (resume schedule)

**Business Logic**:
- Scheduler checks every minute for due schedules
- Schedule is due if: enabled=true AND (last_triggered_at IS NULL OR last_triggered_at < today)
- On trigger: Send notification, update last_triggered_at

---

### 7. `app_config`

**Purpose**: Store user preferences and application settings (managed via confy, but documented here for completeness)

**Schema** (stored as TOML file, not DuckDB table):
```toml
# ~/.config/mental-health-tracker/config.toml

[ui]
theme = "light"  # or "dark"
chart_animation = true

[notifications]
enabled = true
sound = true

[data]
retention_days = null  # null = infinite, or integer for auto-delete old data

[privacy]
analytics_enabled = false
```

**Validation Rules**:
- `theme` must be "light" or "dark"
- `retention_days` must be null or positive integer
- All boolean fields default to false if missing

---

## Queries

### Common Query Patterns

#### 1. Get Assessment History (for charts)
```sql
SELECT
    ar.completed_at,
    ar.total_score,
    ar.severity_level,
    at.name as assessment_name
FROM assessment_responses ar
JOIN assessment_types at ON ar.assessment_type_id = at.id
WHERE at.code = ?  -- e.g., 'PHQ9'
  AND ar.completed_at >= ?  -- date range filter
ORDER BY ar.completed_at ASC;
```

#### 2. Get Mood History with Activities
```sql
SELECT
    mc.id,
    mc.mood_rating,
    mc.notes,
    mc.created_at,
    COALESCE(
        JSON_GROUP_ARRAY(
            JSON_OBJECT(
                'id', a.id,
                'name', a.name,
                'color', a.color,
                'icon', a.icon
            )
        ) FILTER (WHERE a.id IS NOT NULL),
        '[]'
    ) as activities
FROM mood_checkins mc
LEFT JOIN mood_checkin_activities mca ON mc.id = mca.mood_checkin_id
LEFT JOIN activities a ON mca.activity_id = a.id
WHERE mc.created_at >= ?  -- date range filter
GROUP BY mc.id
ORDER BY mc.created_at DESC;
```

#### 3. Get Average Mood by Activity
```sql
SELECT
    a.name as activity_name,
    AVG(mc.mood_rating) as avg_mood,
    COUNT(mc.id) as checkin_count
FROM activities a
JOIN mood_checkin_activities mca ON a.id = mca.activity_id
JOIN mood_checkins mc ON mca.mood_checkin_id = mc.id
WHERE a.deleted_at IS NULL
  AND mc.created_at >= ?  -- date range filter
GROUP BY a.id, a.name
HAVING checkin_count >= 3  -- minimum sample size
ORDER BY avg_mood DESC;
```

#### 4. Check for Due Schedules
```sql
SELECT
    s.id,
    s.assessment_type_id,
    at.name as assessment_name,
    s.frequency,
    s.time_of_day
FROM assessment_schedules s
JOIN assessment_types at ON s.assessment_type_id = at.id
WHERE s.enabled = TRUE
  AND (
    s.last_triggered_at IS NULL
    OR DATE(s.last_triggered_at) < DATE('now', 'localtime')
  )
  AND TIME('now', 'localtime') >= s.time_of_day;
```

#### 5. Get Latest Assessment Score
```sql
SELECT
    ar.total_score,
    ar.severity_level,
    ar.completed_at
FROM assessment_responses ar
WHERE ar.assessment_type_id = (
    SELECT id FROM assessment_types WHERE code = ?
)
ORDER BY ar.completed_at DESC
LIMIT 1;
```

---

## Migrations

### Initial Schema (v1)

```sql
-- Run on first app launch

CREATE TABLE assessment_types (
    id INTEGER PRIMARY KEY,
    code VARCHAR(10) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    question_count INTEGER NOT NULL,
    min_score INTEGER NOT NULL,
    max_score INTEGER NOT NULL,
    thresholds JSON NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE assessment_responses (
    id INTEGER PRIMARY KEY,
    assessment_type_id INTEGER NOT NULL,
    responses JSON NOT NULL,
    total_score INTEGER NOT NULL,
    severity_level VARCHAR(50),
    completed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes TEXT,
    FOREIGN KEY (assessment_type_id) REFERENCES assessment_types(id)
);

CREATE INDEX idx_assessment_responses_completed_at ON assessment_responses(completed_at);
CREATE INDEX idx_assessment_responses_type_date ON assessment_responses(assessment_type_id, completed_at);

CREATE TABLE activities (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7),
    icon VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX idx_activities_deleted_at ON activities(deleted_at);

CREATE TABLE mood_checkins (
    id INTEGER PRIMARY KEY,
    mood_rating INTEGER NOT NULL CHECK (mood_rating BETWEEN 1 AND 5),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_mood_checkins_created_at ON mood_checkins(created_at);
CREATE INDEX idx_mood_checkins_date ON mood_checkins(DATE(created_at));

CREATE TABLE mood_checkin_activities (
    id INTEGER PRIMARY KEY,
    mood_checkin_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    FOREIGN KEY (mood_checkin_id) REFERENCES mood_checkins(id) ON DELETE CASCADE,
    FOREIGN KEY (activity_id) REFERENCES activities(id),
    UNIQUE(mood_checkin_id, activity_id)
);

CREATE INDEX idx_mood_checkin_activities_checkin ON mood_checkin_activities(mood_checkin_id);
CREATE INDEX idx_mood_checkin_activities_activity ON mood_checkin_activities(activity_id);

CREATE TABLE assessment_schedules (
    id INTEGER PRIMARY KEY,
    assessment_type_id INTEGER NOT NULL,
    frequency VARCHAR(20) NOT NULL CHECK (frequency IN ('daily', 'weekly', 'biweekly', 'monthly')),
    time_of_day TIME NOT NULL,
    day_of_week INTEGER,
    day_of_month INTEGER,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    last_triggered_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (assessment_type_id) REFERENCES assessment_types(id)
);

CREATE INDEX idx_assessment_schedules_enabled ON assessment_schedules(enabled);
CREATE INDEX idx_assessment_schedules_next ON assessment_schedules(enabled, last_triggered_at);

-- Seed assessment types
INSERT INTO assessment_types (code, name, description, question_count, min_score, max_score, thresholds) VALUES
('PHQ9', 'Patient Health Questionnaire-9', 'Depression screening tool', 9, 0, 27,
 '{"minimal": 4, "mild": 9, "moderate": 14, "moderately_severe": 19, "severe": 27}'),
('GAD7', 'Generalized Anxiety Disorder-7', 'Anxiety screening tool', 7, 0, 21,
 '{"minimal": 4, "mild": 9, "moderate": 14, "severe": 21}'),
('CESD', 'Center for Epidemiologic Studies Depression Scale', 'Depression assessment', 20, 0, 60,
 '{"minimal": 15, "mild": 21, "moderate": 36, "severe": 60}'),
('OASIS', 'Overall Anxiety Severity and Impairment Scale', 'Anxiety assessment', 5, 0, 20,
 '{"minimal": 7, "moderate": 14, "severe": 20}');
```

### Migration Strategy

```rust
// src-tauri/src/db/migrations.rs

pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Check if schema exists
    let schema_version: i32 = conn
        .query_row("SELECT COALESCE(MAX(version), 0) FROM schema_migrations", [], |row| row.get(0))
        .unwrap_or(0);

    if schema_version < 1 {
        conn.execute_batch(include_str!("migrations/001_initial_schema.sql"))?;
        conn.execute("INSERT INTO schema_migrations (version) VALUES (1)", [])?;
    }

    // Future migrations go here
    // if schema_version < 2 { ... }

    Ok(())
}
```

---

## Data Integrity Rules

### Referential Integrity
- All foreign keys enforced at database level
- Cascading deletes for junction tables (mood_checkin_activities)
- No cascading deletes for assessments/mood (preserve history)

### Soft Deletes
- Activities use soft delete (`deleted_at` timestamp)
- Historical references preserved (mood check-ins still show deleted activity names)

### Immutability
- Assessment responses are immutable (no updates after creation)
- Mood check-ins are immutable (no updates after creation)
- Schedules are mutable (user can edit frequency/time)

### Validation
- Database constraints (CHECK, UNIQUE, FOREIGN KEY)
- Application-level validation in Rust models
- Double validation (defense in depth)

---

## Performance Considerations

### Indexes
- All time-series columns indexed (created_at, completed_at)
- Composite indexes for filtered queries (type + date)
- Junction table indexes on both foreign keys

### Query Optimization
- Use DuckDB's columnar storage for aggregations
- Prepared statements for all queries
- Connection pooling (single connection, managed state)
- Lazy loading (fetch data on-demand, not eagerly)

### Data Retention
- Optional automatic deletion of old records (configurable)
- Default: Keep all data indefinitely
- Vacuum operation after bulk deletes (manual trigger)

---

## Testing Data

### Seed Data for Development

```sql
-- Sample activities
INSERT INTO activities (name, color, icon) VALUES
('Exercise', '#4CAF50', '🏃'),
('Meditation', '#9C27B0', '🧘'),
('Social', '#2196F3', '👥');

-- Sample assessment response (PHQ-9)
INSERT INTO assessment_responses (assessment_type_id, responses, total_score, severity_level, completed_at)
VALUES (
    (SELECT id FROM assessment_types WHERE code = 'PHQ9'),
    '[1, 1, 0, 2, 1, 0, 1, 0, 1]',
    7,
    'mild',
    '2025-10-01 09:00:00'
);

-- Sample mood check-ins
INSERT INTO mood_checkins (mood_rating, notes, created_at) VALUES
(3, 'Feeling okay', '2025-10-14 08:00:00'),
(4, 'Good morning run', '2025-10-14 18:00:00'),
(2, 'Stressful day', '2025-10-15 12:00:00');

-- Link activities to mood check-ins
INSERT INTO mood_checkin_activities (mood_checkin_id, activity_id) VALUES
(2, 1),  -- Check-in 2 linked to Exercise
(3, 3);  -- Check-in 3 linked to Social
```

---

## Summary

This data model provides:

1. **Flexibility**: Extensible for new assessment types
2. **Performance**: Optimized indexes for time-series queries
3. **Integrity**: Enforced constraints and soft deletes
4. **Privacy**: Local-only storage, no PII
5. **Testability**: Clear validation rules and sample data

The model aligns with the feature specification and supports all functional requirements (FR-001 through FR-036).
