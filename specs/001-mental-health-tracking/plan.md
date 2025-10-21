# Implementation Plan: Mental Health Assessment and Tracking Application

**Branch**: `001-mental-health-tracking` | **Date**: 2025-10-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-mental-health-tracking/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This application provides standardized mental health assessments (PHQ-9, CES-D, GAD-7, OASIS) with configurable scheduling, daily mood check-ins with activity tracking, and data visualization via charts. It's a single-user desktop application built with Tauri (Rust + Svelte) using local data storage for privacy, following a feature-based vertical slice architecture with DuckDB for persistence.

## Technical Context

**Language/Version**: Rust (latest stable) + TypeScript/JavaScript (ES2022)
**Primary Dependencies**:
- Backend: Tauri 2.x, tauri-specta, DuckDB, serde, anyhow, thiserror, confy, tracing, tracing-subscriber, tracing-appender
- Frontend: Svelte 5, SvelteKit, Vite, TailwindCSS
- Plugins: tauri-plugin-dialog, tauri-plugin-notification, tauri-plugin-updater
**Storage**: DuckDB (embedded SQL database for local data persistence)
**Testing**: Vitest (frontend), cargo test (backend), integration tests via tauri-specta contracts
**Target Platform**: Desktop (Linux, macOS, Windows) via Tauri
**Project Type**: Desktop application (Tauri = Rust backend + Svelte frontend)
**Architecture**: Feature-based vertical slice architecture (each feature owns its data, commands, queries, and UI)
**Performance Goals**: <100ms UI response time, <500ms for chart rendering, <2s for assessment submission
**Constraints**: Offline-capable (no cloud dependencies), <150MB memory usage, local-only data storage
**Scale/Scope**: Single-user, ~10 screens, 4 assessment types, unlimited historical data (1+ years), ~20-30 Tauri commands

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Ship It (Principle I)
- ✅ **5-Minute Hello World**: Tauri template already working, can extend incrementally
- ✅ **Shippable Increments**: Each user story (P1-P3) is independently testable and deliverable
- ✅ **Real Feedback**: Start with P1 (assessments), ship, iterate to P2 (mood tracking) and P3 (visualization)
- **Risk**: None. Architecture supports incremental delivery.

### Developer Experience (Principle II)
- ✅ **Discoverability**: tauri-specta generates TypeScript types from Rust commands automatically
- ✅ **Error Messages**: Using thiserror for structured errors, tauri-plugin-dialog for user-facing messages
- ✅ **Documentation**: Will include quickstart.md with real examples (Phase 1)
- ✅ **Convention Over Configuration**: Vertical slice architecture keeps related code together
- **Risk**: None. Tooling supports good DX.

### Work → Right → Fast (Principle III)
- ✅ **Make It Work**: P1 (assessments) is minimal viable feature
- ✅ **Make It Right**: Vertical slices ensure maintainability, clean separation
- ✅ **Make It Fast**: Performance optimization deferred until P3 visualization if needed
- **Risk**: None. Sequence is correct.

### README-Driven Development (Principle VI)
- ✅ **Documentation First**: quickstart.md will be written in Phase 1 before implementation
- ✅ **Real Examples**: Each feature will have working code examples
- **Action Required**: Generate quickstart.md in Phase 1

### Test-First Approach (Principle IX)
- ✅ **TDD Workflow**: Vitest (frontend) + cargo test (backend) + integration tests
- ✅ **Real Scenarios**: Tests will validate actual user workflows (assessment flow, mood tracking, etc.)
- ✅ **Contract Testing**: tauri-specta ensures frontend/backend contract compliance
- **Action Required**: Define test strategy in research.md

### Complexity Check
- ✅ **No Over-Engineering**: Using established stack (Tauri + Svelte), standard patterns
- ✅ **No "Just In Case" Features**: Implementing only specified requirements (no speculation)
- ✅ **Vertical Slices**: Simplest architecture that maintains cohesion
- **Potential Violation**: DuckDB for storage (vs simpler SQLite)
  - **Justification**: DuckDB provides better analytics performance for charting (columnar storage)
  - **Alternative Rejected**: SQLite sufficient for CRUD but slower for time-series aggregations
  - **Decision**: Use DuckDB, but can swap to SQLite if performance assumptions wrong

**GATE STATUS**: ✅ PASSED (with one justified complexity decision tracked below)

## Project Structure

### Documentation (this feature)

```
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

**Structure Decision**: Tauri desktop application with feature-based vertical slice architecture. Each feature owns its complete stack: Rust backend (commands/queries/models/db) + Svelte frontend (pages/components/stores).

```
src-tauri/                          # Rust backend
├── src/
│   ├── main.rs                     # Tauri app entry point
│   ├── lib.rs                      # Library exports, tauri-specta setup
│   ├── db/
│   │   ├── mod.rs                  # DuckDB connection management
│   │   ├── migrations.rs           # Schema migrations
│   │   └── schema.sql              # Database schema
│   ├── config/
│   │   └── mod.rs                  # confy-based app configuration
│   ├── features/
│   │   ├── assessments/            # VERTICAL SLICE: Assessment feature
│   │   │   ├── mod.rs              # Feature module exports
│   │   │   ├── models.rs           # Assessment domain models (PHQ9, GAD7, etc.)
│   │   │   ├── commands.rs         # Tauri commands (submit_assessment, etc.)
│   │   │   ├── queries.rs          # Data queries (get_history, get_score)
│   │   │   └── repository.rs       # DuckDB persistence layer
│   │   ├── mood/                   # VERTICAL SLICE: Mood check-in feature
│   │   │   ├── mod.rs
│   │   │   ├── models.rs           # MoodCheckin, Activity models
│   │   │   ├── commands.rs         # log_mood, manage_activities
│   │   │   ├── queries.rs          # get_mood_history, get_activities
│   │   │   └── repository.rs
│   │   ├── scheduling/             # VERTICAL SLICE: Assessment scheduling
│   │   │   ├── mod.rs
│   │   │   ├── models.rs           # Schedule models
│   │   │   ├── commands.rs         # configure_schedule
│   │   │   ├── queries.rs          # get_schedules
│   │   │   ├── repository.rs
│   │   │   └── scheduler.rs        # Background task for notifications
│   │   └── visualization/          # VERTICAL SLICE: Data viz queries
│   │       ├── mod.rs
│   │       ├── queries.rs          # get_chart_data (assessments/mood)
│   │       └── repository.rs       # Optimized aggregation queries
│   └── errors.rs                   # Shared error types (thiserror)
└── tests/
    ├── integration/                # End-to-end tests via Tauri commands
    └── unit/                       # Per-feature unit tests

src/                                # Svelte frontend (SvelteKit)
├── lib/
│   ├── bindings.ts                 # tauri-specta generated types
│   ├── components/
│   │   ├── ui/                     # Shared UI components (buttons, cards, etc.)
│   │   ├── assessments/            # Assessment-specific components
│   │   ├── mood/                   # Mood check-in components
│   │   └── charts/                 # Chart visualization components
│   ├── stores/                     # Svelte stores for state management
│   │   ├── assessments.ts
│   │   ├── mood.ts
│   │   └── settings.ts
│   └── utils/
│       ├── api.ts                  # Tauri command wrappers
│       └── formatting.ts           # Date/score formatting helpers
└── routes/
    ├── +layout.svelte              # App shell layout
    ├── +page.svelte                # Home/dashboard
    ├── assessments/
    │   ├── +page.svelte            # Assessment selection
    │   ├── [type]/+page.svelte     # Take assessment (PHQ9/GAD7/BDI/BAI)
    │   └── history/+page.svelte    # Assessment history
    ├── mood/
    │   ├── +page.svelte            # Log mood check-in
    │   └── history/+page.svelte    # Mood history
    ├── charts/
    │   └── +page.svelte            # Data visualization
    └── settings/
        └── +page.svelte            # Configure schedules

tests/                              # Frontend tests
├── unit/                           # Component unit tests (Vitest)
└── integration/                    # User workflow tests
```

## Complexity Tracking

*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| DuckDB over SQLite | Analytics performance for time-series charting queries (columnar storage) | SQLite is row-oriented, slower for aggregations across 1+ year of daily data points. DuckDB optimized for OLAP queries. |
| Defensive Deletion Pattern | DuckDB doesn't enforce ON DELETE CASCADE constraints | Database-level CASCADE not available. Simpler "ignore the problem" rejected because mental health data is sensitive and requires explicit integrity guarantees. |

**Mitigation**: If DuckDB proves unnecessary (performance adequate with SQLite), migration path is straightforward (both are SQL-based embedded databases).

## Cascading Delete Strategy

**Context**: DuckDB accepts `ON DELETE CASCADE` syntax but does not enforce it (as of v1.1.3). This requires explicit handling of referential integrity at the application layer.

**Documentation**: See `src-tauri/docs/duckdb-practices.md` for technical details.

### Approach by Relationship Type

#### 1. Assessment Types → Responses/Schedules (PREVENT)
**Pattern**: Defensive deletion - block delete if children exist

**Rationale**:
- Assessment types are seeded reference data (PHQ-9, GAD-7, CES-D, OASIS) that shouldn't be deleted
- User assessment responses are precious historical mental health data
- Schedules are user configurations that shouldn't be silently lost
- Fail-safe principle: prevent accidental data loss rather than cascade

**Implementation**:
```rust
// In assessment repository
pub fn delete_assessment_type(&self, id: i32) -> Result<()> {
    // Count child records
    let response_count = self.count_assessment_responses(id)?;
    let schedule_count = self.count_assessment_schedules(id)?;

    // Block deletion if children exist
    if response_count > 0 || schedule_count > 0 {
        return Err(AssessmentError::HasChildren(
            format!("Cannot delete: {} responses, {} schedules exist",
                    response_count, schedule_count)
        ));
    }

    // Safe to delete
    conn.execute("DELETE FROM assessment_types WHERE id = ?", [id])?;
    Ok(())
}
```

#### 2. Mood Check-ins → Activities Junction (CASCADE)
**Pattern**: Application-level cascade with transactions

**Rationale**:
- Junction table (`mood_checkin_activities`) has no independent value without parent
- User expectation: deleting a mood entry removes all associated data
- True cascade scenario

**Implementation**:
```rust
// In mood repository
pub fn delete_mood_checkin(&self, id: i32) -> Result<()> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    conn.execute("BEGIN TRANSACTION", [])?;

    match (|| {
        // Delete children first (junction table)
        conn.execute("DELETE FROM mood_checkin_activities WHERE mood_checkin_id = ?", [id])?;
        // Then delete parent
        conn.execute("DELETE FROM mood_checkins WHERE id = ?", [id])?;
        Ok(())
    })() {
        Ok(()) => {
            conn.execute("COMMIT", [])?;
            Ok(())
        }
        Err(e) => {
            if let Err(rollback_err) = conn.execute("ROLLBACK", []) {
                error!("Failed to rollback transaction: {}", rollback_err);
            }
            Err(e)
        }
    }
}
```

#### 3. Activities → Junction (SOFT DELETE)
**Pattern**: Soft delete with `deleted_at` timestamp (already implemented)

**Rationale**:
- Historical mood check-ins must preserve which activity was selected
- Soft delete maintains data integrity while hiding deleted activities from new selections
- Audit trail preserved

**Status**: ✅ Already correctly implemented in schema with `deleted_at` column

### Deletion Hierarchy

```
┌─────────────────────────────────────────────┐
│ assessment_types (PREVENT if has children)  │
│  - PHQ-9, GAD-7, CES-D, OASIS (seeded)     │
└──┬────────────────────────────┬─────────────┘
   │                            │
   ▼                            ▼
┌──────────────────────┐  ┌────────────────────┐
│ assessment_responses │  │ assessment_schedules│
│ (historical data)    │  │ (user configs)      │
└──────────────────────┘  └────────────────────┘

┌─────────────────────────────────────────────┐
│ mood_checkins (CASCADE to junction)         │
└──┬──────────────────────────────────────────┘
   │
   ▼
┌──────────────────────────────────────────────┐
│ mood_checkin_activities (auto-deleted)       │
│  - Junction table cleared on parent delete   │
└──┬───────────────────────────────────────────┘
   │
   ▼
┌──────────────────────────────────────────────┐
│ activities (SOFT DELETE with deleted_at)     │
│  - Never hard-deleted to preserve history    │
└──────────────────────────────────────────────┘
```

### Testing Requirements

**Integration tests must verify**:
1. Assessment type deletion blocked when responses exist
2. Assessment type deletion blocked when schedules exist
3. Mood check-in deletion cascades to mood_checkin_activities
4. Deleted activities still appear in historical mood check-ins
5. Transaction rollback on cascade failure

**User-facing behavior**:
- Clear error messages when deletion blocked: "Cannot delete PHQ-9: 15 assessment responses exist. Delete responses first or export data."
- Confirmation dialogs for cascade deletions: "This will also delete 3 activity associations. Continue?"
- Soft-deleted activities shown with "(deleted)" badge in historical views

---

## Post-Design Constitution Check

*Re-evaluation after Phase 0 (Research) and Phase 1 (Design) completion*

### Documentation Completeness ✅
- ✅ **quickstart.md**: Generated with <5 minute Hello World path
- ✅ **research.md**: All technology decisions documented with rationale
- ✅ **data-model.md**: Complete database schema with validation rules
- ✅ **contracts/**: API contracts for all 4 features (23 Tauri commands documented)
- **Status**: README-Driven Development principle satisfied

### API Design Review ✅
- ✅ **IntelliSense**: tauri-specta auto-generates TypeScript types from Rust signatures
- ✅ **Language Conventions**: Rust naming (snake_case commands), TypeScript naming (camelCase)
- ✅ **Error Messages**: Structured errors with user-friendly messages (see contracts/*.md)
- ✅ **Discoverability**: Vertical slice architecture, all commands in feature modules
- ✅ **Happy Path**: `submit_assessment` → `get_assessment_history` → `get_assessment_chart_data` (3 commands, clear flow)
- ✅ **Hello World Time**: Quickstart shows PHQ-9 assessment in <5 minutes
- **Status**: All API design checklist items passed

### Test Strategy ✅
- ✅ **Defined**: Multi-layered testing (unit, integration, contract, component) in research.md
- ✅ **TDD Workflow**: Write test → user approval → test fails → implement → test passes
- ✅ **Real Scenarios**: Test cases for user stories (complete PHQ-9, log mood, view chart)
- ✅ **Contract Tests**: tauri-specta ensures type safety at compile time
- **Status**: Test-First principle satisfied

### Complexity Justification ✅
- ✅ **DuckDB**: Justified in Complexity Tracking table (analytics performance)
- ✅ **Repository Pattern**: Justified for testability and migration flexibility
- ✅ **Vertical Slices**: Justified for maintainability and parallel development
- ✅ **CQRS Lite**: Justified for clarity and optimization opportunities
- **Status**: All complexity decisions have documented rationale

### Shipping Readiness ✅
- ✅ **P1 (Assessments)**: Can ship independently with 7 commands
- ✅ **P2 (Mood)**: Can ship independently with 8 commands
- ✅ **P3 (Visualization + Scheduling)**: Depends on P1/P2 data, can ship after
- ✅ **Incremental Path**: P1 → P2 → P3, each provides user value
- ✅ **Success Criteria**: Defined and measurable (see spec.md)
- **Status**: Architecture supports iterative shipping

### Design Decisions Summary

**Resolved During Planning**:
1. ✅ Assessment Content: Replaced copyrighted Beck inventories with free alternatives (CES-D, OASIS)
2. ✅ Database Choice: DuckDB for analytics performance (with SQLite fallback plan)
3. ✅ Charting: Chart.js for simplicity and Svelte compatibility
4. ✅ State Management: Svelte stores (native, simple, reactive)
5. ✅ Error Handling: thiserror (features) + anyhow (commands)
6. ✅ Architecture: Vertical slices + CQRS lite + Repository pattern

**No Unresolved Questions**: All NEEDS CLARIFICATION items from Technical Context resolved in research.md

**Design Artifacts Generated**:
- ✅ `research.md`: 9 technology decisions, 3 architecture patterns, security considerations
- ✅ `data-model.md`: 7 tables, migrations, indexes, query patterns
- ✅ `contracts/README.md`: Contract format and conventions
- ✅ `contracts/assessments.md`: 7 commands, 4 data types, scoring algorithms
- ✅ `contracts/mood.md`: 8 commands, 5 data types, mood scale reference
- ✅ `contracts/scheduling.md`: 5 commands, scheduler logic, notification handling
- ✅ `contracts/visualization.md`: 3 commands, chart recommendations, performance optimization
- ✅ `quickstart.md`: Hello World in <5 minutes, feature walkthrough, common tasks

**FINAL GATE STATUS**: ✅✅ PASSED - Ready for Phase 2 (Task Generation via `/speckit.tasks`)

