For a SQLite automated agent to be effective, here's the critical information and resources they should be aware of:

## Core Technical Knowledge

**SQLite-Specific Characteristics**
- **Serverless architecture** - SQLite is embedded, not client-server
- **Single-file database** format with the entire database in one file
- **Type affinity** system rather than strict typing
- **Limited concurrent write access** (single writer at a time)
- **Transaction modes** - DEFERRED, IMMEDIATE, and EXCLUSIVE
- **Journal modes** - DELETE, TRUNCATE, PERSIST, MEMORY, and WAL (Write-Ahead Logging)

**Critical Limitations to Monitor**
- Database size limit (281 TB theoretical, but practical limits much lower)
- Page size constraints (512 to 65536 bytes)
- No native RIGHT JOIN or FULL OUTER JOIN
- Limited ALTER TABLE capabilities
- Single-threaded write operations
- No built-in user access control

## Essential Operations Knowledge

**Performance Optimization**
- When and how to use indexes effectively
- ANALYZE command for query planner statistics
- VACUUM operations for database maintenance
- Proper use of PRAGMA statements for tuning
- Query optimization using EXPLAIN QUERY PLAN

**Data Integrity Management**
- Foreign key constraint handling (must be explicitly enabled)
- CHECK constraints and triggers
- Understanding of ACID properties in SQLite context
- Backup strategies using the backup API or .backup command
- Corruption detection and recovery procedures

## Key Resources

**Official Documentation**
- **SQLite.org** - The authoritative source for all SQLite documentation
- **SQL syntax diagrams** - Visual representations of valid SQL syntax
- **PRAGMA reference** - Critical for configuration and optimization

**Command-Line Tools**
- **sqlite3 CLI** - Essential for database management and debugging
- **SQLite Expert** or **DB Browser for SQLite** - GUI tools for visual management
- **.schema**, **.tables**, **.indices** commands for structure inspection

**Monitoring and Analysis**
- **PRAGMA integrity_check** - Database corruption detection
- **PRAGMA database_list** - Active database connections
- **sqlite3_analyzer** - Detailed database file analysis tool
- **EXPLAIN and EXPLAIN QUERY PLAN** - Query performance analysis

## Critical Best Practices

**Connection Management**
- Proper connection pooling strategies
- Understanding when to use WAL mode vs. rollback journal
- Connection timeout handling
- Busy timeout configuration

**Error Handling**
- Understanding SQLite error codes (SQLITE_BUSY, SQLITE_LOCKED, etc.)
- Retry strategies for concurrent access issues
- Proper transaction rollback procedures
- Handling database lock scenarios

**Version Awareness**
- Track SQLite version for feature availability
- Understand backward compatibility implications
- Monitor for security updates and patches

## Automation-Specific Considerations

**Programmatic Interfaces**
- Language-specific bindings (Python's sqlite3, Node's better-sqlite3, etc.)
- Prepared statement usage for security and performance
- Batch operation optimization techniques
- Asynchronous operation considerations

**Monitoring Metrics**
- Database file size growth
- Query execution times
- Lock contention frequency
- Cache hit rates (via PRAGMA cache_spill)
- Journal size and checkpoint frequency in WAL mode

**Backup and Recovery**
- Online backup API usage
- Point-in-time recovery strategies
- Automated backup scheduling
- Testing restore procedures

An effective SQLite automated agent should also maintain awareness of common pitfalls like forgetting to enable foreign keys, not handling SQLITE_BUSY errors properly, or using inappropriate isolation levels for the workload. The agent should be able to adapt its strategies based on the specific use case, whether it's an embedded mobile application, a desktop app, or a small-to-medium web application.

For a rusqlite-specialized DBA/agent, here are the critical patterns and anti-patterns to master:

## Rusqlite-Specific Connection Patterns

**Connection Pool Management**
```rust
// PATTERN: Use r2d2 or deadpool for connection pooling
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

// ANTI-PATTERN: Creating new connections for each operation
// GOOD: Reuse connections from a pool with proper sizing
let manager = SqliteConnectionManager::file("db.sqlite")
    .with_init(|c| {
        c.execute_batch("
            PRAGMA journal_mode=WAL;
            PRAGMA synchronous=NORMAL;
            PRAGMA busy_timeout=5000;
            PRAGMA foreign_keys=ON;
        ")
    });
let pool = Pool::builder().max_size(1).build(manager)?; // Single writer!
```

**Transaction Scope Management**
```rust
// ANTI-PATTERN: Long-lived transactions blocking other operations
// PATTERN: Keep transactions as short as possible
conn.execute_batch("BEGIN IMMEDIATE")?; // Acquire write lock immediately
// Do work quickly
conn.execute_batch("COMMIT")?;

// PATTERN: Use rusqlite's transaction API for automatic rollback
let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
// Work here - automatic rollback on error
tx.commit()?;
```

## Deadlock Prevention Strategies

**Lock Ordering and Timeout Configuration**
```rust
// CRITICAL: Always set busy_timeout to prevent immediate SQLITE_BUSY
conn.busy_timeout(Duration::from_secs(5))?;

// PATTERN: Use busy_handler for custom retry logic
conn.busy_handler(Some(|attempts| {
    if attempts >= 3 { 
        false // Give up
    } else {
        thread::sleep(Duration::from_millis(100 * attempts));
        true // Retry
    }
}))?;
```

**Write-Ahead Logging (WAL) Best Practices**
```rust
// PATTERN: Enable WAL mode for better concurrency
conn.pragma_update(None, "journal_mode", "WAL")?;
conn.pragma_update(None, "wal_autocheckpoint", 1000)?;

// ANTI-PATTERN: Not handling checkpoint blocking
// PATTERN: Schedule checkpoints during low activity
conn.pragma_update(None, "wal_checkpoint", "PASSIVE")?;
```

## Query Performance Patterns

**Prepared Statement Caching**
```rust
// ANTI-PATTERN: Preparing statements repeatedly
for id in ids {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?")?;
    // Wasteful!
}

// PATTERN: Use cached statements
conn.set_prepared_statement_cache_capacity(100);
let mut stmt = conn.prepare_cached("SELECT * FROM users WHERE id = ?")?;
for id in ids {
    stmt.query_row([id], |row| ...)?;
}
```

**Batch Operations Optimization**
```rust
// ANTI-PATTERN: Individual inserts
for user in users {
    conn.execute("INSERT INTO users VALUES (?1, ?2)", params![user.name, user.email])?;
}

// PATTERN: Batch inserts in a transaction
let tx = conn.transaction()?;
{
    let mut stmt = tx.prepare_cached("INSERT INTO users VALUES (?1, ?2)")?;
    for user in users {
        stmt.execute(params![user.name, user.email])?;
    }
}
tx.commit()?;
```

## Rust Ownership and Borrowing Patterns

**Connection Lifetime Management**
```rust
// ANTI-PATTERN: Holding connections across await points
async fn bad_pattern(pool: &Pool) {
    let conn = pool.get()?;
    some_async_operation().await; // Connection held during async wait!
    conn.execute(...)?;
}

// PATTERN: Release connections before async operations
async fn good_pattern(pool: &Pool) {
    let result = {
        let conn = pool.get()?;
        conn.query_row(...)?
    }; // Connection released here
    some_async_operation().await;
}
```

**Row Mapping and Memory Efficiency**
```rust
// ANTI-PATTERN: Loading entire result sets into memory
let users: Vec<User> = conn.prepare("SELECT * FROM huge_table")?
    .query_map([], |row| ...)?
    .collect::<Result<Vec<_>, _>>()?;

// PATTERN: Stream results with iterator
let mut stmt = conn.prepare("SELECT * FROM huge_table")?;
let user_iter = stmt.query_map([], |row| ...)?;
for user in user_iter {
    process_user(user?)?;
    // Process one at a time
}
```

## Critical Anti-Patterns to Avoid

**N+1 Query Problem**
```rust
// ANTI-PATTERN: Nested queries
let posts = conn.prepare("SELECT * FROM posts")?
    .query_map([], |row| ...)?;
for post in posts {
    let comments = conn.prepare("SELECT * FROM comments WHERE post_id = ?")?
        .query_map([post.id], |row| ...)?;
}

// PATTERN: Use JOINs or batch fetch
let mut stmt = conn.prepare("
    SELECT p.*, c.* FROM posts p 
    LEFT JOIN comments c ON p.id = c.post_id
")?;
```

**Unsafe Concurrent Access**
```rust
// ANTI-PATTERN: Sharing Connection across threads without sync
let conn = Arc::new(Connection::open("db.sqlite")?);
// DON'T share Connection directly!

// PATTERN: Use Mutex or connection pool
let conn = Arc::new(Mutex::new(Connection::open("db.sqlite")?));
// Or better: use a proper connection pool
```

## Deadlock Detection and Recovery

**Implement Retry Logic**
```rust
// PATTERN: Exponential backoff for lock contention
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

## Monitoring and Diagnostics

**Query Analysis Integration**
```rust
// PATTERN: Wrap queries with performance monitoring
#[cfg(debug_assertions)]
fn analyze_query(conn: &Connection, sql: &str) {
    let plan = conn.prepare(&format!("EXPLAIN QUERY PLAN {}", sql))?
        .query_map([], |row| row.get::<_, String>(3))?
        .collect::<Result<Vec<_>, _>>()?;
    
    if plan.iter().any(|p| p.contains("SCAN TABLE")) {
        log::warn!("Table scan detected in query: {}", sql);
    }
}
```

**Lock Monitoring**
```rust
// PATTERN: Monitor and log lock wait times
conn.profile(Some(|sql, duration| {
    if duration > Duration::from_millis(100) {
        log::warn!("Slow query ({}ms): {}", duration.as_millis(), sql);
    }
}));
```

## Resource Management Best Practices

**PRAGMA Configuration for Rusqlite**
```rust
// PATTERN: Standard initialization for optimal performance
fn initialize_connection(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -64000;  -- 64MB cache
        PRAGMA temp_store = MEMORY;
        PRAGMA mmap_size = 134217728; -- 128MB mmap
        PRAGMA busy_timeout = 5000;
        PRAGMA foreign_keys = ON;
    ")?;
    Ok(())
}
```

A rusqlite specialist should also understand:
- The cost of crossing the FFI boundary between Rust and SQLite's C library
- How to properly handle `Send` and `Sync` traits with SQLite connections
- When to use `tokio-rusqlite` for async workloads vs synchronous approaches
- The implications of Rust's ownership model on connection and statement lifetimes
- How to leverage Rust's type system for compile-time SQL safety with libraries like `sqlx` when appropriate

These patterns will help prevent the most common issues: lock contention from long transactions, memory bloat from unbounded result sets, and the performance penalties of poor connection management in concurrent Rust applications.