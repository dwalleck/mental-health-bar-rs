# Feature Specification: Mental Health Assessment and Tracking Application

**Feature Branch**: `001-mental-health-tracking`
**Created**: 2025-10-15
**Status**: Draft
**Input**: User description: "I want to create an application that lets the user take the PHQ-9, Beck Depression Inventory, GAD-7, and Beck Anxiety Inventory depression and anxiety assessments on a configurable, periodic basis and track their scores. I would also like users to be able to check in multiple times per day and rate their mood on a scale of 1-5, as well as identify any activities the user has been doing. Activities should be creatable by the user. I would like to be able to visualize the depression and anxiety assessment scores as well as mood scores in charts."

## Clarifications

### Session 2025-10-16

- Q: When a user starts an assessment but navigates away without completing it, how should the system handle the partial progress? → A: Save as incomplete draft with timestamp - user can resume or delete later
- Q: When a user tries to complete the same assessment type (e.g., PHQ-9) multiple times in one day, how should the system respond? → A: Prevent - block completing the same assessment type more than once per day with informative message
- Q: What validation rules should apply to activity names regarding length and allowed characters? → A: Max 30 characters, allow all printable characters except < > & " to prevent injection issues. Additionally, users can associate a Lineicons v5 icon with each activity.
- Q: Should users be able to backdate assessments or mood check-ins they forgot to log, and if so, with what restrictions? → A: Limited backdating - allow backdating both assessments and mood check-ins within last 24 hours only
- Q: How should the system display chart views when there's insufficient data (0 or 1 data point)? → A: Show placeholder state with icon, message explaining requirement, and call-to-action button to create first/next entry

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Complete Clinical Assessment (Priority: P1)

A user wants to complete a standardized mental health assessment (PHQ-9, CES-D, GAD-7, or OASIS) to track their mental health status over time. They need to take these assessments periodically to monitor changes and trends.

**Why this priority**: This is the core value proposition of the application - providing standardized, validated mental health assessments that users and healthcare providers can trust. Without this, the application has no clinical foundation.

**Independent Test**: Can be fully tested by loading an assessment, answering all questions, submitting responses, and receiving a calculated score. Delivers immediate value by providing a standardized mental health screening result.

**Acceptance Scenarios**:

1. **Given** a user opens the application, **When** they select an assessment type (PHQ-9, CES-D, GAD-7, or OASIS), **Then** they see all questions for that assessment presented clearly
2. **Given** a user is viewing an assessment, **When** they answer all required questions and submit, **Then** their responses are saved and a total score is calculated according to the assessment's scoring guidelines
3. **Given** a user has completed an assessment, **When** they view their results, **Then** they see their total score, the date completed, and interpretation guidance based on standard clinical thresholds
4. **Given** a user has a recurring assessment scheduled, **When** the scheduled time arrives, **Then** they receive a notification to complete the assessment

---

### User Story 2 - Quick Daily Mood Check-In (Priority: P2)

A user wants to quickly log their current mood state multiple times throughout the day without the burden of completing a full assessment. They can rate their mood on a simple 1-5 scale and optionally note activities they've been doing.

**Why this priority**: Daily mood tracking provides granular data between formal assessments, helping users identify patterns and triggers. It's a lightweight engagement mechanism that encourages regular app usage.

**Independent Test**: Can be tested by opening the check-in feature, selecting a mood rating (1-5), optionally selecting activities, submitting the check-in, and verifying it's saved with a timestamp. Delivers value by creating a mood journal with minimal effort.

**Acceptance Scenarios**:

1. **Given** a user wants to log their current mood, **When** they open the check-in feature, **Then** they see a simple interface to rate mood from 1-5 with clear labels (1=Very Bad, 2=Bad, 3=Neutral, 4=Good, 5=Very Good)
2. **Given** a user is logging a mood check-in, **When** they select their mood rating, **Then** they can optionally select one or more activities from their activity list
3. **Given** a user completes a mood check-in, **When** they submit it, **Then** the check-in is saved with the current date and time, mood rating, and selected activities
4. **Given** a user has completed multiple check-ins in a day, **When** they review their history, **Then** they see all check-ins for that day with timestamps

---

### User Story 3 - Manage Personal Activities (Priority: P2)

A user wants to create and manage a personalized list of activities they engage in (e.g., "exercise", "meditation", "socializing", "work") so they can track which activities correlate with their mood states.

**Why this priority**: Activity tracking enables users to identify patterns between behaviors and mood states, supporting self-awareness and informed decisions about lifestyle changes. This is foundational for mood check-ins to be meaningful.

**Independent Test**: Can be tested by creating new activities, editing activity names, deleting activities, and verifying they appear in the mood check-in interface. Delivers value by enabling personalized activity tracking.

**Acceptance Scenarios**:

1. **Given** a user wants to track a new activity, **When** they access the activity management interface, **Then** they can create a new activity with a custom name
2. **Given** a user has created activities, **When** they view their activity list, **Then** they see all activities they've created in an organized list
3. **Given** a user wants to modify an activity, **When** they select an existing activity, **Then** they can edit its name or delete it
4. **Given** a user deletes an activity, **When** they view past check-ins that referenced that activity, **Then** those check-ins still show the deleted activity name (data integrity preserved)

---

### User Story 4 - Visualize Assessment Trends (Priority: P3)

A user wants to see their assessment scores over time displayed in charts to quickly understand trends in their depression and anxiety levels and share insights with healthcare providers.

**Why this priority**: Visualization makes data actionable by revealing patterns that aren't obvious in raw numbers. This supports clinical decision-making and user self-management.

**Independent Test**: Can be tested by completing multiple assessments over time and viewing charts that plot scores chronologically. Delivers value by making trends immediately apparent.

**Acceptance Scenarios**:

1. **Given** a user has completed multiple assessments of the same type, **When** they view the chart for that assessment, **Then** they see a line graph plotting their scores over time with dates on the x-axis and scores on the y-axis
2. **Given** a user views an assessment chart, **When** they examine the visualization, **Then** clinical threshold lines are displayed (e.g., mild/moderate/severe boundaries) to contextualize their scores
3. **Given** a user has taken different assessment types, **When** they access the visualization feature, **Then** they can select which assessment type to view (PHQ-9, CES-D, GAD-7, or OASIS)
4. **Given** a user wants to understand a specific data point, **When** they interact with a point on the chart, **Then** they see the exact score and date for that assessment

---

### User Story 5 - Visualize Daily Mood Patterns (Priority: P3)

A user wants to see their mood check-in data visualized over time to identify patterns in their daily mood fluctuations and correlations with activities.

**Why this priority**: Mood visualization helps users recognize patterns (e.g., time of day effects, activity correlations) that inform behavior changes and treatment discussions.

**Independent Test**: Can be tested by logging multiple mood check-ins over several days and viewing charts that display mood trends. Delivers value by revealing daily and weekly mood patterns.

**Acceptance Scenarios**:

1. **Given** a user has logged multiple mood check-ins, **When** they view the mood chart, **Then** they see a visualization of their mood ratings over time (e.g., line chart or bar chart)
2. **Given** a user views their mood chart, **When** they select a time range (e.g., last 7 days, last 30 days), **Then** the chart updates to show only data from that period
3. **Given** a user wants to understand activity correlations, **When** they view mood data with activities, **Then** they can see which activities were logged alongside specific mood ratings
4. **Given** a user has multiple check-ins in one day, **When** viewing the chart, **Then** all check-ins for that day are displayed to show intra-day mood variation

---

### User Story 6 - Configure Assessment Schedules (Priority: P3)

A user wants to set up recurring schedules for assessments (e.g., weekly PHQ-9, biweekly GAD-7) so they're reminded to complete assessments at appropriate intervals for clinical monitoring.

**Why this priority**: Regular assessment intervals are clinically important for tracking treatment progress, but this is a supporting feature that enhances the core assessment functionality.

**Independent Test**: Can be tested by configuring a schedule for an assessment, waiting for or simulating the scheduled time, receiving a reminder, and verifying the schedule persists. Delivers value by automating assessment cadence.

**Acceptance Scenarios**:

1. **Given** a user wants to establish a regular assessment routine, **When** they access schedule settings, **Then** they can set up a recurring schedule for each assessment type with customizable frequency (daily, weekly, biweekly, monthly)
2. **Given** a user has configured an assessment schedule, **When** the scheduled time arrives, **Then** they receive a reminder notification to complete the assessment
3. **Given** a user has set up multiple assessment schedules, **When** they view their schedule settings, **Then** they see all active schedules with their frequencies and next scheduled dates
4. **Given** a user wants to modify their routine, **When** they access a configured schedule, **Then** they can edit the frequency or disable the schedule

---

### Edge Cases

- **Incomplete assessments**: When a user starts an assessment but navigates away without completing it, the system saves it as an incomplete draft with a timestamp. Users can resume the draft later or explicitly delete it. Only one incomplete draft per assessment type is allowed at a time.
- **Multiple daily assessments**: The system prevents completing the same assessment type more than once per calendar day. If a user attempts to start an assessment they've already completed today, they receive an informative message explaining the restriction and showing when they can next take that assessment (tomorrow).
- **Activity name validation**: Activity names are limited to 30 characters maximum and may contain all printable characters except < > & " (to prevent injection attacks). The system validates names on creation and edit, displaying clear error messages for violations.
- **Backdating entries**: Users can optionally specify a custom timestamp when creating assessments or mood check-ins, but only within the last 24 hours from the current time. The system validates the backdated timestamp and prevents dates in the future or older than 24 hours, displaying appropriate error messages.
- **Insufficient chart data**: When a user views a chart with fewer than 2 data points, the system displays a placeholder state with an informative icon, a message explaining the minimum data requirement (e.g., "Complete at least 2 assessments to view trends"), and a call-to-action button that navigates the user to create the first/next entry.
- **Timezone changes**: Deferred to v0.2.0 - Current implementation uses local device timezone. Users who travel should be aware that timestamps reflect device timezone at time of entry. Migration to UTC storage with timezone conversion is planned for future release.
- What happens if a user deletes an activity that's associated with historical mood check-ins? (Addressed in User Story 3 - data integrity preserved)

## Requirements *(mandatory)*

### Functional Requirements

#### Assessment Management
- **FR-001**: System MUST provide the complete, validated PHQ-9 (Patient Health Questionnaire-9) assessment with all 9 standard questions and the correct scoring algorithm (0-27 scale)
- **FR-002**: System MUST provide the complete CES-D (Center for Epidemiologic Studies Depression Scale) with all 20 standard questions and correct scoring algorithm (0-60 scale)
- **FR-003**: System MUST provide the complete GAD-7 (Generalized Anxiety Disorder-7) assessment with all 7 standard questions and correct scoring algorithm (0-21 scale)
- **FR-004**: System MUST provide the complete OASIS (Overall Anxiety Severity and Impairment Scale) with all 5 standard questions and correct scoring algorithm (0-20 scale)
- **FR-005**: System MUST calculate and display the total score for each completed assessment according to the assessment's official scoring guidelines
- **FR-006**: System MUST save all completed assessments with the date and time of completion
- **FR-007**: System MUST allow users to view their assessment history, including all past scores and completion dates
- **FR-008**: System MUST present assessment questions in a clear, readable format that matches the original validated instruments (minimum 16px font size, 1.5 line height, 4.5:1 contrast ratio, one question visible at a time on mobile viewports)
- **FR-009**: System MUST provide interpretation guidance for assessment scores using standard clinical thresholds (e.g., minimal, mild, moderate, severe)
- **FR-009a**: System MUST save incomplete assessments as drafts with timestamps when users navigate away before completion, allow users to resume or delete drafts, and limit to one incomplete draft per assessment type at a time
- **FR-009b**: System MUST prevent users from completing the same assessment type more than once per calendar day (based on local timezone), displaying an informative message with the next available date when blocked
- **FR-009c**: System MUST allow users to optionally specify a custom completion timestamp when submitting an assessment, validating that the timestamp is within the last 24 hours and not in the future, defaulting to current time if not specified

#### Assessment Scheduling
- **FR-010**: System MUST allow users to configure recurring schedules for each assessment type independently
- **FR-011**: System MUST support configurable schedule frequencies including daily, weekly, biweekly, and monthly intervals
- **FR-012**: System MUST send notifications/reminders when scheduled assessments are due
- **FR-013**: System MUST track whether scheduled assessments have been completed
- **FR-014**: System MUST allow users to disable or modify existing assessment schedules

#### Mood Check-Ins
- **FR-015**: System MUST allow users to create multiple mood check-ins per day with timestamps
- **FR-015a**: System MUST allow users to optionally specify a custom timestamp when logging a mood check-in, validating that the timestamp is within the last 24 hours and not in the future, defaulting to current time if not specified
- **FR-016**: System MUST provide a 1-5 scale for mood rating with clear labeling (1=Very Bad, 2=Bad, 3=Neutral, 4=Good, 5=Very Good)
- **FR-017**: System MUST allow users to optionally associate one or more activities with each mood check-in
- **FR-018**: System MUST save mood check-ins with the exact date and time, mood rating, and selected activities
- **FR-019**: System MUST allow users to view their mood check-in history

#### Activity Management
- **FR-020**: System MUST allow users to create custom activities with user-defined names
- **FR-020a**: System MUST validate activity names with the following rules: maximum 30 characters, allow all printable characters except < > & " characters, display clear validation error messages when rules are violated
- **FR-020b**: System MUST allow users to optionally associate a Lineicons v5 icon with each activity during creation or editing
- **FR-021**: System MUST allow users to view all activities they've created
- **FR-022**: System MUST allow users to edit activity names and icons
- **FR-023**: System MUST allow users to delete activities
- **FR-024**: System MUST preserve historical data integrity when activities are deleted (past check-ins retain the activity name and icon)
- **FR-025**: System MUST make user-created activities immediately available for selection in mood check-ins

#### Data Visualization
- **FR-026**: System MUST display assessment scores over time in chart format for each assessment type
- **FR-027**: System MUST display mood check-in ratings over time in chart format
- **FR-028**: System MUST allow users to select different time ranges for viewing charts (e.g., last 7 days, last 30 days, last 90 days, all time)
- **FR-029**: System MUST allow users to select which assessment type to visualize (PHQ-9, CES-D, GAD-7, or OASIS)
- **FR-030**: Charts MUST display dates/timestamps on one axis and scores/ratings on the other axis
- **FR-031**: Assessment charts MUST include visual indicators of clinical thresholds (e.g., lines showing mild/moderate/severe boundaries). Threshold values defined in contracts/assessments.md: PHQ-9 (0-4 minimal, 5-9 mild, 10-14 moderate, 15-19 moderately severe, 20-27 severe), GAD-7 (0-4 minimal, 5-9 mild, 10-14 moderate, 15-21 severe), CES-D and OASIS per published guidelines
- **FR-032**: System MUST display a placeholder state when chart views have fewer than 2 data points, showing an informative icon, a message explaining the minimum data requirement, and a call-to-action button to navigate the user to create the first/next entry (assessment or mood check-in)

#### Data Persistence
- **FR-033**: System MUST persist all assessment responses, scores, and completion timestamps
- **FR-034**: System MUST persist all mood check-ins with timestamps, ratings, and associated activities
- **FR-035**: System MUST persist user-created activities
- **FR-036**: System MUST persist assessment schedule configurations

### Key Entities *(include if feature involves data)*

- **Assessment**: Represents a specific mental health assessment instrument (PHQ-9, CES-D, GAD-7, OASIS). Contains questions, scoring rules, and interpretation thresholds. Each assessment has a name, type identifier, questions list, and scoring algorithm.

- **Assessment Response**: Represents an instance of an assessment by a user (completed or incomplete draft). Contains the assessment type, all question responses, calculated total score (null for incomplete), completion status (completed/incomplete), started timestamp, completion timestamp (null for incomplete), and user association. Related to Assessment (which assessment was taken).

- **Mood Check-In**: Represents a single mood logging event. Contains mood rating (1-5), timestamp, optional text note, and user association. Related to Activities (which activities were selected for this check-in).

- **Activity**: Represents a user-defined activity category. Contains activity name (max 30 characters, validated against restricted characters), optional icon identifier (Lineicons v5), creation date, and user association. Related to Mood Check-Ins (can be associated with multiple check-ins).

- **Assessment Schedule**: Represents a recurring schedule configuration for an assessment. Contains assessment type, frequency (daily/weekly/biweekly/monthly), next scheduled date, enabled/disabled status, and user association. Related to Assessment (which assessment is scheduled).

- **User**: Represents the individual using the application. This is a single-user local application with no login or account management. Contains user preferences and timezone settings. All data is stored locally on the user's device. Related to all other entities as the owner/creator.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete a full standardized assessment (PHQ-9, CES-D, GAD-7, or OASIS) in under 5 minutes
- **SC-002**: Users can log a mood check-in in under 30 seconds
- **SC-003**: Users can create a new activity in under 15 seconds
- **SC-004**: Assessment scores are calculated with 100% accuracy according to validated clinical scoring guidelines
- **SC-005**: Users can view historical assessment trends for any assessment type in under 3 seconds
- **SC-006**: Users can view mood check-in visualizations for any time period in under 3 seconds
- **SC-007**: The application successfully delivers assessment reminders within 5 minutes of the scheduled time
- **SC-008**: Users can configure an assessment schedule in under 2 minutes
- **SC-009**: Charts accurately display all recorded data points with correct timestamps and values
- **SC-010**: 95% of users successfully complete their first assessment without errors or confusion
- **SC-011**: Users can access all their historical data (assessments, mood check-ins, activities) at any time
- **SC-012**: The application supports at least 1 year of continuous daily mood check-ins (minimum 365 entries) without performance degradation

## Assumptions

- Assessment questions and scoring algorithms will use the validated, published versions of PHQ-9, CES-D, GAD-7, and OASIS (all public domain, free to use)
- Clinical threshold values for interpretation will use standard published guidelines
- Users have the cognitive and technical ability to complete digital questionnaires independently
- The application is a single-user local application with no authentication or account management
- All data is stored locally on the user's device and is not transmitted to any remote servers
- Mood scale uses a 1-5 rating system with quality-based labels (Very Bad to Very Good)
- Default notification mechanism will be system notifications (platform-dependent)
- Assessment schedules use user's local timezone for timing
- Users can manually trigger assessments at any time, regardless of schedule
- Charts will use standard visualization types (line charts for trends over time)
- The application does not provide clinical diagnosis or treatment recommendations - it is a tracking tool only

## Out of Scope

- Integration with electronic health records (EHR) systems
- Sharing data with healthcare providers or third parties
- AI-powered insights or predictions about mental health trends
- Crisis intervention features or emergency contact systems
- Social features (comparing with other users, sharing progress)
- Integration with wearable devices or other health tracking apps
- Guided meditation, therapy exercises, or intervention content
- Multi-language support (assuming English for initial version)
- Voice input for assessments or check-ins
- Export functionality for data (PDF reports, CSV exports)
- Medication tracking or adherence features
- Appointment reminders or healthcare provider directories

## Dependencies

- Access to validated, copyright-compliant versions of the clinical assessment instruments (PHQ-9, CES-D, GAD-7, OASIS - all public domain)
- Verified scoring algorithms for each assessment from authoritative clinical sources

**Note on Assessment Selection**: Original specification included Beck Depression Inventory (BDI) and Beck Anxiety Inventory (BAI). Research revealed these are copyrighted by Pearson and require licensing fees. They have been replaced with:
- **CES-D** (Center for Epidemiologic Studies Depression Scale): 20-item depression assessment, public domain, 0-60 scale
- **OASIS** (Overall Anxiety Severity and Impairment Scale): 5-item anxiety assessment, public domain, 0-20 scale

Both are validated, widely-used alternatives that provide equivalent clinical utility without licensing restrictions.
- Charting/visualization capability appropriate for the target platform
- Local data storage mechanism with adequate security for sensitive health data
- Notification system capability on the target platform
- Date/time handling that correctly manages timezones and recurring schedules
- Lineicons v5 icon library for activity icons

## Privacy & Security Considerations

- All mental health data (assessment responses, scores, mood ratings) is highly sensitive and must be protected
- Data storage must comply with applicable health data privacy regulations (HIPAA in US, GDPR in EU, etc.)
- Users must have complete control over their data (ability to delete all data)
- Assessment responses and scores should be encrypted at rest
- No personally identifiable health information should be transmitted or stored unnecessarily
- Users should be informed about what data is collected and how it's stored
- Consider implementing optional passcode/biometric protection for app access
- Ensure deleted data (especially deleted activities) doesn't expose private information in remaining historical records
