# Revised Development Plan 2025 - Task Checklist

**Created**: 2025-11-07
**Updated**: 2025-11-10
**Total Releases**: 5 (v0.1, v0.2, v0.3, v0.4, v1.0)
**Estimated Duration**: 10 weeks
**Approach**: Ship value every 1-2 weeks, tests integrated with features

**See**: REVISED-plan.md for full details

---

## ✅ Already Complete (No Work Needed)

The following were in the original plan but are already 100% complete:

- ✅ Week 0: Validation Sprint (Chart.js, Tailwind v4, Heroicons) - ALL DONE
- ✅ Dashboard/Visualization (routes/charts with Chart.js) - ALL DONE
- ✅ Individual Activities CRUD - ALL DONE
- ✅ Mood Check-In (1-5 scale) - ALL DONE
- ✅ All 4 Assessment Types (PHQ-9, GAD-7, CES-D, OASIS) - ALL DONE
- ✅ Scheduling System - ALL DONE
- ✅ Tailwind v4.1.17 - ALL DONE
- ✅ Heroicons v5.2.0 - ALL DONE
- ✅ Modern UI Components (Button, Card, Input, etc.) - ALL DONE

---

## Phase 1 (v0.1): Activity Groups (Weeks 1-4) → SHIP IT

**Goal**: Users can organize activities into groups, set goals, and track progress

**Duration**: 4 weeks (90-100 hours)

### Week 1: Database Schema and Repository Layer (22-25 hours)

**📖 Architecture Reference**: See `data-structures.md` for TypeScript type specifications:
- `ActivityGroupFormData`, `ActivityFormData`, `ActivityGoalFormData`
- `FormState<T>`, `AsyncData<T>`, `ModalState<T>`
- `ActivityGroupUI`, `ActivityUI` (with UI metadata)

#### Database Migration (6-8 hours)

- [X] 1.1 Create migration file `003_activity_groups.sql`
- [X] 1.2 Define `activity_groups` table with schema (id, name CHECK(length(name) <= 100), description, created_at, deleted_at)
- [X] 1.3 Modify `activities` table: Add `group_id INTEGER NOT NULL` FK column (mandatory relationship)
- [X] 1.4 Add FK constraint: `FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE CASCADE`
  - **Behavior**: Deleting a group CASCADE deletes all its activities
  - **Mandatory**: Activities MUST belong to a group (NOT NULL constraint)
  - **UI Warning**: "Deleting this group will permanently delete all X activities in it. This cannot be undone."
- [X] 1.5 Create `activity_logs` table with CASCADE delete:
  ```sql
  CREATE TABLE activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER NOT NULL,
    logged_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    notes TEXT CHECK (length(notes) <= 500 OR notes IS NULL),
    deleted_at TEXT,
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
  );
  ```
- [X] 1.5a Create indexes: `idx_activity_logs_activity`, `idx_activity_logs_logged_at`, `idx_activity_logs_deleted`
- [X] 1.6 Create `activity_goals` table (id, activity_id?, group_id?, goal_type, target_value, period_days)
- [X] 1.7 Add CHECK constraints:
  - `goal_type IN ('days_per_period', 'percent_improvement')`
  - `NOT (activity_id IS NOT NULL AND group_id IS NOT NULL)` -- allows NULL for both
- [X] 1.8 Add partial indexes for soft deletes (`WHERE deleted_at IS NULL`)
- [X] 1.9 Test migration runs successfully
- [X] 1.10 Create rollback script (`DROP TABLE` in reverse order)

#### Repository: Activity Groups (8-10 hours)

- [X] 1.11 Create `features/activities/models.rs` - Add ActivityGroup struct
- [X] 1.12 Write test for `create_activity_group` (TDD: red)
- [X] 1.13 Implement `create_activity_group` method (TDD: green)
- [X] 1.14 Write test for `update_activity_group` (name/description)
- [X] 1.15 Implement `update_activity_group` method
- [X] 1.16 Write test for `delete_activity_group` (soft delete: set deleted_at)
- [X] 1.17 Implement `delete_activity_group` method
- [X] 1.18 Write test for `get_activity_groups` (exclude deleted_at IS NOT NULL)
- [X] 1.19 Implement `get_activity_groups` method
- [X] 1.20 Write test for cascading deletes (delete group → verify activities are CASCADE deleted)
@dev/active/major-refactoring-2025/major-refactoring-2025-
#### Repository: Activity Logs (4-5 hours)

- [X] 1.21 Add ActivityLog struct to models.rs
- [X] 1.22 Write test for `log_activity` (insert into activity_logs)
- [X] 1.23 Implement `log_activity` method
- [X] 1.24 Write test for `get_activity_logs` with date filtering
- [X] 1.25 Implement `get_activity_logs` method (order by logged_at DESC)

#### Repository: Update Existing Activity Methods (4-5 hours)

- [X] 1.26 Write test for `create_activity` with required group_id (NOT NULL)
- [X] 1.27 Update `create_activity` to require group_id parameter (validate NOT NULL)
- [X] 1.27a Update activity name validation to 50 chars max, add icon validation (20 chars max)
- [X] 1.28 Write test for `get_activities_by_group` (filter by group_id)
- [X] 1.29 Implement `get_activities_by_group` method

**Week 1 Total**: ~24 tasks, ~22-25 hours

---

### Week 2: Activity Goals and Reporting Logic (24-28 hours)

**📖 Architecture Reference**: See `algorithms.md` for calculation specifications:
- Goal progress calculation (days_per_period and percent_improvement types)
- Activity frequency calculation (unique days, days per week)
- Activity trend analysis (percent change from previous period)

#### Repository: Activity Goals (10-12 hours)

- [X] 2.1 Add ActivityGoal struct to models.rs (id, activity_id?, group_id?, goal_type, target_value, period_days, created_at, deleted_at)
- [X] 2.2 Write test for `set_activity_goal` (activity-level goal)
- [X] 2.3 Write test for `set_activity_goal` (group-level goal)
- [X] 2.4 Implement `set_activity_goal` method with CHECK constraint validation
- [X] 2.5 Write test for `get_activity_goals` (filter by activity_id OR group_id)
- [X] 2.6 Implement `get_activity_goals` method
- [X] 2.7 Write test for `update_activity_goal` (change target_value or period_days)
- [X] 2.8 Implement `update_activity_goal` method
- [X] 2.9 Write test for `delete_activity_goal` (soft delete)
- [X] 2.10 Implement `delete_activity_goal` method

#### Repository: Reporting Queries (8-10 hours)

- [X] 2.11 Add GoalProgress struct (current_value, target_value, percentage, is_achieved)
- [X] 2.12 Write test for `get_activity_frequency` (count logs in date range → days/week)
- [X] 2.13 Implement `get_activity_frequency` (SQL: COUNT DISTINCT DATE(logged_at) / 7)
- [X] 2.14 Write test for `get_activity_trend` (compare current period vs previous period → % change)
- [X] 2.15 Implement `get_activity_trend` (SQL: period comparison with CASE statements)
- [X] 2.16 Write test for `check_goal_progress` (actual vs target, calculate percentage)
- [X] 2.17 Implement `check_goal_progress` method
- [X] 2.17a Implement goal achievement notification (when progress.is_achieved, send notification using tauri-plugin-notification)

#### Tauri Commands (6-8 hours)

- [X] 2.18 Create `features/activities/commands.rs` (if not exists)
- [X] 2.19 Implement `create_activity_group` command (returns Result<ActivityGroup, CommandError>)
- [X] 2.20 Implement `update_activity_group` command
- [X] 2.21 Implement `delete_activity_group` command
- [X] 2.22 Implement `get_activity_groups` query
- [X] 2.23 Update `create_activity` command to accept optional group_id
- [X] 2.24 Implement `log_activity` command (creates ActivityLog)
- [X] 2.25 Implement `get_activity_logs` query
- [X] 2.26 Implement `set_activity_goal` command
- [X] 2.27 Implement `get_activity_goals` query
- [X] 2.28 Implement `get_activity_frequency` query
- [X] 2.29 Implement `get_activity_trend` query
- [X] 2.30 Implement `check_goal_progress` query
- [X] 2.31 Generate TypeScript bindings with `cargo test` (tauri-specta)
- [ ] 2.32 Write command tests for error handling (invalid IDs, missing FKs)

**Week 2 Total**: ~32 tasks, ~24-28 hours

---

### Week 3: Frontend UI Components (22-26 hours)

**📖 Architecture References**:
- `svelte5-architecture.md` - State management (runes for local state, stores for global)
- `component-architecture.md` - Testing conventions, composition patterns (slots vs snippets), accessibility
- `data-structures.md` - FormState, ModalState, AsyncData types

#### Activity Group Management Page (8-10 hours)

- [X] 3.1 Create `/routes/activity-groups/+page.svelte`
- [X] 3.2 Create `ActivityGroupList.svelte` component (display all groups)
- [X] 3.3 Add expand/collapse functionality for each group (show activities when expanded)
- [X] 3.4 Create `ActivityGroupForm.svelte` modal component (create/edit group)
- [X] 3.5 Implement "Add Group" button → open modal
- [X] 3.6 Implement "Edit Group" button → open modal with existing data
- [X] 3.7 Implement delete group with confirmation dialog
  - **Warning Message**: "Deleting this group will permanently delete all X activities in it. This cannot be undone."
  - **Count Activities**: Query activities by group_id to show count in warning
  - **Cascade Behavior**: Backend CASCADE deletes activities when group is deleted

#### Update Activity Management UI (6-8 hours)

- [X] 3.8 Update `/routes/mood/activities/+page.svelte` to show grouped view
- [X] 3.9 Update `ActivityForm.svelte` to include required group selector (dropdown of all groups)
- [X] 3.10 Create `IconPicker.svelte` component with Heroicon names validation (e.g., "academic-cap", "heart")
- [X] 3.10a Validate icon names against Heroicons list in frontend
- [X] 3.11 Update `ActivityList.svelte` to display activities grouped by Activity Group
- [X] 3.12 Add "Move to Group" action for existing activities
  - **UI**: Dropdown in activity editor to select different group
  - **Backend**: Just update `group_id` column (simple UPDATE query)
  - **Options**: List all groups (required - cannot be null)

#### Activity Logging Interface (4-6 hours)

- [X] 3.14 Create `ActivityLogButton.svelte` (quick log button for each activity)
- [X] 3.15 Create `ActivityLogHistory.svelte` (timeline view with notes)
- [X] 3.16 Add date filtering for log history (date range picker)
- [X] 3.17 Implement "Add Note" feature for logs (textarea with 500 char limit)

#### Goal Setting UI (4-6 hours)

- [X] 3.18 Create `GoalSettingModal.svelte` component
- [X] 3.19 Implement goal type selector (radio buttons: "Days per period" / "Percent improvement")
- [X] 3.20 Create target value input with validation (positive integers only)
- [X] 3.21 Create period selector (dropdown: 7 days, 14 days, 30 days, custom)
- [X] 3.22 Create `GoalProgressIndicator.svelte` (progress bar with percentage)
- [X] 3.23 Display active goals for each activity/group in ActivityGroupList
- [X] 3.23a Wire up goal achievement notification (show toast when goal achieved, option to view details)

**Week 3 Total**: ~24 tasks, ~22-26 hours

---

### Week 4: Reporting Dashboard and Integration (22-26 hours)

**📖 Architecture Reference**: See `algorithms.md` for:
- Chart data transformation (raw data → Chart.js format)
- Report aggregation (weekly/monthly summaries)
- All calculation algorithms referenced in Week 2

#### Reporting Components (8-10 hours)

- [ ] 4.1 Create `ActivityReportCard.svelte` (days/week display with bar chart)
- [ ] 4.2 Create `ActivityTrendChart.svelte` (% change visualization with arrow indicators)
- [ ] 4.3 Create `GoalProgressDashboard.svelte` (all active goals with progress bars)
- [ ] 4.4 Integrate reporting into existing `/charts` route (add "Activities" tab)

#### Integration Testing (8-10 hours)

- [ ] 4.5 Test end-to-end: Create group → Add activities → Assign group → View grouped list
- [ ] 4.6 Test activity logging: Log activity → View log history → Add note
- [ ] 4.7 Test goal setting: Set goal → Log activities → View progress → Achieve goal
- [ ] 4.8 Test reporting: View days/week → View % change → View goal progress
- [ ] 4.9 Test activity deletion (soft delete preserves logs and goals)
- [ ] 4.10 Test group deletion CASCADE behavior:
  - Create group with 3 activities
  - Delete group
  - Verify all 3 activities are CASCADE deleted from database
  - Verify warning message showed correct count before deletion
- [ ] 4.11 Test "Move to Group" functionality (update activity.group_id)
- [ ] 4.12 Write component tests for ActivityGroupList
- [ ] 4.13 Write component tests for GoalSettingModal
- [ ] 4.14 Write component tests for ActivityLogButton

#### Performance Testing (3-4 hours)

- [ ] 4.15 Test with 50 activity groups
- [ ] 4.16 Test with 500 activity logs
- [ ] 4.17 Test reporting queries with large datasets (>1000 logs)
- [ ] 4.18 Optimize queries if needed (add indexes, rewrite SQL)
- [ ] 4.19 Verify performance target (<200ms for activity list)

#### Documentation (3-4 hours)

- [ ] 4.20 Update README.md with Activity Groups feature description
- [ ] 4.21 Write user guide for Activity Groups (how to create, log, set goals, move between groups)
- [ ] 4.22 Document database schema changes in CLAUDE.md (CASCADE delete behavior)
- [ ] 4.23 Add screenshots to documentation

**Week 4 Total**: ~22 tasks, ~22-26 hours

**v0.1 Shippable Criteria**:

- ✅ Users can create activity groups and activities
- ✅ Users can log activities with notes
- ✅ Users can set goals (days/period or % improvement)
- ✅ Reporting shows days/week and % change
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Activity list loads <200ms
- 🚀 **SHIP v0.1 to users**

---

## 💡 Future Enhancements (Nice to Have)

**Note**: These are enhancement ideas for future iterations, not required for v0.1 release.

### Activity Logging Enhancements
- [ ] FE-1 Add optimistic UI updates for quick log button (show in list immediately before backend confirms)
- [ ] FE-2 Add bulk activity logging interface ("I did these 5 activities today")
- [ ] FE-3 Add export functionality for activity logs (CSV/JSON download)
- [ ] FE-4 Add activity log editing (change timestamp, update notes)
- [ ] FE-5 Add activity log search/filter (by date range, activity, notes content)

### Goal Management Enhancements
- [ ] FE-6 Add goal deletion functionality (currently can only create/update)
- [ ] FE-7 Add goal history/archive (track completed goals over time)
- [ ] FE-8 Add goal templates (pre-defined common goals like "Exercise 3x/week")
- [ ] FE-9 Add goal streaks visualization ("7 day streak!")
- [ ] FE-10 Add goal reminders/notifications (when falling behind)

### Reporting Enhancements
- [ ] FE-11 Add activity heatmap calendar view (GitHub-style contribution graph)
- [ ] FE-12 Add comparative reporting (this month vs last month)
- [ ] FE-13 Add custom report builder (user-defined date ranges, metrics)
- [ ] FE-14 Add PDF export for reports
- [ ] FE-15 Add data insights/recommendations ("You tend to exercise more on weekends")

### Activity Group Enhancements
- [ ] FE-16 Add group color theming (each group has custom color scheme)
- [ ] FE-17 Add group sorting/reordering (drag-and-drop)
- [ ] FE-18 Add group archiving (hide inactive groups without deleting)
- [ ] FE-19 Add group templates (pre-configured activity sets)
- [ ] FE-20 Add group-level statistics dashboard

**Priority for Next Phase**: Focus on FE-1, FE-2, FE-6, FE-7 (most requested features)

---

## Phase 2 (v0.2): Check-In v2.0 (Week 5) → SHIP IT

**Goal**: Users get improved check-in experience with 7-point mood scale and Activity Group integration

**Duration**: 1 week (22-26 hours)

### Week 5: Check-In v2.0 Implementation

**📖 Architecture References**:
- `svelte5-architecture.md` - State management (runes for component state)
- `component-architecture.md` - Component patterns, accessibility (keyboard navigation, ARIA labels)

#### Database Migration (1-2 hours)
**Note**: No data migration needed (no users exist yet)

- [ ] 5.1 Create migration file `004_mood_scale_1_to_7.sql`
- [ ] 5.2 Update CHECK constraint only:
  ```sql
  ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
  ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 7);
  ```
- [ ] 5.3 Test migration runs successfully

#### Backend Updates (4-6 hours)

- [ ] 5.4 Update mood rating validation to 1-7 range in repository
- [ ] 5.5 Update `MoodCheckin` struct documentation (1=Terrible ... 7=Excellent)
- [ ] 5.6 Update `log_mood` command validation
- [ ] 5.7 Generate new TypeScript bindings (`cargo test`)
- [ ] 5.8 Update existing mood tests for 1-7 scale
- [ ] 5.9 Write new tests for 7-point scale edge cases (boundary values: 0, 1, 7, 8)

#### Frontend: Mood Selector Update (6-8 hours)

- [ ] 5.10 Update `MoodScaleInput.svelte` to 7-point scale
- [ ] 5.11 Design visual representation (7 emoji buttons: 😢 😞 🙁 😐 🙂 😊 😄)
- [ ] 5.12 Add accessibility labels for each level:
  - 1 = "Terrible"
  - 2 = "Very Bad"
  - 3 = "Bad"
  - 4 = "Neutral"
  - 5 = "Good"
  - 6 = "Very Good"
  - 7 = "Excellent"
- [ ] 5.13 Implement keyboard navigation (Left/Right arrows to change selection)
- [ ] 5.14 Update mood history display to show 7-point scale (change emoji set)

#### Frontend: Activity Integration (6-8 hours)

- [ ] 5.15 Update `ActivitySelector.svelte` to group by Activity Group
- [ ] 5.16 Implement collapsible sections for each group (click to expand/collapse)
- [ ] 5.17 Display icons instead of names (use Heroicons from icon field)
- [ ] 5.18 Add "Ungrouped Activities" section at bottom (for activities with group_id IS NULL)
- [ ] 5.19 Test multi-select with 0 activities, 1 activity, many activities across groups
- [ ] 5.20 Update check-in history (`MoodHistoryList.svelte`) to show grouped activities with icons

#### Integration Testing (3-4 hours)

- [ ] 5.21 Test end-to-end check-in flow: Select 7-point mood → Select grouped activities → Submit
- [ ] 5.22 Test activity selection with all combinations (0, 1, many activities; grouped/ungrouped)
- [ ] 5.23 Write component tests for updated `MoodScaleInput` (7 levels)
- [ ] 5.24 Write component tests for updated `ActivitySelector` (grouped view)

**Week 5 Total**: ~24 tasks, ~18-22 hours

**v0.2 Shippable Criteria**:

- ✅ Users can rate mood on 7-point scale
- ✅ Users select activities grouped by category
- ✅ Icons display correctly for all activities
- ✅ Multi-select works smoothly
- ✅ Old check-ins still viewable (backward compatible)
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Check-in response <100ms
- 🚀 **SHIP v0.2 to users**

---

## Phase 3 (v0.3): Spec Gap Completion (Week 6) → SHIP IT

**Goal**: Complete missing features from original spec

**Duration**: 1 week (24-30 hours)

### Week 6: Spec Gap Implementation

#### FR-009a: Draft Assessments (8-10 hours)

- [ ] 6.1 Add `status` column to `assessment_responses` table (TEXT CHECK IN ('draft', 'completed'), DEFAULT 'completed')
- [ ] 6.2 Add migration to create column with default value
- [ ] 6.3 Update `submit_assessment` repository method to accept status parameter
- [ ] 6.4 Create `save_draft_assessment` Tauri command
- [ ] 6.5 Create `get_draft_assessments` Tauri query
- [ ] 6.6 Update frontend `AssessmentForm.svelte` to add "Save Draft" button (secondary button)
- [ ] 6.7 Create draft assessment list UI (`/assessments/drafts` route)
- [ ] 6.8 Implement "Resume Draft" functionality (load saved responses into form)
- [ ] 6.9 Write repository tests for draft functionality
- [ ] 6.10 Write command tests for save_draft_assessment

#### FR-009b: Daily Assessment Limit (6-8 hours)

- [ ] 6.11 Add validation function `check_assessment_already_completed_today` to repository
- [ ] 6.12 Update `submit_assessment` command to check for existing assessment today (same type_code, DATE(submitted_at) = today)
- [ ] 6.13 Return user-friendly errdragor: "You have already completed PHQ-9 today. Please try again tomorrow."
- [ ] 6.14 Update UI to show "Already completed today" state (disable submit button with tooltip)
- [ ] 6.15 Add "View Today's Result" link to redirect to existing result
- [ ] 6.16 Write tests for daily limit validation (allow first, reject second on same day, allow next day)

#### FR-009c/FR-015a: Backdating (6-8 hours)

- [ ] 6.17 Add optional `backdated_to` column to `assessment_responses` table (TEXT, nullable)
- [ ] 6.18 Add optional `backdated_to` column to `mood_checkins` table (TEXT, nullable)
- [ ] 6.19 Add validation: Allow backdating within 24 hours (reject if backdated_to < now - 24 hours)
- [ ] 6.20 Update frontend forms with optional date picker (default = now, max = now, min = now - 24h)
- [ ] 6.21 Add "Log for Yesterday" quick action button (sets backdated_to to yesterday's date)
- [ ] 6.22 Write tests for backdating validation:
  - Allow backdating within 24h
  - Reject backdating >24h
  - Reject future dates
- [ ] 6.23 Update UI to show backdated entries with indicator ("Logged for yesterday")

#### FR-020a: Activity Name Validation (2-4 hours)

- [ ] 6.24 Update activity name validation to max 50 chars (change from 100)
- [ ] 6.25 Add validation to disallow < > & " characters (regex: `^[^<>&"]+$`)
- [ ] 6.26 Update error messages: "Activity name must be 1-50 characters and cannot contain < > & \""
- [ ] 6.27 Write tests for validation rules:
  - Accept valid names (1-50 chars, no forbidden chars)
  - Reject >50 chars
  - Reject names with < > & "
  - Reject empty names

**Week 6 Total**: ~27 tasks, ~24-30 hours

**v0.3 Shippable Criteria**:

- ✅ Users can save draft assessments
- ✅ System prevents duplicate assessments per day
- ✅ Users can backdate entries (within 24h)
- ✅ Activity names validated per spec
- ✅ Tests pass, no P0 bugs
- 🚀 **SHIP v0.3 to users**

---

## Phase 4 (v0.4): Catalyst UI Refresh (Weeks 7-8) → SHIP IT

**Goal**: Users see modern, professional UI across all pages

**Duration**: 2 weeks (28-36 hours)

**Note**: Tailwind v4 already installed. This phase extracts Catalyst design patterns.

**📖 Architecture References**:
- `tailwind4-design-system.md` - Design tokens, CSS specifications, component implementations
- `catalyst-integration-spec.md` - Component priority matrix, React→Svelte translation guide, Melt UI integration strategy, migration checklist
- `component-architecture.md` - Testing conventions, accessibility patterns, composition examples
- `data-structures.md` - TypeScript type specifications (FormState, AsyncData, component props)

### Week 7: Design Tokens and Core Components (14-18 hours)

#### Foundation (4-6 hours)

**Reference**: `tailwind4-design-system.md` § Design Tokens (file organization in separate token files)

- [ ] 7.1 Create `src/styles/tokens/colors.css` with Catalyst zinc palette + accent colors:
  - Full zinc scale (50-950) - see spec for exact values
  - Blue, red, green palettes for actions
  - Keep existing mood/assessment colors
  - Semantic color aliases (--color-primary, --color-danger)

- [ ] 7.2 Create `src/styles/tokens/typography.css` with Catalyst text/line-height combinations:
  - Add utilities: `text-xs/4`, `text-sm/6`, `text-base/6`, `text-lg/7`, `text-xl/8`, etc.
  - See spec for complete scale and usage guidelines

- [ ] 7.3 Create `src/styles/tokens/spacing.css` with fractional spacing values:
  - Add `--spacing-3_5: 0.875rem`, `--spacing-4_5: 1.125rem`
  - Border radius tokens (Catalyst exact values)

- [ ] 7.4 Create `src/styles/tokens/shadows.css` with Catalyst shadow depth system
  - Light and dark mode shadow variants
  - See spec for exact shadow values

- [ ] 7.5 Update `src/app.css` to import all token files and add dark mode CSS variables
  - See spec § Dark Mode Implementation for variable definitions

- [ ] 7.6 Test dark mode compatibility with new tokens (verify all pages render correctly)

#### Enhanced Components (10-12 hours)

**Reference**: `tailwind4-design-system.md` § Component Specifications

- [ ] 7.7 Create `src/styles/components/button.css` and `lib/components/ui/Button.svelte`:
  - **Use `@layer components`** for reusable CSS classes (`.btn`, `.btn-solid-zinc`, etc.)
  - Implement ALL variants × colors: solid/outline/plain × zinc/blue/red/green (12 total)
  - See spec for complete button implementation with all states
  - Follow `svelte5-architecture.md` for component pattern (presentation component with `$props()`)

- [ ] 7.8 Create `src/styles/components/input.css` and `lib/components/ui/Input.svelte`:
  - **Use `@layer components`** for `.input`, `.input-error`, `.input-label` classes
  - Implement all states: default, hover, focus, error, disabled
  - See spec § Input Component for complete implementation
  - Include helper text and error message support

- [ ] 7.9 Create `src/styles/components/card.css` and `lib/components/ui/Card.svelte`:
  - Implement card base, header, title, description, footer classes
  - See spec § Card Component for structure

- [ ] 7.10 Create `src/styles/components/badge.css` and `lib/components/ui/Badge.svelte`:
  - All color variants (zinc, blue, green, red, yellow)
  - See spec § Badge Component

- [ ] 7.11 Create `src/styles/components/alert.css` and `lib/components/ui/Alert.svelte`:
  - All types: info, success, warning, error
  - See spec § Alert Component

- [ ] 7.12 Create `src/styles/components/modal.css` (headless UI pattern - overlay, panel, header, body, footer)

- [ ] 7.13 Write component tests for all components (Button, Input, Card, Badge, Alert)

**Week 7 Total**: ~13 tasks, ~14-18 hours

---

### Week 8: Component Migration and Polish (14-18 hours)

#### Component Migration (8-10 hours)

**Reference**: `tailwind4-design-system.md` § Component Architecture (follow presentation vs container pattern)

- [ ] 8.1 Migrate assessment forms to new Input components (`/assessments/[type]/+page.svelte`)
  - Use presentation `Input` component with `$props()` pattern
  - Container components manage state with `$state()`

- [ ] 8.2 Migrate all buttons to new Button component (search for `<button>` tags, replace with `<Button>`)
  - Use variant/color props from Button spec

- [ ] 8.3 Migrate dashboard to new Card components (`/routes/+page.svelte`, `/routes/charts/+page.svelte`)
  - Use Card component with header/body/footer slots

- [ ] 8.4 Migrate activity management UI to new components (`/routes/mood/activities/+page.svelte`)
  - Use Badge for activity tags
  - Use Alert for validation messages

- [ ] 8.5 Migrate activity group management to new components (`/routes/activity-groups/+page.svelte`)
  - Use Modal component for create/edit dialogs

- [ ] 8.6 Update navigation with new Button styles (`lib/components/layout/Navigation.svelte`)

- [ ] 8.7 Update icon sizing to Catalyst conventions (use `size-4`, `size-5`, `size-6` classes)
  - See spec § Icon System for size guidelines

- [ ] 8.8 Test all pages for visual consistency (verify no regressions)

#### Accessibility and Performance (6-8 hours)

- [ ] 8.8 Run accessibility audit (Axe DevTools or Lighthouse)
- [ ] 8.9 Fix keyboard navigation issues found (Tab, Enter, Escape, Arrow keys)
- [ ] 8.10 Test responsive design on mobile (375px), tablet (768px), desktop (1440px)
- [ ] 8.11 Run Lighthouse performance audit (target >90 score)
- [ ] 8.12 Fix performance issues:
  - Optimize images (compress, lazy load)
  - Reduce JavaScript bundle size (check imports)
  - Minimize CSS (remove unused styles)
- [ ] 8.13 Verify WCAG compliance (color contrast, focus indicators, alt text)

**Week 8 Total**: ~13 tasks, ~14-18 hours

**v0.4 Shippable Criteria**:

- ✅ All components use Catalyst-inspired design
- ✅ Button, Input, Select, Card components enhanced
- ✅ Dark mode functional
- ✅ Accessibility score >90 (Lighthouse)
- ✅ Tests pass, visual consistency verified
- ✅ Performance: No regressions from UI changes
- 🚀 **SHIP v0.4 to users**

---

## Phase 5 (v1.0): Test Coverage Expansion (Weeks 9-10) → SHIP IT

**Goal**: Reach 80%+ test coverage, production-ready quality

**Duration**: 2 weeks (36-44 hours)

### Week 9: Backend Test Expansion (18-22 hours)

#### Repository Tests (10-12 hours)

- [ ] 9.1 Add edge case tests for Activity Groups:
  - Empty groups (group with no activities)
  - Cascading deletes (delete group → verify activities.group_id SET NULL)
  - Soft delete (verify deleted_at set, not shown in queries)
- [ ] 9.2 Add edge case tests for Activity Goals:
  - Invalid targets (negative numbers, zero)
  - Expired goals (period_days in the past)
  - Multiple active goals (same activity, different periods)
- [ ] 9.3 Add edge case tests for Activity Logging:
  - Duplicate logs (same activity, same day)
  - Invalid dates (future dates, >1 year in past)
  - Long notes (max 500 chars)
- [ ] 9.4 Add edge case tests for 7-point mood scale:
  - Boundary values (0, 1, 7, 8)
  - Invalid types (strings, floats)
- [ ] 9.5 Add concurrency tests:
  - Simultaneous activity logs (same activity, different users - future-proofed)
  - Simultaneous goal updates (same goal, different values)

#### Command Validation Tests (8-10 hours)

- [ ] 9.6 Test notes length validation (max 5000 chars for assessments, 500 for activities)
- [ ] 9.7 Test control character filtering (reject \x00-\x1F except \n \r \t)
- [ ] 9.8 Test type code format validation (uppercase letters, dashes, max 20 chars)
- [ ] 9.9 Test boundary values:
  - Exact threshold scores (PHQ-9: 4, 9, 14, 19)
  - Max string lengths (activity name 30, group name 100)
- [ ] 9.10 Test error handling:
  - Database failures (simulate with invalid connection)
  - Transaction rollbacks (verify no partial writes)
  - Foreign key violations (try to create activity with invalid group_id)

**Week 9 Total**: ~10 tasks, ~18-22 hours

**Acceptance Criteria**:

- Backend test coverage reaches 80%+
- All edge cases covered
- All error paths tested

---

### Week 10: Frontend Test Expansion and Final QA (18-22 hours)

#### Component Tests (8-10 hours)

- [ ] 10.1 Test Activity Group components:
  - ActivityGroupList (expand/collapse, delete)
  - ActivityGroupForm (create, edit, validation)
- [ ] 10.2 Test Activity Goal components:
  - GoalSettingModal (set, update, validate)
  - GoalProgressIndicator (calculate percentage, display correctly)
- [ ] 10.3 Test updated Mood Selector:
  - 7-point scale (all 7 levels selectable)
  - Keyboard navigation (Left/Right arrows)
  - Accessibility (aria-labels)
- [ ] 10.4 Test Draft Assessment functionality:
  - Save draft (incomplete assessment)
  - Resume draft (load saved responses)
  - Delete draft
- [ ] 10.5 Test Backdating features:
  - Date picker (max = now, min = now - 24h)
  - "Log for Yesterday" button
  - Validation (reject >24h)
- [ ] 10.6 Test Catalyst UI components:
  - Button (all variants, colors)
  - Input (label, error, focus)
  - Card (dark mode, responsive)

#### End-to-End Testing (6-8 hours)

- [ ] 10.7 Test complete Activity Group workflow:
  - Create group → Add activities → Log activity → Set goal → View report → Achieve goal
- [ ] 10.8 Test complete Check-In v2.0 workflow:
  - Select 7-point mood → Select grouped activities with icons → Submit → View history
- [ ] 10.9 Test draft assessment workflow:
  - Start PHQ-9 → Save draft → Resume later → Complete → View result
- [ ] 10.10 Test backdating workflow:
  - Click "Log for Yesterday" → Select mood → Select activities → Submit → Verify date
- [ ] 10.11 Cross-browser testing:
  - Chrome (latest)
  - Firefox (latest)
  - Edge (latest)
  - Safari (if available)

#### Final QA and Documentation (4-6 hours)

- [ ] 10.12 Update README.md with all new features (Activity Groups, Check-In v2.0, Drafts, Backdating)
- [ ] 10.13 Write user guide for Activity Groups (how to create, log, set goals, view reports)
- [ ] 10.14 Write user guide for 7-point mood scale (what each level means)
- [ ] 10.15 Update developer docs (CLAUDE.md) with new patterns:
  - Activity Groups architecture
  - Goal calculation logic
  - Migration strategy (1-5 to 1-7)
- [ ] 10.16 Draft release notes (list all features, breaking changes, migration notes)
- [ ] 10.17 Final accessibility audit (Lighthouse, Axe DevTools)
- [ ] 10.18 Final performance audit (Lighthouse, measure key metrics)
- [ ] 10.19 Final security review:
  - SQL injection (verify all queries parameterized)
  - XSS (verify no innerHTML usage)
  - CSRF (verify Tauri's built-in protection)

**Week 10 Total**: ~19 tasks, ~18-22 hours

**v1.0 Shippable Criteria**:

- ✅ All features from v0.1-v0.4 integrated
- ✅ Test coverage >80%
- ✅ E2E tests pass (all user flows work)
- ✅ No P0/P1 bugs
- ✅ Documentation complete
- ✅ Performance targets met (<100ms UI, <500ms charts)
- ✅ Accessibility validated (>90 score)
- ✅ Ready for real users
- 🚀 **SHIP v1.0 to production**

---

## Summary by Phase

| Release | Duration | Tasks | Estimated Hours | Ship? |
|---------|----------|-------|-----------------|-------|
| **v0.1**: Activity Groups | 4 weeks | ~107 tasks | ~90-100 hours | ✅ **SHIP IT** |
| **v0.2**: Check-In v2.0 | 1 week | ~28 tasks | ~22-26 hours | ✅ **SHIP IT** |
| **v0.3**: Spec Gap Completion | 1 week | ~27 tasks | ~24-30 hours | ✅ **SHIP IT** |
| **v0.4**: Catalyst UI Refresh | 2 weeks | ~26 tasks | ~28-36 hours | ✅ **SHIP IT** |
| **v1.0**: Test Coverage + Production Polish | 2 weeks | ~29 tasks | ~36-44 hours | ✅ **SHIP IT** |
| **Total** | **10 weeks** | **~217 tasks** | **~200-236 hours** | **5 releases** |

---

## Progress Tracking

### Release Completion Status

- [ ] **v0.1**: Activity Groups Shipped (59/107 tasks - Weeks 1-2 Complete! Week 3 Pending) - End of Week 4
  - Week 1: ✅ 29/29 tasks complete
  - Week 2: 🟡 30/32 tasks complete (bindings generation and command tests pending)
  - Week 3: ⏳ 0/24 tasks (Frontend UI Components)
  - Week 4: ⏳ 0/22 tasks (Reporting Dashboard)
- [ ] **v0.2**: Check-In v2.0 Shipped (0/28 tasks) - End of Week 5
- [ ] **v0.3**: Spec Gaps Shipped (0/27 tasks) - End of Week 6
- [ ] **v0.4**: Catalyst UI Shipped (0/26 tasks) - End of Week 8
- [ ] **v1.0**: Production Shipped (0/29 tasks) - End of Week 10

### Shippable Milestone Checklist

- [ ] **v0.1 SHIPPED**: Users can organize activities into groups with goals and reporting (Week 4)
- [ ] **v0.2 SHIPPED**: Users get 7-point mood scale + grouped activity selection (Week 5)
- [ ] **v0.3 SHIPPED**: Users can save drafts, backdate entries, spec-compliant validation (Week 6)
- [ ] **v0.4 SHIPPED**: Users see modern Catalyst UI across all pages (Week 8)
- [ ] **v1.0 SHIPPED**: Stable, production-ready application with 80%+ test coverage (Week 10)

### What "Shipped" Means

Each release must meet its shippable criteria:

- ✅ Core functionality works
- ✅ Critical tests pass (TDD throughout)
- ✅ No P0 bugs
- ✅ Performance targets met
- ✅ Users can accomplish stated goals

**Remember**: Ship at 80%, iterate based on feedback

---

## Notes

- Update this file weekly with completed tasks (mark with `[X]`)
- Add new tasks as discovered during implementation
- Track blockers in a separate issues list
- Review estimates weekly and adjust timeline if needed
- Every task estimate includes: implementation + testing + debugging
