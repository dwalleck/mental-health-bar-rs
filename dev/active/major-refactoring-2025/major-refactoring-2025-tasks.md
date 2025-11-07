# Major Refactoring 2025 - Task Checklist (Pragmatic Approach)

**Created**: 2025-11-06
**Updated**: 2025-11-06 (Pragmatic Overhaul)
**Total Releases**: 5 (v0.1, v0.2, v0.3, v0.4, v1.0)
**Estimated Duration**: 10 weeks + 1 day
**Approach**: Ship value every 1-2 weeks, tests integrated with features

**See**: `PRAGMATIC-PRINCIPLES.md` for philosophy

---

## Week 0: Validation Sprint (1 day, 8 hours) - Prove It Works

**Goal**: Validate riskiest technical assumptions BEFORE building

**Why**: "Don't spend 10 weeks building only to discover Chart.js doesn't work" - Sam Rivera

### Validation Tasks (Must Complete Before Week 1)

- [ ] 0.1 **Validate Chart.js + Svelte 5 + Tauri** (2 hrs)
  - [ ] 0.1a Create test component with ONE line chart
  - [ ] 0.1b Pass mock PHQ-9 data from Rust backend → frontend
  - [ ] 0.1c Verify chart renders correctly
  - [ ] 0.1d **Success**: Chart displays data OR **Pivot**: Choose alternative library

- [ ] 0.2 **Upgrade to Tailwind v4 Beta** (3 hrs)
  - [ ] 0.2a Tag current release with `git tag pre-tailwind-v4`
  - [ ] 0.2b Backup `tailwind.config.js` to separate branch
  - [ ] 0.2c Upgrade `tailwindcss` to v4 beta (pin version: `4.1.13` or latest)
  - [ ] 0.2d Run `npm install` and `npm run build`
  - [ ] 0.2e Test existing components still render
  - [ ] 0.2f **Success**: Build passes OR **Rollback**: Revert to v3

- [ ] 0.3 **Commit to Heroicons** (30 min)
  - [ ] 0.3a Install `@heroicons/svelte` via npm
  - [ ] 0.3b Create test component with 3 icons (e.g., home, chart, activity)
  - [ ] 0.3c **Success**: Icons render OR **Pivot**: Use alternative library

- [ ] 0.4 **Build End-to-End Proof-of-Concept** (2.5 hrs)
  - [ ] 0.4a Complete flow: Submit PHQ-9 assessment
  - [ ] 0.4b Store assessment in database
  - [ ] 0.4c Query assessment history from Rust
  - [ ] 0.4d Display result in Chart.js line chart
  - [ ] 0.4e **Success**: Full stack integration works

**Week 0 Outcome**: ✅ Confidence to proceed OR ❌ Discovered blocker, adjust plan

---

## Phase 1 (v0.1): Dashboard (Week 1) → SHIP IT

**Goal**: Users can view assessment trends with charts

**Duration**: 1 week (25 hours)

### Week 1: Dashboard Implementation (User-Outcome Focused)

**Pragmatic Note**: Tests written AS features are built (TDD), not separate phase.

#### User Outcome 1: Users can view assessment history (8 hrs)
- [ ] 1.1 **Build complete feature**: Assessment history with filtering (includes backend + frontend + tests)
  - [ ] 1.1a Write test for `get_assessment_history` repository method (TDD: red)
  - [ ] 1.1b Implement `get_assessment_history` with date filtering (TDD: green)
  - [ ] 1.1c Create Tauri command `get_assessment_history`
  - [ ] 1.1d Create `AssessmentHistoryList.svelte` with filters
  - [ ] 1.1e Generate TypeScript bindings
  - [ ] 1.1f Verify tests pass (TDD: refactor if needed)

#### User Outcome 2: Users can see trend charts (6 hrs)
- [ ] 1.2 **Build complete feature**: Trend visualization (includes backend + frontend + tests)
  - [ ] 1.2a Write test for `get_assessment_trends` repository method (TDD: red)
  - [ ] 1.2b Implement trend calculation logic (average, min, max per period) (TDD: green)
  - [ ] 1.2c Create Tauri command `get_assessment_trends`
  - [ ] 1.2d Create `AssessmentTrendChart.svelte` (line chart)
  - [ ] 1.2e Verify tests pass and chart renders <500ms

#### User Outcome 3: Users can filter by date range (3 hrs)
- [ ] 1.3 **Build complete feature**: Date filtering (includes UI + backend integration + tests)
  - [ ] 1.3a Create date range picker component
  - [ ] 1.3b Integrate with assessment history query
  - [ ] 1.3c Add preset filters (30/60/90 days)
  - [ ] 1.3d Test filtering works correctly

#### User Outcome 4: Users can see mood correlation (6 hrs)
- [ ] 1.4 **Build complete feature**: Mood-activity correlation (includes backend + frontend + tests)
  - [ ] 1.4a Write test for `get_mood_correlation` repository method (TDD: red)
  - [ ] 1.4b Implement correlation calculation (TDD: green)
  - [ ] 1.4c Create Tauri command `get_mood_correlation`
  - [ ] 1.4d Create `MoodCorrelationChart.svelte` (bar chart)
  - [ ] 1.4e Verify tests pass

#### Dashboard Integration (2 hrs)
- [ ] 1.5 **Complete dashboard page**
  - [ ] 1.5a Create `/routes/dashboard/+page.svelte` with layout
  - [ ] 1.5b Add navigation link to dashboard
  - [ ] 1.5c Add loading states and error handling
  - [ ] 1.5d Test responsive design (mobile, tablet, desktop)

**Week 1 Total**: ~25 hours

**v0.1 Shippable Criteria**:
- ✅ Users can view assessment history (PHQ-9, GAD-7, CES-D, OASIS)
- ✅ Users can see trend charts
- ✅ Date filtering works (30/60/90 days)
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Chart rendering <500ms
- 🚀 **SHIP v0.1 to users**

---

## Phase 2 (v0.2): Activity Groups (Weeks 2-5) → SHIP IT

**Note**: Week 2 test coverage phase DELETED (tests integrated into features via TDD)

**Duration**: 4 weeks (97 hours)

### Week 2: Database Schema and Repository Layer

#### Database Migration
- [ ] 2.1 Create migration file `003_activity_groups.sql` (1 hr)
- [ ] 2.1a **CRITICAL**: Drop existing `activities` table (no user data to preserve) (30 min)
- [ ] 2.2 Define `activity_groups` table with schema (30 min)
- [ ] 2.3 Define new `activities` table with FK to groups (30 min)
- [ ] 2.4 Define `activity_logs` table with FK to activities (30 min)
- [ ] 2.5 Define `activity_goals` table with conditional FK (1 hr)
- [ ] 2.6 Add CHECK constraints for validation (30 min)
- [ ] 2.7 Add partial indexes for soft deletes (30 min)
- [ ] 2.8 Test migration with rollback script (1 hr)

#### Repository: Activity Groups
- [ ] 2.9 Create `features/activities/models.rs` with domain types (1 hr)
- [ ] 2.10 Create `features/activities/repository.rs` stub (30 min)
- [ ] 2.11 Write test for `create_activity_group` (TDD: write first) (1 hr)
- [ ] 2.12 Implement `create_activity_group` method (1 hr)
- [ ] 2.13 Verify test passes (green) (30 min)
- [ ] 2.14 Write test for `update_activity_group` (1 hr)
- [ ] 2.15 Implement `update_activity_group` method (1 hr)
- [ ] 2.16 Write test for `delete_activity_group` (soft delete) (1 hr)
- [ ] 2.17 Implement `delete_activity_group` method (1 hr)
- [ ] 2.18 Write test for `get_activity_groups` (exclude deleted) (1 hr)
- [ ] 2.19 Implement `get_activity_groups` method (1 hr)

#### Repository: Activities
- [ ] 2.20 Write test for `create_activity` (1 hr)
- [ ] 2.21 Implement `create_activity` method (1 hr)
- [ ] 2.22 Write test for `update_activity` (1 hr)
- [ ] 2.23 Implement `update_activity` method (1 hr)
- [ ] 2.24 Write test for `delete_activity` (soft delete) (1 hr)
- [ ] 2.25 Implement `delete_activity` method (1 hr)
- [ ] 2.26 Write test for `get_activities_by_group` (1 hr)
- [ ] 2.27 Implement `get_activities_by_group` method (1 hr)

#### Repository: Activity Logging
- [ ] 2.28 Write test for `log_activity` (1 hr)
- [ ] 2.29 Implement `log_activity` method (1 hr)
- [ ] 2.30 Write test for `get_activity_logs` with date filtering (1 hr)
- [ ] 2.31 Implement `get_activity_logs` method (1 hr)

**Week 2 Total**: ~27 hours

---

### Week 3: Backend Commands and Reporting Logic

#### Tauri Commands: CRUD Operations
- [ ] 3.1 Create `features/activities/commands.rs` (30 min)
- [ ] 3.2 Implement `create_activity_group` command (1 hr)
- [ ] 3.3 Implement `update_activity_group` command (1 hr)
- [ ] 3.4 Implement `delete_activity_group` command (1 hr)
- [ ] 3.5 Implement `get_activity_groups` query (1 hr)
- [ ] 3.6 Implement `create_activity` command (1 hr)
- [ ] 3.7 Implement `update_activity` command (1 hr)
- [ ] 3.8 Implement `delete_activity` command (1 hr)
- [ ] 3.9 Implement `get_activities_by_group` query (1 hr)
- [ ] 3.10 Implement `log_activity` command (1 hr)
- [ ] 3.11 Implement `get_activity_logs` query (1 hr)

#### Activity Goals
- [ ] 3.12 Write repository test for `set_activity_goal` (1 hr)
- [ ] 3.13 Implement `set_activity_goal` repository method (2 hrs)
- [ ] 3.14 Write repository test for `get_activity_goals` (1 hr)
- [ ] 3.15 Implement `get_activity_goals` repository method (1 hr)
- [ ] 3.16 Implement `set_activity_goal` Tauri command (1 hr)
- [ ] 3.17 Implement `get_activity_goals` Tauri query (1 hr)

#### Reporting Logic
- [ ] 3.18 Write repository test for `get_activity_frequency` (1 hr)
- [ ] 3.19 Implement `get_activity_frequency` (days per week calculation) (2 hrs)
- [ ] 3.20 Write repository test for `get_activity_trend` (1 hr)
- [ ] 3.21 Implement `get_activity_trend` (percent change calculation) (2 hrs)
- [ ] 3.22 Write repository test for `check_goal_progress` (1 hr)
- [ ] 3.23 Implement `check_goal_progress` (compare actual vs target) (2 hrs)
- [ ] 3.24 Implement `get_activity_frequency` Tauri query (1 hr)
- [ ] 3.25 Implement `get_activity_trend` Tauri query (1 hr)
- [ ] 3.26 Implement `check_goal_progress` Tauri query (1 hr)

#### Bindings and Testing
- [ ] 3.27 Generate TypeScript bindings with `tauri-specta` (30 min)
- [ ] 3.28 Write command tests for all CRUD operations (2 hrs)
- [ ] 3.29 Write command tests for reporting queries (2 hrs)
- [ ] 3.30 Test error handling (invalid IDs, missing FKs) (1 hr)

**Week 3 Total**: ~34 hours

---

### Week 4: Frontend UI Components

**Note**: Heroicons already installed and validated in Week 0.

#### Activity Group Management Page
- [ ] 4.1 Create `/routes/activities/+page.svelte` (30 min)
- [ ] 4.2 Create activity group list component (1 hr)
- [ ] 4.3 Add expand/collapse for each group (1 hr)
- [ ] 4.4 Create "Add Group" modal component (2 hrs)
- [ ] 4.5 Create "Edit Group" modal component (1 hr)
- [ ] 4.6 Implement delete group with confirmation dialog (1 hr)
- [ ] 4.7 Add drag-and-drop reordering (optional, advanced) (3 hrs)

#### Activity Management UI
- [ ] 4.8 Create activity list within group (1 hr)
- [ ] 4.9 Create "Add Activity" modal component (2 hrs)
- [ ] 4.10 Create icon picker component using Heroicons (dropdown or grid) (3 hrs)
- [ ] 4.11 Create "Edit Activity" modal component (1 hr)
- [ ] 4.12 Implement delete activity with confirmation (1 hr)

#### Activity Logging Interface
- [ ] 4.13 Create quick log button for each activity (1 hr)
- [ ] 4.14 Create activity log history view (table or timeline) (2 hrs)
- [ ] 4.15 Add date filtering for log history (1 hr)
- [ ] 4.16 Implement "Add Note" feature for logs (1 hr)

#### Goal Setting UI
- [ ] 4.17 Create goal setting modal component (2 hrs)
- [ ] 4.18 Implement goal type selector (radio buttons) (1 hr)
- [ ] 4.19 Create target value input with validation (1 hr)
- [ ] 4.20 Create period selector (dropdown: week, month, custom) (1 hr)
- [ ] 4.21 Create goal progress indicator (progress bar or percentage) (2 hrs)
- [ ] 4.22 Display active goals for each activity/group (1 hr)

#### Integration and Testing
- [ ] 4.23 Connect all components to Tauri commands via `lib/bindings.ts` (2 hrs)
- [ ] 4.24 Add loading states for all async operations (1 hr)
- [ ] 4.25 Add error handling with user-friendly messages (1 hr)
- [ ] 4.26 Test responsive design (mobile, tablet, desktop) (2 hrs)
- [ ] 4.27 Write component tests for critical UI (3 hrs)

**Week 4 Total**: ~36 hours

**v0.2 Shippable Criteria**:
- ✅ Users can create activity groups and activities
- ✅ Users can log activities with notes
- ✅ Users can set goals and track progress
- ✅ Reporting shows days/week and % change
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Activity list loads <200ms
- 🚀 **SHIP v0.2 to users**

---

## Phase 3 (v0.3): Check-In v2.0 (Week 5) → SHIP IT

### Database Migration: Mood Scale 1-7
- [ ] 5.1 Create migration file `004_mood_scale_migration.sql` (30 min)
- [ ] 5.2 Update CHECK constraint to 1-7 range (no data migration needed) (30 min)
- [ ] 5.3 Test migration (1 hr)
- [ ] 5.4 Create rollback script (30 min)

**Note**: No user data exists. Linear stretch mapping (1→1, 2→3, 3→4, 4→5, 5→7) documented for future reference only.

### Database Schema: Link Check-Ins to Activities
- [ ] 5.5 Create junction table `mood_checkin_activities` (30 min)
- [ ] 5.6 Add foreign keys to mood_checkins and activities (30 min)
- [ ] 5.7 Update repository to insert activity associations (1 hr)
- [ ] 5.8 Write tests for activity association logic (1 hr)

### Backend: Update Validation
- [ ] 5.9 Update mood rating validation to 1-7 range (30 min)
- [ ] 5.10 Update `MoodCheckinRequest` struct to include `activity_ids` (30 min)
- [ ] 5.11 Update `submit_mood_checkin` command to save activity associations (1 hr)
- [ ] 5.12 Update `get_mood_history` query to include activities (1 hr)
- [ ] 5.13 Generate TypeScript bindings (30 min)
- [ ] 5.14 Write command tests for new behavior (1 hr)

### Frontend: Mood Selector Update
- [ ] 5.15 Update mood selector component to 7-point scale (2 hrs)
- [ ] 5.16 Design visual representation (emoji, slider, or buttons) (1 hr)
- [ ] 5.17 Add accessibility labels for each level (1 hr)
- [ ] 5.18 Test keyboard navigation for 7-point selector (1 hr)

### Frontend: Activity Integration
- [ ] 5.19 Create `ActivitySelector.svelte` component (2 hrs)
- [ ] 5.20 Group activities by Activity Group (collapsible sections) (2 hrs)
- [ ] 5.21 Display icons instead of names (using Heroicons) (1 hr)
- [ ] 5.22 Implement multi-select for activities (checkboxes) (1 hr)
- [ ] 5.23 Integrate ActivitySelector into check-in form (1 hr)

### Frontend: Check-In History Update
- [ ] 5.24 Update check-in history to display 7-point scale (1 hr)
- [ ] 5.25 Display associated activities with icons (1 hr)

### Testing and Validation
- [ ] 5.26 Test end-to-end check-in flow (1 hr)
- [ ] 5.27 Test activity selection with 0, 1, many activities (1 hr)
- [ ] 5.28 Write component tests for updated mood selector (1 hr)

**Week 5 Total**: ~26 hours

**v0.3 Shippable Criteria**:
- ✅ Users can rate mood on 7-point scale
- ✅ Users select activities grouped by category
- ✅ Icons display correctly for all activities
- ✅ Multi-select works smoothly
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Check-in response <100ms
- 🚀 **SHIP v0.3 to users**

---

## Phase 4 (v0.4): UI Refresh with Catalyst (Weeks 6-7) → SHIP IT

**Note**: Tailwind v4 already upgraded in Week 0. This phase focuses on Catalyst component migration.

### Week 6: Design Tokens and Core Components

#### Phase 4.1: Foundation with v4 Syntax (3-4 hours)
- [ ] 6.1 Update `tailwind.config.js` with Catalyst zinc color palette (v4 format) (1 hr)
- [ ] 6.2 Add Catalyst typography scale using v4 syntax (`text-sm/6`, `text-base/6`) (30 min)
- [ ] 6.3 Add spacing refinements (`3.5: 0.875rem`) (30 min)
- [ ] 6.4 Update border radius values (30 min)
- [ ] 6.5 Test dark mode compatibility with new tokens and v4 (1 hr)
- [ ] 6.6 Document design tokens in project docs (30 min)

#### Phase 4.2: Button Component (3-4 hours)
- [ ] 6.7 Create `lib/components/ui/Button.svelte` with Catalyst styling (2 hrs)
- [ ] 6.8 Implement variants (solid, outline, plain) (1 hr)
- [ ] 6.9 Implement colors (zinc, blue, red) (1 hr)
- [ ] 6.10 Add proper focus rings and transitions (1 hr)
- [ ] 6.11 Add dark mode support (30 min)
- [ ] 6.12 Write component tests for Button (1 hr)
- [ ] 6.13 Document Button usage with examples (30 min)

#### Phase 4.3: Input Component (2-3 hours)
- [ ] 6.14 Create `lib/components/ui/Input.svelte` with Catalyst styling (2 hrs)
- [ ] 6.15 Add label support (30 min)
- [ ] 6.16 Implement error state styling (30 min)
- [ ] 6.17 Add hover and focus states (30 min)
- [ ] 6.18 Add dark mode support (30 min)
- [ ] 6.19 Write component tests for Input (1 hr)
- [ ] 6.20 Document Input usage with examples (30 min)

#### Phase 4.4: Select and Card Components (2-3 hours)
- [ ] 6.21 Create `lib/components/ui/Select.svelte` with Catalyst styling (1 hr)
- [ ] 6.22 Create `lib/components/ui/Card.svelte` with Catalyst styling (1 hr)
- [ ] 6.23 Test accessibility (keyboard navigation) (1 hr)
- [ ] 6.24 Write component tests (1 hr)

**Week 6 Total**: ~12 hours

---

### Week 7: Component Migration and Polish

#### Phase 4.5: Component Migration (4-6 hours)
- [ ] 7.1 Migrate assessment forms to new Input components (2 hrs)
- [ ] 7.2 Migrate all buttons to new Button component (2 hrs)
- [ ] 7.3 Migrate dashboard to new Card components (1 hr)
- [ ] 7.4 Migrate activity management UI to new components (2 hrs)
- [ ] 7.5 Update navigation with new Button styles (1 hr)
- [ ] 7.6 Test all pages for visual consistency (1 hr)

#### Phase 4.6: v4 Syntax Migration in Existing Components (3-4 hours)
**Note**: Tailwind v4 already upgraded in Week 0. This phase updates existing component syntax.
- [ ] 7.7 Audit existing components for v3 syntax (30 min)
- [ ] 7.8 Update syntax in existing components (bg-(--var) → bg-[var(--var)]) (2 hrs)
- [ ] 7.9 Run build and fix any Tailwind v4 errors (1 hr)
- [ ] 7.10 Test all components for visual regressions (1 hr)

#### Phase 4.7: Accessibility and Performance (2-4 hours)
- [ ] 7.11 Run accessibility audit with Axe DevTools (1 hr)
- [ ] 7.12 Fix keyboard navigation issues found (2 hrs)
- [ ] 7.13 Optimize chart rendering performance (1 hr)
- [ ] 7.14 Test responsive design on mobile, tablet, desktop (2 hrs)
- [ ] 7.15 Run Lighthouse performance audit (30 min)
- [ ] 7.16 Fix performance issues (target >90 score) (2 hrs)

**Week 7 Total**: ~18 hours

**v0.4 Shippable Criteria**:
- ✅ All components use Catalyst-inspired design
- ✅ Button, Input, Select, Card components migrated
- ✅ Tailwind v4 syntax applied throughout
- ✅ Dark mode functional
- ✅ Accessibility score >90 (Lighthouse)
- ✅ Tests pass, visual consistency verified
- ✅ Performance: No regressions from UI changes
- 🚀 **SHIP v0.4 to users**

---

## Phase 5 (v1.0): Production Polish (Weeks 8-9) → SHIP IT

**Goal**: Deliver stable, polished, production-ready mental health tracking application

### Week 8: Integration Testing and Bug Fixes

#### End-to-End Testing Scenarios
- [ ] 8.1 Test complete PHQ-9 assessment flow end-to-end (1 hr)
- [ ] 8.2 Test complete GAD-7 assessment flow end-to-end (1 hr)
- [ ] 8.3 Test daily check-in with 7-point scale and activity logging (1 hr)
- [ ] 8.4 Test activity group creation → add activities → log activity (1 hr)
- [ ] 8.5 Test goal setting → track progress → view in dashboard (1 hr)
- [ ] 8.6 Test dashboard visualization with all data types (1 hr)
- [ ] 8.7 Test scheduling and reminder system (1 hr)

#### Cross-Browser Testing
- [ ] 8.8 Test on Chrome (latest) (1 hr)
- [ ] 8.9 Test on Firefox (latest) (1 hr)
- [ ] 8.10 Test on Safari (latest, if available) (1 hr)
- [ ] 8.11 Test on Edge (latest) (1 hr)

#### Performance Testing
- [ ] 8.12 Test chart rendering with 100 data points (30 min)
- [ ] 8.13 Test chart rendering with 1000 data points (30 min)
- [ ] 8.14 Test database query performance with 1000+ records (1 hr)
- [ ] 8.15 Profile slow operations with browser DevTools (1 hr)
- [ ] 8.16 Optimize identified bottlenecks (2 hrs)

#### Bug Triage and Fixes
- [ ] 8.17 Create bug tracking list (in GitHub Issues or similar) (1 hr)
- [ ] 8.18 Prioritize bugs (P0, P1, P2) (1 hr)
- [ ] 8.19 Fix all P0 bugs (blocking issues) (8 hrs)
- [ ] 8.20 Fix all P1 bugs (high priority) (6 hrs)
- [ ] 8.21 Defer P2 bugs to post-launch (30 min)

**Week 8 Total**: ~33 hours

---

### Week 9: Documentation and Deployment Prep

#### User Documentation
- [ ] 9.1 Update README.md with new features (1 hr)
- [ ] 9.2 Write user guide for Activity Groups (2 hrs)
- [ ] 9.3 Write user guide for Goal Setting (2 hrs)
- [ ] 9.4 Write user guide for Dashboard (2 hrs)
- [ ] 9.5 Create screenshots for documentation (1 hr)

#### Developer Documentation
- [ ] 9.6 Update CLAUDE.md if needed (architectural changes) (1 hr)
- [ ] 9.7 Document new database schema (ERD or text) (1 hr)
- [ ] 9.8 Document new Tauri commands (API reference) (1 hr)
- [ ] 9.9 Update component library documentation (1 hr)

#### Migration Guide (Future-Proofed)
**Note**: No users exist yet, but documenting migration strategy for future reference
- [ ] 9.10 Write migration guide for future users (if applicable) (2 hrs)
- [ ] 9.11 Document mood scale 1-7 rationale and mapping formula (1 hr)
- [ ] 9.12 Document rollback procedures (1 hr)

#### Release Preparation
- [ ] 9.13 Draft release notes (1 hr)
- [ ] 9.14 Update version number in `Cargo.toml` and `package.json` (30 min)
- [ ] 9.15 Create git tag for release (30 min)
- [ ] 9.16 Build production binaries (Windows, macOS, Linux) (2 hrs)
- [ ] 9.17 Test production builds on each platform (2 hrs)

#### Final QA Pass
- [ ] 9.18 Full regression testing (all features) (4 hrs)
- [ ] 9.19 Final accessibility audit (1 hr)
- [ ] 9.20 Final performance audit (1 hr)
- [ ] 9.21 Final security review (check for SQL injection, XSS, etc.) (1 hr)
- [ ] 9.22 Sign-off from stakeholders (if applicable) (1 hr)

**Week 9 Total**: ~30 hours

**v1.0 Shippable Criteria**:
- ✅ All features from v0.1-v0.4 integrated
- ✅ E2E tests pass (all user flows work)
- ✅ No P0/P1 bugs
- ✅ Documentation complete
- ✅ Performance targets met (<100ms UI, <500ms charts)
- ✅ Accessibility validated
- ✅ Ready for real users
- 🚀 **SHIP v1.0 to production**

---

## Summary by Phase (Pragmatic Approach)

| Release | Duration | Tasks | Estimated Hours | Ship? |
|---------|----------|-------|-----------------|-------|
| **Week 0**: Validation Sprint | 1 day | 4 tasks | 8 hours | Proof-of-concept |
| **v0.1**: Dashboard | 1 week | ~18 tasks | ~22 hours | ✅ **SHIP IT** |
| **v0.2**: Activity Groups | 4 weeks (Weeks 2-5) | ~82 tasks | ~115 hours | ✅ **SHIP IT** |
| **v0.3**: Check-In v2.0 | 1 week (Week 5) | ~28 tasks | ~26 hours | ✅ **SHIP IT** |
| **v0.4**: UI Refresh | 2 weeks (Weeks 6-7) | ~40 tasks | ~30 hours | ✅ **SHIP IT** |
| **v1.0**: Production Polish | 2 weeks (Weeks 8-9) | ~43 tasks | ~63 hours | ✅ **SHIP IT** |
| **Total** | **10 weeks + 1 day** | **~215 tasks** | **~264 hours** | **5 releases** |

**Key Changes from Original Plan**:
- ✅ **Week 0 added**: Validate Chart.js, Tailwind v4, Heroicons BEFORE building
- ❌ **Week 2 deleted**: Test coverage phase removed (tests now integrated via TDD)
- 🚀 **5 shippable releases**: Users see value every 1-2 weeks
- 📊 **User-outcome focused**: Tasks framed as user value, not technical steps
- ⚡ **Faster delivery**: Ship Dashboard v0.1 in Week 1 (not Week 10)

**Assumptions**:
- Single full-time developer (40 hours/week)
- TDD approach: Tests written AS features are built (not separate phase)
- Estimates include testing, debugging, and documentation
- Week 0 validation prevents late-stage failures (Chart.js, Tailwind v4)

---

## Progress Tracking

### Release Completion Status
- [ ] **Week 0**: Validation Sprint Complete (0/4 tasks)
- [ ] **v0.1**: Dashboard Shipped (0/~18 tasks) - End of Week 1
- [ ] **v0.2**: Activity Groups Shipped (0/~82 tasks) - End of Week 5
- [ ] **v0.3**: Check-In v2.0 Shipped (0/~28 tasks) - End of Week 5
- [ ] **v0.4**: UI Refresh Shipped (0/~40 tasks) - End of Week 7
- [ ] **v1.0**: Production Shipped (0/~43 tasks) - End of Week 9

### Shippable Milestone Checklist
- [ ] **Week 0**: Technical risks validated (Chart.js, Tailwind v4, Heroicons)
- [ ] **v0.1 SHIPPED**: Users can view assessment trends (Week 1)
- [ ] **v0.2 SHIPPED**: Users can track activities with goals (Week 5)
- [ ] **v0.3 SHIPPED**: Users get 7-point mood scale + grouped activities (Week 5)
- [ ] **v0.4 SHIPPED**: Users see modern Catalyst UI (Week 7)
- [ ] **v1.0 SHIPPED**: Stable, production-ready application (Week 9)

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

- Update this file weekly with completed tasks
- Mark tasks with `[X]` when complete
- Add new tasks as discovered during implementation
- Track blockers in a separate issues list
- Review estimates weekly and adjust timeline if needed
