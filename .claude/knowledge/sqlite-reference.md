# SQLite & Rusqlite Reference

**Comprehensive guide to SQLite characteristics, rusqlite API patterns, and optimization strategies**

Last Updated: 2025-10-27
Based On: sqlite-topics.md + sqlite-links.md + official SQLite documentation

---

## Table of Contents
1. [SQLite Core Characteristics](#sqlite-core-characteristics)
2. [Critical PRAGMAs for Tauri Apps](#critical-pragmas-for-tauri-apps)
3. [Rusqlite Connection Patterns](#rusqlite-connection-patterns)
4. [Transaction Management](#transaction-management)
5. [Query Optimization](#query-optimization)
6. [Performance Tuning](#performance-tuning)
7. [Data Integrity](#data-integrity)
8. [Rust-Specific Patterns](#rust-specific-patterns)
9. [Official Documentation Links](#official-documentation-links)

---

## SQLite Core Characteristics

### Architecture

**Serverless & Embedded**
- SQLite runs in-process, not as separate server
- No network overhead, direct file I/O
- Entire database in single `.sqlite` file
- Ideal for desktop apps (Tauri), mobile, embedded systems

**Single Writer, Multiple Readers**
- Only ONE write transaction at a time
- Multiple concurrent readers allowed
- Write blocks all other writes (but not reads in WAL mode)
- **Implication:** Design for short write transactions

### Type Affinity (Not Strict Typing)

**SQLite uses "type affinity" instead of strict types:**

| Declared Type | Affinity | Storage Classes |
|---------------|----------|-----------------|
| INTEGER, INT, BIGINT | INTEGER | Stored as 1-8 byte integer |
| REAL, DOUBLE, FLOAT | REAL | Stored as 8-byte float |
| TEXT, VARCHAR, CHAR | TEXT | Stored as UTF-8/UTF-16 |
| BLOB | NONE | Stored as-is |
| (no type) | BLOB | No type conversion |

**Implication:**
```sql
CREATE TABLE users (age INTEGER);
INSERT INTO users VALUES ('25');  -- Stored as INTEGER 25 (auto-converted)
INSERT INTO users VALUES ('abc'); -- Stored as TEXT 'abc' (can't convert)
```

### Limitations to Monitor

| Limitation | Value | Workaround |
|------------|-------|------------|
| Database size | 281 TB theoretical, ~100 GB practical | Archive old data, multiple databases |
| Page size | 512 to 65,536 bytes | Set via `PRAGMA page_size` before first write |
| SQL statement length | ~1 million bytes | Batch large operations |
| Columns per table | 2,000 default, 32,767 max | Normalize schema |
| No RIGHT JOIN | Not supported | Rewrite as LEFT JOIN |
| No FULL OUTER JOIN | Not supported | UNION of LEFT and RIGHT |
| Single writer | One write transaction at a time | Use WAL mode, minimize lock time |

**ALTER TABLE Limitations:**
- Can't DROP COLUMN (before SQLite 3.35)
- Can't modify column type
- Can't add constraints to existing column

**Workaround:** Create new table, copy data, rename

---

## Critical PRAGMAs for Tauri Apps

**⚠️ MISSING IN CURRENT CODEBASE - Add to `db/mod.rs`**

### Essential Configuration

```rust
// src-tauri/src/db/mod.rs - Add after Connection::open
fn initialize_connection(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        -- Enable foreign key enforcement (CRITICAL!)
        PRAGMA foreign_keys = ON;

        -- Use WAL mode for better concurrency
        PRAGMA journal_mode = WAL;

        -- Safe for WAL mode (faster than FULL)
        PRAGMA synchronous = NORMAL;

        -- Prevent immediate SQLITE_BUSY errors
        PRAGMA busy_timeout = 5000;

        -- 64MB cache (negative = KB)
        PRAGMA cache_size = -64000;

        -- Faster temp tables
        PRAGMA temp_store = MEMORY;

        -- Memory-mapped I/O (128MB)
        PRAGMA mmap_size = 134217728;
    ")?;
    Ok(())
}
```

### PRAGMA Reference

#### foreign_keys (CRITICAL!)
```sql
PRAGMA foreign_keys = ON;  -- MUST set on each connection
```
**Why:** SQLite defaults to OFF for backwards compatibility
**Impact:** Foreign key constraints ignored if not enabled
**When:** Set immediately after opening connection

#### journal_mode
```sql
PRAGMA journal_mode = WAL;  -- Write-Ahead Logging
```

**Modes:**
| Mode | Concurrency | Use Case |
|------|-------------|----------|
| DELETE (default) | Poor | Single-user, simple apps |
| WAL (recommended) | Excellent | Multi-user, concurrent reads |
| TRUNCATE | Fair | Embedded systems |
| MEMORY | Excellent | In-memory databases |

**WAL Benefits:**
- Readers don't block writers
- Writers don't block readers
- Faster commits (append-only)
- Atomic commits even if crash

**WAL Considerations:**
- Creates `.sqlite-wal` and `.sqlite-shm` files
- Need checkpoint operations (auto or manual)
- Not recommended for network filesystems

#### synchronous
```sql
PRAGMA synchronous = NORMAL;  -- For WAL mode
PRAGMA synchronous = FULL;     -- For DELETE journal
```

**Levels:**
| Level | Safety | Speed | When to Use |
|-------|--------|-------|-------------|
| OFF | Risky | Fastest | Testing only |
| NORMAL | Safe with WAL | Fast | Production with WAL |
| FULL | Safest | Slower | Production with DELETE |

#### busy_timeout
```sql
PRAGMA busy_timeout = 5000;  -- 5 seconds in milliseconds
```
**Purpose:** How long to wait for lock before returning SQLITE_BUSY
**Without it:** Immediate SQLITE_BUSY on contention
**Recommendation:** 3000-10000ms for desktop apps

---

## Rusqlite Connection Patterns

### Connection Pooling (Not Currently Used)

**For concurrent applications:**

```rust
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

// Single writer pool (SQLite limitation)
let manager = SqliteConnectionManager::file("db.sqlite")
    .with_init(|c| {
        c.execute_batch("
            PRAGMA journal_mode=WAL;
            PRAGMA synchronous=NORMAL;
            PRAGMA busy_timeout=5000;
            PRAGMA foreign_keys=ON;
        ")
    });

let pool = Pool::builder()
    .max_size(1)  // Single writer!
    .build(manager)?;

// Get connection from pool
let conn = pool.get()?;
```

**When to use:**
- Multiple concurrent Tauri commands
- Long-running operations
- Need connection lifecycle management

**Current codebase:** Uses `Arc<Mutex<Connection>>` (simpler, sufficient for desktop app)

### Statement Caching

**⚠️ NOT USED IN CODEBASE - Recommended for hot paths**

```rust
// Set cache size (default: 0)
conn.set_prepared_statement_cache_capacity(100);

// Use cached preparation
let mut stmt = conn.prepare_cached("SELECT * FROM users WHERE id = ?")?;
for id in ids {
    let user = stmt.query_row([id], |row| { ... })?;
}
```

**Benefits:**
- Avoids re-parsing SQL
- Significant performance improvement for repeated queries
- Especially beneficial in loops

**Recommended for:**
- `get_activities_for_checkin_with_conn` (called frequently)
- `get_assessment_history` (common query)
- Any query in a loop

### Connection Lifetime in Async Code

**⚠️ ANTI-PATTERN: Holding connection across `await`**

```rust
// ❌ BAD: Connection held during async wait
async fn bad_pattern(pool: &Pool) {
    let conn = pool.get()?;
    some_async_operation().await;  // Connection blocked!
    conn.execute(...)?;
}

// ✅ GOOD: Release connection before async
async fn good_pattern(pool: &Pool) {
    let result = {
        let conn = pool.get()?;
        conn.query_row(...)?
    };  // Connection released
    some_async_operation().await;
}
```

---

## Transaction Management

### Transaction Modes

| Mode | Behavior | When to Use |
|------|----------|-------------|
| DEFERRED (default) | Lock acquired on first write | Read-heavy, optimistic |
| IMMEDIATE | Write lock acquired on BEGIN | Guaranteed write access |
| EXCLUSIVE | Exclusive lock, blocks all | Bulk operations, schema changes |

### Rusqlite Transaction API (RAII Pattern)

**⚠️ NOT USED IN CODEBASE - Recommended for refactoring**

```rust
// PATTERN: Automatic rollback on drop
let mut tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;

// Insert operations
{
    let mut stmt = tx.prepare_cached("INSERT INTO users VALUES (?1, ?2)")?;
    for user in users {
        stmt.execute(params![user.name, user.email])?;
    }
}

tx.commit()?;  // Explicit commit
// If commit not called or panic occurs, automatic ROLLBACK on drop
```

**Benefits over manual BEGIN/COMMIT:**
- Automatic rollback on drop (panic-safe)
- Can't forget to commit/rollback
- Safer error handling
- Cleaner code

### Savepoints (Nested Transactions)

```rust
let tx = conn.transaction()?;

// Main transaction work...

let sp = tx.savepoint()?;
// Inner transaction work...
sp.commit()?;  // or rollback

tx.commit()?;
```

---

## Query Optimization

### EXPLAIN QUERY PLAN

**Analyze query performance:**

```rust
#[cfg(debug_assertions)]
fn analyze_query(conn: &Connection, sql: &str) -> Result<()> {
    let plan = conn
        .prepare(&format!("EXPLAIN QUERY PLAN {}", sql))?
        .query_map([], |row| row.get::<_, String>(3))?
        .collect::<Result<Vec<_>, _>>()?;

    for step in plan {
        if step.contains("SCAN TABLE") {
            log::warn!("Table scan detected: {}", sql);
        }
    }
    Ok(())
}
```

**Look for:**
- `SCAN TABLE` - Full table scan (bad, add index)
- `SEARCH TABLE ... USING INDEX` - Good
- `USING COVERING INDEX` - Excellent (no table access needed)

### Index Strategy

**Current schema has minimal indexes:**
```sql
-- Only explicit index in codebase:
CREATE UNIQUE INDEX idx_activities_name_unique
ON activities(name) WHERE deleted_at IS NULL;
```

**Recommended additions:**
```sql
-- Foreign key indexes (improve JOIN performance)
CREATE INDEX idx_mood_checkin_activities_mood_id
ON mood_checkin_activities(mood_checkin_id);

CREATE INDEX idx_mood_checkin_activities_activity_id
ON mood_checkin_activities(activity_id);

-- Date range queries
CREATE INDEX idx_mood_checkins_created_at
ON mood_checkins(created_at);

CREATE INDEX idx_assessment_responses_completed_at
ON assessment_responses(completed_at);
```

### ANALYZE Command

```sql
ANALYZE;  -- Update query planner statistics
```

**When to run:**
- After bulk data loads
- After significant data changes
- If queries become slow

**In Rust:**
```rust
conn.execute("ANALYZE", [])?;
```

### Batch Operations

**⚠️ ANTI-PATTERN: Individual inserts**

```rust
// ❌ BAD: 1000 individual transactions
for user in users {
    conn.execute("INSERT INTO users VALUES (?1, ?2)",
                 params![user.name, user.email])?;
}
```

**✅ GOOD PATTERN: Batch in transaction**

```rust
let tx = conn.transaction()?;
{
    let mut stmt = tx.prepare_cached("INSERT INTO users VALUES (?1, ?2)")?;
    for user in users {
        stmt.execute(params![user.name, user.email])?;
    }
}
tx.commit()?;
```

**Performance impact:** 100x-1000x faster for bulk inserts

---

## Performance Tuning

### Query Profiling

```rust
// Enable query profiling
conn.profile(Some(|sql, duration| {
    if duration > Duration::from_millis(100) {
        log::warn!("Slow query ({}ms): {}", duration.as_millis(), sql);
    }
}));
```

### Memory Management

**Row Iterator vs Collect All**

```rust
// ❌ ANTI-PATTERN: Load entire result set
let users: Vec<User> = conn
    .prepare("SELECT * FROM huge_table")?
    .query_map([], |row| { ... })?
    .collect::<Result<Vec<_>, _>>()?;  // All in memory!

// ✅ PATTERN: Stream results
let mut stmt = conn.prepare("SELECT * FROM huge_table")?;
let user_iter = stmt.query_map([], |row| { ... })?;
for user in user_iter {
    process_user(user?)?;  // One at a time
}
```

### VACUUM

```sql
VACUUM;  -- Rebuild database file, reclaim space
```

**When to run:**
- After large deletes
- Database file fragmented
- Periodic maintenance

**Warning:** Locks database exclusively, can take time for large databases

**In Rust:**
```rust
conn.execute("VACUUM", [])?;
```

---

## Data Integrity

### Foreign Key Enforcement

**⚠️ CRITICAL: Must enable on EVERY connection**

```rust
// ALWAYS run after opening connection:
conn.execute("PRAGMA foreign_keys = ON", [])?;
```

**Check enforcement:**
```rust
let enabled: i32 = conn.query_row(
    "PRAGMA foreign_keys",
    [],
    |row| row.get(0)
)?;
assert_eq!(enabled, 1, "Foreign keys must be enabled!");
```

### CHECK Constraints

```sql
CREATE TABLE mood_checkins (
    mood_rating INTEGER NOT NULL CHECK (mood_rating BETWEEN 1 AND 5),
    notes TEXT CHECK (length(notes) <= 5000 OR notes IS NULL)
);
```

**Used in codebase:** Yes, in schema
**Runtime validation:** Also performed in Rust models

### UNIQUE Constraints

**Partial Unique Constraint (Soft Delete Pattern):**

```sql
CREATE UNIQUE INDEX idx_activities_name_unique
ON activities(name) WHERE deleted_at IS NULL;
```

**Benefit:** Unique only for active records, allows name reuse after deletion

### Triggers (Not Used in Codebase)

**Example: Audit trail**
```sql
CREATE TRIGGER audit_mood_changes
AFTER UPDATE ON mood_checkins
BEGIN
    INSERT INTO audit_log VALUES (NEW.id, datetime('now'));
END;
```

### Database Integrity Check

```rust
// Check for corruption
let result: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
if result != "ok" {
    log::error!("Database corruption detected: {}", result);
}
```

---

## Rust-Specific Patterns

### Send + Sync Traits

**Connection is NOT `Sync`:**
```rust
// ❌ Can't share Connection across threads
let conn = Arc::new(Connection::open("db.sqlite")?);  // Compile error!

// ✅ Must use Mutex for thread-safety
let conn = Arc::new(Mutex::new(Connection::open("db.sqlite")?));
```

### Error Handling

**Convert rusqlite errors to domain errors:**

```rust
// Automatic conversion via #[from]
#[derive(Error, Debug)]
pub enum MoodError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

// Use ? operator
let result = conn.query_row(...)?;  // Auto-converts
```

**Handle specific errors:**

```rust
match conn.execute(...) {
    Err(rusqlite::Error::SqliteFailure(err, _)) => {
        if err.code == ErrorCode::ConstraintViolation {
            return Err(MoodError::DuplicateActivityName(name));
        }
        Err(MoodError::Database(...))
    }
    Ok(rows) => Ok(rows),
    Err(e) => Err(MoodError::Database(e)),
}
```

### Type-Safe Queries with FromSql/ToSql

**Custom type mapping:**

```rust
struct UserId(i64);

impl rusqlite::types::ToSql for UserId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0))
    }
}

impl rusqlite::types::FromSql for UserId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_i64().map(UserId)
    }
}
```

### Retry Logic for SQLITE_BUSY

```rust
fn execute_with_retry<F, R>(conn: &Connection, f: F) -> Result<R>
where
    F: Fn(&Connection) -> Result<R>,
{
    let mut attempts = 0;
    loop {
        match f(conn) {
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.code == ErrorCode::DatabaseBusy && attempts < 5 => {
                thread::sleep(Duration::from_millis(100 * 2_u64.pow(attempts)));
                attempts += 1;
            }
            result => return result,
        }
    }
}
```

---

## Official Documentation Links

### SQLite Core Documentation
- [PRAGMA Statements](https://sqlite.org/pragma.html#toc) - Configuration options
- [Core Functions](https://sqlite.org/lang_corefunc.html) - Built-in SQL functions
- [Date/Time Functions](https://sqlite.org/lang_datefunc.html) - Temporal operations
- [Window Functions](https://sqlite.org/windowfunctions.html#biwinfunc) - Advanced analytics
- [Aggregate Functions](https://sqlite.org/lang_aggfunc.html#aggfunclist) - GROUP BY operations
- [Math Functions](https://sqlite.org/lang_mathfunc.html) - Numeric operations
- [JSON Functions](https://sqlite.org/json1.html) - JSON manipulation
- [SQLite Quirks](https://sqlite.org/quirks.html) - Behavior differences from other databases
- [FAQ](https://sqlite.org/faq.html) - Common questions

### Performance & Optimization
- [Query Planner](https://sqlite.org/queryplanner.html) - How SQLite optimizes queries
- [Optimization Overview](https://sqlite.org/optoverview.html) - Performance tips

### Advanced Topics
- [WAL Mode](https://sqlite.org/wal.html) - Write-Ahead Logging explained
- [Atomic Commit](https://sqlite.org/atomiccommit.html) - How transactions work
- [Foreign Keys](https://sqlite.org/foreignkeys.html) - Referential integrity
- [Triggers](https://sqlite.org/lang_createtrigger.html) - Automated actions

### Rusqlite Documentation
- [Rusqlite API Docs](https://docs.rs/rusqlite/latest/rusqlite/) - Rust bindings reference
- [rusqlite::Connection](https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html)
- [rusqlite::Transaction](https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html)
- [rusqlite::params!](https://docs.rs/rusqlite/latest/rusqlite/macro.params.html)

---

## Quick Reference: Common Operations

### Database Creation
```rust
let conn = Connection::open("database.sqlite")?;
```

### Execute DDL
```rust
conn.execute_batch("
    CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
    CREATE INDEX idx_users_name ON users(name);
")?;
```

### Insert with Parameters
```rust
conn.execute(
    "INSERT INTO users (name, email) VALUES (?1, ?2)",
    params![name, email],
)?;
```

### Query Single Row
```rust
let user: User = conn.query_row(
    "SELECT id, name FROM users WHERE id = ?",
    [user_id],
    |row| Ok(User {
        id: row.get(0)?,
        name: row.get(1)?,
    }),
)?;
```

### Query Multiple Rows
```rust
let mut stmt = conn.prepare("SELECT id, name FROM users")?;
let users = stmt.query_map([], |row| {
    Ok(User {
        id: row.get(0)?,
        name: row.get(1)?,
    })
})?;

for user in users {
    println!("{:?}", user?);
}
```

### Transaction
```rust
let tx = conn.transaction()?;
tx.execute("INSERT INTO users VALUES (?1, ?2)", params![1, "Alice"])?;
tx.execute("INSERT INTO users VALUES (?1, ?2)", params![2, "Bob"])?;
tx.commit()?;
```

---

**For codebase-specific patterns, see:**
- [database-patterns.md](./database-patterns.md) - Project patterns & architecture
- [../CLAUDE.md](../CLAUDE.md) - Development guidelines
