# Major Refactoring 2025 - Context Document

**Created**: 2025-11-06
**Plan Version**: 1.0

This document provides essential context, critical decisions, and reference information for the major refactoring project.

---

## Project Overview

### What We're Building
A comprehensive mental health tracking application with clinical assessments, daily mood check-ins, and activity tracking with goal setting.

### Why This Refactoring
1. **Complete Unfinished Work**: US7 Dashboard and test coverage gaps from original spec
2. **New Feature Requirements**: Activity Groups hierarchy, UI redesign, improved check-in flow
3. **Technical Debt**: Upgrade to Tailwind 4, improve component consistency
4. **User Experience**: Modern UI, better visualization, more intuitive workflows

---

## Development Approach: Pragmatic Principles

**Philosophy**: "Shipping beats planning. Real feedback beats theoretical design." - Sam Rivera

This refactoring follows **pragmatic development principles** to deliver user value incrementally while managing technical risk.

### Key Principles Applied

1. **Ship Early, Ship Often**
   - Version 0.1 doesn't need to be perfect
   - **Week 1**: Ship Dashboard v0.1 (users see value immediately)
   - **Week 5**: Ship Activity Groups v0.2 (complete but validated through usage)
   - **Week 6**: Ship Check-In v0.3 (improved UX iteration)
   - **Week 8**: Ship UI v0.4 (visual refresh with feedback)

2. **Validate Risks Before Building**
   - **Week 0**: Validation Sprint
     - Prove Chart.js works with Svelte 5 + Tauri (build example)
     - Upgrade Tailwind v4 beta NOW, validate or rollback
     - Commit to Heroicons (remove decision paralysis)
     - Build end-to-end proof-of-concept
   - **Not**: Discover Chart.js doesn't work in Week 2, or Tailwind v4 breaks in Week 7

3. **Make It Work, Make It Right, Make It Fast** (in that order)
   - **v0.1**: Dashboard works with basic charts (make it work)
   - **v0.2-v0.3**: Refine UX based on feedback (make it right)
   - **v0.4-v1.0**: Optimize performance, polish UI (make it fast)

4. **TDD Means Tests First**
   - Tests written AS features are built (red → green → refactor)
   - No separate "catch-up on tests" phase (Week 2 deleted)
   - Tests integrated into feature work (Weeks 1-10)

5. **YAGNI (You Aren't Gonna Need It)**
   - Build what's explicitly requested in `new-features.md`
   - No plugin architecture, API versioning, or advanced features "for later"
   - Standard charts only (no custom visualizations yet)

### Iterative Release Strategy

| Release | Week | User Value Delivered | Ship? |
|---------|------|---------------------|-------|
| **v0.1** | Week 1 | Users can view assessment trends | ✅ **SHIP IT** |
| **v0.2** | Week 5 | Users can track activities with goals | ✅ **SHIP IT** |
| **v0.3** | Week 5 | Users get better check-in UX (7-point scale) | ✅ **SHIP IT** |
| **v0.4** | Week 7 | Users see modern Catalyst UI | ✅ **SHIP IT** |
| **v1.0** | Week 9 | Stable, production-ready application | ✅ **SHIP IT** |

**Key Difference**: Each milestone = **shippable user value**, not "phase complete"

### Decision Framework

When faced with a choice:
1. **Does this block shipping?** → Yes: Fix now | No: Defer to next iteration
2. **Have we validated this works?** → Yes: Proceed | No: Spike first (2-4 hours max)
3. **Will users notice this?** → Yes: Prioritize | No: Defer or cut
4. **Can we iterate later?** → Yes: Ship minimal version now | No: Build it right first

**See**: `PRAGMATIC-PRINCIPLES.md` for full philosophy and reference card

---

## Key Files and Their Purposes

### Specification Documents

**`/specs/001-mental-health-tracking/spec.md`**
- **Purpose**: Original feature specification with user stories
- **Status**: ~65% complete (Phases 1-8 mostly done)
- **Key Sections**:
  - US1: Complete Clinical Assessment (PHQ-9, GAD-7, CES-D, OASIS) ✅
  - US2: Quick Daily Mood Check-In ✅
  - US3: Manage Personal Activities ✅ (will be expanded in Phase 2)
  - US4: Schedule Assessments ✅
  - US5: Assessment Reminders ✅
  - US6: Data Visualization ⚠️ (partially complete)
  - US7: Dashboard ❌ (not started - 22 tasks)

**`/specs/001-mental-health-tracking/tasks.md`**
- **Purpose**: Checklist of all implementation tasks (250 tasks total)
- **Status**: Phases 1-8 mostly complete, Phase 8.5 (US7) not started
- **Usage**: Track completion with `[X]` checkboxes
- **Key Incomplete Sections**:
  - Phase 7.5: Test Coverage Gaps (P0 tasks)
  - Phase 8.5: US7 Dashboard (all 22 tasks)
  - Phase 9: Polish (partially complete)

**`/new-features.md`**
- **Purpose**: New requirements not in original spec
- **Key Features**:
  1. UI redesign with Catalyst UI Kit
  2. Activity Groups (2-level hierarchy)
  3. Daily Check-In changes (1-7 scale, icons, grouping)

**`/catalyst-css-extraction-guide.md`**
- **Purpose**: Analysis of Catalyst UI Kit integration approaches
- **Recommendation**: Selective Extraction (16-24 hours effort)
  - Extract design tokens
  - Enhance 3-5 key components
  - Stay on Tailwind v3 initially
  - 70-80% visual parity with Catalyst

### Project Configuration

**`/CLAUDE.md`**
- **Purpose**: Project coding guidelines and development practices
- **Key Sections**:
  - Rust coding standards
  - Database development patterns (SQLite + rusqlite)
  - Svelte 5 component guidelines
  - Error handling patterns (`CommandError`, `ToCommandError` trait)
  - TDD requirements

**`/tailwind.config.js`**
- **Purpose**: Tailwind CSS configuration
- **Status**: Currently Tailwind v3
- **Planned Changes** (Phase 4):
  - Add Catalyst design tokens (zinc palette, typography scale)
  - Add spacing refinements (`3.5: 0.875rem`)
  - Upgrade to v4 syntax

**`/Cargo.toml`** (Backend)
- **Purpose**: Rust dependencies
- **Key Dependencies**:
  - `tauri` (desktop app framework)
  - `rusqlite` (SQLite database)
  - `thiserror` (error handling)
  - `serde` (serialization)
  - `tauri-specta` (TypeScript binding generation)

**`/package.json`** (Frontend)
- **Purpose**: Node.js dependencies
- **Key Dependencies**:
  - `svelte` 5.x (with runes)
  - `@sveltejs/kit` (full-stack framework)
  - `tailwindcss` (styling)
  - `@tauri-apps/api` (Tauri frontend bindings)

### Source Code Structure

**Backend (`/src-tauri/src/`)**
```
src-tauri/src/
├── main.rs                 # Application entry point
├── db/
│   ├── mod.rs             # Database initialization, migrations
│   └── schema.sql         # Database schema
├── features/
│   ├── assessments/       # Clinical assessment system
│   │   ├── mod.rs
│   │   ├── models.rs      # Domain types
│   │   ├── commands.rs    # Tauri commands (mutations)
│   │   ├── queries.rs     # Tauri queries (reads)
│   │   └── repository.rs  # Database access
│   ├── mood/              # Daily mood check-ins
│   │   └── [same structure]
│   ├── scheduling/        # Assessment scheduling
│   │   └── [same structure]
│   └── visualization/     # Chart data aggregation
│       └── [same structure]
└── utils/
    ├── errors.rs          # CommandError struct, ToCommandError trait
    └── validation.rs      # Input validation helpers
```

**Frontend (`/src/`)**
```
src/
├── routes/                # SvelteKit file-based routing
│   ├── +layout.svelte    # Root layout
│   ├── +page.svelte      # Home page
│   ├── assessments/      # Assessment pages
│   ├── mood/             # Mood check-in pages
│   ├── activities/       # Activity management (to be created)
│   └── dashboard/        # Dashboard (to be created)
├── lib/
│   ├── components/       # Svelte components
│   │   ├── ui/          # Reusable UI components (Button, Input, etc.)
│   │   ├── mood/        # Mood-specific components
│   │   ├── assessments/ # Assessment forms
│   │   └── charts/      # Visualization components (to be created)
│   ├── stores/          # Svelte stores for state management
│   ├── utils/           # Frontend utilities
│   └── bindings.ts      # Auto-generated Tauri types (from tauri-specta)
└── app.css              # Global styles, Tailwind imports
```

---

## Critical Decisions

### Decision 0: No User Data Exists (Simplifies Everything)
**Status**: ✅ CONFIRMED
**Impact**:
- No data migration concerns for any schema changes
- Can drop and recreate tables freely
- No backward compatibility requirements
- Simplified testing (no migration validation needed)

### Decision 1: Complete Existing Work Before New Features
**Status**: ✅ Decided
**Rationale**:
- Reduces risk of abandoning partially complete work
- Provides stable foundation for new features
- US7 Dashboard is P2 priority (only 22 tasks)
- Test coverage gaps are P0 priority (critical for stability)

**Impact**:
- Phase 1 focuses on US7 and test coverage (2 weeks)
- New features start in Phase 2 (Week 3)

### Decision 2: Activity Groups Schema Design
**Status**: ✅ DECIDED
**Options Considered**:
1. **Single table with self-referencing FK** (simpler)
2. **Separate tables for groups and activities** (chosen)

**Chosen Approach**: Separate tables, drop existing `activities` table
**Rationale**:
- Existing `activities` table conflicts with new schema
- No user data exists - can drop and recreate safely
- Clearer separation of concerns (groups have different properties than activities)
- Easier to query all activities in a group
- Better extensibility (can add group-level properties later)
- Aligns with vertical slice architecture

**Migration Plan**:
1. Drop existing `activities` table (no data loss concerns)
2. Create new `activity_groups` table
3. Create new `activities` table with `group_id` FK
4. Update all code references

**Schema** (see plan for full SQL):
```
activity_groups (id, name, description, created_at, deleted_at)
activities (id, group_id FK, name, icon, created_at, deleted_at)
activity_logs (id, activity_id FK, logged_at, notes)
activity_goals (id, activity_id FK OR group_id FK, goal_type, target_value, period_days)
```

**Key Design Choices**:
- Goals can target either an activity OR an activity group (mutually exclusive)
- Soft deletes with `deleted_at` column (preserve historical data)
- Icons stored as text identifiers (e.g., "hiking", "swimming")
- Foreign keys with CASCADE delete (maintain referential integrity)

### Decision 3: UI Migration Approach (Catalyst Integration)
**Status**: ✅ DECIDED
**Chosen**: Selective Extraction + Week 0 Tailwind v4 Beta Validation

**Options Considered**:
| Approach | Effort | Outcome | Risk |
|----------|--------|---------|------|
| Full Extraction | 40-60 hrs | Pixel-perfect, Tailwind v4 beta | HIGH (v4 beta instability) |
| **Selective Extraction + v4 (Week 0)** | **16-24 hrs** | **70-80% parity, v4 syntax** | **MEDIUM** |
| Selective + stay v3 | 16-24 hrs | 70-80% parity, stable v3 | LOW |

**Chosen**: Selective Extraction + Week 0 Tailwind v4 Beta Validation (DECISION: Option A)
**Rationale**:
- **Validate v4 in Week 0 BEFORE building features** (pragmatic risk management)
- Use v4 syntax from the start (avoid migration later)
- Matches Catalyst examples exactly
- Accepts beta risks in exchange for no future migration work
- Pin specific beta version for stability

**Week 0 Validation Process**:
1. Tag current release with `git tag pre-tailwind-v4` (create rollback point)
2. Backup `tailwind.config.js` to separate branch
3. Upgrade `tailwindcss` to v4 beta (pin version: e.g., `4.1.13`)
4. Run `npm install` and `npm run build`
5. Test existing components still render
6. **Success**: Build passes → proceed with v4 | **Failure**: Rollback to v3, stay on v3

**Risk Mitigation**:
- Week 0 validation prevents late-stage failures (don't wait until Week 7)
- Tag release before v4 upgrade (rollback point)
- Keep v3 config in git history
- Pin specific v4 beta version (e.g., 4.1.13)
- Feature flags for new components (quick disable if issues)

**Components to Extract** (Priority Order):
1. Button (solid, outline, plain variants)
2. Input (with error states, labels)
3. Select (similar to Input)
4. Card (simple, high visibility)
5. Modal/Dialog (refine styles only, keep existing logic)

### Decision 4: Mood Scale Migration (1-5 → 1-7)
**Status**: ✅ DECIDED
**Options Considered**:
1. Simple remapping with gaps (1→1, 2→2, 3→4, 4→6, 5→7)
2. **Linear stretch mapping** (1→1, 2→3, 3→4, 4→5, 5→7) ✅ CHOSEN
3. Proportional mapping (rating × 1.4, rounded)
4. No migration (start fresh)

**Chosen**: Linear stretch mapping (DECISION: Option A)
**Rationale**:
- **No user data exists** - no actual migration needed
- Maintains even spacing across the scale
- Documented for future reference if data migration ever needed
- Avoids gaps in the scale (unlike simple remapping)

**Implementation** (Phase 3):
```sql
-- Simple constraint update (no data to migrate)
ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 7);
```

**For Future Reference** (if data migration ever needed):
```sql
UPDATE mood_checkins SET mood_rating = CASE
    WHEN mood_rating = 1 THEN 1
    WHEN mood_rating = 2 THEN 3
    WHEN mood_rating = 3 THEN 4
    WHEN mood_rating = 4 THEN 5
    WHEN mood_rating = 5 THEN 7
END
WHERE mood_rating <= 5;
```

### Decision 5: Icon Library for Activities
**Status**: ✅ COMMITTED (Week 0 Validation)
**Chosen**: Heroicons

**Options Considered**:
1. **Heroicons** ✅ CHOSEN
   - Pros: Excellent Svelte support, 300+ icons, maintained by Tailwind Labs
   - Cons: None significant
2. **Lucide**
   - Pros: 1000+ icons, community-driven
   - Cons: Slightly larger bundle size
3. **Custom SVG Set**
   - Pros: Full control, minimal bundle
   - Cons: High maintenance, limited variety

**Rationale**:
- **Week 0 validation removes decision paralysis** (commit now, not defer to Week 4)
- Maintained by Tailwind Labs (matches Catalyst ecosystem)
- Good coverage of activity types (sports, hobbies, health)
- Easy Svelte integration with `@heroicons/svelte`
- Consistent style with Catalyst UI

**Week 0 Validation**:
- Install `@heroicons/svelte` via npm
- Create test component with 3 icons (e.g., home, chart, activity)
- Verify icons render correctly

**Usage Pattern**:
```svelte
<script lang="ts">
  import { Icon } from '@heroicons/svelte/24/outline';
  export let icon: string; // e.g., "AcademicCapIcon"
</script>

<Icon name={icon} class="w-6 h-6" />
```

### Decision 6: Chart Library for Dashboard
**Status**: ✅ VALIDATED (Week 0 Validation)
**Chosen**: Chart.js

**Options Considered**:
1. **Chart.js** ✅ CHOSEN
   - Pros: Simple API, good for standard charts, well-documented
   - Cons: Limited customization for complex visualizations
2. **D3.js**
   - Pros: Maximum flexibility, powerful
   - Cons: Steep learning curve, verbose code
3. **Svelte-native solutions** (e.g., LayerCake)
   - Pros: Full Svelte integration, reactive
   - Cons: Smaller ecosystem, less mature

**Rationale**:
- **Week 0 validation proves it works BEFORE building Dashboard** (pragmatic risk management)
- Dashboard needs standard charts (line, bar, pie)
- Simple API reduces implementation time
- Good Svelte wrapper available (`svelte-chartjs`)
- Sufficient for assessment trend visualization

**Week 0 Validation**:
- Build ONE working chart component (Svelte 5 + Chart.js + Tauri)
- Pass mock PHQ-9 data from Rust backend → frontend
- Verify chart renders correctly
- **Success**: Chart displays data → proceed | **Pivot**: Choose alternative library

**Usage Pattern**:
```svelte
<script lang="ts">
  import { Line } from 'svelte-chartjs';
  import { Chart, registerables } from 'chart.js';

  Chart.register(...registerables);

  const data = {
    labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'],
    datasets: [{
      label: 'PHQ-9 Scores',
      data: [12, 10, 8, 6, 5],
    }],
  };
</script>

<Line {data} />
```

---

## Dependencies Between Components

### Database Schema Dependencies
```
activity_groups
    ↓ (1:many)
activities
    ↓ (1:many)
activity_logs

activities OR activity_groups
    ↓ (1:many)
activity_goals
```

### Feature Dependencies (Iterative Approach)

**Week 0 (Validation Sprint)**:
```
Week 0: Validate risks → Blocks all other work
    ├── Validate Chart.js integration
    ├── Validate Tailwind v4 upgrade
    ├── Validate Heroicons
    └── Build E2E proof-of-concept
```

**Release Dependencies**:
```
v0.1 (Dashboard - Week 1) → Independent, requires Week 0 validation

v0.2 (Activity Groups - Weeks 2-5) → Independent
    ↓ (blocks)
v0.3 (Daily Check-In Migration - Week 5) → Requires activity_groups table

v0.4 (UI Refresh - Weeks 6-7) → Independent (Tailwind v4 already validated in Week 0)
    ↓ (integrates with)
All UI components from v0.1-v0.3

v1.0 (Production Polish - Weeks 8-9) → Requires all previous releases
```

**Key Difference**: Week 0 validation unblocks all risky decisions early

### Component Dependencies (Frontend)
```
lib/components/ui/Button.svelte → No dependencies (base component)
lib/components/ui/Input.svelte → No dependencies (base component)

lib/components/activities/ActivityGroupList.svelte
    → lib/components/ui/Button.svelte
    → lib/components/ui/Card.svelte

lib/components/mood/MoodCheckinForm.svelte
    → lib/components/activities/ActivitySelector.svelte
    → lib/components/ui/Button.svelte

routes/dashboard/+page.svelte
    → lib/components/charts/* (all chart components)
    → lib/components/ui/Card.svelte
```

---

## API Surface Area

### New Tauri Commands (Phase 2: Activity Groups)

**Activity Group Management**:
```rust
#[tauri::command]
async fn create_activity_group(request: CreateActivityGroupRequest) -> Result<ActivityGroup, CommandError>

#[tauri::command]
async fn update_activity_group(id: i64, request: UpdateActivityGroupRequest) -> Result<ActivityGroup, CommandError>

#[tauri::command]
async fn delete_activity_group(id: i64) -> Result<(), CommandError>

#[tauri::command]
async fn get_activity_groups() -> Result<Vec<ActivityGroup>, CommandError>
```

**Activity Management**:
```rust
#[tauri::command]
async fn create_activity(group_id: i64, request: CreateActivityRequest) -> Result<Activity, CommandError>

#[tauri::command]
async fn update_activity(id: i64, request: UpdateActivityRequest) -> Result<Activity, CommandError>

#[tauri::command]
async fn delete_activity(id: i64) -> Result<(), CommandError>

#[tauri::command]
async fn get_activities_by_group(group_id: i64) -> Result<Vec<Activity>, CommandError>
```

**Activity Logging**:
```rust
#[tauri::command]
async fn log_activity(activity_id: i64, notes: Option<String>) -> Result<ActivityLog, CommandError>

#[tauri::command]
async fn get_activity_logs(activity_id: i64, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<ActivityLog>, CommandError>
```

**Activity Goals**:
```rust
#[tauri::command]
async fn set_activity_goal(request: SetActivityGoalRequest) -> Result<ActivityGoal, CommandError>

#[tauri::command]
async fn get_activity_goals(activity_id: Option<i64>, group_id: Option<i64>) -> Result<Vec<ActivityGoal>, CommandError>

#[tauri::command]
async fn check_goal_progress(goal_id: i64) -> Result<GoalProgress, CommandError>
```

**Reporting**:
```rust
#[tauri::command]
async fn get_activity_frequency(activity_id: i64, start_date: String, end_date: String) -> Result<ActivityFrequency, CommandError>

#[tauri::command]
async fn get_activity_trend(activity_id: i64, current_start: String, current_end: String, previous_start: String, previous_end: String) -> Result<ActivityTrend, CommandError>
```

### New Tauri Commands (Phase 1: Dashboard)

**Dashboard Queries**:
```rust
#[tauri::command]
async fn get_assessment_history(assessment_type: Option<String>, start_date: Option<String>, end_date: Option<String>) -> Result<Vec<AssessmentHistoryItem>, CommandError>

#[tauri::command]
async fn get_assessment_trends(assessment_type: String, period: Period) -> Result<AssessmentTrends, CommandError>

#[tauri::command]
async fn get_mood_correlation(start_date: String, end_date: String) -> Result<MoodCorrelation, CommandError>
```

### Updated Tauri Commands (Phase 3: Mood Scale)

**Mood Check-In** (updated for 1-7 scale and activities):
```rust
#[tauri::command]
async fn submit_mood_checkin(request: MoodCheckinRequest) -> Result<MoodCheckin, CommandError>

// MoodCheckinRequest updated:
pub struct MoodCheckinRequest {
    pub mood_rating: i32,      // Now 1-7 (was 1-5)
    pub activity_ids: Vec<i64>, // NEW: Activities done today
    pub notes: Option<String>,
}
```

---

## Testing Strategy

### Repository Layer Testing (TDD)
**Pattern** (from `CLAUDE.md`):
1. Write integration test FIRST
2. Verify test FAILS (red)
3. Implement repository method
4. Verify test PASSES (green)
5. Refactor if needed

**Test Structure**:
```rust
fn setup_test_repo() -> (Repository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db = Arc::new(Database::new(temp_dir.path()).expect("Failed to create database"));
    (Repository::new(db), temp_dir)
}

#[test]
fn test_create_activity_group() {
    let (repo, _temp_dir) = setup_test_repo();

    // Arrange
    let name = "Exercise".to_string();
    let description = Some("Physical activities".to_string());

    // Act
    let group = repo.create_activity_group(name.clone(), description.clone()).expect("Failed to create group");

    // Assert
    assert_eq!(group.name, name);
    assert_eq!(group.description, description);
    assert!(group.id > 0);
}
```

### Component Testing (Svelte)
**Pattern**:
```typescript
import { render, fireEvent } from '@testing-library/svelte';
import ActivityGroupList from './ActivityGroupList.svelte';

test('displays activity groups', async () => {
  const groups = [
    { id: 1, name: 'Exercise', description: 'Physical activities' },
    { id: 2, name: 'Hobbies', description: 'Creative pursuits' },
  ];

  const { getByText } = render(ActivityGroupList, { props: { groups } });

  expect(getByText('Exercise')).toBeInTheDocument();
  expect(getByText('Hobbies')).toBeInTheDocument();
});
```

### E2E Testing (Phase 5)
**Scenarios to Test**:
1. Complete PHQ-9 assessment end-to-end
2. Create activity group → add activities → log activity
3. Set goal → track progress → view in dashboard
4. Daily check-in with 7-point scale and activity selection
5. View dashboard with charts for all data types

---

## Performance Targets

### UI Responsiveness
- **Target**: <100ms for all interactions
- **Measurement**: Browser DevTools Performance tab
- **Critical Paths**:
  - Button clicks
  - Form input changes
  - Page navigation

### Chart Rendering
- **Target**: <500ms for initial render
- **Measurement**: `performance.now()` before/after chart creation
- **Optimization Strategies**:
  - Limit data points displayed (e.g., last 30 days default)
  - Lazy load chart library
  - Use canvas rendering (not SVG for large datasets)

### Database Queries
- **Target**: <200ms for 95th percentile
- **Measurement**: Logging in repository methods
- **Critical Queries**:
  - Dashboard data aggregation
  - Activity frequency calculation
  - Assessment history with filtering

### Overall Application
- **Target**: <2s end-to-end for assessment submission
- **Includes**: Frontend validation + Tauri IPC + database write + response
- **Measurement**: Browser Network tab timing

---

## Known Constraints and Limitations

### Technical Constraints
1. **SQLite Limitations**:
   - No `ALTER COLUMN` or `DROP COLUMN` in SQLite <3.35
   - Foreign keys disabled by default (must enable with `PRAGMA foreign_keys=ON`)
   - Limited concurrent writes (mitigated with WAL mode)

2. **Tauri Desktop App**:
   - No web deployment (desktop-only)
   - Platform-specific builds (Windows, macOS, Linux)
   - Must bundle all dependencies

3. **Svelte 5 Runes**:
   - Breaking changes from Svelte 4 (not backward compatible)
   - New syntax requires learning curve
   - Limited ecosystem (some libraries may not support runes yet)

### Business Constraints
1. **Clinical Assessment Accuracy**:
   - PHQ-9, GAD-7, CES-D, OASIS must follow official scoring rules exactly
   - Cannot modify question wording or scoring algorithms

2. **Data Privacy**:
   - All data stored locally (no cloud sync)
   - No telemetry or analytics without user consent

### Timeline Constraints
1. **Single Developer**: Assumes one full-time developer (or equivalent effort)
2. **10-Week Timeline**: Aggressive but achievable with focused effort
3. **No Major Blockers**: Assumes no external dependencies or major bugs

---

## Glossary

### Assessment Types
- **PHQ-9**: Patient Health Questionnaire (9 items) - measures depression severity
- **GAD-7**: Generalized Anxiety Disorder (7 items) - measures anxiety severity
- **CES-D**: Center for Epidemiologic Studies Depression Scale - depression screening
- **OASIS**: Overall Anxiety Severity and Impairment Scale - anxiety assessment

### Architecture Terms
- **Vertical Slice**: Feature organized by domain (models → repository → commands → UI)
- **CQRS Lite**: Separation of commands (mutations) and queries (reads)
- **Repository Pattern**: Encapsulation of all database access

### Database Terms
- **WAL Mode**: Write-Ahead Logging (better concurrency for SQLite)
- **PRAGMA**: SQLite configuration command
- **Soft Delete**: Mark records as deleted with `deleted_at` timestamp (preserve data)

### Frontend Terms
- **Runes**: Svelte 5's new reactivity system (`$state`, `$derived`, `$props`, etc.)
- **Snippet**: Svelte 5's replacement for slots (reusable template fragments)
- **Bindable**: Two-way data binding between components (`$bindable()`)

---

## References and External Resources

### Official Documentation
- [Svelte 5 Docs](https://svelte.dev/docs/svelte/overview)
- [SvelteKit Docs](https://kit.svelte.dev/docs)
- [Tauri Docs](https://tauri.app/v1/guides/)
- [Tailwind CSS v4 Docs](https://tailwindcss.com/docs) (beta)
- [Rust Book](https://doc.rust-lang.org/book/)
- [rusqlite Docs](https://docs.rs/rusqlite/latest/rusqlite/)

### Project-Specific Docs
- `CLAUDE.md`: Project coding guidelines
- `catalyst-css-extraction-guide.md`: UI migration analysis
- `specs/001-mental-health-tracking/spec.md`: Original specification

### External Tools
- [Heroicons](https://heroicons.com/) - Icon library
- [Chart.js](https://www.chartjs.org/) - Chart library
- [Catalyst UI Kit](https://tailwindui.com/templates/catalyst) - Design reference

---

## Changelog

### Version 2.0 (2025-11-06) - Pragmatic Refactoring
- **Major Change**: Applied pragmatic development principles (Sam Rivera approach)
- Added "Development Approach: Pragmatic Principles" section
- Updated Decision 3 (Tailwind): Week 0 validation instead of Week 7 upgrade
- Updated Decision 5 (Heroicons): Committed in Week 0 (removed decision paralysis)
- Updated Decision 6 (Chart.js): Validated in Week 0 (prove before building)
- Updated Feature Dependencies: Iterative releases (v0.1-v1.0) with Week 0 validation
- Added Iterative Release Strategy table (5 shippable releases)
- See: `PRAGMATIC-PRINCIPLES.md` for full philosophy

### Version 1.0 (2025-11-06)
- Initial context document created
- All critical decisions documented
- API surface area defined
- Testing strategy outlined
