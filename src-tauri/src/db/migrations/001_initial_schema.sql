-- Migration 001: Initial Schema (SQLite)
-- Mental Health Assessment and Tracking Application

-- Assessment Types (PHQ-9, GAD-7, CES-D, OASIS)
CREATE TABLE assessment_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    question_count INTEGER NOT NULL,
    min_score INTEGER NOT NULL,
    max_score INTEGER NOT NULL,
    thresholds TEXT NOT NULL,  -- JSON stored as TEXT
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Assessment Responses
CREATE TABLE assessment_responses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    assessment_type_id INTEGER NOT NULL,
    responses TEXT NOT NULL,  -- JSON stored as TEXT
    total_score INTEGER NOT NULL,
    severity_level TEXT,
    completed_at TEXT NOT NULL DEFAULT (datetime('now')),
    notes TEXT CHECK (length(notes) <= 10000 OR notes IS NULL),
    FOREIGN KEY (assessment_type_id) REFERENCES assessment_types(id)
);

CREATE INDEX idx_assessment_responses_completed_at ON assessment_responses(completed_at);
CREATE INDEX idx_assessment_responses_type_date ON assessment_responses(assessment_type_id, completed_at);
CREATE INDEX idx_assessment_responses_severity ON assessment_responses(severity_level);

-- Activities (user-defined for mood tracking)
CREATE TABLE activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    icon TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT  -- Soft delete timestamp
);

CREATE INDEX idx_activities_deleted_at ON activities(deleted_at);

-- Mood Check-Ins
CREATE TABLE mood_checkins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mood_rating INTEGER NOT NULL CHECK (mood_rating BETWEEN 1 AND 5),
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_mood_checkins_created_at ON mood_checkins(created_at);
CREATE INDEX idx_mood_checkins_date ON mood_checkins(DATE(created_at));

-- Mood Check-In Activities (junction table)
-- SQLite supports ON DELETE CASCADE, so we can use it for referential integrity
CREATE TABLE mood_checkin_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mood_checkin_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    FOREIGN KEY (mood_checkin_id) REFERENCES mood_checkins(id) ON DELETE CASCADE,
    FOREIGN KEY (activity_id) REFERENCES activities(id),
    UNIQUE(mood_checkin_id, activity_id)
);

CREATE INDEX idx_mood_checkin_activities_checkin ON mood_checkin_activities(mood_checkin_id);
CREATE INDEX idx_mood_checkin_activities_activity ON mood_checkin_activities(activity_id);

-- Assessment Schedules
CREATE TABLE assessment_schedules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    assessment_type_id INTEGER NOT NULL,
    frequency TEXT NOT NULL CHECK (frequency IN ('daily', 'weekly', 'biweekly', 'monthly')),
    time_of_day TEXT NOT NULL,  -- HH:MM format
    day_of_week INTEGER,
    day_of_month INTEGER,
    enabled INTEGER NOT NULL DEFAULT 1,  -- Boolean stored as INTEGER (0=false, 1=true)
    last_triggered_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
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
