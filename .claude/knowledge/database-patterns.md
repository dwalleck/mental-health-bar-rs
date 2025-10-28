# Database Patterns - mental-health-bar-rs

**Comprehensive reference for database architecture, patterns, and anti-patterns in this codebase**

Last Updated: 2025-10-27
Based On: Codebase analysis + deadlock debugging session

---

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Connection Management Pattern](#connection-management-pattern)
3. [The _with_conn Deadlock Prevention Pattern](#the-_with_conn-deadlock-prevention-pattern)
4. [Repository Pattern](#repository-pattern)
5. [Query Patterns](#query-patterns)
6. [Transaction Management](#transaction-management)
7. [Error Handling](#error-handling)
8. [Testing Patterns](#testing-patterns)
9. [Schema Patterns](#schema-patterns)
10. [Common Pitfalls & Solutions](#common-pitfalls--solutions)

---

## Architecture Overview

### Database Structure
**Location:** `src-tauri/src/db/mod.rs:10-72`

```rust
pub struct Database {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl Database {
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)  // Line 71
    }
}
```

**Key Characteristics:**
- **Single shared connection** wrapped in `Arc<Mutex<rusqlite::Connection>>`
- Enables **thread-safe access** across async Tauri commands
- **Cloneable Arc references** distributed via `get_connection()` method
- Connections **automatically initialized** with schema migrations

### Vertical Slice Architecture

Features organized as independent slices:
```
features/
├── mood/
│   ├── models.rs         # Domain types & errors
│   ├── repository.rs     # Data access (36-777 lines)
│   ├── commands.rs       # Tauri commands (mutations)
│   └── queries.rs        # Tauri queries (reads)
├── assessments/
│   ├── models.rs
│   ├── repository.rs     # (13-376 lines)
│   ├── commands.rs
│   └── queries.rs
└── visualization/
    ├── models.rs
    ├── repository.rs     # (12-362 lines)
    └── queries.rs
```

**Pattern Benefits:**
- Each feature owns complete data lifecycle
- No cross-feature repository dependencies
- Clear ownership boundaries
- Easy to test in isolation

---

## Connection Management Pattern

### Standard Lock Acquisition
**Found in:** All 3 repositories (mood, assessments, visualization)

```rust
// PATTERN: Every public repository method follows this:
pub fn operation(&self, ...) -> Result<T, Error> {
    let conn = self.db.get_connection();  // Get Arc clone
    let conn = conn.lock().map_err(|_| Error::LockPoisoned)?;  // Acquire lock

    // Use connection for all operations
    let result = conn.query_row(...)?;
    Ok(result)
}  // Lock released automatically when conn goes out of scope
```

### Lock Scope Discipline

**✅ GOOD PATTERN:**
```rust
// Lock held for minimal scope
pub fn create_mood_checkin(&self, ...) -> Result<MoodCheckin, MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // All database operations here
    // ...

    Ok(mood_checkin)
}  // Lock released
```

**❌ ANTI-PATTERN:**
```rust
// DON'T: Lock held across non-database operations
pub fn bad_pattern(&self) -> Result<()> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    let data = conn.query_row(...)?;

    // Lock still held during expensive processing!
    complex_calculation(data)?;  // WASTEFUL
    external_api_call(data).await?;  // BLOCKS OTHER OPERATIONS

    Ok(())
}
```

### Lock Poisoning Handling

Every repository maps lock poisoning consistently:

```rust
let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

// Error type definition (mood/models.rs:7-8):
#[error("Database lock poisoned - a panic occurred while holding the database lock. The application should restart.")]
LockPoisoned,
```

---

## The _with_conn Deadlock Prevention Pattern

### The Problem: Nested Lock Acquisition

**Real Bug Example:** `src-tauri/src/features/mood/repository.rs:365` (fixed)

```rust
// ❌ THIS DEADLOCKS:
pub fn get_mood_stats(&self, ...) -> Result<MoodStats, MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;  // ← Lock acquired here

    // ... do work ...

    // Calls helper that tries to acquire SAME lock:
    let correlations = self.get_activity_correlations(from_date, to_date)?;
    //                      ^^^ DEADLOCK - tries to lock() again!

    Ok(MoodStats { ... })
}

fn get_activity_correlations(&self, ...) -> Result<...> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;  // ← DEADLOCK! Lock already held by caller
    // ...
}
```

**Why It Deadlocks:**
1. `get_mood_stats()` acquires the mutex lock
2. While holding that lock, it calls `get_activity_correlations()`
3. `get_activity_correlations()` tries to acquire the **same lock**
4. Rust's `Mutex` is **not re-entrant** - can't lock twice
5. Thread blocks waiting for itself → **deadlock**

### The Solution: _with_conn Pattern

**✅ CORRECT PATTERN:**

```rust
// Public method - acquires lock once
pub fn get_mood_stats(&self, ...) -> Result<MoodStats, MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;  // Lock acquired once

    // Pass connection reference to helper (no re-lock)
    let correlations = self.get_activity_correlations_with_conn(&conn, from_date, to_date)?;
    //                                                           ^^^^^ Connection reference

    Ok(MoodStats { ... })
}

// Helper accepts connection reference - NO locking
fn get_activity_correlations_with_conn(
    &self,
    conn: &rusqlite::Connection,  // ← Borrows existing connection
    from_date: Option<String>,
    to_date: Option<String>,
) -> Result<Vec<ActivityCorrelation>, MoodError> {
    // Use conn directly, no lock acquisition
    let mut stmt = conn.prepare("SELECT ...")?;
    // ...
}
```

### Naming Convention

**Rule:** Helpers that accept a connection reference MUST have `_with_conn` suffix

| Method Signature | Purpose | Locks? |
|------------------|---------|--------|
| `pub fn get_mood_stats()` | Public API | ✅ Acquires lock |
| `fn get_activity_correlations_with_conn(&conn)` | Internal helper | ❌ Uses provided conn |

### All _with_conn Methods in Codebase

**Location:** `src-tauri/src/features/mood/repository.rs`

1. **`get_activities_for_checkin_with_conn`** (line 266-295)
   - Used in: `create_mood_checkin` (line 130), `get_mood_history` (line 216), `get_mood_checkin` (line 250)
   - Purpose: Fetch activities for a check-in within existing lock

2. **`get_activity_correlations_with_conn`** (line 376-437)
   - Used in: `get_mood_stats` (line 365)
   - Purpose: Calculate activity-mood correlations within existing lock

**Usage Example:**

```rust
// src-tauri/src/features/mood/repository.rs:130
let activities = self.get_activities_for_checkin_with_conn(&conn, mood_checkin_id)?;
```

### Detection Checklist

Look for these deadlock indicators:

- [ ] Method acquires lock via `conn.lock()`
- [ ] Method calls `self.other_method()` that isn't `_with_conn`
- [ ] Other method also acquires lock
- [ ] **= DEADLOCK** (thread waits for itself)

---

## Repository Pattern

### Standard Structure

**All 3 repositories follow this pattern:**

```rust
pub struct MoodRepository {
    db: Arc<Database>,
}

impl MoodRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // Public methods acquire lock once
    pub fn create_mood_checkin(&self, ...) -> Result<MoodCheckin, MoodError> {
        let conn = self.db.get_connection();
        let conn = conn.lock()?;
        // ... operations ...
    }

    // Helpers accept connection reference
    fn helper_with_conn(&self, conn: &Connection, ...) -> Result<T, Error> {
        // No locking - uses provided connection
    }
}
```

### Repositories in Codebase

| Repository | Lines | Public Methods | _with_conn Helpers |
|------------|-------|----------------|-------------------|
| `MoodRepository` | 742 | 10 | 2 |
| `AssessmentRepository` | 364 | 8 | 0 |
| `VisualizationRepository` | 351 | 2 | 0 |

### Construction Pattern

**In Tauri Commands:**

```rust
#[tauri::command]
pub async fn log_mood(
    request: LogMoodRequest,
    state: State<'_, AppState>,
) -> Result<MoodCheckin, String> {
    let repo = MoodRepository::new(state.db.clone());  // Clone Arc, not Database
    repo.create_mood_checkin(...)
        .map_err(|e| e.to_string())
}
```

**In Tests:**

```rust
fn setup_test_repo() -> (MoodRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db = Arc::new(Database::new(temp_dir.path()).expect("Failed to create database"));
    (MoodRepository::new(db), temp_dir)
}
```

---

## Query Patterns

### Parameterized Queries (100% Coverage)

**✅ ALL queries in codebase use parameterization - NO string interpolation found**

#### Pattern 1: Static Parameters with `params!` Macro

```rust
// src-tauri/src/features/mood/repository.rs:86-90
let (mood_checkin_id, created_at): (i32, String) = conn.query_row(
    "INSERT INTO mood_checkins (mood_rating, notes) VALUES (?, ?)
     RETURNING id, CAST(created_at AS VARCHAR)",
    rusqlite::params![mood_rating, notes],  // ← Static params
    |row| Ok((row.get(0)?, row.get(1)?)),
)?;
```

#### Pattern 2: Dynamic Parameters with Vec

**Use when:** Query conditions are optional

```rust
// src-tauri/src/features/mood/repository.rs:181-202
let mut query = String::from("SELECT ... WHERE 1=1");
let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

if let Some(ref from) = from_date {
    query.push_str(" AND created_at >= ?");
    params.push(from);  // ← Add param only if condition applies
}
if let Some(ref to) = to_date {
    query.push_str(" AND created_at <= ?");
    params.push(to);
}

let mut stmt = conn.prepare(&query)?;
stmt.query_map(params.as_slice(), |row| { ... })?;
```

### RETURNING Clause Usage

**SQLite Version:** Requires 3.35+ (bundled version supports this)

**Benefits:**
- Atomic insert + select in single query
- Eliminates race conditions
- Cleaner than `last_insert_rowid()`

```rust
// src-tauri/src/features/mood/repository.rs:479-484
let result = conn.query_row(
    "INSERT INTO activities (name, color, icon) VALUES (?, ?, ?)
     RETURNING id, name, color, icon, CAST(created_at AS VARCHAR)",
    rusqlite::params![name, color, icon],
    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
)?;
```

### JOIN Patterns

**Pattern:** Fetch related data in single query

```rust
// src-tauri/src/features/mood/repository.rs:271-276
conn.prepare(
    "SELECT a.id, a.name, a.color, a.icon,
            CAST(a.created_at AS VARCHAR), CAST(a.deleted_at AS VARCHAR)
     FROM activities a
     JOIN mood_checkin_activities mca ON a.id = mca.activity_id
     WHERE mca.mood_checkin_id = ?",
)?;
```

**Anti-Pattern (N+1 Query):** Avoided in this codebase

```rust
// ❌ DON'T DO THIS:
let mood_checkins = conn.prepare("SELECT * FROM mood_checkins")?.query_map(...)?;
for checkin in mood_checkins {
    let activities = conn.prepare("SELECT * FROM activities WHERE checkin_id = ?")?.query_map([checkin.id], ...)?;
    // N+1 queries!
}

// ✅ DO THIS:
conn.prepare("
    SELECT mc.*, a.*
    FROM mood_checkins mc
    LEFT JOIN mood_checkin_activities mca ON mc.id = mca.mood_checkin_id
    LEFT JOIN activities a ON mca.activity_id = a.id
")?;
```

### Query Result Mapping

```rust
// src-tauri/src/features/mood/repository.rs:278-287
let activity_rows = stmt.query_map([mood_checkin_id], |row| {
    Ok(Activity {
        id: row.get(0)?,
        name: row.get(1)?,
        color: row.get(2)?,
        icon: row.get(3)?,
        created_at: row.get(4)?,
        deleted_at: row.get(5)?,
    })
})?;

let mut activities = Vec::new();
for activity_result in activity_rows {
    activities.push(activity_result?);  // Propagate errors
}
```

---

## Transaction Management

### Current Pattern: Manual BEGIN/COMMIT

**Locations:** 3 occurrences in codebase
1. `mood/repository.rs`: `create_mood_checkin` (lines 82-158)
2. `mood/repository.rs`: `delete_mood_checkin` (lines 742-775)
3. `assessments/repository.rs`: `save_assessment` (lines 39-67)

```rust
// src-tauri/src/features/mood/repository.rs:82-158
conn.execute("BEGIN TRANSACTION", [])?;

let result = (|| {
    // ... operations that might fail ...
    Ok(mood_checkin)
})();

match result {
    Ok(mood_checkin) => {
        conn.execute("COMMIT", [])?;
        Ok(mood_checkin)
    }
    Err(e) => {
        if let Err(rollback_err) = conn.execute("ROLLBACK", []) {
            error!("CRITICAL: Failed to rollback transaction: {}", rollback_err);
            return Err(MoodError::TransactionFailure(format!(
                "Original error: {}. Rollback error: {}",
                e, rollback_err
            )));
        }
        Err(e)
    }
}
```

### Recommended Pattern: RAII with rusqlite::Transaction

**⚠️ NOT YET USED IN CODEBASE - Recommended for future refactoring**

```rust
// RECOMMENDED PATTERN:
let mut tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;

// Operations here...
// Automatic rollback on drop if not committed or if panic occurs

tx.commit()?;  // Explicit commit
```

**Benefits of RAII Pattern:**
- Automatic rollback on drop (safer)
- Can't forget to COMMIT
- Panic-safe (rollback even during unwinding)
- Less boilerplate

---

## Error Handling

### Feature-Level Errors (thiserror)

**Pattern:** Each feature defines typed errors

```rust
// src-tauri/src/features/mood/models.rs:4-42
#[derive(Error, Debug)]
pub enum MoodError {
    #[error("Invalid mood rating: {0}. Must be 1-5")]
    InvalidRating(i32),

    #[error("Database lock poisoned - a panic occurred while holding the database lock. The application should restart.")]
    LockPoisoned,

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Transaction rollback failed: {0}. Database may be in inconsistent state")]
    TransactionFailure(String),

    #[error("Mood check-in not found: {0}")]
    MoodCheckinNotFound(i32),

    #[error("Activity not found: {0}")]
    ActivityNotFound(i32),

    // ... 9 total variants
}
```

### Error Type Consistency

| Feature | Error Type | Enum? |
|---------|----------|-------|
| Mood | `MoodError` | ✅ thiserror |
| Assessments | `AssessmentError` | ✅ thiserror |
| Visualization | `String` | ❌ ad-hoc (should be improved) |

**Recommendation:** Create `VisualizationError` enum for consistency

### Error Propagation

**Pattern:** Use `?` operator with `From` trait

```rust
// Automatic conversion from rusqlite::Error to MoodError::Database
let result = conn.query_row(...)?;
//                              ^^^ Calls MoodError::from(rusqlite::Error)
```

### Error Context (anyhow)

**Used in:** `db/mod.rs`, `db/migrations.rs`

```rust
// src-tauri/src/db/mod.rs:18-19
std::fs::create_dir_all(&app_data_dir)
    .context("Failed to create app data directory")?;
```

---

## Testing Patterns

### Test Database Setup

**Standard pattern across all test files:**

```rust
// src-tauri/tests/test_mood.rs:12-19
fn setup_test_repo() -> (MoodRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().to_path_buf();
    let db = Arc::new(Database::new(db_path).expect("Failed to create database"));
    let repo = MoodRepository::new(db);
    (repo, temp_dir)
}
```

**Pattern Benefits:**
- `TempDir` creates isolated test database
- Automatically cleaned up when dropped
- Migrations run automatically via `Database::new()`
- Each test gets fresh database

### Integration Test Structure

```rust
#[test]
fn test_operation() {
    let (repo, _temp_dir) = setup_test_repo();

    // Arrange
    let activity = repo.create_activity("Exercise", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Act
    let mood = repo.create_mood_checkin(4, vec![activity.id], Some("Feeling great"))
        .expect("Failed to create mood check-in");

    // Assert
    assert_eq!(mood.mood_rating, 4);
    assert_eq!(mood.activities.len(), 1);
}
```

### Test Coverage Types

**1. Security Tests**
```rust
// src-tauri/tests/test_assessments.rs:604-673
#[test]
fn test_sql_injection_protection() {
    let sql_injections = vec![
        "PHQ9'; DROP TABLE assessments; --",
        "PHQ9' OR '1'='1",
        "' UNION SELECT * FROM assessment_types --",
    ];

    for injection in sql_injections {
        let result = repo.get_assessment_type_by_code(injection);
        // Should safely return NotFound, not expose SQL errors
    }
}
```

**2. Boundary Tests**
```rust
// src-tauri/tests/test_mood.rs:355-394
#[test]
fn test_log_mood_with_invalid_boundary_ratings() {
    // Test ratings: 0, 6, -1, 100
    // All should fail with clear error messages
}
```

**3. Edge Case Tests**
```rust
// src-tauri/tests/test_mood.rs:436-475
#[test]
fn test_get_mood_history_invalid_date_formats() {
    let invalid_dates = vec![
        "2024-13-01",    // Invalid month
        "not-a-date",    // Completely invalid
        "",              // Empty string
    ];
    // Document behavior for each case
}
```

### Test Count Summary

| Test File | Tests | Lines | Coverage |
|-----------|-------|-------|----------|
| test_mood.rs | 20 | 585 | Mood feature |
| test_assessments.rs | 9 | 434 | Assessment feature |
| test_activities.rs | 22 | 478 | Activity management |
| test_visualization.rs | 18 | ~400 | Chart data |
| **Total** | **69** | ~1897 | ~70% coverage |

---

## Schema Patterns

### Foreign Keys with CASCADE

```sql
-- src-tauri/src/db/migrations/001_initial_schema.sql:66-67
FOREIGN KEY (mood_checkin_id) REFERENCES mood_checkins(id) ON DELETE CASCADE,
FOREIGN KEY (activity_id) REFERENCES activities(id),
```

**Pattern:** CASCADE delete from parent to junction table, but preserve referenced entities

### Soft Delete with Partial Unique Index

```sql
-- Line 47
CREATE UNIQUE INDEX idx_activities_name_unique
ON activities(name) WHERE deleted_at IS NULL;
```

**Purpose:** Allows name reuse after soft deletion

**Usage:**
- Activity "Exercise" deleted (deleted_at set)
- New activity "Exercise" can be created (different ID)
- Unique constraint only applies to active (non-deleted) records

### Date/Time as TEXT

**Pattern:** Store as ISO 8601 TEXT, cast when retrieving

```sql
-- Schema (line 54)
created_at TEXT NOT NULL DEFAULT (datetime('now'))

-- Query (assessments/repository.rs:162)
CAST(created_at AS VARCHAR) as created_at
```

**Why:** SQLite has no native date type, TEXT allows string comparison due to ISO format

### JSON Storage

```sql
-- Schema (assessment_types, line 13)
thresholds TEXT NOT NULL,  -- JSON stored as TEXT
```

```rust
// Serialize (assessments/repository.rs:34)
let responses_json = serde_json::to_string(responses)?;

// Deserialize (assessments/repository.rs:91)
thresholds: serde_json::from_str(&row.get::<_, String>(7)?)?
```

### CHECK Constraints

```sql
-- Line 52
mood_rating INTEGER NOT NULL CHECK (mood_rating BETWEEN 1 AND 5)

-- Line 53
notes TEXT CHECK (length(notes) <= 5000 OR notes IS NULL)
```

---

## Common Pitfalls & Solutions

### ⚠️ Pitfall 1: Nested Lock Acquisition (DEADLOCK)

**Symptom:** Application hangs, test freezes

**Detection:**
```rust
// Check if method acquires lock:
let conn = conn.lock()?;  // ← Acquires lock

// Then calls another method that also locks:
let data = self.other_method()?;  // ← If other_method locks, DEADLOCK
```

**Solution:** Use `_with_conn` pattern

```rust
// ✅ FIX:
let data = self.other_method_with_conn(&conn)?;  // Pass connection reference
```

### ⚠️ Pitfall 2: Forgetting Foreign Key Enforcement

**Current Status:** ⚠️ **MISSING IN CODEBASE**

**Problem:** Foreign key constraints defined in schema but not enforced (SQLite default)

**Solution:**
```rust
// Add to db/mod.rs after connection open:
conn.execute("PRAGMA foreign_keys = ON", [])?;
```

### ⚠️ Pitfall 3: Long-Lived Transactions

**Symptom:** Other operations blocked waiting for write lock

**Detection:**
```rust
conn.execute("BEGIN TRANSACTION", [])?;
// ... many operations ...
expensive_calculation()?;  // ← Lock held during non-DB work
// ... more operations ...
conn.execute("COMMIT", [])?;
```

**Solution:** Keep transactions minimal

```rust
// ✅ FIX: Narrow transaction scope
let data = conn.query_row(...)?;

// Release lock before expensive work
drop(conn);

expensive_calculation(data)?;
```

### ⚠️ Pitfall 4: Not Setting busy_timeout

**Current Status:** Not explicitly set (uses SQLite default)

**Problem:** Concurrent operations may fail with SQLITE_BUSY immediately

**Solution:**
```rust
// Add to db/mod.rs:
conn.busy_timeout(Duration::from_secs(5))?;
```

### ⚠️ Pitfall 5: Prepared Statement Re-preparation

**Symptom:** Performance degradation with repeated queries

**Current Pattern:** Every query calls `prepare()`

```rust
// ❌ CURRENT (in loops):
for id in ids {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?")?;  // Wasteful
    stmt.query_row([id], ...)?;
}
```

**Recommended:**
```rust
// ✅ BETTER:
conn.set_prepared_statement_cache_capacity(100);
let mut stmt = conn.prepare_cached("SELECT * FROM users WHERE id = ?")?;
for id in ids {
    stmt.query_row([id], ...)?;
}
```

---

## Quick Reference: Code Locations

### Key Files

| File | Purpose | Key Patterns |
|------|---------|--------------|
| `db/mod.rs` | Database struct | Arc<Mutex> pattern, connection management |
| `db/migrations.rs` | Schema versioning | Migration runner |
| `db/migrations/001_initial_schema.sql` | Schema | Foreign keys, soft deletes, CHECK constraints |
| `features/mood/repository.rs` | Mood data access | _with_conn pattern (2 methods, 6 call sites) |
| `features/mood/models.rs` | Mood errors | thiserror enums, validation |
| `features/assessments/repository.rs` | Assessment data | JSON serialization, thresholds |
| `features/visualization/repository.rs` | Chart data | Aggregation queries |
| `tests/test_mood.rs` | Mood tests | TempDir pattern, security tests |

### Deadlock Pattern Locations

**Fixed Deadlock:**
- `mood/repository.rs:365` - `get_mood_stats` now calls `get_activity_correlations_with_conn`

**_with_conn Pattern:**
- `mood/repository.rs:266` - `get_activities_for_checkin_with_conn`
- `mood/repository.rs:376` - `get_activity_correlations_with_conn`

**Call Sites:**
- Line 130, 216, 250 - `get_activities_for_checkin_with_conn`
- Line 365 - `get_activity_correlations_with_conn`

---

## Metrics Summary

| Metric | Count |
|--------|-------|
| Total Repositories | 3 |
| Lock Acquisitions | 6 files |
| _with_conn Methods | 2 (6 call sites) |
| Transaction Blocks | 3 (manual BEGIN/COMMIT) |
| Parameterized Queries | 100% |
| SQL Injection Tests | 1 comprehensive |
| Integration Tests | 69 across 4 files |
| Error Types | 3 (2 thiserror, 1 ad-hoc) |
| TODO/FIXME (DB) | 0 |

---

**For implementation examples, see:**
- [sqlite-reference.md](./sqlite-reference.md) - SQLite/rusqlite best practices
- [sqlite-anti-patterns.md](./sqlite-anti-patterns.md) - Common mistakes to avoid
- [../CLAUDE.md](../CLAUDE.md) - Database Development Guidelines
