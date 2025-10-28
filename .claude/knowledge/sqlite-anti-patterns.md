# SQLite & Rusqlite Anti-Patterns

**Common mistakes to avoid when working with SQLite and rusqlite**

Last Updated: 2025-10-27

---

## Deadlock Anti-Patterns

### ❌ Nested Lock Acquisition

**Real Bug from Codebase** (`mood/repository.rs:365` - Fixed)

```rust
// ❌ DEADLOCK:
pub fn get_mood_stats(&self) -> Result<MoodStats, MoodError> {
    let conn = conn.lock()?;  // ← First lock

    let correlations = self.get_activity_correlations(...)?;
    //                      ^^^ Tries to lock() again → DEADLOCK

    Ok(...)
}

fn get_activity_correlations(&self) -> Result<...> {
    let conn = conn.lock()?;  // ← Second lock on same mutex
    // ...
}
```

**✅ FIX: Use _with_conn Pattern**

```rust
pub fn get_mood_stats(&self) -> Result<MoodStats, MoodError> {
    let conn = conn.lock()?;
    let correlations = self.get_activity_correlations_with_conn(&conn, ...)?;
    Ok(...)
}

fn get_activity_correlations_with_conn(&self, conn: &Connection, ...) -> Result<...> {
    // Uses provided connection, no locking
}
```

### ❌ Long-Lived Transactions

```rust
// ❌ BAD: Lock held during non-database work
conn.execute("BEGIN TRANSACTION", [])?;
let data = conn.query_row(...)?;

expensive_calculation(data)?;  // ← Blocks all writers!
external_api_call(data).await?;  // ← Even worse with async

conn.execute("COMMIT", [])?;
```

**✅ FIX: Minimize Transaction Scope**

```rust
// Fetch data
let data = {
    let conn = conn.lock()?;
    conn.query_row(...)?
};  // Lock released

// Process outside transaction
expensive_calculation(data)?;
external_api_call(data).await?;
```

### ❌ Holding Connection Across `await`

```rust
// ❌ BAD: Async anti-pattern
async fn bad(pool: &Pool) {
    let conn = pool.get()?;
    some_async_operation().await;  // Connection blocked!
    conn.execute(...)?;
}
```

**✅ FIX: Release Before Await**

```rust
async fn good(pool: &Pool) {
    let result = {
        let conn = pool.get()?;
        conn.query_row(...)?
    };  // Released
    some_async_operation().await;
}
```

---

## Security Anti-Patterns

### ❌ String Interpolation in Queries

```rust
// ❌ SQL INJECTION RISK:
let query = format!("SELECT * FROM users WHERE name = '{}'", user_input);
conn.query_row(&query, [], |row| { ... })?;
```

**✅ FIX: Use Parameters**

```rust
conn.query_row(
    "SELECT * FROM users WHERE name = ?",
    [user_input],
    |row| { ... }
)?;
```

### ❌ Forgetting Foreign Key Enforcement

```rust
// ❌ Schema has FK constraints but they're not enforced!
let conn = Connection::open("db.sqlite")?;
// SQLite defaults to foreign_keys = OFF
```

**✅ FIX: Enable on Every Connection**

```rust
let conn = Connection::open("db.sqlite")?;
conn.execute("PRAGMA foreign_keys = ON", [])?;  // ← CRITICAL
```

---

## Performance Anti-Patterns

### ❌ N+1 Query Problem

```rust
// ❌ BAD: Nested queries
let posts = conn.prepare("SELECT * FROM posts")?.query_map(...)?;
for post in posts {
    let comments = conn.prepare("SELECT * FROM comments WHERE post_id = ?")?
        .query_map([post.id], ...)?;  // ← Repeated query!
}
```

**✅ FIX: Use JOINs**

```rust
conn.prepare("
    SELECT p.*, c.*
    FROM posts p
    LEFT JOIN comments c ON p.id = c.post_id
")?.query_map(...)?;
```

### ❌ Repeated Statement Preparation

```rust
// ❌ WASTEFUL:
for id in ids {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?")?;
    stmt.query_row([id], ...)?;
}
```

**✅ FIX: Use Statement Caching**

```rust
conn.set_prepared_statement_cache_capacity(100);
let mut stmt = conn.prepare_cached("SELECT * FROM users WHERE id = ?")?;
for id in ids {
    stmt.query_row([id], ...)?;
}
```

### ❌ Individual Inserts Without Transaction

```rust
// ❌ SLOW: 1000 individual commits
for user in users {
    conn.execute("INSERT INTO users VALUES (?1, ?2)", params![...])?;
}
```

**✅ FIX: Batch in Transaction**

```rust
let tx = conn.transaction()?;
{
    let mut stmt = tx.prepare_cached("INSERT INTO users VALUES (?1, ?2)")?;
    for user in users {
        stmt.execute(params![...])?;
    }
}
tx.commit()?;  // 100x-1000x faster
```

### ❌ Loading Entire Result Set into Memory

```rust
// ❌ MEMORY BLOAT:
let users: Vec<User> = conn.prepare("SELECT * FROM huge_table")?
    .query_map([], |row| { ... })?
    .collect::<Result<Vec<_>, _>>()?;
```

**✅ FIX: Stream Results**

```rust
let mut stmt = conn.prepare("SELECT * FROM huge_table")?;
for user in stmt.query_map([], |row| { ... })? {
    process(user?)?;  // One at a time
}
```

---

## Concurrency Anti-Patterns

### ❌ Sharing Connection Without Mutex

```rust
// ❌ COMPILE ERROR: Connection is !Sync
let conn = Arc::new(Connection::open("db.sqlite")?);
```

**✅ FIX: Use Mutex**

```rust
let conn = Arc::new(Mutex::new(Connection::open("db.sqlite")?));
```

### ❌ Not Setting busy_timeout

```rust
// ❌ Immediate SQLITE_BUSY on contention
let conn = Connection::open("db.sqlite")?;
```

**✅ FIX: Configure Timeout**

```rust
let conn = Connection::open("db.sqlite")?;
conn.busy_timeout(Duration::from_secs(5))?;
```

---

## Transaction Anti-Patterns

### ❌ Manual Transaction Without Rollback Handling

```rust
// ❌ No rollback on error
conn.execute("BEGIN", [])?;
conn.execute("INSERT ...", [])?;  // ← Might fail
conn.execute("COMMIT", [])?;  // ← Never reached if error
```

**✅ FIX: Use RAII Transaction**

```rust
let tx = conn.transaction()?;
tx.execute("INSERT ...", [])?;
tx.commit()?;  // Auto-rollback on drop if not committed
```

### ❌ Wrong Transaction Mode

```rust
// ❌ DEFERRED for write-heavy operations
let tx = conn.transaction()?;  // Defaults to DEFERRED
// Might get SQLITE_BUSY later
```

**✅ FIX: Use IMMEDIATE for Writes**

```rust
let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
// Lock acquired on BEGIN, won't fail mid-transaction
```

---

## Schema Anti-Patterns

### ❌ Missing Indexes on Foreign Keys

```sql
-- ❌ FK without index (slow JOINs)
CREATE TABLE comments (
    post_id INTEGER REFERENCES posts(id)
);
```

**✅ FIX: Index Foreign Keys**

```sql
CREATE TABLE comments (
    post_id INTEGER REFERENCES posts(id)
);
CREATE INDEX idx_comments_post_id ON comments(post_id);
```

### ❌ TEXT for Boolean

```sql
-- ❌ Wastes space and error-prone
is_active TEXT CHECK (is_active IN ('true', 'false'))
```

**✅ FIX: Use INTEGER**

```sql
is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1))
```

---

## Error Handling Anti-Patterns

### ❌ Ignoring Lock Poisoning

```rust
// ❌ Panic in one thread poisons lock for all
let conn = conn.lock().unwrap();  // ← unwrap() hides problem
```

**✅ FIX: Handle LockPoisoned**

```rust
let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;
```

### ❌ Generic Error Messages

```rust
// ❌ Loses context
conn.execute(...).map_err(|_| "Database error")?;
```

**✅ FIX: Preserve Error Chain**

```rust
conn.execute(...)
    .map_err(|e| format!("Failed to insert mood check-in: {}", e))?;
```

---

**For correct patterns, see:**
- [database-patterns.md](./database-patterns.md)
- [sqlite-reference.md](./sqlite-reference.md)
