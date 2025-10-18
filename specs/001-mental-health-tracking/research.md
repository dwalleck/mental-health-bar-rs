# Research & Technology Decisions

**Feature**: Mental Health Assessment and Tracking Application
**Date**: 2025-10-15
**Purpose**: Document technology choices, architectural patterns, and best practices

## Overview

This document consolidates research findings that informed the implementation plan. All NEEDS CLARIFICATION items from the Technical Context have been resolved through research and analysis.

## Technology Stack Decisions

### 1. Database: DuckDB vs SQLite

**Decision**: Use DuckDB for local data persistence

**Rationale**:
- **Analytics Optimized**: DuckDB is columnar-oriented (OLAP), optimized for time-series aggregations needed for charting
- **Performance**: Significantly faster for analytical queries (GROUP BY, aggregations) over large datasets (1+ year daily data)
- **Embedded**: Like SQLite, DuckDB runs in-process with no server required
- **Rust Support**: Good Rust bindings available via `duckdb` crate
- **Zero-Copy Arrow**: Efficient data transfer for visualization libraries

**Alternatives Considered**:
- **SQLite**: Row-oriented (OLTP), excellent for CRUD but slower for analytical queries. Rejected because charting queries (aggregating mood scores by day/week/month) would be slower with 365+ daily entries.
- **JSON Files**: Simple but no query capabilities, would require loading entire dataset into memory. Rejected for performance and scalability reasons.
- **sled**: Rust-native embedded key-value store. Rejected because we need SQL for complex queries (e.g., "show average mood score by activity type").

**Implementation Notes**:
- Use DuckDB 0.9+ for best Rust API stability
- Create single database file in user's app data directory
- Schema migrations managed manually (simple SQL scripts)
- Fallback plan: If DuckDB proves problematic, migration to SQLite is straightforward (both SQL-based)

---

### 2. Charting Library: Chart.js vs Recharts vs D3

**Decision**: Use Chart.js (via svelte-chartjs wrapper)

**Rationale**:
- **Simplicity**: Declarative API, easy to integrate with Svelte
- **Performance**: Canvas-based rendering handles 1000+ data points well
- **Responsive**: Built-in responsive design, mobile-friendly
- **Maintained**: Active development, large community
- **Types**: Full TypeScript support

**Alternatives Considered**:
- **Recharts**: React-specific, would require adapters for Svelte. Rejected.
- **D3**: Extremely powerful but complex, overkill for standard line/bar charts. Higher learning curve. Rejected for complexity.
- **ApexCharts**: Good option but heavier bundle size. Chart.js is lighter and sufficient.

**Implementation Notes**:
- Install: `npm install chart.js svelte-chartjs`
- Line charts for assessment score trends over time
- Bar charts for mood frequency distributions
- Time-series x-axis with date formatting
- Clinical threshold lines via Chart.js annotations plugin

---

### 3. State Management: Svelte Stores vs Context API

**Decision**: Use Svelte stores (writable/readable)

**Rationale**:
- **Native**: Built into Svelte, no extra dependencies
- **Reactive**: Automatic UI updates via `$store` syntax
- **Simple**: Minimal boilerplate, easy to reason about
- **TypeScript**: Full type safety with generic stores
- **Async Support**: Readable stores can handle async data loading

**Alternatives Considered**:
- **Context API**: Good for prop drilling, but stores are better for global state (user settings, current assessment).
- **External libraries** (Zustand, Redux): Overkill for simple state needs. Rejected for unnecessary complexity.

**Implementation Notes**:
- Store per feature domain: `assessments.ts`, `mood.ts`, `settings.ts`
- Derived stores for computed values (e.g., `currentStreak` from mood check-ins)
- Persist settings to localStorage via store subscriptions

---

### 4. Notification Strategy: tauri-plugin-notification

**Decision**: Use tauri-plugin-notification for assessment reminders

**Rationale**:
- **Cross-Platform**: Works on Linux, macOS, Windows
- **Native**: Uses OS notification system (no custom UI needed)
- **Permissions**: Tauri handles permission requests automatically
- **Scheduling**: Can trigger from Rust backend based on assessment schedules

**Alternatives Considered**:
- **Web Notifications API**: Requires app to be running in foreground. Rejected because we need background notifications.
- **Custom notification UI**: Complex, non-native feel. Rejected for poor UX.

**Implementation Notes**:
- Schedule check runs every minute (lightweight background task)
- Compare current time against `assessment_schedules` table
- Send notification if schedule due and not yet completed today
- Notification click opens app to assessment page

---

### 5. Assessment Content: Licensing & Compliance

**Decision**: Use public domain or validated open-source assessment content

**Rationale**:
- **PHQ-9**: Public domain (Pfizer), free to use with proper citation
- **GAD-7**: Public domain (Spitzer et al.), free to use
- **Beck Inventories (BDI, BAI)**: **COPYRIGHTED** by Pearson - cannot include without license

**Alternatives Considered**:
- **License Beck Inventories**: Expensive ($2-5 per administration), requires agreement with Pearson. Rejected for cost.
- **Use Beck Inventories anyway**: Legal risk, copyright infringement. Rejected.
- **Replace with alternatives**: Use free alternatives (see below)

**IMPLEMENTATION CHANGE REQUIRED**:

Since Beck Depression Inventory (BDI) and Beck Anxiety Inventory (BAI) are copyrighted, we must use free alternatives:

**Recommended Replacements**:
1. **For Depression** (instead of BDI):
   - **PHQ-9** (already included): 9 items, 0-27 scale, public domain
   - **CES-D** (Center for Epidemiologic Studies Depression Scale): 20 items, 0-60 scale, public domain

2. **For Anxiety** (instead of BAI):
   - **GAD-7** (already included): 7 items, 0-21 scale, public domain
   - **OASIS** (Overall Anxiety Severity and Impairment Scale): 5 items, 0-20 scale, public domain

**Decision**:
- **Keep PHQ-9 and GAD-7** as primary assessments (already free, widely validated)
- **Add CES-D** (depression) and **OASIS** (anxiety) as secondary assessments
- **Remove BDI and BAI** from specification and implementation

**Implementation Notes**:
- Store assessment questions/scoring in Rust constants or JSON files
- Include attribution in UI footer (e.g., "PHQ-9 © Pfizer Inc.")
- Document citations in README
- Assessment scoring algorithms must match published guidelines exactly

---

### 6. Error Handling Pattern

**Decision**: Use thiserror for Rust errors + anyhow for command handlers

**Rationale**:
- **thiserror**: Define custom error enums with derive macro, great for library code (features)
- **anyhow**: Convenient error propagation with context, great for application code (commands)
- **Pattern**: Feature modules use thiserror, Tauri commands convert to anyhow for user-facing messages

**Alternatives Considered**:
- **Only anyhow**: Good for apps but loses type safety. Partial rejection.
- **Only thiserror**: Verbose for simple errors. Partial rejection.
- **Both**: Best of both worlds, standard Rust practice.

**Implementation Notes**:
```rust
// Feature module (src-tauri/src/features/assessments/models.rs)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssessmentError {
    #[error("Invalid assessment type: {0}")]
    InvalidType(String),
    #[error("Incomplete responses")]
    IncompleteResponses,
    #[error("Database error: {0}")]
    DatabaseError(#[from] duckdb::Error),
}

// Tauri command (src-tauri/src/features/assessments/commands.rs)
use anyhow::Context;

#[tauri::command]
pub async fn submit_assessment(...) -> Result<AssessmentResponse, String> {
    let result = repository::save_assessment(...)
        .context("Failed to save assessment")?;

    Ok(result)
}
```

---

### 7. Testing Strategy

**Decision**: Multi-layered testing approach

**Test Layers**:

1. **Unit Tests** (Rust - `cargo test`)
   - Domain logic: Assessment scoring algorithms
   - Data validation: Input sanitization, constraints
   - Per-feature: Each feature module has its own test module
   - Example: Test PHQ-9 scoring: [0,0,0,0,0,0,0,0,0] = score 0

2. **Integration Tests** (Rust - `tests/integration/`)
   - Database operations: CRUD via repositories
   - Command handlers: Full command execution path
   - Example: Save assessment, retrieve it, verify score matches

3. **Contract Tests** (TypeScript + Rust via tauri-specta)
   - API contract validation: Types generated from Rust match usage in TypeScript
   - Compile-time safety: TypeScript compiler catches contract violations
   - Example: If Rust command changes signature, TypeScript build fails

4. **Component Tests** (Svelte - Vitest)
   - UI components: Render correctly, handle interactions
   - State management: Stores update properly
   - Example: Assessment form validation, button states

5. **E2E Tests** (Optional - Tauri WebDriver)
   - Full user workflows: Not required for v0.1.0, defer until needed
   - Example: Complete PHQ-9, view score, verify chart updates

**Rationale**:
- **Bottom-Up**: Unit tests catch logic bugs early
- **Integration**: Verify features work end-to-end on backend
- **Contract**: Prevent frontend/backend drift
- **Component**: Ensure UI behaves correctly
- **E2E Deferred**: Per Constitution Principle III (work → right → fast), E2E tests are slow and complex. Defer until post-v0.1.0.

**Implementation Notes**:
- TDD workflow: Write test → implement → refactor
- Test data fixtures: Sample PHQ-9 responses, mood check-ins
- Mock DuckDB for unit tests (use in-memory database)
- CI pipeline: Run all tests on PR (future)

---

### 8. Configuration Management: confy

**Decision**: Use confy for user preferences

**Rationale**:
- **Simple**: One function to load/save config
- **Cross-Platform**: Handles platform-specific config directories (XDG on Linux, ~/Library on macOS, AppData on Windows)
- **Serde**: Automatic serialization from Rust structs
- **Type-Safe**: Strongly typed configuration

**Configuration Structure**:
```rust
#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub theme: Theme,                    // Light/Dark
    pub notification_enabled: bool,
    pub data_retention_days: Option<u32>, // None = infinite
}
```

**Implementation Notes**:
- Config file: `~/.config/mental-health-tracker/config.toml` (Linux)
- Load on app start, save on settings change
- Provide defaults via `Default` trait

---

### 9. Date/Time Handling

**Decision**: Use chrono for Rust date/time, store UTC timestamps in DuckDB

**Rationale**:
- **chrono**: Standard Rust date/time library, comprehensive timezone support
- **UTC Storage**: Avoid timezone ambiguity, convert to local for display
- **ISO 8601**: Standard format for serialization
- **DuckDB**: Supports `TIMESTAMP` type natively

**Implementation Notes**:
- Store all timestamps as `TIMESTAMP WITH TIME ZONE` in UTC
- Convert to user's local timezone for display (use system timezone)
- Assessment "due today" logic: Compare current date in local timezone
- Handle edge cases: DST transitions, timezone changes

---

### 10. Logging: tracing + tracing-subscriber + tracing-appender

**Decision**: Use tracing ecosystem for structured logging

**Rationale**:
- **Structured**: Key-value pairs, easier to parse than strings
- **Levels**: TRACE, DEBUG, INFO, WARN, ERROR
- **Spans**: Context across async boundaries
- **Performance**: Zero-cost abstractions, minimal overhead
- **Ecosystem**: Compatible with tokio, popular in Rust community

**Implementation Notes**:
```rust
// Setup in main.rs
use tracing_subscriber::prelude::*;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_writer(file_appender))
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .init();

// Usage in code
#[tracing::instrument]
async fn submit_assessment(...) {
    tracing::info!("Submitting assessment", assessment_type = %type);
    // ...
}
```

- Log file: `~/.local/share/mental-health-tracker/logs/app.log` (Linux)
- Rotation: Daily, keep last 7 days
- Levels: INFO+ in production, DEBUG in development

---

## Architecture Patterns

### Vertical Slice Architecture

**Decision**: Organize code by feature (vertical slices) rather than technical layers

**Structure**:
```
features/
├── assessments/       # Everything for assessments feature
│   ├── models.rs      # Domain models
│   ├── commands.rs    # Tauri commands (write operations)
│   ├── queries.rs     # Tauri commands (read operations)
│   └── repository.rs  # Database access
└── mood/              # Everything for mood tracking feature
    ├── models.rs
    ├── commands.rs
    ├── queries.rs
    └── repository.rs
```

**Benefits**:
- **Cohesion**: Related code lives together (easier to navigate)
- **Independence**: Features can be developed/tested/deployed separately
- **Scalability**: Adding new features doesn't touch existing features
- **Team-Friendly**: Multiple devs can work on different features without merge conflicts

**Rationale**: Aligns with Constitution Principle II (Developer Experience). Code is discoverable, changes are localized.

---

### CQRS Lite (Command/Query Separation)

**Decision**: Separate write operations (commands) from read operations (queries)

**Pattern**:
- **Commands**: Mutate state, return success/error (e.g., `submit_assessment`)
- **Queries**: Read state, return data (e.g., `get_assessment_history`)

**Benefits**:
- **Clarity**: Intent is clear from function name
- **Optimization**: Queries can be optimized differently than commands
- **Testability**: Easier to mock/stub read vs write operations

**Implementation Notes**:
- Commands in `commands.rs`, queries in `queries.rs`
- Commands return `Result<(), Error>` or `Result<Id, Error>`
- Queries return `Result<Data, Error>`

---

### Repository Pattern

**Decision**: Encapsulate database access in repository modules

**Pattern**:
```rust
// repository.rs
pub struct AssessmentRepository {
    conn: Arc<Mutex<Connection>>,
}

impl AssessmentRepository {
    pub fn save(&self, assessment: &Assessment) -> Result<i64, DatabaseError> {
        // DuckDB SQL
    }

    pub fn find_by_id(&self, id: i64) -> Result<Option<Assessment>, DatabaseError> {
        // DuckDB SQL
    }
}
```

**Benefits**:
- **Testability**: Easy to mock repository for unit tests
- **Abstraction**: Commands/queries don't know about SQL details
- **Migration**: If we swap DuckDB for SQLite, only repositories change

**Trade-off**: Adds indirection (violates YAGNI slightly), but justified for testability and maintainability per Constitution Principle III (Make It Right).

---

## Best Practices

### Tauri Best Practices

1. **Command Naming**: Use verb_noun pattern (e.g., `submit_assessment`, `get_mood_history`)
2. **Error Handling**: Convert Rust errors to `String` for Tauri (frontend can't deserialize complex errors)
3. **Async**: Use async commands for database operations (don't block main thread)
4. **State**: Use Tauri's managed state for shared resources (database connection pool)
5. **Events**: Emit events for background notifications (assessment due)

### DuckDB Best Practices

1. **Connection Pooling**: Single connection per app instance (DuckDB is single-writer)
2. **Transactions**: Wrap multi-statement operations in transactions
3. **Prepared Statements**: Use parameterized queries to prevent SQL injection
4. **Indexes**: Add indexes on frequently queried columns (e.g., `created_at` for time-series queries)
5. **Schema Migrations**: Manual SQL scripts in `migrations.rs`, applied on app start

### Svelte Best Practices

1. **Component Size**: Keep components under 200 lines (split into smaller components if needed)
2. **Props vs Stores**: Use props for component input, stores for global state
3. **Reactivity**: Leverage Svelte's reactive declarations (`$:`) for derived values
4. **Accessibility**: Use semantic HTML, ARIA labels, keyboard navigation
5. **Performance**: Use `{#key}` blocks to force re-renders sparingly

---

## Security Considerations

### Data Protection

1. **Local Storage**: All data stored locally, never transmitted
2. **File Permissions**: DuckDB file should have user-only read/write permissions (0600)
3. **Encryption**: For v0.1.0, no encryption at rest (defer to v0.2.0 if needed)
4. **Input Validation**: Sanitize all user input (SQL injection, XSS)
5. **Dependencies**: Regular `cargo audit` and `npm audit` to check for vulnerabilities

### Privacy

1. **No Telemetry**: No analytics, crash reporting, or usage tracking
2. **No Network**: App operates entirely offline (except updater in future)
3. **User Control**: User can delete all data via settings
4. **Transparency**: Document what data is collected (assessment responses, mood scores) in README

---

## Performance Targets

Based on Success Criteria from spec:

1. **Assessment Completion**: <5 minutes (mostly user reading/answering time)
2. **Mood Check-In**: <30 seconds (UI responsiveness)
3. **Activity Creation**: <15 seconds
4. **Chart Rendering**: <500ms (DuckDB query + Chart.js render)
5. **Data Access**: <3 seconds (all historical queries)
6. **Memory**: <150MB total (Tauri + Chromium + DuckDB)

**Monitoring**: Add basic instrumentation via `tracing` to log operation durations in development builds.

---

## Open Questions & Deferred Decisions

### Deferred to Implementation

1. **Chart Library Details**: Specific plugins (annotations for threshold lines) - decide during P3 implementation
2. **Notification Scheduling**: Exact cron-like syntax or simple interval presets - decide during P3 implementation
3. **Data Export**: Not in v0.1.0 scope, but consider format (CSV, JSON, PDF) for v0.2.0
4. **Backup/Restore**: Not in v0.1.0, but consider for v0.2.0 (copy DuckDB file?)

### Resolved During Planning

All NEEDS CLARIFICATION items from Technical Context have been resolved:
- ✅ Database: DuckDB
- ✅ Assessment Content: PHQ-9, GAD-7, CES-D, OASIS (removed copyrighted Beck inventories)
- ✅ Charting: Chart.js
- ✅ State Management: Svelte stores
- ✅ Error Handling: thiserror + anyhow
- ✅ Testing: Multi-layered (unit, integration, contract, component)
- ✅ Configuration: confy
- ✅ Logging: tracing ecosystem
- ✅ Architecture: Vertical slices + CQRS lite + Repository pattern

---

## Summary

This research phase has established a solid technical foundation:

1. **Stack**: Tauri + Rust + Svelte + DuckDB (all specified in user input)
2. **Architecture**: Vertical slices for maintainability, CQRS for clarity
3. **Assessment Content**: PHQ-9, GAD-7, CES-D, OASIS (free, validated alternatives)
4. **Testing**: TDD workflow with multiple test layers
5. **Privacy**: Local-only, no network, user control over data
6. **Performance**: Targets defined, monitoring plan in place

Ready to proceed to Phase 1 (Data Model & Contracts).
