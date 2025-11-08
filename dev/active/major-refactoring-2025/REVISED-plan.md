# Revised Development Plan 2025 - Focused on Actual New Work

**Created**: 2025-11-07
**Status**: Planning
**Estimated Duration**: 7-11 weeks
**Priority**: High

**Related Architecture Docs**:
- **svelte5-architecture.md** - State management patterns (runes vs stores)
- **tailwind4-design-system.md** - Complete Catalyst UI design token specifications
- **catalyst-integration-spec.md** - Practical integration guide (component priority, Melt UI strategy, migration plan)
- **component-architecture.md** - Component patterns (directory structure, composition, testing, accessibility)
- **data-structures.md** - TypeScript type specifications (UI state, domain types, component props)
- **DECISION-LOG.md** - Plan review decisions and resolutions
- **REVISED-tasks.md** - Actionable task checklist

## Executive Summary

This plan focuses on **actual new work** needed, verified against the codebase audit. The original plan included many features that are already 100% complete (Dashboard, Activities, Assessments, Tailwind v4, Heroicons).

**What's Already Complete (No Work Needed)**:
- ✅ Dashboard with Chart.js visualization (routes/charts)
- ✅ All 4 assessment types (PHQ-9, GAD-7, CES-D, OASIS)
- ✅ Individual Activities with CRUD operations
- ✅ Mood Check-In (1-5 scale)
- ✅ Scheduling system
- ✅ Tailwind CSS v4.1.17
- ✅ Heroicons v5.2.0
- ✅ Modern UI components (Button, Card, Input, etc.)

**Actual New Work Needed**:
1. **Activity Groups** - 2-level hierarchy feature (NEW from new-features.md)
2. **Check-In v2.0** - Upgrade to 1-7 scale + Activity Group integration (NEW from new-features.md)
3. **Spec Gap Completion** - Draft assessments, daily limits, backdating (ORIGINAL spec)
4. **Catalyst UI Refresh** - Extract design patterns, enhance components (NEW from new-features.md)
5. **Test Coverage Expansion** - Reach 80%+ coverage (QUALITY)

**Development Approach**: Pragmatic & Iterative
- Ship value every 1-2 weeks
- TDD throughout (tests integrated with features)
- Each release delivers tangible user benefit

---

## Current State Analysis

### Verified Complete ✅
- **Backend**: Compiles successfully, zero warnings
- **Frontend**: 0 TypeScript errors, 15 warnings
- **Database**: SQLite with all core tables (assessments, mood_checkins, activities, schedules)
- **Testing**: Excellent coverage with **851 frontend tests** + 112 backend tests = **963 total tests**
- **UI**: Modern Tailwind v4 components with dark mode support

### Verified Incomplete ⚠️
From **original spec** (spec.md):
- FR-009a: Draft assessments (save incomplete)
- FR-009b: Daily assessment limit (prevent duplicates)
- FR-009c/FR-015a: Backdating entries (within 24 hours)
- FR-020a: Activity name validation (50 char limit, disallow < > & ")
- Test coverage gaps (~40-50 tests needed to reach 80%)

From **new-features.md**:
- Activity Groups (2-level hierarchy: Groups → Activities)
- Activity Goals (x days in y period OR x% improvement)
- Activity Reporting (days/week, % change from previous week)
- Mood scale upgrade (1-5 → 1-7)
- Daily Check-In integration with Activity Groups
- Catalyst UI pattern extraction

---

## Implementation Phases

### Phase 1: Activity Groups (Weeks 1-4) → SHIP v0.1

**Goal**: Users can organize activities into groups, set goals, and track progress

**Duration**: 4 weeks (90-100 hours)

**User Value**: Structured habit tracking with meaningful insights

#### Week 1: Database Schema and Repository Layer (22-25 hours)

**Database Migration**:
- [ ] Create migration `003_activity_groups.sql`
- [ ] Define `activity_groups` table (id, name CHECK(length(name) <= 100), description, created_at, deleted_at)
- [ ] Modify `activities` table: Add `group_id INTEGER NOT NULL` (mandatory relationship)
- [ ] Add FK constraint: `FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE CASCADE`
  - **Behavior**: Deleting a group CASCADE deletes all its activities
  - **Mandatory**: Activities MUST belong to a group (NOT NULL constraint)
  - **UI Warning**: "Deleting this group will permanently delete all X activities in it. This cannot be undone."
- [ ] Create `activity_logs` table with CASCADE delete:
  ```sql
  CREATE TABLE activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER NOT NULL,
    logged_at TEXT NOT NULL,           -- When activity occurred
    created_at TEXT NOT NULL DEFAULT (datetime('now')), -- When log was created
    notes TEXT CHECK (length(notes) <= 500 OR notes IS NULL),
    deleted_at TEXT,                   -- Soft delete support
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
  );
  ```
- [ ] Create indexes: `idx_activity_logs_activity`, `idx_activity_logs_logged_at`, `idx_activity_logs_deleted`
- [ ] Create `activity_goals` table (id, activity_id?, group_id?, goal_type, target_value, period_days)
- [ ] Add CHECK constraints:
  - `goal_type IN ('days_per_period', 'percent_improvement')`
  - `NOT (activity_id IS NOT NULL AND group_id IS NOT NULL)` -- prevents ambiguous targets
- [ ] Add partial indexes for soft deletes (`WHERE deleted_at IS NULL`)
- [ ] Test migration runs successfully (rollback testing skipped - no users exist)

**Repository Layer** (TDD):
- [ ] Create `features/activities/models.rs` with new domain types (ActivityGroup, ActivityLog, ActivityGoal)
- [ ] Write tests for `create_activity_group` (TDD: red)
- [ ] Implement `create_activity_group` (TDD: green)
- [ ] Write tests for `update_activity_group`, `delete_activity_group` (soft delete)
- [ ] Implement update/delete methods
- [ ] Write tests for `get_activity_groups` (exclude deleted)
- [ ] Implement get methods
- [ ] Write tests for activity CRUD with group_id FK
- [ ] Update existing activity methods to support groups
- [ ] Write tests for `log_activity` (create ActivityLog)
- [ ] Implement activity logging repository methods

**Acceptance Criteria**:
- Migration runs successfully with FK constraints enabled
- All repository tests pass (TDD throughout)
- Parameterized queries (zero SQL injection risk)

---

#### Week 2: Activity Goals and Reporting Logic (24-28 hours)

**Goals Repository** (TDD):
- [ ] Write tests for `set_activity_goal` (both activity-level and group-level)
- [ ] Implement `set_activity_goal` repository method
- [ ] Write tests for `get_activity_goals`
- [ ] Implement `get_activity_goals` repository method
- [ ] Write tests for `update_activity_goal`, `delete_activity_goal`
- [ ] Implement update/delete methods

**Reporting Queries** (TDD):
- [ ] Write tests for `get_activity_frequency` (days per week calculation with date range params)
- [ ] Implement `get_activity_frequency` with start_date, end_date, period_days parameters
- [ ] Write tests for `get_activity_trend` (percent change calculation)
- [ ] Implement `get_activity_trend` (compare current vs previous period)
- [ ] Write tests for `check_goal_progress` (actual vs target)
- [ ] Implement `check_goal_progress` with GoalProgress struct
- [ ] **NEW**: Implement goal achievement notification (when progress.is_achieved, send notification)

**Tauri Commands**:
- [ ] Implement `create_activity_group`, `update_activity_group`, `delete_activity_group` commands
- [ ] Implement `get_activity_groups` query
- [ ] Update existing activity commands to support group_id
- [ ] Implement `log_activity`, `get_activity_logs` commands
- [ ] Implement `set_activity_goal`, `get_activity_goals` commands
- [ ] Implement `get_activity_frequency`, `get_activity_trend`, `check_goal_progress` queries
- [ ] Generate TypeScript bindings with `tauri-specta`
- [ ] Write command tests (verify error handling, validation)

**Acceptance Criteria**:
- All commands return `Result<T, CommandError>`
- Bindings updated in `src/lib/bindings.ts`
- Error messages user-friendly and actionable

---

#### Week 3: Frontend UI Components (22-26 hours)

**Architecture Note**: Follow patterns in `svelte5-architecture.md`:
- Use `$state()` for component-local form state
- Use `$props()` for all component inputs
- Use `$derived()` for validation and computed values
- Keep global state in existing stores (theme, toast)

**Activity Group Management Page**:
- [ ] Create `/routes/activity-groups/+page.svelte`
- [ ] Create `ActivityGroupList.svelte` with expand/collapse
- [ ] Create `ActivityGroupForm.svelte` (modal for create/edit)
- [ ] Implement delete group with confirmation dialog

**Activity Management UI**:
- [ ] Update `/routes/mood/activities/+page.svelte` to support groups
- [ ] Update `ActivityForm.svelte` to include group selector
- [ ] Create `IconPicker.svelte` component with Heroicon names validation (e.g., "academic-cap", "heart")
- [ ] Update activity name validation to 50 chars max, disallow < > & "
- [ ] Update `ActivityList.svelte` to display grouped activities

**Activity Logging Interface**:
- [ ] Create `ActivityLogButton.svelte` (quick log for each activity)
- [ ] Create `ActivityLogHistory.svelte` (timeline view)
- [ ] Add date filtering for log history
- [ ] Implement "Add Note" feature for logs

**Goal Setting UI**:
- [ ] Create `GoalSettingModal.svelte` component
- [ ] Implement goal type selector (radio buttons: days per period / percent improvement)
- [ ] Create target value input with validation
- [ ] Create period selector (dropdown: week, month, custom)
- [ ] Create `GoalProgressIndicator.svelte` (progress bar or percentage)
- [ ] Display active goals for each activity/group
- [ ] **NEW**: Wire up goal achievement notification (show toast when goal achieved)

**Acceptance Criteria**:
- User can create groups and activities end-to-end
- Icons display correctly for all activities
- Goal progress calculates accurately
- UI responsive (mobile-first design)

---

#### Week 4: Reporting Dashboard and Integration (22-26 hours)

**Reporting Components**:
- [ ] Create `ActivityReportCard.svelte` (days/week display)
- [ ] Create `ActivityTrendChart.svelte` (% change visualization)
- [ ] Create `GoalProgressDashboard.svelte` (all active goals)
- [ ] Integrate reporting into existing `/charts` route

**Integration Testing**:
- [ ] Test end-to-end: Create group → Add activities → Log activity → View report
- [ ] Test goal setting → Track progress → Achieve goal workflow
- [ ] Test activity deletion (soft delete preserves logs)
- [ ] Test group deletion (cascade to activities)
- [ ] Write component tests for new UI (Activity Groups, Goals, Logging)

**Performance Testing**:
- [ ] Test with 100+ activity logs
- [ ] Optimize reporting queries if needed (<200ms target)

**v0.1 Shippable Criteria**:
- ✅ Users can create activity groups and activities
- ✅ Users can log activities with notes
- ✅ Users can set goals (days/period or % improvement)
- ✅ Reporting shows days/week and % change
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Activity list loads <200ms
- 🚀 **SHIP v0.1 to users**

---

### Phase 2: Check-In v2.0 (Week 5) → SHIP v0.2

**Goal**: Users get improved check-in experience with 7-point mood scale and Activity Group integration

**Duration**: 1 week (22-26 hours)

#### Database Migration (1-2 hours)
**Note**: No data migration needed (no users exist yet)
- [ ] Create migration `004_mood_scale_1_to_7.sql`
- [ ] Update CHECK constraint only:
  ```sql
  ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
  ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 7);
  ```
- [ ] Test migration runs successfully

#### Backend Updates (4-6 hours)
- [ ] Update mood rating validation to 1-7 range
- [ ] Update `MoodCheckin` model and documentation
- [ ] Generate new TypeScript bindings
- [ ] Update existing mood tests for 1-7 scale
- [ ] Write new tests for 7-point scale edge cases

#### Frontend: Mood Selector Update (6-8 hours)
- [ ] Update `MoodScaleInput.svelte` to 7-point scale
- [ ] Design visual representation (emoji/slider/buttons with 7 levels)
- [ ] Add accessibility labels for each level (1=Terrible ... 7=Excellent)
- [ ] Test keyboard navigation for 7-point selector
- [ ] Update mood history display to show 7-point scale

#### Frontend: Activity Integration (6-8 hours)
- [ ] Update `ActivitySelector.svelte` to group by Activity Group
- [ ] Implement collapsible sections for each group
- [ ] Display icons instead of names (using Heroicons)
- [ ] Test multi-select with grouped activities
- [ ] Update check-in history to show grouped activities with icons

#### Integration Testing (3-4 hours)
- [ ] Test end-to-end check-in flow with 7-point scale
- [ ] Test activity selection with 0, 1, many activities across groups
- [ ] Test backward compatibility (old 1-5 check-ins still display correctly)
- [ ] Write component tests for updated mood selector

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

### Phase 3: Spec Gap Completion (Week 6) → SHIP v0.3

**Goal**: Complete missing features from original spec

**Duration**: 1 week (24-30 hours)

#### FR-009a: Draft Assessments (8-10 hours)
- [ ] Add `status` column to `assessment_responses` table ('draft' or 'completed')
- [ ] Update assessment repository to save drafts
- [ ] Create `save_draft_assessment` Tauri command
- [ ] Update frontend `AssessmentForm.svelte` to support "Save Draft" button
- [ ] Create draft assessment list UI
- [ ] Write tests for draft functionality

#### FR-009b: Daily Assessment Limit (6-8 hours)
- [ ] Add validation to prevent >1 assessment of same type per day
- [ ] Update `submit_assessment` command with date check
- [ ] Add user-friendly error message
- [ ] Update UI to show "Already completed today" state
- [ ] Write tests for daily limit validation

#### FR-009c/FR-015a: Backdating (6-8 hours)
- [ ] Add optional `backdated_to` field to assessment_responses and mood_checkins
- [ ] Add validation: Allow backdating within 24 hours
- [ ] Update frontend forms with date picker (optional, default = now)
- [ ] Add "Log for yesterday" quick action
- [ ] Write tests for backdating validation (allow 24h, reject >24h)

#### FR-020a: Activity Name Validation (2-4 hours)
**Note**: No data migration needed (no users exist yet)
- [ ] Update activity name validation to max 50 chars (from current 100)
- [ ] Add validation to disallow < > & " characters (regex: `^[^<>&"]+$`)
- [ ] Update error messages: "Activity name must be 1-50 characters and cannot contain < > & \""
- [ ] Write tests for validation rules

**v0.3 Shippable Criteria**:
- ✅ Users can save draft assessments
- ✅ System prevents duplicate assessments per day
- ✅ Users can backdate entries (within 24h)
- ✅ Activity names validated per spec
- ✅ Tests pass, no P0 bugs
- 🚀 **SHIP v0.3 to users**

---

### Phase 4: Catalyst UI Refresh (Weeks 7-8) → SHIP v0.4

**Goal**: Users see modern, professional UI across all pages

**Duration**: 2 weeks (28-36 hours)

**Note**: Tailwind v4 already installed. This phase extracts Catalyst **design tokens and visual styling only** (not React component logic). Catalyst is React-only; we recreate visual styling in Svelte components.

#### Week 7: Design Tokens and Core Components (14-18 hours)

**Architecture References**:
- `tailwind4-design-system.md` - Design tokens and CSS specifications
- `catalyst-integration-spec.md` - Component priority matrix (Week 7: Button, Input, Card, Badge, Alert)

**Foundation** (4-6 hours):
- [ ] Create token files: `src/styles/tokens/{colors,typography,spacing,shadows}.css`
- [ ] Implement Catalyst zinc palette (50-950) + blue/red/green accent colors
- [ ] Add Catalyst typography scale with text/line-height utilities (`text-sm/6`, etc.)
- [ ] Add fractional spacing values (`3.5: 0.875rem`, `4.5: 1.125rem`)
- [ ] Implement CSS custom properties for dark mode theme switching
- [ ] Test dark mode compatibility with new tokens

**Enhanced Components** (10-12 hours):
- [ ] Create `Button.svelte` with `@layer components` architecture
  - Implement ALL 12 variants: solid/outline/plain × zinc/blue/red/green
  - See spec for complete implementation with all states (hover, active, focus, disabled)
  - Follow Svelte 5 presentation component pattern (`$props()` only)
- [ ] Create `Input.svelte` with full state support
  - All states: default, hover, focus, error, disabled
  - Label, helper text, error message support
  - See spec § Input Component
- [ ] Create `Card.svelte`, `Badge.svelte`, `Alert.svelte`, `Modal.css`
  - Dashboard suite components per spec
- [ ] Write component tests for all components
- [ ] Document components (usage examples in spec)

**Acceptance Criteria**:
- Core components match Catalyst visual style (70-80% parity)
- Dark mode works across all components
- Focus indicators meet WCAG standards
- Components documented

---

#### Week 8: Component Migration and Polish (14-18 hours)

**Architecture References**:
- `svelte5-architecture.md` - Component patterns (presentation vs container)
- `catalyst-integration-spec.md` - Migration checklist, Melt UI integration (Select, Checkbox, Radio)

**Component Migration** (8-10 hours):
- [ ] Migrate assessment forms to new Input components
- [ ] Migrate all buttons to new Button component
- [ ] Migrate dashboard to new Card components
- [ ] Migrate activity management UI to new components
- [ ] Update navigation with new styles
- [ ] Test all pages for visual consistency

**Accessibility and Performance** (6-8 hours):
- [ ] Run accessibility audit (Axe DevTools or Lighthouse)
- [ ] Fix keyboard navigation issues
- [ ] Test responsive design (mobile, tablet, desktop)
- [ ] Run Lighthouse performance audit
- [ ] Fix performance issues (target >90 score)

**v0.4 Shippable Criteria**:
- ✅ All components use Catalyst-inspired design
- ✅ Button, Input, Select, Card components enhanced
- ✅ Dark mode functional
- ✅ Accessibility score >90 (Lighthouse)
- ✅ Tests pass, visual consistency verified
- ✅ Performance: No regressions from UI changes
- 🚀 **SHIP v0.4 to users**

---

### Phase 5: Test Coverage Expansion (Weeks 9-10) → SHIP v1.0

**Goal**: Reach 80%+ test coverage, production-ready quality

**Duration**: 2 weeks (36-44 hours)

#### Week 9: Backend Test Expansion (18-22 hours)

**Repository Tests** (10-12 hours):
- [ ] Add edge case tests for Activity Groups (empty groups, cascading deletes)
- [ ] Add edge case tests for Activity Goals (invalid targets, expired goals)
- [ ] Add edge case tests for Activity Logging (duplicate logs, invalid dates)
- [ ] Add edge case tests for 7-point mood scale (boundary values)
- [ ] Add concurrency tests (simultaneous operations)

**Command Validation Tests** (8-10 hours):
- [ ] Test notes length validation (max 5000 chars)
- [ ] Test control character filtering
- [ ] Test type code format validation
- [ ] Test boundary values (exact thresholds, max string lengths)
- [ ] Test error handling (database failures, transaction rollbacks)

**Acceptance Criteria**:
- Backend test coverage reaches 80%+
- All edge cases covered
- All error paths tested

---

#### Week 10: Frontend Test Expansion and Final QA (18-22 hours)

**Component Tests** (8-10 hours):
- [ ] Test Activity Group components (create, edit, delete)
- [ ] Test Activity Goal components (set, update, progress)
- [ ] Test updated Mood Selector (7-point scale)
- [ ] Test Draft Assessment functionality
- [ ] Test Backdating features
- [ ] Test Catalyst UI components

**End-to-End Testing** (6-8 hours):
- [ ] Test complete Activity Group workflow (create → log → report → goal)
- [ ] Test complete Check-In v2.0 workflow (7-point + grouped activities)
- [ ] Test draft assessment workflow (save → resume → complete)
- [ ] Test backdating workflow (yesterday's entry)
- [ ] Cross-browser testing (Chrome, Firefox, Edge)

**Final QA and Documentation** (4-6 hours):
- [ ] Update README.md with new features
- [ ] Write user guide for Activity Groups
- [ ] Update developer docs (CLAUDE.md) with new patterns
- [ ] Draft release notes
- [ ] Final accessibility audit
- [ ] Final performance audit
- [ ] Final security review (SQL injection, XSS)

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

## Timeline Estimate

| Release | Duration | Key Deliverable | Ship? |
|---------|----------|-----------------|-------|
| **v0.1**: Activity Groups | 4 weeks | Users track activities with groups, goals, reporting | ✅ **SHIP IT** |
| **v0.2**: Check-In v2.0 | 1 week | 7-point scale + grouped activity selection | ✅ **SHIP IT** |
| **v0.3**: Spec Gap Completion | 1 week | Drafts, daily limits, backdating | ✅ **SHIP IT** |
| **v0.4**: Catalyst UI Refresh | 2 weeks | Modern, professional UI | ✅ **SHIP IT** |
| **v1.0**: Test Coverage + Production Polish | 2 weeks | 80%+ coverage, production-ready | ✅ **SHIP IT** |
| **Total** | **10 weeks** | **5 shippable releases** | |

---

## Risk Assessment

### High Risks
1. **Activity Groups Scope Creep** (Feature is complex with goals, reporting, logging)
   - **Mitigation**: Strict MVP scope, defer advanced features to v1.1 (drag-and-drop deferred)
   - **Impact**: High (timeline delays)
   - **Probability**: Medium

### Medium Risks
3. **Test Coverage Reveals Critical Bugs**
   - **Mitigation**: Allocate buffer time in Phase 5, prioritize fixes
   - **Impact**: Medium (timeline delays)
   - **Probability**: Medium

4. **Performance Degradation** (Activity logging at scale)
   - **Mitigation**: Performance testing with synthetic data, optimize queries early
   - **Impact**: Medium (user experience)
   - **Probability**: Low

### Low Risks
5. **Catalyst UI Pattern Extraction** (Design consistency)
   - **Mitigation**: Focus on 3-4 core components, accept 70-80% parity
   - **Impact**: Low (visual only)
   - **Probability**: Low

---

## Success Metrics

### Technical Metrics
- **Test Coverage**: >80% for all code
- **Build Status**: Zero TypeScript errors, <10 warnings
- **Performance**:
  - UI responsiveness: <100ms for all interactions
  - Chart rendering: <500ms
  - Database queries: <200ms for 95th percentile
- **Accessibility**: Lighthouse score >90

### Feature Metrics
- **Activity Groups**: Full CRUD + goals + reporting + logging
- **Check-In v2.0**: 7-point scale + grouped activity selection
- **Spec Gaps**: 100% of FR-009a, FR-009b, FR-009c, FR-015a, FR-020a complete
- **UI Migration**: Core pages use Catalyst-inspired components

---

## Dependencies and Prerequisites

### Technical Dependencies
- **Existing Codebase**: All base features complete (assessments, activities, mood, scheduling)
- **External Libraries**:
  - Chart.js v4.5.1 (already working)
  - Heroicons v5.2.0 (already installed)
  - Tailwind CSS v4.1.17 (already installed)

### Knowledge Requirements
- Rust database migrations (SQLite schema changes with FK constraints)
- **Svelte 5 runes system** - See `svelte5-architecture.md` for:
  - State management decision tree (runes vs stores)
  - Component patterns (`$state`, `$derived`, `$effect`, `$props`)
  - Context API for feature-scoped state
  - Migration strategy from stores to runes
- Tailwind CSS v4 syntax
- Catalyst UI design patterns

### Release Dependencies
- **v0.1 (Activity Groups)**: No dependencies (can start immediately)
- **v0.2 (Check-In v2.0)**: Requires v0.1 Activity Groups (grouped activity selection)
- **v0.3 (Spec Gaps)**: Independent (can run parallel with v0.1-v0.2)
- **v0.4 (Catalyst UI)**: Independent (can run parallel with v0.1-v0.3)
- **v1.0 (Production)**: Requires all previous releases (integration testing)

---

## Rollback Strategy

### Database Rollbacks
1. **Activity Groups Schema**: Keep migration scripts reversible
   ```sql
   -- Down migration
   DROP TABLE activity_goals;
   DROP TABLE activity_logs;
   ALTER TABLE activities DROP COLUMN group_id;
   DROP TABLE activity_groups;
   ```

2. **Mood Scale Migration**: Provide reverse migration
   ```sql
   -- Reverse 1-7 to 1-5 (lossy conversion)
   UPDATE mood_checkins SET mood_rating = CASE
     WHEN mood_rating BETWEEN 1 AND 2 THEN 1
     WHEN mood_rating BETWEEN 3 AND 4 THEN 2
     WHEN mood_rating = 5 THEN 3
     WHEN mood_rating = 6 THEN 4
     WHEN mood_rating = 7 THEN 5
   END;
   ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
   ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 5);
   ```

### Feature Flags (Recommended)
Implement feature flags for:
- Activity Groups UI (toggle between old/new views)
- 7-point mood scale (toggle between 1-5 and 1-7)
- Catalyst UI components (gradual rollout)

---

## Communication Plan

### Progress Reporting
- **Weekly**: Update tasks.md with completed tasks
- **Bi-weekly**: Update this plan with risks/blockers
- **Phase Completion**: Git tag and release notes

### Documentation Updates
- Update `CLAUDE.md` with new architectural decisions
- Document Activity Groups schema in spec
- Create user guide for new features
- Reference `svelte5-architecture.md` for component state patterns
- Update architecture docs as patterns evolve

---

## Critical Decisions Made

1. **Database Migration Strategy**: ✅ DECIDED
   - **Decision**: Create new Activity Groups tables, add group_id FK to existing activities
   - **Rationale**: Extends existing activities feature without breaking changes
   - **Risk**: Migration complexity, but reversible

2. **Mood Scale Migration**: ✅ DECIDED
   - **Decision**: Linear stretch mapping (1→1, 2→3, 3→4, 4→5, 5→7) with data migration
   - **Rationale**: Maintains relative spacing, updates existing data
   - **Risk**: Medium (data migration complexity)

3. **Catalyst UI Approach**: ✅ DECIDED
   - **Decision**: Selective extraction (design tokens + 3-4 core components)
   - **Rationale**: 70-80% visual parity without full component library
   - **Risk**: Low (visual only)

4. **Svelte 5 State Management**: ✅ DECIDED
   - **Decision**: Use runes for component state, keep stores for global/external state (documented in `svelte5-architecture.md`)
   - **Rationale**: Runes provide better component reactivity, stores remain best for cross-cutting concerns
   - **Approach**:
     - Component-local: `$state()`, `$derived()`, `$effect()`
     - Feature-scoped: Runes + Context API
     - Global (theme, toast): Keep existing Svelte stores
     - No mass migration - adopt runes for new components only
   - **Risk**: Low (both patterns are officially supported)

## Open Questions

1. **Activity Group Limits**: Max activities per group? Max groups per user?
   - **Decision Needed By**: Week 1 (schema design)
   - **Recommendation**: Soft limits via UI (20 groups, 50 activities/group)

2. **Goal Progress Notifications**: Should users get notified when goals are achieved?
   - **Decision Needed By**: Week 2 (backend commands)
   - **Recommendation**: Add to v1.1 (post-MVP)

3. **Backdating UI**: Date picker vs "Yesterday" button?
   - **Decision Needed By**: Week 6 (spec gap implementation)
   - **Recommendation**: Both (quick action + manual picker)

---

## Next Steps

1. **Review this revised plan** with stakeholders
2. **Make decisions** on open questions
3. **Create task branch**: `git checkout -b feature/activity-groups-2025`
4. **Begin Phase 1, Week 1**: Start with Activity Groups database migration
5. **Setup tracking**: Copy tasks from this plan to tasks.md with checkboxes

---

## References

- **Original Spec**: `/specs/001-mental-health-tracking/spec.md`
- **New Features**: `/new-features.md`
- **Audit Report**: Comprehensive codebase audit (2025-11-07)
- **Project Guidelines**: `/CLAUDE.md`
- **Test Coverage Analysis**: `/TEST_COVERAGE_ANALYSIS.md`
- **Architecture Specifications**:
  - `svelte5-architecture.md` - State management patterns (runes vs stores, component architecture)
  - `tailwind4-design-system.md` - Complete Catalyst UI design system (tokens, components, dark mode)
  - `catalyst-integration-spec.md` - Catalyst integration guide (React→Svelte translation, Melt UI usage, 70-80% parity metrics)
  - `component-architecture.md` - Component patterns (composition, testing conventions, accessibility patterns)
  - `data-structures.md` - TypeScript type specifications (UI state, domain types, component props, chart data)
  - `DECISION-LOG.md` - Plan review decisions and rationale
