-- Migration 001: Initial Schema
-- Mental Health Assessment and Tracking Application

-- Assessment Types (PHQ-9, GAD-7, CES-D, OASIS)
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

-- Assessment Responses
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

-- Activities (user-defined for mood tracking)
CREATE TABLE activities (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7),
    icon VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX idx_activities_deleted_at ON activities(deleted_at);

-- Mood Check-Ins
CREATE TABLE mood_checkins (
    id INTEGER PRIMARY KEY,
    mood_rating INTEGER NOT NULL CHECK (mood_rating BETWEEN 1 AND 5),
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_mood_checkins_created_at ON mood_checkins(created_at);
CREATE INDEX idx_mood_checkins_date ON mood_checkins(DATE(created_at));

-- Mood Check-In Activities (junction table)
CREATE TABLE mood_checkin_activities (
    id INTEGER PRIMARY KEY,
    mood_checkin_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    FOREIGN KEY (mood_checkin_id) REFERENCES mood_checkins(id),
    FOREIGN KEY (activity_id) REFERENCES activities(id),
    UNIQUE(mood_checkin_id, activity_id)
);

CREATE INDEX idx_mood_checkin_activities_checkin ON mood_checkin_activities(mood_checkin_id);
CREATE INDEX idx_mood_checkin_activities_activity ON mood_checkin_activities(activity_id);

-- Assessment Schedules
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
