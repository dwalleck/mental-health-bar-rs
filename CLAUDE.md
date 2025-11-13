# mental-health-bar-rs Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-10-15

## Active Technologies
- Rust (latest stable) + TypeScript/JavaScript (ES2022) (001-mental-health-tracking)

## Project Structure
```
src/
tests/
```

## Commands
cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style
Rust (latest stable) + TypeScript/JavaScript (ES2022): Follow standard conventions

## Recent Changes
- 001-mental-health-tracking: Added Rust (latest stable) + TypeScript/JavaScript (ES2022)

<!-- MANUAL ADDITIONS START -->

## Rust Coding Guidelines

### Style & Formatting
- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix all warnings
- Use `rustfmt.toml` configuration (if present)
- Maximum line length: 100 characters

### Error Handling
- Use `thiserror` for feature-level error types (models, repository, domain logic)
- Use `CommandError` struct for Tauri commands (provides structured error responses)
- Implement `ToCommandError` trait for feature errors to convert to `CommandError`
- Always provide context with `.context()` or `.with_context()`
- Example:
  ```rust
  // Feature error type
  #[derive(Error, Debug)]
  pub enum AssessmentError {
      #[error("Assessment not found: {0}")]
      NotFound(String),
      #[error("Database error: {0}")]
      Database(#[from] rusqlite::Error),
  }

  // Implement ToCommandError trait for feature errors
  impl ToCommandError for AssessmentError {
      fn to_command_error(&self) -> CommandError {
          match self {
              AssessmentError::NotFound(id) => CommandError::permanent(
                  format!("Assessment not found: {}", id),
                  ErrorType::NotFound,
              ),
              AssessmentError::Database(e) => CommandError::from_rusqlite_error(e),
          }
      }
  }

  // Command usage with structured error responses
  #[tauri::command]
  pub async fn submit_assessment(req: Request) -> Result<Response, CommandError> {
      commands::submit(req)
          .await
          .map_err(|e| e.to_command_error())
  }
  ```

### Naming Conventions
- **Modules**: `snake_case` (e.g., `mood_checkin`, `assessment_types`)
- **Structs/Enums**: `PascalCase` (e.g., `AssessmentResponse`, `MoodRating`)
- **Functions/Variables**: `snake_case` (e.g., `get_assessment_history`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_ACTIVITY_NAME_LENGTH`)
- **Type Parameters**: Single uppercase letter or `PascalCase` (e.g., `T`, `AppState`)

### Architecture Patterns
- **Vertical Slice**: Each feature owns its complete stack (models, commands, queries, repository)
- **CQRS Lite**: Separate commands (mutations) from queries (reads) in separate files
- **Repository Pattern**: Encapsulate all database access in `repository.rs`
- Feature module structure:
  ```
  features/
  └── assessments/
      ├── mod.rs          # Public exports
      ├── models.rs       # Domain types
      ├── commands.rs     # Tauri commands (mutations)
      ├── queries.rs      # Tauri queries (reads)
      └── repository.rs   # Database access
  ```

### Testing
- **TDD Mandatory**: Write tests before implementation per project constitution
- Unit tests: In same file with `#[cfg(test)]` module
- Integration tests: In `tests/` directory
- Test naming: `test_<scenario>_<expected_behavior>`
- Example:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_incomplete_assessment_saves_as_draft() {
          // Arrange, Act, Assert
      }
  }
  ```

### Database Access (DuckDB)
- All queries in repository layer only
- Use parameterized queries (never string interpolation)
- Validate input before database operations
- Handle connection errors gracefully

### Dependencies
- Minimize external crates - justify additions
- Prefer maintained crates with 1.0+ versions
- Use workspace dependencies in `Cargo.toml`

---

## Database Development Guidelines (SQLite + Rusqlite)

### Core Principles
- **TDD Required**: Write integration tests before repository methods
- **Deadlock Prevention**: Use `_with_conn` pattern for all internal helpers
- **Security First**: 100% parameterized queries, zero string interpolation
- **PRAGMA Enforcement**: `foreign_keys=ON` required on all connections

### Connection Management Pattern

Every public repository method must follow this pattern:

```rust
// Public method - acquires lock ONCE
pub fn operation(&self, ...) -> Result<T, Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| Error::LockPoisoned)?;

    // Use connection for operations
    let result = self.helper_with_conn(&conn, ...)?;
    Ok(result)
}

// Helper - accepts connection reference (NO locking)
fn helper_with_conn(&self, conn: &Connection, ...) -> Result<T, Error> {
    // Uses provided connection, never calls conn.lock()
}
```

**Critical Rule**: Methods that accept `&Connection` MUST have `_with_conn` suffix

### Deadlock Prevention

**❌ ANTI-PATTERN (Causes Deadlock):**
```rust
pub fn get_stats(&self) -> Result<Stats> {
    let conn = conn.lock()?;  // First lock
    let data = self.get_helper()?;  // Second lock → DEADLOCK
}

fn get_helper(&self) -> Result<Data> {
    let conn = conn.lock()?;  // Tries to acquire same lock
}
```

**✅ CORRECT PATTERN:**
```rust
pub fn get_stats(&self) -> Result<Stats> {
    let conn = conn.lock()?;
    let data = self.get_helper_with_conn(&conn)?;  // Pass connection
}

fn get_helper_with_conn(&self, conn: &Connection) -> Result<Data> {
    // No locking - uses provided connection
}
```

**Real Bug Fixed**: `mood/repository.rs:365` - `get_mood_stats` deadlocked by calling `get_activity_correlations` which tried to re-lock.

### Transaction Pattern (RAII)

Prefer `rusqlite::Transaction` over manual BEGIN/COMMIT:

```rust
// ❌ Manual (error-prone):
conn.execute("BEGIN TRANSACTION", [])?;
// ... operations ...
conn.execute("COMMIT", [])?;  // Forgot rollback on error!

// ✅ RAII (automatic rollback on drop):
let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
// ... operations ...
tx.commit()?;  // Auto-rollback if dropped without commit or on panic
```

**When to use**:
- IMMEDIATE mode for write operations (acquire lock on BEGIN)
- DEFERRED mode for read-heavy, optimistic locking
- EXCLUSIVE mode for bulk operations or schema changes

### Required PRAGMAs

**⚠️ CRITICAL**: Add to `db/mod.rs` after connection open:

```rust
// In Database::new() after Connection::open():
conn.execute_batch("
    PRAGMA foreign_keys = ON;         -- CRITICAL: Enable FK constraints
    PRAGMA journal_mode = WAL;         -- Better concurrency
    PRAGMA synchronous = NORMAL;       -- Safe for WAL mode
    PRAGMA busy_timeout = 5000;        -- Prevent immediate SQLITE_BUSY
")?;
```

**Why foreign_keys is critical**: SQLite defaults to OFF for backwards compatibility. Without this, FK constraints in schema are ignored!

### Query Security (100% Parameterization)

**✅ ALWAYS USE:**
```rust
// Static parameters with params! macro:
conn.query_row(
    "SELECT * FROM users WHERE id = ?",
    rusqlite::params![user_id],
    |row| { ... }
)?;

// Dynamic parameters with Vec:
let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
if let Some(ref from) = from_date {
    query.push_str(" AND created_at >= ?");
    params.push(from);
}
stmt.query_map(params.as_slice(), |row| { ... })?;
```

**❌ NEVER USE:**
```rust
// SQL injection risk:
let query = format!("SELECT * FROM users WHERE name = '{}'", user_input);
```

### Testing Pattern

Every repository test must follow this structure:

```rust
fn setup_test_repo() -> (Repository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db = Arc::new(Database::new(temp_dir.path()).expect("Failed to create database"));
    (Repository::new(db), temp_dir)
}

#[test]
fn test_operation() {
    let (repo, _temp_dir) = setup_test_repo();

    // Arrange
    let entity = repo.create(...).expect("Failed to create");

    // Act
    let result = repo.operation(...).expect("Failed to operate");

    // Assert
    assert_eq!(result.field, expected_value);
}
```

**TempDir benefits**:
- Isolated test database (no cross-test contamination)
- Automatic cleanup on drop
- Migrations run automatically via `Database::new()`

### Migration Checklist

Before merging database migrations:

- [ ] Foreign key constraints defined with appropriate CASCADE
- [ ] CHECK constraints for validation (e.g., rating BETWEEN 1 AND 5)
- [ ] Partial indexes for soft deletes (`WHERE deleted_at IS NULL`)
- [ ] Migration tested with rollback (if applicable)
- [ ] Updated repository methods for new/modified tables
- [ ] Integration tests added for new functionality
- [ ] No ALTER COLUMN or DROP COLUMN (not supported < SQLite 3.35)

### Performance Best Practices

**Statement Caching** (for repeated queries):
```rust
conn.set_prepared_statement_cache_capacity(100);
let mut stmt = conn.prepare_cached("SELECT * FROM users WHERE id = ?")?;
for id in ids {
    stmt.query_row([id], |row| { ... })?;
}
```

**Batch Operations** (wrap in transaction):
```rust
let tx = conn.transaction()?;
{
    let mut stmt = tx.prepare_cached("INSERT INTO users VALUES (?1, ?2)")?;
    for user in users {
        stmt.execute(params![user.name, user.email])?;
    }
}
tx.commit()?;  // 100x-1000x faster than individual commits
```

**Avoid N+1 Queries** (use JOINs):
```rust
// ❌ BAD:
for post in posts {
    let comments = get_comments_for_post(post.id)?;  // N queries
}

// ✅ GOOD:
SELECT p.*, c.* FROM posts p LEFT JOIN comments c ON p.id = c.post_id
```

### Common Pitfalls

| Pitfall | Detection | Fix |
|---------|-----------|-----|
| Nested lock acquisition | Method locks then calls method that locks | Use `_with_conn` pattern |
| Forgotten foreign keys | Schema has FK but runtime doesn't enforce | Add `PRAGMA foreign_keys=ON` |
| Long-lived transactions | Lock held during non-DB operations | Minimize transaction scope |
| SQL injection | String interpolation in queries | Use parameterized queries |
| Statement re-preparation | `prepare()` in loops | Use `prepare_cached()` |

### Error Handling

Use `thiserror` for feature-level error types:

```rust
#[derive(Error, Debug)]
pub enum FeatureError {
    #[error("Invalid rating: {0}. Must be 1-5")]
    InvalidRating(i32),

    #[error("Database lock poisoned")]
    LockPoisoned,

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Transaction rollback failed: {0}")]
    TransactionFailure(String),
}
```

**Error Propagation**:
```rust
let result = conn.query_row(...)?;  // Auto-converts via #[from]
```

### Resources

- `.claude/knowledge/database-patterns.md` - Codebase-specific patterns & architecture
- `.claude/knowledge/sqlite-reference.md` - SQLite/rusqlite API reference
- `.claude/knowledge/sqlite-anti-patterns.md` - Common mistakes to avoid
- Slash commands: `/db-review`, `/db-implement`, `/db-refactor`

---

## Svelte Coding Guidelines

### Style & Formatting
- Use Prettier for formatting (if configured)
- Use ESLint with Svelte plugin
- Prefer `<script lang="ts">` for TypeScript
- Maximum line length: 100 characters

### Component Structure
- Order: `<script>`, `<style>`, then markup
- Use SvelteKit file-based routing conventions
- Component file naming: `PascalCase.svelte` for components, `+page.svelte` for routes
- Example structure:
  ```svelte
  <script lang="ts">
    // Imports
    import { invoke } from '@tauri-apps/api/tauri';

    // Props
    export let assessmentType: string;

    // State
    let score: number | null = null;

    // Functions
    async function submitAssessment() { ... }
  </script>

  <div class="assessment">
    <!-- Markup -->
  </div>

  <style>
    /* Scoped styles */
  </style>
  ```

### State Management
- Use Svelte stores for shared state (`lib/stores/`)
- Writable stores for mutable state
- Derived stores for computed values
- Keep stores focused and single-purpose
- Example:
  ```typescript
  // lib/stores/assessments.ts
  import { writable } from 'svelte/store';

  export const currentAssessment = writable<Assessment | null>(null);
  export const assessmentHistory = writable<AssessmentResponse[]>([]);
  ```

### Tauri Integration
- Use auto-generated types from `tauri-specta` (in `lib/bindings.ts`)
- Wrap Tauri commands in utility functions for error handling
- Example:
  ```typescript
  // lib/utils/api.ts
  import { invoke } from '@tauri-apps/api/tauri';
  import type { AssessmentResponse } from '$lib/bindings';

  export async function submitAssessment(req: SubmitAssessmentRequest): Promise<AssessmentResponse> {
    try {
      return await invoke('submit_assessment', { request: req });
    } catch (error) {
      console.error('Failed to submit assessment:', error);
      throw error;
    }
  }
  ```

### Naming Conventions
- **Components**: `PascalCase.svelte` (e.g., `AssessmentCard.svelte`)
- **Routes**: `+page.svelte`, `+layout.svelte`
- **Stores**: `camelCase.ts` (e.g., `assessmentStore.ts`)
- **Utilities**: `camelCase.ts` (e.g., `formatDate.ts`)
- **Variables/Functions**: `camelCase`
- **Constants**: `SCREAMING_SNAKE_CASE`

### Accessibility
- Always include `aria-label` for icon buttons
- Use semantic HTML (`<button>`, `<nav>`, `<main>`, etc.)
- Ensure keyboard navigation works (test with Tab/Enter/Escape)
- Maintain color contrast ratios (use TailwindCSS utilities)
- Add focus indicators for all interactive elements

### TailwindCSS Usage
- Use utility classes in markup (avoid custom CSS when possible)
- Extract common patterns into components, not `@apply` directives
- Use Tailwind's color palette for consistency
- Responsive design: Mobile-first with `sm:`, `md:`, `lg:` breakpoints

### Testing
- Component tests with Vitest + Testing Library
- Test user interactions, not implementation details
- Example:
  ```typescript
  import { render, fireEvent } from '@testing-library/svelte';
  import AssessmentCard from './AssessmentCard.svelte';

  test('displays assessment score', () => {
    const { getByText } = render(AssessmentCard, { props: { score: 15 } });
    expect(getByText('Score: 15')).toBeInTheDocument();
  });
  ```

### Performance
- Use `{#key}` blocks to force re-renders when needed
- Avoid expensive computations in reactive statements
- Use `onMount` for initialization, `onDestroy` for cleanup
- Lazy load heavy components with dynamic imports

---

## General Project Guidelines

### Git Workflow
- Commit messages: Follow conventional commits (`feat:`, `fix:`, `docs:`, `test:`)
- Branch naming: `feature/description`, `fix/description`
- Small, focused commits (each commit should pass tests)

### Documentation
- Document public APIs with rustdoc (`///`) and JSDoc (`/**`)
- Keep README.md updated with setup instructions
- Add inline comments for complex logic only

### Performance Targets (from spec.md)
- UI responsiveness: <100ms for all interactions
- Chart rendering: <500ms
- Assessment submission: <2s end-to-end

---

## Extended Guidelines (from GitHub Awesome Copilot)

### Self-Explanatory Code Commenting

**Core Principle**: Write code that speaks for itself. Comment only when necessary to explain WHY, not WHAT.

**Comments to Avoid**:
- **Obvious Comments**: Don't state what the code clearly shows ("Initialize counter to zero", "Increment counter by one")
- **Redundant Comments**: Avoid repeating the code's meaning in prose form
- **Outdated Comments**: Never let documentation drift from actual implementation

**Comments Worth Writing**:
- **Complex Business Logic**: Clarify non-obvious calculations or domain-specific rules
- **Algorithm Choices**: Explain why you selected a particular algorithm
  - Example: "Using Floyd-Warshall for all-pairs shortest paths because we need distances between all nodes"
- **Regex Patterns**: Describe what complex regular expressions match in plain language
- **API Constraints**: Document external limitations
  - Example: "GitHub API rate limit: 5000 requests/hour for authenticated users"

**Decision Framework** (before commenting):
1. Is the code self-explanatory?
2. Would better naming eliminate the need?
3. Does this explain WHY, not WHAT?
4. Will future maintainers benefit?

**Special Cases**:
- **Public APIs**: Use structured documentation (rustdoc `///`, JSDoc `/**`)
- **Constants**: Explain reasoning ("Based on network reliability studies")
- **Annotations**: Use standard markers: TODO, FIXME, HACK, NOTE, WARNING, PERF, SECURITY, BUG, REFACTOR, DEPRECATED

**Anti-Patterns**:
- Don't comment out code; use version control instead
- Never maintain change history in comments
- Avoid decorative divider lines

---

### Rust - Extended Guidelines (GitHub Awesome Copilot)

**Overview**: Follow idiomatic Rust practices based on The Rust Book, Rust API Guidelines, RFC 430, and community standards.

**General Instructions**:
- Prioritize readability, safety, and maintainability throughout
- Leverage strong typing and Rust's ownership system for memory safety
- Decompose complex functions into smaller, manageable units
- Include explanations for algorithm-related code
- Handle errors gracefully using `Result<T, E>` with meaningful messages
- Document external dependencies and their purposes
- Follow RFC 430 naming conventions consistently
- Ensure code compiles without warnings

**Ownership, Borrowing, and Lifetimes**:
- Prefer borrowing (`&T`) over cloning unless ownership transfer is necessary
- Use `&mut T` when modifying borrowed data
- Explicitly annotate lifetimes when the compiler cannot infer them
- Use `Rc<T>` for single-threaded reference counting; `Arc<T>` for thread-safe scenarios
- Use `RefCell<T>` for interior mutability in single-threaded contexts; `Mutex<T>` or `RwLock<T>` for multi-threaded

**Patterns to Follow**:
- Use modules (`mod`) and public interfaces (`pub`) for encapsulation
- Handle errors properly with `?`, `match`, or `if let`
- Employ `serde` for serialization and `thiserror`/`anyhow` for custom errors
- Implement traits to abstract services or dependencies
- Structure async code using `async/await` with `tokio` or `async-std`
- Prefer enums over flags for type safety
- Use builders for complex object creation
- Separate binary and library code for testability
- Use `rayon` for data parallelism
- Prefer iterators over index-based loops
- Use `&str` instead of `String` for function parameters when ownership isn't needed
- Favor borrowing and zero-copy operations

---

### TypeScript 5.x / ES2022 - Extended Guidelines (GitHub Awesome Copilot)

**Core Principles**:
- Respect existing architecture and coding standards
- Prioritize readable, explicit solutions over clever shortcuts
- Extend current abstractions before creating new ones
- Focus on maintainability, clarity, and clean code

**General Requirements**:
- Target TypeScript 5.x compiling to ES2022 JavaScript baseline
- Use pure ES modules exclusively; avoid `require`, `module.exports`, or CommonJS
- Rely on project's build, lint, and test scripts
- Document design trade-offs when intent isn't obvious

**Project Organization**:
- Follow repository's folder structure and responsibility layout
- Use kebab-case filenames (e.g., `user-session.ts`, `data-service.ts`)
- Keep tests, types, and helpers near implementation for discoverability
- Reuse or extend shared utilities before adding new ones

**Naming Conventions**:
- PascalCase for classes, interfaces, enums, and type aliases
- camelCase for variables, functions, and properties
- Avoid interface prefixes like `I`; use descriptive names instead
- Name entities for their behavior or domain meaning, not implementation

**Code Style & Formatting**:
- Run project's lint/format scripts before submission
- Match project's indentation, quote style, and trailing comma rules
- Keep functions focused; extract helpers when logic branches grow
- Favor immutable data and pure functions when practical

**Type System**:
- Avoid `any` (implicit or explicit); prefer `unknown` with type narrowing
- Use discriminated unions for realtime events and state machines
- Centralize shared contracts instead of duplicating type shapes
- Leverage TypeScript utility types (`Readonly`, `Partial`, `Record`)

**Async & Error Handling**:
- Use `async/await` wrapped in try/catch with structured error handling
- Guard edge cases early to prevent deep nesting
- Route errors through project's logging/telemetry utilities
- Surface user-facing errors via repository's notification pattern
- Debounce configuration-driven updates; dispose resources deterministically

**Architecture & Patterns**:
- Follow repository's dependency injection or composition pattern
- Keep modules single-purpose
- Observe existing initialization and disposal sequences

---

### Svelte 5 - Extended Guidelines (GitHub Awesome Copilot)

**Project Foundation**:
- Svelte 5.x with runes system (`$state`, `$derived`, `$effect`, `$props`, `$bindable`)
- SvelteKit for full-stack applications
- TypeScript integration
- Component-scoped styling with CSS custom properties

**Core Architecture Principles**:
- Organize components by feature or domain for scalability
- Separate presentation components from logic-heavy components
- Extract reusable logic into composable functions
- Implement proper component composition with slots and snippets
- Use SvelteKit's file-based routing with proper load functions

**TypeScript Standards**:
- Enable strict mode in `tsconfig.json` for maximum type safety
- Define interfaces for component props using the `$props()` syntax
- Type event handlers and refs
- Leverage generated `$types.ts` files
- Implement proper type checking with `svelte-check`

**Reactivity with Runes**:
- Use `$state()` for reactive local state management
- Implement `$derived()` for computed values and expensive calculations
- Use `$effect()` for side effects with proper cleanup
- Define component props with `$props()` and destructuring
- Use `$bindable()` for two-way data binding between components

**Component Design**:
- Follow single responsibility principle
- Keep components small and focused on one concern
- Implement proper prop validation with TypeScript
- Use slots and snippets for flexible composition
- Design components to be testable and reusable

**State Management Approach**:
- Use `$state()` for local component state
- Implement context API with `setContext`/`getContext` for shared state
- Use SvelteKit stores for global application state when needed
- Keep state normalized for complex data structures
- Implement proper state persistence for client-side data

**SvelteKit Patterns**:
- Use `+page.svelte` for page components with proper SEO
- Implement `+layout.svelte` for shared layouts and navigation
- Use `+page.server.ts` for server-side data loading and API calls
- Implement form actions in `+page.server.ts` for data mutations
- Use `+server.ts` for API endpoints and server-side logic

**Styling Conventions**:
- Use component-scoped styles with `<style>` blocks
- Implement CSS custom properties for theming
- Follow mobile-first responsive design patterns
- Use proper CSS specificity and avoid `!important`
- Leverage Tailwind utilities when configured

---

## Database Schema - Activity Groups Feature

### Overview

The Activity Groups feature extends the activities tracking system with organization, goal setting, and reporting capabilities. All data is stored locally in SQLite with proper indexes for performance.

### Schema Tables

#### `activity_groups` Table

Organizes related activities into categories for easier management and group-level reporting.

```sql
CREATE TABLE IF NOT EXISTS activity_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK(length(trim(name)) > 0 AND length(name) <= 100),
    description TEXT CHECK(description IS NULL OR length(description) <= 500),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT,
    CONSTRAINT unique_active_group_name UNIQUE (name) WHERE deleted_at IS NULL
);

-- Index for soft-delete filtering
CREATE INDEX IF NOT EXISTS idx_activity_groups_deleted
    ON activity_groups(deleted_at)
    WHERE deleted_at IS NULL;
```

**Key Features**:
- **Soft Delete**: `deleted_at` column for data recovery
- **Unique Names**: Only among active (non-deleted) groups
- **Validation**: CHECK constraints for name length and non-empty
- **Partial Index**: Fast filtering of non-deleted groups

#### Updated `activities` Table

Extended with `group_id` foreign key to support group membership.

```sql
-- New column added to existing activities table
ALTER TABLE activities ADD COLUMN group_id INTEGER REFERENCES activity_groups(id) ON DELETE SET NULL;

-- Index for group-based queries
CREATE INDEX IF NOT EXISTS idx_activities_group ON activities(group_id);
```

**Changes**:
- **Optional Group**: `group_id` can be NULL (ungrouped activities)
- **CASCADE NULL**: If group deleted, activities become ungrouped
- **Indexed**: Fast retrieval of activities by group

#### `activity_logs` Table

Tracks when activities are performed, with timestamps and optional notes.

```sql
CREATE TABLE IF NOT EXISTS activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER NOT NULL REFERENCES activities(id) ON DELETE CASCADE,
    logged_at TEXT NOT NULL,
    notes TEXT CHECK(notes IS NULL OR length(notes) <= 1000),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_activity_logs_activity ON activity_logs(activity_id);
CREATE INDEX IF NOT EXISTS idx_activity_logs_logged_at ON activity_logs(logged_at);
CREATE INDEX IF NOT EXISTS idx_activity_logs_deleted ON activity_logs(deleted_at) WHERE deleted_at IS NULL;
```

**Key Features**:
- **Activity Link**: CASCADE DELETE removes logs when activity deleted
- **Soft Delete**: Supports data recovery
- **Timestamp Index**: Fast date-range queries for reporting
- **Notes**: Optional context for each log entry

#### `activity_goals` Table

Defines targets for activities or groups, supporting two goal types.

```sql
CREATE TABLE IF NOT EXISTS activity_goals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_id INTEGER REFERENCES activities(id) ON DELETE CASCADE,
    group_id INTEGER REFERENCES activity_groups(id) ON DELETE CASCADE,
    goal_type TEXT NOT NULL CHECK(goal_type IN ('days_per_period', 'percent_improvement')),
    target_value INTEGER NOT NULL CHECK(target_value > 0),
    period_days INTEGER NOT NULL CHECK(period_days > 0),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT,
    CHECK(
        (activity_id IS NOT NULL AND group_id IS NULL) OR
        (activity_id IS NULL AND group_id IS NOT NULL)
    )
);

-- Indexes for goal queries
CREATE INDEX IF NOT EXISTS idx_activity_goals_activity ON activity_goals(activity_id);
CREATE INDEX IF NOT EXISTS idx_activity_goals_group ON activity_goals(group_id);
CREATE INDEX IF NOT EXISTS idx_activity_goals_deleted ON activity_goals(deleted_at) WHERE deleted_at IS NULL;
```

**Key Features**:
- **XOR Constraint**: Goal applies to EITHER activity OR group, not both
- **Goal Types**:
  - `days_per_period`: Track consistency (e.g., "5 days per 7 days")
  - `percent_improvement`: Track growth (e.g., "25% increase")
- **Validation**: Positive targets and periods required
- **CASCADE DELETE**: Goals removed when parent deleted

### Data Integrity

**Foreign Key Constraints**:
- `activities.group_id → activity_groups.id` (SET NULL on delete)
- `activity_logs.activity_id → activities.id` (CASCADE on delete)
- `activity_goals.activity_id → activities.id` (CASCADE on delete)
- `activity_goals.group_id → activity_groups.id` (CASCADE on delete)

**CHECK Constraints**:
- String length limits (names, descriptions, notes)
- Non-empty trimmed strings (prevents whitespace-only entries)
- Valid enum values (`goal_type`)
- Positive numeric values (`target_value`, `period_days`)
- XOR logic (activity_id/group_id exclusivity)

**Soft Deletes**:
All tables use `deleted_at` timestamps instead of hard deletes:
- Supports data recovery and audit trails
- Indexed for fast filtering of active records
- UNIQUE constraints apply only to non-deleted records

### Performance Optimizations

**Indexes by Use Case**:

1. **List Activities by Group**:
   ```sql
   -- Uses: idx_activities_group
   SELECT * FROM activities WHERE group_id = ? AND deleted_at IS NULL;
   ```

2. **Get Activity Logs in Date Range**:
   ```sql
   -- Uses: idx_activity_logs_logged_at, idx_activity_logs_activity
   SELECT * FROM activity_logs
   WHERE activity_id = ?
     AND logged_at BETWEEN ? AND ?
     AND deleted_at IS NULL;
   ```

3. **Find Active Goals**:
   ```sql
   -- Uses: idx_activity_goals_deleted, idx_activity_goals_activity
   SELECT * FROM activity_goals
   WHERE activity_id = ?
     AND deleted_at IS NULL;
   ```

4. **Group-Level Statistics**:
   ```sql
   -- Uses: idx_activities_group, idx_activity_logs_activity
   SELECT COUNT(DISTINCT DATE(logged_at)) as days
   FROM activity_logs
   WHERE activity_id IN (
       SELECT id FROM activities WHERE group_id = ?
   )
   AND logged_at > ?
   AND activity_logs.deleted_at IS NULL;
   ```

**Benchmark Results** (Criterion):
- Group CRUD operations: **~50 µs** (0.05 ms)
- Retrieve 1000 logs: **~328 µs** (0.33 ms)
- Complex reporting queries (1200 logs): **~160 µs** (0.16 ms)
- Goal progress calculation: **~198 µs** (0.20 ms)

All operations are **1000x-6000x faster** than target thresholds.

### Migration History

**Migration 002**: Activity Groups Foundation
- Created `activity_groups` table
- Added `group_id` to `activities` table
- Added indexes for group queries

**Migration 003**: Goals and Tracking
- Created `activity_logs` table
- Created `activity_goals` table
- Added comprehensive indexes for reporting queries
- Fixed foreign key constraints for proper CASCADE behavior

### Query Examples

**Get Activities with Groups**:
```rust
let query = "
    SELECT a.id, a.name, a.color, a.icon,
           g.id as group_id, g.name as group_name
    FROM activities a
    LEFT JOIN activity_groups g ON a.group_id = g.id
    WHERE a.deleted_at IS NULL
    ORDER BY g.name, a.name
";
```

**Calculate Activity Frequency**:
```rust
let query = "
    SELECT COUNT(DISTINCT DATE(logged_at)) as unique_days
    FROM activity_logs
    WHERE activity_id = ?
      AND logged_at BETWEEN ? AND ?
      AND deleted_at IS NULL
";
// Result: days / total_days_in_period * 7 = days_per_week
```

**Check Goal Progress (Days per Period)**:
```rust
let query = "
    SELECT COUNT(DISTINCT DATE(logged_at)) as logged_days
    FROM activity_logs
    WHERE activity_id = ?
      AND logged_at >= ?
      AND deleted_at IS NULL
";
// Progress: (logged_days / target_value) * 100
```

**Group-Level Goal Progress**:
```rust
let query = "
    SELECT COUNT(DISTINCT DATE(al.logged_at)) as logged_days
    FROM activity_logs al
    JOIN activities a ON al.activity_id = a.id
    WHERE a.group_id = ?
      AND al.logged_at >= ?
      AND al.deleted_at IS NULL
      AND a.deleted_at IS NULL
";
```

### Best Practices

**When Querying**:
- Always filter by `deleted_at IS NULL` for active records
- Use date indexes for time-range queries
- Leverage partial indexes for soft-delete filtering
- Use JOINs instead of N+1 queries for group statistics

**When Mutating**:
- Wrap multiple operations in transactions (RAII pattern)
- Use CASCADE for dependent deletes (logs, goals)
- Use SET NULL for optional relationships (group membership)
- Validate input before database operations

**Performance Tips**:
- Date filtering is highly optimized (use it!)
- GROUP BY on dates is fast with proper indexes
- Avoid DISTINCT unless necessary
- Use `prepare_cached()` for repeated queries

### Related Documentation

- **User Guide**: [docs/ACTIVITY_GROUPS_GUIDE.md](../docs/ACTIVITY_GROUPS_GUIDE.md)
- **Repository Code**: `src-tauri/src/features/activities/repository.rs`
- **Migration Files**: `src-tauri/migrations/002_activity_groups.sql`, `003_goals_and_tracking.sql`
- **Benchmarks**: `src-tauri/benches/reporting_benchmarks.rs`

<!-- MANUAL ADDITIONS END -->
