# Major Refactoring 2025 - Development Plan

**Created**: 2025-11-06
**Status**: Planning
**Estimated Duration**: 8-10 weeks
**Priority**: High

## Executive Summary

This plan addresses a major refactoring to integrate new features while completing existing work. The project is approximately 65% complete based on the original spec, with significant new requirements identified in `new-features.md`.

**Development Approach**: **Pragmatic & Iterative** (Sam Rivera Principles)
- Ship value every 1-2 weeks (v0.1, v0.2, v0.3, v0.4, v1.0)
- Validate risks BEFORE building (Week 0: Validation Sprint)
- Tests integrated with features (TDD), not separate phase
- See: `PRAGMATIC-PRINCIPLES.md` for full philosophy

**Current State**:
- 153 source files (Rust backend + Svelte 5 frontend)
- Phases 1-8 of original spec mostly complete
- US7 (Dashboard) not started (22 tasks)
- Test coverage gaps identified but not filled
- Recent work: Error handling refactoring, TypeScript/Svelte 5 compatibility fixes

**New Requirements**:
1. UI redesign with Catalyst UI Kit + Tailwind 4 upgrade
2. Activity Groups feature (2-level hierarchy)
3. Daily Check-In changes (1-7 scale, grouped activities, icons)

**Strategic Approach**:
- **Week 0**: Validate risks (Chart.js, Tailwind v4, proof-of-concept)
- **Incremental delivery**: Ship v0.1 (Week 1), v0.2 (Week 5), v0.3 (Week 6), v0.4 (Week 8), v1.0 (Week 10)
- **TDD throughout**: Tests written AS features are built (no separate test phase)
- **User value first**: Each release delivers tangible user benefit

---

## Current State Analysis

### Completed Work (✅)
- **Phases 1-7**: Core assessment system (PHQ-9, GAD-7, CES-D, OASIS)
- **Phase 8**: Scheduling system (32 tests passing)
- **Backend**: Compiles successfully
- **Frontend**: 0 TypeScript errors, 15 warnings
- **Error Handling**: Migrated to `CommandError` struct pattern

### Incomplete Original Spec (⚠️)
- **Phase 7.5**: Test Coverage Gaps (P0 priority tasks)
  - Repository integration tests
  - Edge case coverage
  - Error path testing
- **Phase 8.5**: US7 Dashboard (22 tasks, not started)
  - Visualization components
  - Chart integration
  - Assessment history display
- **Phase 9**: Polish tasks (partially complete)
  - Accessibility improvements
  - Performance optimization

### New Feature Requirements (🆕)

#### 1. Activity Groups Feature
**Impact**: Database schema, backend models, frontend UI
**Complexity**: High
**Dependencies**: None (new feature)

**Requirements**:
- 2-level hierarchy: Activity Group → Activities (1:many)
- Reporting: days/week, % change from previous week
- Goals: x days in y period OR x% improvement over y days
- Integration with Daily Check-In

**Technical Changes Needed**:
- **SIMPLIFIED**: No users yet - existing `activities` table will be dropped and recreated
  - Clean schema migration without data preservation concerns
- New tables: `activity_groups`, `activities`, `activity_logs`, `activity_goals`
- Repository layer: CRUD operations for groups and activities
- Tauri commands: Create/update/delete groups and activities
- Frontend components: Activity group management, activity selection
- Reporting logic: Aggregation queries, trend calculation

**Migration Strategy** (DECISION: Drop and Recreate - No User Data):
1. Drop existing `activities` table
2. Create new schema with `activity_groups` and `activities` tables
3. Update all existing references in code

#### 2. UI Redesign with Catalyst
**Impact**: All frontend components, styling system
**Complexity**: High
**Dependencies**: None (can be done in parallel with backend work)

**Requirements**:
- Integrate Catalyst UI Kit design patterns
- Upgrade Tailwind CSS v3 → v4 **IMMEDIATELY** (beta version)
- Migrate existing components to new design system
- Maintain Svelte 5 (no framework change)

**Approach Decision** (DECISION: Upgrade to Tailwind v4 Beta):
- **CHOSEN**: Selective Extraction with Immediate v4 Upgrade (16-24 hours)
  - **WARNING**: Tailwind v4 is currently in BETA - production risks accepted
  - **MITIGATION**: Pin to specific v4 beta version, monitor for breaking changes
  - Extract design tokens (colors, typography, spacing)
  - Enhance 3-5 key components (Button, Input, Select, Card)
  - Use Tailwind v4 syntax from the start (no migration needed later)
  - 70-80% visual parity with Catalyst

**Risks Accepted**:
- Breaking changes in Tailwind v4 beta releases
- Potential incompatibility with some Tailwind plugins
- Less community support/documentation for v4 beta
- May need to revert to v3 if critical bugs found

#### 3. Daily Check-In Changes
**Impact**: Database schema, backend validation, frontend UI
**Complexity**: Medium
**Dependencies**: Activity Groups feature

**Requirements**:
- Mood scale: 1-5 → 1-7
- Activity display: Grouped by Activity Group
- Activity representation: Icons instead of names

**Technical Changes Needed**:
- Database migration: Update mood rating constraints (1-7)
- Validation: Update backend checks for new range
- UI components: Update mood selector (7-point scale)
- Activity icons: Define icon set, store in database or config
- Activity grouping: Query activities by group for check-in

---

## Proposed Future State

### Architecture Improvements
1. **Activity Management System**: Complete vertical slice (models → repository → commands → UI)
2. **Enhanced UI Layer**: Catalyst-inspired design system with reusable components
3. **Improved Check-In Flow**: Grouped activities with 7-point mood scale
4. **Complete Dashboard**: Visualization of assessment trends and activity correlations

### Technical Stack
- **Backend**: Rust (latest stable) + SQLite + Tauri
- **Frontend**: Svelte 5 (runes) + SvelteKit + Tailwind CSS v4
- **UI Components**: Catalyst-inspired custom components
- **Testing**: TDD with integration tests for all repositories

### User Experience
- Modern, professional UI matching Catalyst design language
- Intuitive activity grouping for better habit tracking
- More granular mood tracking (7-point scale)
- Visual dashboard with charts and trend analysis

### Release Strategy (Pragmatic Approach)
- **v0.1 (Week 1)**: Dashboard with basic charts → **SHIP IT**
- **v0.2 (Week 5)**: Activity Groups complete feature → **SHIP IT**
- **v0.3 (Week 6)**: Check-In v2.0 with 7-point scale → **SHIP IT**
- **v0.4 (Week 8)**: UI refresh with Catalyst components → **SHIP IT**
- **v1.0 (Week 10)**: Production-ready, polished → **SHIP IT**

Each release delivers tangible user value and can be validated through real usage.

---

## Implementation Phases

### Week 0: Validation Sprint (NEW - Pragmatic Approach)

**Goal**: Validate riskiest technical assumptions BEFORE building

**Duration**: 10-11 hours (1-1.5 days) - **UPDATED** due to Tailwind v4 breaking changes

**Why This Matters**: "Don't build for 10 weeks only to discover Chart.js doesn't work or Tailwind v4 breaks everything" - Sam Rivera

**Tasks**:
1. **Validate Chart.js Integration** (2 hrs)
   - Build ONE working chart component (Svelte 5 + Chart.js + Tauri)
   - Verify data flows from Rust backend → frontend → chart
   - Confirm Chart.js v4.5.1 works with current setup
   - **Success Criteria**: Line chart displays mock PHQ-9 data

2. **Upgrade to Tailwind v4.0.0** (3 hrs) ✅ **COMPLETE**
   - ✅ Tag current release with `pre-tailwind-v4` (rollback point created)
   - ✅ Upgraded `tailwindcss@4.0.0` (stable, not beta!)
   - ✅ Installed `@tailwindcss/postcss` (required for v4)
   - ✅ Updated `postcss.config.js` to use `'@tailwindcss/postcss'`
   - ✅ **MAJOR CHANGE**: Updated `src/app.css` imports

   **Breaking Changes Fixed**:

   **1. CSS Import Syntax Change**:
   - **Old (v3)**: `@tailwind base; @tailwind components; @tailwind utilities;`
   - **New (v4)**: `@import "tailwindcss";`
   - **File**: `src/app.css`

   **2. Typography Utilities**:
   - **Issue**: `sm:text-sm sm:leading-6` → `sm:text-sm/6` (slash notation)
   - **Fixed 5 occurrences** in 2 files:
     - `src/lib/components/ui/Combobox.svelte` (2 fixes)
     - `src/lib/components/ui/FormLayout.svelte` (3 fixes in `@apply` directives)

   **3. @apply in Component Style Blocks**:
   - **Issue**: Component `<style>` blocks can't access Tailwind utilities by default
   - **Solution**: Add `@reference 'tailwindcss';` at top of `<style>` block
   - **File**: `src/lib/components/ui/FormLayout.svelte`

   **Files NOT Needing Changes** (standalone `sm:text-sm` valid in v4):
   - `src/routes/charts/+page.svelte`
   - `src/lib/components/charts/TimeRangeSelector.svelte`

   **Result**: ✅ Build successful, Tailwind v4.0.0 fully integrated

   **Potential Future Migrations** (Build passes, but may have changed behavior):
   - `shrink-0` → `shrink-0` (7 occurrences in Toast, Modal, etc.)
   - `bg-opacity-75` → `bg-gray-500/75` (2 occurrences in Modal)
   - `shadow-xs` → `shadow-2xs` (10+ occurrences across routes)
   - `dark:ring-3` → `dark:ring-3` (1 occurrence, default width changed 3px→1px)

   **Note**: These utilities still build successfully in v4. Migration can be done incrementally during UI refresh (Weeks 6-7) rather than blocking Week 0 validation. See upgrade guide: https://tailwindcss.com/docs/upgrade-guide

3. **Commit to Heroicons** (30 min)
   - Install `@heroicons/svelte`
   - Render 3 test icons in a component
   - **Success Criteria**: Icons display correctly

4. **Build End-to-End Proof-of-Concept** (2.5 hrs)
   - Complete flow: Submit PHQ-9 → Store in database → Display in chart
   - Proves full stack integration works
   - **Success Criteria**: User can see assessment result visualized

**Outcome**:
- ✅ **CONFIDENCE**: Technical approach validated, proceed with plan
- ❌ **PIVOT**: Discovered blocker (Chart.js doesn't work, Tailwind v4 breaks), adjust plan

**Acceptance Criteria**:
- Chart.js displays data from Tauri backend
- Tailwind v4 builds without errors (or rolled back to v3)
- Heroicons render correctly
- Full assessment → chart flow demonstrated

---

### Phase 1 (v0.1): Dashboard (Week 1) → SHIP IT

**Goal**: Users can view assessment trends with charts

**Duration**: 1 week (25 hours)

**User Value**: First time users see their mental health data visualized

#### Implementation Approach (User-Focused)

**Reframed Tasks** (User Outcomes, not technical steps):
1. **Users can view assessment history** - Backend queries + Frontend list + Filters + Tests (8 hrs)
2. **Users can see trend charts** - Aggregation + Chart.js integration + Tests (6 hrs)
3. **Users can filter by date range** - Date picker + Query logic + Tests (3 hrs)
4. **Users can see mood correlation** - Correlation calc + Bar chart + Tests (6 hrs)

**Tests Integrated** (TDD Approach):
- Tests written AS features are built (not separate phase)
- Red → Green → Refactor cycle for each user outcome
- See: `PRAGMATIC-PRINCIPLES.md` for TDD approach

**Shippable Criteria (v0.1)**:
- ✅ Users can view PHQ-9, GAD-7, CES-D, OASIS history
- ✅ Users can see trend line charts
- ✅ Basic date filtering works (30/60/90 days)
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Chart rendering <500ms

**Acceptance Criteria**:
- Dashboard accessible from navigation
- At least 3 assessment types displayed with charts
- Date filtering functional
- Integration tests pass (dashboard queries)
- Ready to ship to users for feedback

---

### Phase 2 (v0.2): Activity Groups (Weeks 2-5) → SHIP IT

**Goal**: Users can track activities with groups, logging, goals, and reporting

**Duration**: 4 weeks (97 hours)

**User Value**: Habit tracking with meaningful insights

**Why Complete Feature**: Per requirements, Activity Groups includes groups + activities + logging + goals + reporting as an integrated system. Ship complete to ensure consistent UX.

#### Week 3: Database Schema and Repository Layer
**Tasks**:
1. Database migration: Create `activity_groups`, `activities`, `activity_logs`, `activity_goals` tables
2. Define foreign key relationships with CASCADE rules
3. Create repository struct and methods (TDD approach):
   - `create_activity_group(name, description) -> Result<ActivityGroup>`
   - `create_activity(group_id, name, icon) -> Result<Activity>`
   - `log_activity(activity_id, date) -> Result<ActivityLog>`
   - `set_activity_goal(activity_id, goal_type, target, period) -> Result<Goal>`
4. Write integration tests for all repository methods
5. Handle soft deletes with `deleted_at` column

**Database Schema**:
```sql
CREATE TABLE activity_groups (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT
);

CREATE TABLE activities (
    id INTEGER PRIMARY KEY,
    group_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    icon TEXT, -- Icon identifier (e.g., "hiking", "swimming")
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT,
    FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE CASCADE
);

CREATE TABLE activity_logs (
    id INTEGER PRIMARY KEY,
    activity_id INTEGER NOT NULL,
    logged_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes TEXT,
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
);

CREATE TABLE activity_goals (
    id INTEGER PRIMARY KEY,
    activity_id INTEGER,
    group_id INTEGER,
    goal_type TEXT NOT NULL CHECK(goal_type IN ('days_per_period', 'percent_improvement')),
    target_value INTEGER NOT NULL,
    period_days INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT,
    CHECK ((activity_id IS NOT NULL AND group_id IS NULL) OR (activity_id IS NULL AND group_id IS NOT NULL)),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE CASCADE
);
```

**Acceptance Criteria**:
- Migration runs successfully with foreign keys enabled
- All repository tests pass (TDD: write tests first, verify fail, implement, verify pass)
- Parameterized queries (zero SQL injection risk)

#### Week 4: Backend Commands and Reporting Logic
**Tasks**:
1. Tauri commands for CRUD operations:
   - `create_activity_group`, `update_activity_group`, `delete_activity_group`
   - `create_activity`, `update_activity`, `delete_activity`
   - `log_activity`, `get_activity_logs`
   - `set_activity_goal`, `get_activity_goals`
2. Reporting queries:
   - `get_activity_frequency(activity_id, start_date, end_date) -> days_per_week`
   - `get_activity_trend(activity_id, current_period, previous_period) -> percent_change`
   - `check_goal_progress(goal_id) -> GoalProgress`
3. Generate TypeScript bindings with `tauri-specta`
4. Write command tests (verify error handling)

**Acceptance Criteria**:
- All commands return `Result<T, CommandError>`
- Bindings updated in `src/lib/bindings.ts`
- Error messages user-friendly and actionable

#### Week 5: Frontend UI Components
**Tasks**:
1. Activity Group management page:
   - List all groups with expand/collapse
   - Create/edit/delete group modal
   - Drag-and-drop reordering (optional)
2. Activity management within groups:
   - Add activity to group
   - Icon picker component (dropdown or grid)
   - Edit/delete activities
3. Activity logging interface:
   - Quick log button for each activity
   - Log history view
4. Goal setting UI:
   - Goal type selector (days per period / percent improvement)
   - Target value input
   - Period selector (week, month, custom)
   - Progress indicator

**Icon Set Decision**:
- Use Heroicons, Lucide, or similar icon library
- Store icon identifier in database (e.g., "hiking", "swimming")
- Render dynamically in frontend

**Acceptance Criteria**:
- User can create groups and activities end-to-end
- Icons display correctly for all activities
- Goal progress calculates accurately
- UI responsive (mobile-first design)

---

### Phase 3 (v0.3): Check-In v2.0 (Week 6) → SHIP IT

**Goal**: Users get improved check-in experience with 7-point mood scale and grouped activity selection

**Duration**: 1 week (26 hours)

**User Value**: More granular mood tracking + better activity selection UX

#### Implementation Approach (User-Focused)

**Reframed Tasks** (User Outcomes):
1. **Users can rate mood on 7-point scale** - Update UI + Validation + Migration + Tests (6 hrs)
2. **Users see activities grouped by category** - Group display + Collapsible sections + Tests (4 hrs)
3. **Users select activities with icons** - Icon display + Multi-select + Tests (4 hrs)
4. **Users can log check-in with activities** - Submit form + Backend integration + Tests (4 hrs)

**Database Migration** (Simplified - No User Data):
```sql
-- Simple constraint update
ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 7);
```

**Note**: Linear stretch mapping (1→1, 2→3, 3→4, 4→5, 5→7) documented for future reference.

**Shippable Criteria (v0.3)**:
- ✅ Mood selector shows 7 levels with clear labels
- ✅ Activities displayed grouped by Activity Group
- ✅ Icons render correctly (Heroicons integration)
- ✅ Multi-select activities works smoothly
- ✅ Check-in form submits activities + mood rating
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Check-in response <100ms

**Acceptance Criteria**:
- Users can complete check-in with new 7-point scale
- Activity selection feels intuitive (grouped + icons)
- Old check-ins still viewable (backward compatible)
- Ready to ship for user feedback

---

### Phase 4 (v0.4): UI Refresh with Catalyst (Weeks 7-8) → SHIP IT

**Goal**: Users see modern, professional UI across all pages

**Duration**: 2 weeks (43 hours)

**User Value**: Visual polish + consistent design language

**Note**: Tailwind v4 already upgraded in Week 0, so this phase focuses on component migration.

#### Week 7: Design Tokens and Core Components

**Phase 4.1: Foundation** (4-6 hours)
**Tasks**:
1. Update `tailwind.config.js` with Catalyst design tokens:
   - Zinc color palette (50-950)
   - Typography scale (`text-sm/6`, `text-base/6`)
   - Spacing refinements (`3.5: 0.875rem`)
   - Border radius updates
2. Test dark mode compatibility with new tokens

**Phase 4.2: Button Component** (3-4 hours)
**Tasks**:
1. Create enhanced `Button.svelte` component:
   - Variants: `solid`, `outline-solid`, `plain`
   - Colors: `zinc`, `blue`, `red`
   - Proper focus rings and transitions
   - Dark mode support
2. Replace existing button usages (gradual migration)
3. Write component tests

**Phase 4.3: Input Component** (2-3 hours)
**Tasks**:
1. Create enhanced `Input.svelte` component:
   - Label support
   - Error state styling
   - Hover and focus states
   - Dark mode support
2. Replace form inputs across application
3. Write component tests

**Phase 4.4: Select and Card Components** (2-3 hours)
**Tasks**:
1. Create `Select.svelte` and `Card.svelte` components
2. Apply Catalyst styling patterns
3. Test accessibility (keyboard navigation)

**Acceptance Criteria**:
- All core components match Catalyst visual style (70-80% parity)
- Dark mode works across all components
- Focus indicators meet WCAG standards
- Components documented with usage examples

#### Week 8: UI Polish and Tailwind 4 Upgrade

**Phase 4.5: Component Migration** (4-6 hours)
**Tasks**:
1. Migrate assessment forms to new components
2. Migrate dashboard to new Card components
3. Migrate activity management UI
4. Update navigation with new Button styles

**Phase 4.6: Tailwind 4 Upgrade** (4-6 hours)
**Tasks**:
1. Upgrade Tailwind CSS dependency to v4
2. Update syntax in existing components:
   - `bg-(--var)` → `bg-(--var)`
   - `--spacing(3.5)` → `theme(spacing.3.5)`
3. Test all components for visual regressions
4. Update `tailwind.config.js` for v4 compatibility
5. Run full build and address any warnings

**Phase 4.7: Accessibility and Performance** (2-4 hours)
**Tasks**:
1. Run accessibility audit (Axe DevTools or Lighthouse)
2. Fix keyboard navigation issues
3. Optimize chart rendering performance
4. Test responsive design (mobile, tablet, desktop)

**Acceptance Criteria**:
- All components use Tailwind v4 syntax
- No visual regressions from migration
- Accessibility score >90 (Lighthouse)
- UI responsiveness <100ms for interactions

---

### Phase 5 (v1.0): Production-Ready Polish (Weeks 9-10) → SHIP IT

**Goal**: Stable, polished application ready for real users

**Duration**: 2 weeks (63 hours)

**User Value**: Confidence that the application works reliably

#### Week 9: Integration Testing and Bug Fixes

**Pragmatic Approach**: We've been shipping incrementally (v0.1-v0.4), so integration should be smooth. This week validates everything works together.

**User-Focused Testing Scenarios**:
1. **Users complete full assessment workflow** - PHQ-9, GAD-7, CES-D, OASIS end-to-end (2 hrs)
2. **Users track activities over time** - Create group → Add activities → Log daily → View goals (2 hrs)
3. **Users review progress in dashboard** - View charts, filter dates, see correlations (2 hrs)
4. **Users check in with new flow** - 7-point scale + activity selection (1 hr)

**Technical Validation**:
- Cross-browser testing (Chrome, Firefox, Safari, Edge) (4 hrs)
- Performance testing with realistic data (1000+ records) (4 hrs)
- Accessibility audit (Lighthouse, Axe DevTools) (2 hrs)

**Bug Triage & Fixes**:
- P0 bugs: Fix immediately (block release)
- P1 bugs: Fix if time permits, document for v1.1
- P2 bugs: Document for v1.1+

#### Week 10: Documentation and Deployment Prep

**Pragmatic Approach**: Documentation proves we understand what we built.

**Tasks**:
1. Update `README.md` with new features (2 hrs)
2. Write user guide for Activity Groups (4 hrs)
3. Update developer docs (`CLAUDE.md`) with pragmatic lessons learned (2 hrs)
4. Prepare release notes (2 hrs)
5. Final QA pass (4 hrs)

**Shippable Criteria (v1.0)**:
- ✅ All P0 bugs fixed
- ✅ All user workflows tested and working
- ✅ Performance targets met (<100ms UI, <500ms charts)
- ✅ Accessibility score >90
- ✅ Documentation complete
- ✅ Release notes drafted
- ✅ Application ready for real users

**Acceptance Criteria**:
- Users can successfully complete all workflows
- No blockers preventing real-world usage
- Team feels confident deploying to production

---

## Risk Assessment (Pragmatic Mitigation)

### Risks Validated in Week 0 ✅

**Pragmatic Principle**: "Validate risks BEFORE building, not during."

1. **Chart.js Integration Risk** - VALIDATED Week 0
   - **Original Risk**: Chart.js might not work with Svelte 5 + Tauri
   - **Mitigation**: Build working example in Week 0
   - **Status**: ✅ Validated or ❌ Pivot to alternative

2. **Tailwind v4 Beta Risk** - VALIDATED Week 0
   - **Original Risk**: v4 beta might break existing components
   - **Mitigation**: Upgrade in Week 0, test build, keep v3 config as fallback
   - **Status**: ✅ Validated or ❌ Rollback to v3

3. **Heroicons Integration Risk** - VALIDATED Week 0
   - **Original Risk**: Icons might not render properly
   - **Mitigation**: Install and test 3 icons in Week 0
   - **Status**: ✅ Validated or ❌ Use alternative library

### Remaining Risks

4. **Scope Creep** (New feature requests during refactoring)
   - **Mitigation**: Strict change control, defer non-critical features to Phase 6
   - **Impact**: High (timeline delays)
   - **Probability**: Medium

### Medium Risks
4. **Test Coverage Reveals Critical Bugs**
   - **Mitigation**: Allocate buffer time in Phase 1, prioritize fixes
   - **Impact**: Medium (timeline delays)
   - **Probability**: Medium

5. **Performance Degradation** (With activity logging at scale)
   - **Mitigation**: Performance testing with synthetic data, optimize queries early
   - **Impact**: Medium (user experience)
   - **Probability**: Low

### Low Risks
6. **Icon Set Compatibility Issues**
   - **Mitigation**: Choose well-maintained library (Heroicons), test early
   - **Impact**: Low (visual only)
   - **Probability**: Low

---

## Success Metrics

### Technical Metrics
- **Test Coverage**: >80% for all new code
- **Build Status**: Zero TypeScript errors, <10 warnings
- **Performance**:
  - UI responsiveness: <100ms for all interactions
  - Chart rendering: <500ms
  - Database queries: <200ms for 95th percentile
- **Accessibility**: Lighthouse score >90

### Feature Metrics
- **Completion**: 100% of tasks marked complete in `tasks.md`
- **US7 Dashboard**: All 22 tasks complete with tests
- **Activity Groups**: Full CRUD operations with reporting
- **UI Migration**: All pages use new Catalyst-inspired components

### User Experience Metrics
- **Visual Consistency**: All components follow Catalyst design language
- **Dark Mode**: Works across all pages without visual bugs
- **Responsive Design**: Works on mobile (375px), tablet (768px), desktop (1440px)

---

## Timeline Estimate (Pragmatic Iterative Releases)

### Detailed Breakdown
| Release | Duration | Key Deliverable | Ship? |
|---------|----------|-----------------|-------|
| **Week 0**: Validation Sprint | 1 day | Risks validated (Chart.js, Tailwind v4, Heroicons) | Proof-of-concept |
| **v0.1**: Dashboard | 1 week | Users can view trends | ✅ **SHIP IT** |
| **v0.2**: Activity Groups | 4 weeks | Users can track activities with goals | ✅ **SHIP IT** |
| **v0.3**: Check-In v2.0 | 1 week | 7-point scale + grouped activities | ✅ **SHIP IT** |
| **v0.4**: UI Refresh | 2 weeks | Catalyst components + Tailwind v4 | ✅ **SHIP IT** |
| **v1.0**: Production Polish | 2 weeks | E2E testing, docs, final QA | ✅ **SHIP IT** |
| **Total** | **10 weeks + 1 day** | **5 shippable releases** | |

### Milestones (Shippable Releases)
- **Week 0**: ✅ Risks validated, technical approach confirmed
- **Week 1**: ✅ **v0.1 SHIPPED** - Users see dashboard with charts
- **Week 5**: ✅ **v0.2 SHIPPED** - Users track activities with goals
- **Week 6**: ✅ **v0.3 SHIPPED** - Improved check-in experience
- **Week 8**: ✅ **v0.4 SHIPPED** - Modern UI across all pages
- **Week 10**: ✅ **v1.0 SHIPPED** - Production-ready application

**Key Difference**: Each milestone = **shippable user value**, not "phase complete"

### Assumptions
- Single full-time developer (or equivalent effort)
- Week 0 validates technical approach (no blockers discovered)
- Tests integrated with features (TDD throughout)
- User feedback collected after each release, incorporated if time permits

---

## Dependencies and Prerequisites

### Technical Dependencies (Validated Week 0)
- **External Libraries**:
  - Chart.js v4.5.1 (validated Week 0 - working with Svelte 5 + Tauri)
  - Heroicons (validated Week 0 - renders correctly)
  - Tailwind CSS v4 beta (validated Week 0 - builds successfully OR rolled back to v3)

**Pragmatic Note**: All high-risk dependencies validated in Week 0 before building features.

### Knowledge Requirements
- Rust database migrations (SQLite schema changes)
- Svelte 5 runes system (`$state`, `$derived`, `$props`)
- Tailwind CSS v4 syntax (if validated in Week 0)
- Catalyst UI design patterns

### Release Dependencies (Iterative)
- **v0.1 (Dashboard)**: No dependencies
- **v0.2 (Activity Groups)**: No dependencies (can start immediately)
- **v0.3 (Check-In v2.0)**: Requires v0.2 Activity Groups (activity data exists)
- **v0.4 (UI Refresh)**: Can run parallel with v0.2-v0.3 (independent work)
- **v1.0 (Production)**: Requires all previous releases (integration testing)

**Key Change**: v0.4 UI work can start in Week 7 while v0.2 Activity Groups completes in Week 5, reducing overall timeline risk.

---

## Rollback Strategy

**Note**: No user data exists, so rollback strategy is simplified.

### Database Rollbacks
1. **Activity Groups Schema**: Keep migration scripts reversible
   ```sql
   -- Down migration
   DROP TABLE activity_goals;
   DROP TABLE activity_logs;
   DROP TABLE activities;
   DROP TABLE activity_groups;
   ```

2. **Mood Scale Migration**: Simple constraint rollback
   ```sql
   ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
   ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 5);
   ```

### UI Rollbacks
- **CRITICAL**: Keep Tailwind v3 configuration in git history (v4 beta fallback)
- Tag release before each major phase (v4 upgrade, Activity Groups, etc.)
- Component library versioning: Use feature flags to toggle old/new components

### Feature Flags (Recommended for v4 Beta Risk)
Implement feature flags for:
- Tailwind v4 components (critical for quick rollback)
- Activity Groups UI (toggle between old/new check-in flow)
- Catalyst UI components (gradual rollout)

---

## Communication Plan

### Progress Reporting
- **Weekly**: Update `tasks.md` with completed tasks
- **Bi-weekly**: Update this plan with risks/blockers
- **Phase Completion**: Git tag and release notes

### Documentation Updates
- Update `CLAUDE.md` with new architectural decisions
- Document Activity Groups schema in `specs/001-mental-health-tracking/spec.md`
- Create user guide for new features

---

## Critical Decisions Made

1. **Database Migration Strategy**: ✅ DECIDED
   - **Decision**: Drop and recreate activities table (Option C)
   - **Rationale**: No users exist, clean migration without data concerns

2. **Tailwind 4 Upgrade Timing**: ✅ DECIDED
   - **Decision**: Upgrade to v4 beta immediately (Option A)
   - **Rationale**: Use v4 syntax from the start, avoid migration later
   - **Risk Mitigation**: Pin specific beta version, maintain v3 config in git history

3. **Mood Scale Migration**: ✅ DECIDED
   - **Decision**: Linear stretch mapping (1→1, 2→3, 3→4, 4→5, 5→7)
   - **Rationale**: Maintains even spacing, though no data to migrate currently

## Open Questions (Still Pending)

1. **Icon Library Choice**: Heroicons, Lucide, or custom SVG set?
   - **Decision Needed By**: Week 4 (before Activity UI implementation)
   - **Recommendation**: Heroicons (good Svelte support, maintained by Tailwind Labs)

2. **Chart Library Choice**: Chart.js, D3.js, or Svelte-native solution?
   - **Decision Needed By**: Week 1 (US7 Dashboard)
   - **Recommendation**: Chart.js (already in project, simpler API)

3. **Activity Group Limits**: Max activities per group? Max groups per user?
   - **Decision Needed By**: Week 3 (schema design)
   - **Recommendation**: Soft limits via UI (20 groups, 50 activities/group)

4. **Foreign Key Enforcement**: Where to set PRAGMA foreign_keys = ON?
   - **Decision Needed By**: Week 3 (before migrations)
   - **Recommendation**: Database connection initialization (once, not per migration)

---

## Next Steps

1. **Review this plan** with stakeholders/team
2. **Make decisions** on open questions (icon library, chart library)
3. **Create task branch**: `git checkout -b feature/major-refactoring-2025`
4. **Begin Phase 1, Week 1**: Start with US7 Dashboard backend queries
5. **Setup tracking**: Copy tasks from this plan to `tasks.md` with checkboxes

---

## References

- Original Spec: `/home/dwalleck/repos/mental-health-bar-rs/specs/001-mental-health-tracking/spec.md`
- Task Status: `/home/dwalleck/repos/mental-health-bar-rs/specs/001-mental-health-tracking/tasks.md`
- New Features: `/home/dwalleck/repos/mental-health-bar-rs/new-features.md`
- Catalyst Guide: `/home/dwalleck/repos/mental-health-bar-rs/catalyst-css-extraction-guide.md`
- Project Guidelines: `/home/dwalleck/repos/mental-health-bar-rs/CLAUDE.md`
