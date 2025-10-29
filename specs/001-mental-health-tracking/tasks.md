# Tasks: Mental Health Assessment and Tracking Application

**Input**: Design documents from `/specs/001-mental-health-tracking/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: TDD is MANDATORY per project constitution (Principle IX). Tests written → user approved → tests fail → implement.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Backend (Rust)**: `src-tauri/src/`
- **Frontend (Svelte)**: `src/`
- **Tests**: `src-tauri/tests/` (Rust), `tests/` (Svelte/Vitest)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Add DuckDB dependency to src-tauri/Cargo.toml (version 0.9+)
- [X] T002 [P] Add Chart.js and svelte-chartjs to package.json
- [X] T003 [P] Add chrono dependency to src-tauri/Cargo.toml for date/time handling
- [X] T004 [P] Configure TailwindCSS theme colors for mood scale in tailwind.config.js
- [X] T005 [P] Setup tracing-subscriber and tracing-appender in src-tauri/Cargo.toml
- [X] T006 Create vertical slice directory structure in src-tauri/src/features/
- [X] T007 [P] Create frontend component directories in src/lib/components/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 Initialize DuckDB connection manager in src-tauri/src/db/mod.rs with app data directory path
- [X] T009 Create migrations framework in src-tauri/src/db/migrations.rs with version tracking
- [X] T010 Create initial database schema (001_initial_schema.sql) in src-tauri/src/db/migrations/ with assessment_types, assessment_responses, activities, mood_checkins, mood_checkin_activities, assessment_schedules tables
- [X] T011 Implement schema migration runner that executes on app startup in src-tauri/src/db/migrations.rs
- [X] T012 [P] Seed assessment_types table with PHQ-9, GAD-7, CES-D, OASIS in migration
- [X] T012b [P] Set DuckDB file permissions to 0600 (user-only read/write) on database creation in src-tauri/src/db/mod.rs
- [X] T013 [P] Setup shared error types using thiserror in src-tauri/src/errors.rs with DatabaseError, ValidationError, NotFoundError
- [X] T014 [P] Configure confy-based app configuration in src-tauri/src/config/mod.rs with AppConfig struct
- [X] T015 [P] Setup tracing initialization in src-tauri/src/main.rs with file appender and console output
- [X] T016 Setup Tauri managed state in src-tauri/src/lib.rs with Arc<Mutex<Connection>> for DuckDB
- [X] T017 [P] Configure tauri-specta type generation test in src-tauri/src/lib.rs (#[cfg(test)] fn generate_types)
- [X] T018 Generate initial TypeScript bindings by running cargo test generate_types
- [X] T019 [P] Create base Svelte layout in src/routes/+layout.svelte with navigation sidebar
- [X] T020 [P] Create reusable UI components in src/lib/components/ui/ (Button, Card, Input, Select)
- [X] T020a [P] Add HasChildren error variant to AssessmentError in src-tauri/src/features/assessments/models.rs for defensive deletion
- [X] T020b [P] Create helper functions for counting child records (count_assessment_responses, count_assessment_schedules) in assessment repository

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Complete Clinical Assessment (Priority: P1) 🎯 MVP

**Goal**: Users can complete PHQ-9, GAD-7, CES-D, or OASIS assessments and receive calculated scores with severity interpretation

**Independent Test**: Load an assessment, answer all questions, submit responses, receive calculated score with severity level

### Tests for User Story 1 (TDD - Write First, Verify Fail)

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T021 [P] [US1] Unit test: PHQ-9 scoring algorithm (9 questions, score 0-27) in src-tauri/src/features/assessments/models.rs
- [X] T022 [P] [US1] Unit test: GAD-7 scoring algorithm (7 questions, score 0-21) in src-tauri/src/features/assessments/models.rs
- [X] T023 [P] [US1] Unit test: CES-D scoring algorithm (20 questions, score 0-60) in src-tauri/src/features/assessments/models.rs
- [X] T024 [P] [US1] Unit test: OASIS scoring algorithm (5 questions, score 0-20) in src-tauri/src/features/assessments/models.rs
- [X] T025 [P] [US1] Unit test: Severity level calculation for each assessment type in src-tauri/src/features/assessments/models.rs
- [X] T026 [P] [US1] Unit test: Response validation (count, range) in src-tauri/src/features/assessments/models.rs
- [X] T027 [P] [US1] Integration test: submit_assessment command end-to-end in src-tauri/tests/test_assessments.rs
- [X] T028 [P] [US1] Integration test: get_assessment_history query in src-tauri/tests/test_assessments.rs
- [ ] T029 [P] [US1] Component test: AssessmentForm renders questions correctly (DEFERRED: Requires SvelteKit $app module mocking - will implement with frontend PR)

### Implementation for User Story 1

- [X] T030 [P] [US1] Create AssessmentType model in src-tauri/src/features/assessments/models.rs with specta::Type derive
- [X] T031 [P] [US1] Create AssessmentResponse model in src-tauri/src/features/assessments/models.rs
- [X] T032 [P] [US1] Create AssessmentQuestion model in src-tauri/src/features/assessments/models.rs
- [X] T033 [P] [US1] Define PHQ-9 questions constant in src-tauri/src/features/assessments/content.rs
- [X] T034 [P] [US1] Define GAD-7 questions constant in src-tauri/src/features/assessments/content.rs
- [X] T035 [P] [US1] Define CES-D questions constant in src-tauri/src/features/assessments/content.rs
- [X] T036 [P] [US1] Define OASIS questions constant in src-tauri/src/features/assessments/content.rs
- [X] T037 [US1] Implement PHQ-9 scoring function in src-tauri/src/features/assessments/models.rs (calculate_phq9_score)
- [X] T038 [US1] Implement GAD-7 scoring function in src-tauri/src/features/assessments/models.rs
- [X] T039 [US1] Implement CES-D scoring function in src-tauri/src/features/assessments/models.rs
- [X] T040 [US1] Implement OASIS scoring function in src-tauri/src/features/assessments/models.rs
- [X] T041 [US1] Implement severity level mapping for PHQ-9 in src-tauri/src/features/assessments/models.rs (get_phq9_severity)
- [X] T042 [US1] Implement severity level mapping for GAD-7 in src-tauri/src/features/assessments/models.rs
- [X] T043 [US1] Implement severity level mapping for CES-D in src-tauri/src/features/assessments/models.rs
- [X] T044 [US1] Implement severity level mapping for OASIS in src-tauri/src/features/assessments/models.rs
- [X] T045 [P] [US1] Create AssessmentRepository in src-tauri/src/features/assessments/repository.rs with save_assessment method
- [X] T046 [P] [US1] Implement get_assessment_types query in repository
- [X] T047 [P] [US1] Implement get_assessment_history query in repository with date filtering
- [X] T048 [P] [US1] Implement get_assessment_response query in repository
- [X] T049 [US1] Implement submit_assessment Tauri command in src-tauri/src/features/assessments/commands.rs with validation and error handling
- [X] T050 [US1] Implement get_assessment_types Tauri command in src-tauri/src/features/assessments/commands.rs
- [X] T051 [US1] Implement get_assessment_questions Tauri command in src-tauri/src/features/assessments/commands.rs
- [X] T052 [US1] Implement get_assessment_history Tauri command in src-tauri/src/features/assessments/queries.rs
- [X] T053 [US1] Implement get_assessment_response Tauri command in src-tauri/src/features/assessments/queries.rs
- [X] T054 [US1] Implement get_latest_assessment Tauri command in src-tauri/src/features/assessments/queries.rs
- [X] T055 [US1] Implement delete_assessment Tauri command in src-tauri/src/features/assessments/commands.rs
- [X] T056 [US1] Register all assessment commands in src-tauri/src/lib.rs invoke_handler
- [X] T057 [US1] Regenerate TypeScript bindings (cargo test generate_types)
- [X] T058 [P] [US1] Create AssessmentList component in src/lib/components/assessments/AssessmentList.svelte to display available assessments
- [X] T059 [P] [US1] Create AssessmentForm component in src/lib/components/assessments/AssessmentForm.svelte with question rendering and response collection
- [X] T060 [P] [US1] Create AssessmentResult component in src/lib/components/assessments/AssessmentResult.svelte to show score and severity
- [ ] T061 [US1] Create assessments Svelte store in src/lib/stores/assessments.ts with reactive state (SKIPPED: used component-level state instead)
- [X] T062 [US1] Create /assessments route page in src/routes/assessments/+page.svelte for assessment selection
- [X] T063 [US1] Create /assessments/[type] dynamic route in src/routes/assessments/[type]/+page.svelte for taking assessments
- [X] T064 [US1] Create /assessments/history route in src/routes/assessments/history/+page.svelte for viewing past assessments
- [X] T065 [US1] Add assessment navigation links to layout sidebar
- [X] T066 [US1] Add error handling with user-friendly messages using tauri-plugin-dialog
- [X] T067 [US1] Add logging for assessment submission with tracing::info!
- [X] T067a [P] [US1] Integration test: Verify assessment_type deletion blocked when responses exist in src-tauri/tests/repository_integration.rs
- [X] T067b [P] [US1] Integration test: Verify assessment_type deletion blocked when schedules exist in src-tauri/tests/repository_integration.rs
- [X] T067c [US1] Implement delete_assessment_type with defensive checks (prevent if children exist) in src-tauri/src/features/assessments/repository.rs
- [X] T067d [US1] Implement delete_assessment_type Tauri command in src-tauri/src/features/assessments/commands.rs with user-friendly error messages

**Checkpoint**: User Story 1 complete - Run all tests, verify PHQ-9/GAD-7/CES-D/OASIS flow works end-to-end

---

## Phase 4: User Story 2 - Quick Daily Mood Check-In (Priority: P2)

**Goal**: Users can quickly log mood ratings (1-5) multiple times per day with optional activity associations

**Independent Test**: Open check-in feature, select mood rating, optionally select activities, submit, verify saved with timestamp

### Tests for User Story 2 (TDD - Write First, Verify Fail)

- [X] T068 [P] [US2] Unit test: Mood rating validation (1-5 only) in src-tauri/src/features/mood/models.rs
- [X] T069 [P] [US2] Integration test: log_mood command in src-tauri/tests/test_mood.rs
- [X] T070 [P] [US2] Integration test: get_mood_history query with date filtering in src-tauri/tests/test_mood.rs
- [X] T071 [P] [US2] Integration test: Mood check-in with multiple activities in src-tauri/tests/test_mood.rs
- [ ] T072 [P] [US2] Component test: MoodScaleInput renders 1-5 buttons (DEFERRED: Component not yet implemented - will add with T086)

### Implementation for User Story 2

- [X] T073 [P] [US2] Create MoodCheckin model in src-tauri/src/features/mood/models.rs with specta::Type
- [X] T074 [P] [US2] Create LogMoodRequest model in src-tauri/src/features/mood/models.rs
- [X] T075 [P] [US2] Create MoodRepository in src-tauri/src/features/mood/repository.rs with create_mood_checkin method
- [X] T076 [P] [US2] Implement get_mood_history query in repository with date range and limit
- [X] T077 [P] [US2] Implement get_mood_checkin query in repository
- [X] T078 [P] [US2] Implement get_mood_stats query in repository (average, distribution, correlations)
- [X] T079 [P] [US2] Implement link_activities method in repository for mood_checkin_activities junction table (NOTE: Implemented inline in create_mood_checkin, not as separate method)
- [X] T080 [US2] Implement log_mood Tauri command in src-tauri/src/features/mood/commands.rs with mood rating validation
- [X] T081 [US2] Implement get_mood_history Tauri command in src-tauri/src/features/mood/queries.rs
- [X] T082 [US2] Implement get_mood_checkin Tauri command in src-tauri/src/features/mood/queries.rs
- [X] T083 [US2] Implement get_mood_stats Tauri command in src-tauri/src/features/mood/queries.rs
- [X] T084 [US2] Register all mood commands in src-tauri/src/lib.rs invoke_handler
- [X] T085 [US2] Regenerate TypeScript bindings
- [X] T086 [P] [US2] Create MoodScaleInput component in src/lib/components/mood/MoodScaleInput.svelte with 1-5 buttons and color coding
- [X] T087 [P] [US2] Create ActivitySelector component in src/lib/components/mood/ActivitySelector.svelte for multi-select
- [X] T088 [P] [US2] Create MoodHistoryList component in src/lib/components/mood/MoodHistoryList.svelte to display check-ins
- [X] T089 [US2] Create mood Svelte store in src/lib/stores/mood.ts
- [X] T090 [US2] Create /mood route page in src/routes/mood/+page.svelte for logging mood check-ins
- [X] T091 [US2] Create /mood/history route in src/routes/mood/history/+page.svelte for viewing past check-ins
- [X] T092 [US2] Add mood navigation links to layout sidebar
- [X] T093 [US2] Add mood color constants (Very Bad=red, Very Good=green) in src/lib/utils/colors.ts
- [X] T093a [P] [US2] Integration test: Deleting mood_checkin cascades to mood_checkin_activities in src-tauri/tests/test_mood.rs
- [X] T093b [US2] Implement delete_mood_checkin with transactional cascade in src-tauri/src/features/mood/repository.rs
- [X] T093c [US2] Implement delete_mood_checkin Tauri command in src-tauri/src/features/mood/commands.rs

**Checkpoint**: User Stories 1 AND 2 both work independently - Mood logging functional

---

## Phase 5: User Story 3 - Manage Personal Activities (Priority: P2)

**Goal**: Users can create, edit, and delete custom activities for mood tracking

**Independent Test**: Create activities, edit names, delete activities, verify they appear in mood check-in interface

### Tests for User Story 3 (TDD - Write First, Verify Fail)

- [X] T094 [P] [US3] Unit test: Activity name validation (1-100 chars, non-empty) in src-tauri/src/features/mood/models.rs
- [X] T095 [P] [US3] Unit test: Activity color validation (hex format) in src-tauri/src/features/mood/models.rs
- [X] T096 [P] [US3] Integration test: create_activity command in src-tauri/tests/test_activities.rs
- [X] T097 [P] [US3] Integration test: Soft delete preserves historical data in src-tauri/tests/test_activities.rs
- [X] T097b [P] [US3] Integration test: Verify deleted activity names still display correctly in historical mood check-ins
- [ ] T098 [P] [US3] Component test: ActivityForm validates input in tests/unit/ActivityForm.test.ts

### Implementation for User Story 3

- [X] T099 [P] [US3] Create Activity model in src-tauri/src/features/mood/models.rs (if not already created for US2)
- [X] T100 [P] [US3] Create CreateActivityRequest model in src-tauri/src/features/mood/models.rs
- [X] T101 [P] [US3] Create UpdateActivityRequest model in src-tauri/src/features/mood/models.rs
- [ ] T101a [P] [US3] Implement Lineicons v5 icon picker component in src/lib/components/ui/IconPicker.svelte with search and preview
- [ ] T101b [P] [US3] Add icon identifier (string) field to Activity model and validate Lineicons v5 icon codes in src-tauri/src/features/mood/models.rs
- [X] T102 [P] [US3] Implement create_activity method in repository with duplicate name check
- [X] T103 [P] [US3] Implement update_activity method in repository
- [X] T104 [P] [US3] Implement delete_activity method in repository with soft delete (set deleted_at)
- [X] T105 [P] [US3] Implement get_activities query in repository with include_deleted parameter
- [X] T106 [US3] Implement create_activity Tauri command in src-tauri/src/features/mood/commands.rs with name/color validation
- [X] T107 [US3] Implement update_activity Tauri command in src-tauri/src/features/mood/commands.rs
- [X] T108 [US3] Implement delete_activity Tauri command in src-tauri/src/features/mood/commands.rs
- [X] T109 [US3] Implement get_activities Tauri command in src-tauri/src/features/mood/queries.rs
- [X] T110 [US3] Register activity commands in src-tauri/src/lib.rs invoke_handler
- [X] T111 [US3] Regenerate TypeScript bindings
- [X] T112 [P] [US3] Create ActivityForm component in src/lib/components/mood/ActivityForm.svelte with name/color/icon inputs
- [X] T113 [P] [US3] Create ActivityList component in src/lib/components/mood/ActivityList.svelte with edit/delete actions
- [X] T114 [US3] Create /mood/activities route in src/routes/mood/activities/+page.svelte for activity management
- [X] T115 [US3] Integrate ActivitySelector with get_activities query (filter deleted_at IS NULL for new check-ins, include deleted activities in historical view with "(deleted)" badge)
- [X] T116 [US3] Add activity management link to mood section in sidebar

**Checkpoint**: User Stories 1, 2, AND 3 all work independently - Activity CRUD functional

---

## Phase 6: User Story 4 - Visualize Assessment Trends (Priority: P3)

**Goal**: Users can view assessment scores over time in line charts with clinical threshold lines

**Independent Test**: Complete multiple assessments, view charts with scores plotted chronologically, see threshold lines

### Tests for User Story 4 (TDD - Write First, Verify Fail)

- [X] T117 [P] [US4] Unit test: Trend calculation (improving/worsening/stable) in src-tauri/src/features/visualization/models.rs (tests included in models.rs)
- [X] T118 [P] [US4] Integration test: get_assessment_chart_data query with time ranges in src-tauri/tests/test_visualization.rs
- [X] T119 [P] [US4] Integration test: Chart data aggregation for year+ data in src-tauri/tests/test_visualization.rs
- [ ] T120 [P] [US4] Component test: AssessmentChart renders with Chart.js in tests/unit/AssessmentChart.test.ts (DEFERRED: Requires frontend implementation)

### Implementation for User Story 4

- [X] T121 [P] [US4] Create ChartDataPoint model in src-tauri/src/features/visualization/models.rs
- [X] T122 [P] [US4] Create AssessmentChartData model in src-tauri/src/features/visualization/models.rs with thresholds and statistics
- [X] T123 [P] [US4] Create TimeRange enum in src-tauri/src/features/visualization/models.rs (Week, Month, Quarter, Year, AllTime, Custom)
- [X] T124 [US4] Implement get_assessment_chart_data query in src-tauri/src/features/visualization/repository.rs with DuckDB aggregations
- [X] T125 [US4] Implement calculate_statistics helper in repository (min, max, average, trend)
- [X] T126 [US4] Implement get_assessment_chart_data Tauri command in src-tauri/src/features/visualization/queries.rs
- [X] T127 [US4] Register visualization commands in src-tauri/src/lib.rs invoke_handler
- [X] T128 [US4] Regenerate TypeScript bindings
- [X] T129 [P] [US4] Create AssessmentChart component in src/lib/components/charts/AssessmentChart.svelte using Chart.js (native Svelte 5, not svelte-chartjs)
- [X] T130 [P] [US4] Implement threshold line annotations in AssessmentChart using Chart.js annotation plugin
- [X] T131 [P] [US4] Create TimeRangeSelector component in src/lib/components/charts/TimeRangeSelector.svelte
- [X] T132 [P] [US4] Create ChartStatistics component in src/lib/components/charts/ChartStatistics.svelte to show min/max/avg/trend
- [X] T132b [US4] Implement empty state UI in AssessmentChart component for <2 data points in src/lib/components/charts/AssessmentChart.svelte (show message: "Complete at least 2 assessments to view trends") ✅ IMPLEMENTED
- [X] T133 [US4] Create /charts route in src/routes/charts/+page.svelte with assessment type selector and chart display
- [X] T134 [US4] Add charts navigation link to sidebar (already present)
- [X] T135 [US4] Configure Chart.js defaults in src/lib/utils/chart-config.ts (responsive, animations, colors)

**Checkpoint**: User Stories 1-4 functional - Assessment visualization working

---

## Phase 7: User Story 5 - Visualize Daily Mood Patterns (Priority: P3)

**Goal**: Users can view mood check-in data in charts with activity correlations

**Independent Test**: Log multiple mood check-ins, view charts displaying mood trends and activity breakdowns

### Tests for User Story 5 (TDD - Write First, Verify Fail)

- [X] T136 [P] [US5] Integration test: get_mood_chart_data query in src-tauri/tests/test_visualization.rs
- [X] T137 [P] [US5] Integration test: Activity correlation calculation in src-tauri/tests/test_visualization.rs
- [ ] T138 [P] [US5] Component test: MoodChart renders mood colors correctly in tests/unit/MoodChart.test.ts (DEFERRED: Requires frontend implementation)

### Implementation for User Story 5

- [X] T139 [P] [US5] Create MoodChartData model in src-tauri/src/features/visualization/models.rs with activity_breakdown
- [X] T140 [P] [US5] Create ActivityMoodData model in src-tauri/src/features/visualization/models.rs
- [X] T141 [P] [US5] Create MoodStatistics model in src-tauri/src/features/visualization/models.rs (min, max, avg, median, mode)
- [X] T142 [US5] Implement get_mood_chart_data query in repository with activity grouping
- [X] T143 [US5] Implement calculate_mood_statistics helper in repository
- [X] T144 [US5] Implement get_mood_chart_data Tauri command in src-tauri/src/features/visualization/queries.rs
- [X] T145 [US5] Register mood chart commands in src-tauri/src/lib.rs invoke_handler
- [X] T146 [US5] Regenerate TypeScript bindings
- [X] T147 [P] [US5] Create MoodChart component in src/lib/components/charts/MoodChart.svelte with mood color gradient
- [X] T147b [US5] Implement empty state UI in MoodChart component for <2 data points in src/lib/components/charts/MoodChart.svelte (show message: "Log at least 2 moods to view patterns") ✅ IMPLEMENTED
- [X] T148 [P] [US5] Create ActivityCorrelationChart component in src/lib/components/charts/ActivityCorrelationChart.svelte (horizontal bar chart)
- [X] T149 [US5] Add mood chart section to /charts route page with time range selector and tab-based navigation
- [X] T150 [US5] Add activity correlation visualization to /charts route page with mood statistics panel

**Checkpoint**: User Stories 1-5 functional - Mood visualization complete

---

## Phase 7.5: Critical Test Coverage Gaps (Priority: P0 - URGENT)

**Goal**: Address missing test coverage identified in gap analysis before proceeding to scheduling

**Analysis**: See TEST_COVERAGE_ANALYSIS.md for full details (60% estimated coverage, target 80%+)

### Critical Tests - Visualization (IMMEDIATE)

- [X] T150a [P0] [CRITICAL] Verify test_visualization.rs is discovered by cargo test ✅ PASSING
- [X] T150b [P0] [CRITICAL] Run visualization tests and fix any failures ✅ 18/18 TESTS PASSED
- [ ] T150c [P0] [CRITICAL] Add test_visualization to CI pipeline

### High-Priority Tests - Command Validation

- [ ] T150d [P0] [US1] Test submit_assessment with notes exceeding 10,000 chars in tests/test_assessments.rs
- [ ] T150e [P0] [US1] Test submit_assessment with notes containing invalid control characters in tests/test_assessments.rs
- [ ] T150f [P0] [US1] Test submit_assessment with assessment type code > 10 chars in tests/test_assessments.rs
- [ ] T150g [P0] [US1] Test submit_assessment with non-alphanumeric type code in tests/test_assessments.rs
- [ ] T150h [P0] [US1] Test submit_assessment with empty type code in tests/test_assessments.rs
- [ ] T150i [P0] [US2] Test log_mood with notes exceeding 5,000 chars in tests/test_mood.rs
- [ ] T150j [P0] [US2] Test log_mood with boundary ratings (0, 6, -1, 100) in tests/test_mood.rs
- [ ] T150k [P0] [US2] Test log_mood with very large activity_ids array (50+ ids) in tests/test_mood.rs
- [ ] T150l [P0] [US3] Test create_activity with icon exceeding 20 chars in tests/test_activities.rs
- [ ] T150m [P0] [US3] Test create_activity with edge case color formats (#RGB, #12345, RGB) in tests/test_activities.rs

### High-Priority Tests - Query Edge Cases

- [ ] T150n [P0] [US1] Test get_assessment_history with invalid date format in tests/test_assessments.rs
- [ ] T150o [P0] [US1] Test get_assessment_history with limit=0 and limit=-1 in tests/test_assessments.rs
- [ ] T150p [P0] [US1] Test get_latest_assessment when no assessments exist in tests/test_assessments.rs
- [ ] T150q [P0] [US2] Test get_mood_stats with no data in tests/test_mood.rs
- [ ] T150r [P0] [US2] Test get_mood_stats with single check-in in tests/test_mood.rs
- [ ] T150s [P0] [US2] Test get_mood_history with limit=0 in tests/test_mood.rs

### High-Priority Tests - Boundary Conditions

- [ ] T150t [P0] [US1] Test assessment scores at exact threshold boundaries (PHQ9: 5,10,15,20) in tests/test_assessments.rs
- [ ] T150u [P0] [US1] Test assessment scores at exact threshold boundaries (GAD7: 5,10,15) in tests/test_assessments.rs
- [ ] T150v [P0] [US3] Test activity name at exactly 100 characters in tests/test_activities.rs
- [ ] T150w [P0] [US3] Test activity name at 99, 100, 101 chars (boundary) in tests/test_activities.rs
- [ ] T150x [P0] [US1] Test notes at exactly 10,000 characters in tests/test_assessments.rs
- [ ] T150y [P0] [US2] Test notes at exactly 5,000 characters in tests/test_mood.rs

### Medium-Priority Tests - Error Handling

- [ ] T150z [P1] Test database transaction rollback on constraint violation in tests/repository_integration.rs
- [ ] T150aa [P1] Test concurrent activity creation with same name in tests/test_activities.rs
- [ ] T150ab [P1] Test update_activity with all fields set to None in tests/test_activities.rs

### Medium-Priority Tests - Data Consistency

- [ ] T150ac [P1] Test assessment responses maintain timestamp order in tests/test_assessments.rs
- [ ] T150ad [P1] Test mood check-ins maintain timestamp order in tests/test_mood.rs
- [ ] T150ae [P1] Test soft-deleted activity still accessible in historical mood check-ins in tests/test_activities.rs
- [ ] T150af [P1] Test duplicate assessment submission prevention in tests/test_assessments.rs

### Low-Priority Tests - Performance & Concurrency

- [ ] T150ag [P2] Performance test: Chart data aggregation with 1000+ assessments in tests/test_visualization.rs
- [ ] T150ah [P2] Performance test: Mood stats with 1000+ check-ins in tests/test_mood.rs
- [ ] T150ai [P2] Concurrency test: Simultaneous assessment submissions in tests/test_assessments.rs
- [ ] T150aj [P2] Concurrency test: Simultaneous activity updates in tests/test_activities.rs

**Checkpoint**: All P0 tests passing before proceeding to Phase 8

---

## Phase 8: User Story 6 - Configure Assessment Schedules (Priority: P3)

**Goal**: Users can set up recurring assessment reminders with configurable frequency

**Independent Test**: Configure schedule, simulate scheduled time, receive notification, verify schedule persists

### Tests for User Story 6 (TDD - Write First, Verify Fail)

- [X] T151 [P] [US6] Unit test: Time format validation (HH:MM) in src-tauri/src/features/scheduling/models.rs (test_valid_time_format, test_create_schedule_request_validation)
- [X] T152 [P] [US6] Unit test: Next trigger calculation for each frequency type (covered in models.rs tests)
- [X] T153 [P] [US6] Integration test: create_schedule command in src-tauri/tests/test_scheduling.rs (test_create_schedule_* tests)
- [X] T154 [P] [US6] Integration test: get_due_schedules query logic in src-tauri/tests/test_scheduling.rs (32 tests - ALL PASSING ✅)
- [X] T155 [P] [US6] Component test: ScheduleForm and ScheduleList validation (31 tests - ALL PASSING ✅)

### Implementation for User Story 6

- [X] T156 [P] [US6] Create AssessmentSchedule model in src-tauri/src/features/scheduling/models.rs
- [X] T157 [P] [US6] Create ScheduleFrequency enum in src-tauri/src/features/scheduling/models.rs (Daily, Weekly, Biweekly, Monthly)
- [X] T158 [P] [US6] Create CreateScheduleRequest model in src-tauri/src/features/scheduling/models.rs
- [X] T159 [P] [US6] Create UpdateScheduleRequest model in src-tauri/src/features/scheduling/models.rs
- [X] T160 [P] [US6] Implement create_schedule method in repository with validation
- [X] T161 [P] [US6] Implement update_schedule method in repository
- [X] T162 [P] [US6] Implement delete_schedule method in repository
- [X] T163 [P] [US6] Implement get_schedules query in repository
- [X] T164 [P] [US6] Implement get_due_schedules query in repository with time/date logic
- [X] T165 [US6] Implement create_schedule Tauri command in src-tauri/src/features/scheduling/commands.rs
- [X] T166 [US6] Implement update_schedule Tauri command in src-tauri/src/features/scheduling/commands.rs
- [X] T167 [US6] Implement delete_schedule Tauri command in src-tauri/src/features/scheduling/commands.rs
- [X] T168 [US6] Implement get_schedules Tauri command in src-tauri/src/features/scheduling/queries.rs
- [X] T169 [US6] Implement get_schedule Tauri command in src-tauri/src/features/scheduling/queries.rs
- [X] T170 [US6] Register scheduling commands in src-tauri/src/lib.rs invoke_handler
- [X] T171 [US6] Implement background scheduler in src-tauri/src/features/scheduling/scheduler.rs with tokio::spawn and 1-minute interval
- [X] T172 [US6] Implement send_notification function using tauri-plugin-notification in scheduler.rs
- [X] T173 [US6] Start background scheduler on app launch in src-tauri/src/lib.rs
- [X] T174 [US6] Regenerate TypeScript bindings
- [X] T175 [P] [US6] Create ScheduleForm component in src/lib/components/scheduling/ScheduleForm.svelte with frequency-specific fields
- [X] T176 [P] [US6] Create ScheduleList component in src/lib/components/scheduling/ScheduleList.svelte with enable/disable/edit/delete actions
- [X] T177 [US6] Create /settings route in src/routes/settings/+page.svelte with schedules tab
- [X] T178 [US6] Add settings navigation link to sidebar
- [X] T179 [US6] Handle notification click to navigate to assessment page

**Checkpoint**: All 6 user stories complete - Full application functional

---

## Phase 8.5: User Story 7 - Dashboard Assessment Score Overview (Priority: P2)

**Goal**: Users can quickly see their most recent assessment scores on the main dashboard with visual progress bars showing severity ranges

**Independent Test**: Complete assessments of various types, navigate to dashboard, verify scores are displayed with progress bars and severity indicators

### Tests for User Story 7 (TDD - Write First, Verify Fail)

- [ ] T211 [P] [US7] Component test: AssessmentScoreBar renders progress bar with score in src/lib/components/dashboard/AssessmentScoreBar.test.ts
- [ ] T212 [P] [US7] Component test: AssessmentScoreBar displays severity segments with correct colors in src/lib/components/dashboard/AssessmentScoreBar.test.ts
- [ ] T213 [P] [US7] Component test: DashboardScores fetches latest assessments on mount in src/lib/components/dashboard/DashboardScores.test.ts
- [ ] T214 [P] [US7] Component test: DashboardScores shows "Not taken yet" for assessments without data in src/lib/components/dashboard/DashboardScores.test.ts
- [ ] T215 [P] [US7] Component test: Clicking score bar navigates to chart view in src/lib/components/dashboard/DashboardScores.test.ts

### Implementation for User Story 7

**Backend**: No new backend work required - leverages existing `get_latest_assessment` command from User Story 1

**Frontend Components**:

- [ ] T216 [P] [US7] Create AssessmentScoreBar component in src/lib/components/dashboard/AssessmentScoreBar.svelte with progress bar visualization
- [ ] T217 [P] [US7] Implement severity range segments in AssessmentScoreBar using TailwindCSS gradients and markers
- [ ] T218 [P] [US7] Add score value overlay to progress bar in AssessmentScoreBar component
- [ ] T219 [P] [US7] Implement severity color mapping in AssessmentScoreBar (reuse from AssessmentResults.svelte getSeverityColor function)
- [ ] T220 [US7] Create DashboardScores component in src/lib/components/dashboard/DashboardScores.svelte with assessment type iteration
- [ ] T221 [US7] Implement get_latest_assessment calls for all 4 assessment types in DashboardScores using Promise.all for parallel loading
- [ ] T222 [US7] Add loading skeleton states to DashboardScores using existing SkeletonLoader component
- [ ] T223 [US7] Implement "Not taken yet" empty state for each assessment type in DashboardScores
- [ ] T224 [US7] Add click handlers to navigate to chart view (/charts?type=<assessment_type>) for each score bar
- [ ] T225 [US7] Add error handling with user-friendly messages in DashboardScores component

**Integration**:

- [ ] T226 [US7] Update src/routes/+page.svelte to import and render DashboardScores component
- [ ] T227 [US7] Replace or update assessment list section in dashboard with DashboardScores component
- [ ] T228 [US7] Ensure responsive layout for mobile/tablet/desktop viewports (stack vertically on mobile, 2-column on tablet, 4-column on desktop)
- [ ] T229 [US7] Add accessibility labels (ARIA) to progress bars and interactive elements
- [ ] T230 [US7] Test navigation flow: dashboard → click score bar → verify chart page loads with correct assessment type

**Documentation**:

- [ ] T231 [P] [US7] Update README.md to mention dashboard score overview feature
- [ ] T232 [P] [US7] Add dashboard screenshot to docs showing score visualizations

**Checkpoint**: User Story 7 complete - Dashboard shows current assessment scores with visual indicators

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T180 [P] Add comprehensive error boundaries in src/routes/+error.svelte ✅ IMPLEMENTED (handles 404, 500, 503 with user-friendly UI)
- [ ] T181 [P] Implement loading states for all async operations with Svelte transitions
- [X] T182 [P] Add empty state UI for lists (no assessments, no activities, no chart data) ✅ IMPLEMENTED (assessment history, mood history, activities, charts all have empty states)
- [ ] T183 [P] Implement data persistence error handling with retry logic
- [ ] T184 [P] Add keyboard shortcuts for common actions (N for new assessment, M for mood log)
- [ ] T185 [P] Implement dark mode theme toggle using TailwindCSS dark: classes
- [ ] T186 [P] Add accessibility labels (ARIA) to all interactive components
- [ ] T187 [P] Optimize SQLite queries with EXPLAIN ANALYZE and add indexes if needed
- [ ] T188 [P] Add performance monitoring with tracing::instrument on slow operations
- [X] T189 [P] Create user documentation in README.md with screenshots ✅ IMPROVED (corrected architecture from DuckDB→SQLite, fixed activity creation steps, clarified privacy/security)
- [ ] T190 [P] Add data export functionality (optional - CSV export of all data)
- [ ] T191 [P] Implement database vacuum/optimization command for maintenance
- [ ] T192 Run full quickstart.md validation (complete PHQ-9, log mood, view charts) in under 5 minutes
- [X] T193 Run all tests (cargo test && npm run test) and verify 100% pass rate (100 backend + 133 frontend = 233 tests PASSING)
- [ ] T194 Build production bundle (npm run tauri build) and verify app size <150MB
- [ ] T195 Benchmark assessment submission end-to-end (load PHQ-9, answer 9 questions, submit, view result) and verify <2s per plan.md performance goal
- [ ] T196 Benchmark chart rendering for 90-day assessment history (load chart, render, measure time) and verify <500ms per plan.md performance goal
- [ ] T197 Benchmark UI interaction responsiveness (button clicks, form inputs, navigation) across 10 common actions and verify <100ms per plan.md performance goal
- [ ] T198 [P] Implement delete_all_data Tauri command in src-tauri/src/features/admin/commands.rs for GDPR user control (drops all tables, recreates schema)
- [ ] T199 [P] Add data deletion confirmation dialog in src/routes/settings/+page.svelte with "type DELETE to confirm" safety check
- [ ] T200 [P] Add startup check in src-tauri/src/main.rs to verify DuckDB file permissions, log warning if not 0600 (Unix) or equivalent (Windows)
- [ ] T201 [P] Create security checklist in docs/SECURITY.md documenting: file permissions, no network transmission, local-only storage, GDPR deletion procedure, encryption roadmap (v0.2.0)
- [ ] T201a [P] Create DATA_MANAGEMENT.md in src-tauri/docs/ documenting deletion policies, cascading behavior, and data retention for users
- [ ] T201b [P] Integration test: Verify transaction rollback on cascade failure in src-tauri/tests/integration/test_deletion.rs
- [ ] T201c [P] Integration test: Verify deleted activities display correctly with "(deleted)" badge in historical mood check-ins

### CI/CD Pipeline Setup

- [X] T202 [P] Create GitHub Actions workflow file .github/workflows/ci.yml with basic structure (on push/PR to main and feature branches)
- [X] T203 [P] Add lint job to CI workflow: Clippy (cargo clippy -- -D warnings), rustfmt check (cargo fmt -- --check)
- [X] T204 [P] Add backend test job to CI workflow: cargo test with coverage report upload
- [X] T205 [P] Add frontend test job to CI workflow: npm test (Vitest) with coverage report upload
- [X] T206 Add multi-platform build job to CI workflow: Build on ubuntu-latest, macos-latest, windows-latest with proper Tauri dependencies
- [X] T207 [P] Configure Rust dependency caching in CI workflow using actions/cache for ~/.cargo and src-tauri/target
- [X] T208 [P] Add code coverage job to CI workflow using cargo-llvm-cov for backend and Vitest coverage for frontend, upload to Codecov
- [X] T209 [P] Add build artifact upload steps for each platform (.deb/.AppImage for Linux, .dmg/.app for macOS, .msi/.exe for Windows)
- [ ] T210 Test CI pipeline end-to-end: push to feature branch, verify all jobs pass, verify artifacts are uploaded

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phases 3-8)**: All depend on Foundational phase completion
  - User Story 1 (P1): No dependencies on other stories ✅
  - User Story 2 (P2): No dependencies on other stories ✅
  - User Story 3 (P2): No dependencies on other stories ✅ (though US2 uses activities)
  - User Story 4 (P3): Requires US1 data (assessments) to visualize
  - User Story 5 (P3): Requires US2 data (mood check-ins) to visualize
  - User Story 6 (P3): Uses US1 (schedules assessments) but independently testable
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies Graph

```
Phase 2: Foundational
       ↓
    ┌──┴──┬──────┬──────┐
    ↓     ↓      ↓      ↓
  US1   US2    US3    US6
  (P1)  (P2)   (P2)   (P3)
    ↓     ↓
    ↓     ↓
  US4   US5
  (P3)  (P3)
```

**Execution Order**:
1. **Sequential (MVP-first)**: Phase 1 → Phase 2 → US1 → US2 → US3 → US4 → US5 → US6 → Polish
2. **Parallel (if team capacity)**: After Phase 2, US1/US2/US3/US6 can start in parallel, then US4/US5 after data exists

### Within Each User Story

**TDD Workflow (MANDATORY per Constitution Principle IX)**:
1. **Tests First**: Write all test tasks for story
2. **User Approval**: Review tests with user/stakeholder
3. **Verify Fail**: Run tests, confirm they fail (no implementation yet)
4. **Implement**: Execute implementation tasks
5. **Verify Pass**: Run tests, confirm they pass
6. **Checkpoint**: Verify story works independently

**Task Order**:
- Tests (T0XX) → Models (T0XX) → Repository (T0XX) → Commands (T0XX) → Frontend Components (T0XX) → Routes (T0XX) → Integration

### Parallel Opportunities

**Phase 1 (Setup)**: T002, T003, T004, T005, T007 can run in parallel (different files)

**Phase 2 (Foundational)**: T012, T013, T014, T015, T017, T019, T020 can run in parallel after database setup

**Within Each User Story**:
- All test tasks marked [P] can run in parallel
- All model tasks marked [P] can run in parallel
- All repository tasks marked [P] can run in parallel
- All frontend component tasks marked [P] can run in parallel

**Across User Stories** (after Phase 2):
- US1, US2, US3, US6 can be developed in parallel by different team members
- US4 waits for US1 completion (needs assessment data)
- US5 waits for US2 completion (needs mood data)

---

## Parallel Example: User Story 1

```bash
# Phase 1: Launch all tests together (TDD - write first):
Task: "Unit test: PHQ-9 scoring algorithm in src-tauri/src/features/assessments/models.rs"
Task: "Unit test: GAD-7 scoring algorithm in src-tauri/src/features/assessments/models.rs"
Task: "Unit test: CES-D scoring algorithm in src-tauri/src/features/assessments/models.rs"
Task: "Unit test: OASIS scoring algorithm in src-tauri/src/features/assessments/models.rs"
Task: "Unit test: Severity level calculation in src-tauri/src/features/assessments/models.rs"
Task: "Component test: AssessmentForm in tests/unit/AssessmentForm.test.ts"

# Phase 2: Verify tests fail, then launch all models together:
Task: "Create AssessmentType model in src-tauri/src/features/assessments/models.rs"
Task: "Create AssessmentResponse model in src-tauri/src/features/assessments/models.rs"
Task: "Create AssessmentQuestion model in src-tauri/src/features/assessments/models.rs"

# Phase 3: Launch question definitions together:
Task: "Define PHQ-9 questions in src-tauri/src/features/assessments/content.rs"
Task: "Define GAD-7 questions in src-tauri/src/features/assessments/content.rs"
Task: "Define CES-D questions in src-tauri/src/features/assessments/content.rs"
Task: "Define OASIS questions in src-tauri/src/features/assessments/content.rs"

# Phase 4: Launch repository methods together:
Task: "Create AssessmentRepository in src-tauri/src/features/assessments/repository.rs"
Task: "Implement get_assessment_types query in repository"
Task: "Implement get_assessment_history query in repository"
Task: "Implement get_assessment_response query in repository"

# Phase 5: Launch frontend components together:
Task: "Create AssessmentList component in src/lib/components/assessments/AssessmentList.svelte"
Task: "Create AssessmentForm component in src/lib/components/assessments/AssessmentForm.svelte"
Task: "Create AssessmentResult component in src/lib/components/assessments/AssessmentResult.svelte"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

**Goal**: Ship a working PHQ-9/GAD-7/CES-D/OASIS assessment tool in 2-3 days

1. Complete Phase 1: Setup (~2 hours)
2. Complete Phase 2: Foundational (~4 hours) - CRITICAL PATH
3. Complete Phase 3: User Story 1 (~1-2 days with TDD)
4. **STOP and VALIDATE**:
   - Run all US1 tests (should pass 100%)
   - Complete PHQ-9 assessment end-to-end
   - Verify score calculation matches published guidelines
   - Test on Linux/macOS/Windows
5. **Deploy/Demo MVP** - working clinical assessment tool!

**Deliverables**: Users can take validated mental health assessments and see their scores

### Incremental Delivery

**Week 1**: MVP (US1) → Assessment tool
**Week 2**: Add US2 + US3 → Mood tracking with activities
**Week 3**: Add US4 + US5 → Data visualization
**Week 4**: Add US6 → Scheduling + reminders
**Week 5**: Polish → Production-ready v0.1.0

Each increment:
- Adds user value
- Is independently testable
- Doesn't break previous features
- Can be demoed/deployed

### Parallel Team Strategy

**With 3 developers**:

**Phase 1-2** (All together):
- Dev A: Database setup (T008-T012)
- Dev B: Error handling + config (T013-T015)
- Dev C: Frontend layout + UI components (T019-T020)

**Phase 3+** (Parallel):
- Dev A: User Story 1 (Assessments) - Priority P1 🎯
- Dev B: User Story 2 + 3 (Mood + Activities) - Priority P2
- Dev C: User Story 4 + 5 (Visualization) - Priority P3

**Integration**: Each dev's story is independently testable, minimal merge conflicts due to vertical slice architecture

---

## Notes

- **[P] tasks** = Can run in parallel (different files, no dependencies)
- **[Story] labels** = Map task to specific user story for traceability
- **TDD MANDATORY**: Constitution Principle IX requires tests written first
- **Test Order**: Write → User approve → Fail → Implement → Pass → Checkpoint
- Each user story should be independently completable and testable
- Commit after each task or logical group of [P] tasks
- Stop at any checkpoint to validate story independently
- Constitution Principle I: Ship US1 first, get feedback, iterate to US2/US3/etc.
- Constitution Principle III: Make it work (MVP) → Make it right (refactor) → Make it fast (optimize)

---

## Task Summary

**Total Tasks**: 250 (updated from 228 after dashboard score visualization addition)
- **Phase 1 (Setup)**: 7 tasks
- **Phase 2 (Foundational)**: 16 tasks (BLOCKS all stories) - Added T020a/T020b for defensive deletion infrastructure
- **Phase 3 (US1 - Assessments)**: 51 tasks (11 tests + 40 implementation) - Added T067a-T067d for assessment type deletion
- **Phase 4 (US2 - Mood Check-In)**: 29 tasks (6 tests + 23 implementation) - Added T093a-T093c for mood checkin cascade deletion
- **Phase 5 (US3 - Activities)**: 27 tasks (6 tests + 21 implementation) - Added T097b for deleted activity display test, T101a/T101b for Lineicons v5 icon picker
- **Phase 6 (US4 - Assessment Charts)**: 20 tasks (4 tests + 16 implementation) - Added T132b for empty chart state
- **Phase 7 (US5 - Mood Charts)**: 16 tasks (3 tests + 13 implementation) - Added T147b for empty mood chart state
- **Phase 8 (US6 - Scheduling)**: 28 tasks (5 tests + 23 implementation)
- **Phase 8.5 (US7 - Dashboard Scores)**: 22 tasks (5 tests + 17 implementation) - NEW: Dashboard assessment score visualization with progress bars
- **Phase 9 (Polish)**: 35 tasks - Added T201a-T201c for deletion documentation and testing

**Parallel Opportunities**: 120 tasks marked [P] (48% can run in parallel) - Added dashboard component parallel tests

**Independent Test Checkpoints**: 7 (one per user story)

**MVP Scope**: Phases 1-3 only (68 tasks) - Shippable clinical assessment tool

**TDD Coverage**: 37 test tasks across all stories (15% of total tasks are tests)

**Remediation Applied**: 11 tasks added to address security (GDPR/file permissions), performance validation, and edge cases; 9 tasks added for CI/CD pipeline setup; 22 tasks added for dashboard score visualization

**Estimated Timeline**:
- MVP (US1): 2-3 days
- MVP + US2 + US3: 1 week
- Full feature set (US1-7): 3-4 weeks
- Production-ready: 4-5 weeks with polish

**Constitution Alignment**:
- ✅ Principle I (Ship It): MVP defined, incremental delivery path
- ✅ Principle III (Work→Right→Fast): US1 first, then iterate
- ✅ Principle VI (README-Driven): quickstart.md validation in polish phase
- ✅ Principle IX (Test-First): TDD workflow enforced, tests written before implementation
