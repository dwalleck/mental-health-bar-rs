---
description: Refactor existing database code to fix deadlocks, apply RAII transactions, add PRAGMAs, and optimize performance.
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Goal

Refactor existing database code to eliminate anti-patterns and improve:
- **Deadlock elimination**: Convert nested locks to `_with_conn` pattern
- **Transaction safety**: Replace manual BEGIN/COMMIT with RAII `Transaction` API
- **PRAGMA enforcement**: Add missing critical PRAGMAs to connection initialization
- **Performance optimization**: Add statement caching, batch operations, eliminate N+1 queries
- **Error handling**: Standardize error types and improve error context

All refactoring must:
- Maintain backward compatibility (same public API)
- Include tests verifying behavior unchanged
- Follow incremental, reviewable changes
- Document rationale for each refactoring

## Knowledge Base References

Load these knowledge base documents for context:
- `.claude/knowledge/database-patterns.md` - Codebase-specific patterns
- `.claude/knowledge/sqlite-anti-patterns.md` - Patterns to eliminate
- `.claude/knowledge/sqlite-reference.md` - SQLite best practices
- `CLAUDE.md` - Project coding guidelines

## Execution Steps

### 1. Parse Refactoring Request

Extract from $ARGUMENTS:
- **Target files**: Specific files to refactor, or "all" for comprehensive refactoring
- **Focus area**: Deadlocks, transactions, performance, error handling, or "all"
- **Scope**: Single method, entire file, or codebase-wide

If no arguments provided, perform comprehensive analysis and ask user to prioritize.

### 2. Analyze Current Code

**Scan target files for:**

1. **Deadlock patterns**:
   - Methods that call `conn.lock()` and then call other methods
   - Methods that should have `_with_conn` variants but don't
   - Nested repository calls without passing connection

2. **Transaction anti-patterns**:
   - Manual `BEGIN`/`COMMIT` without rollback handling
   - Long-lived transactions holding locks during non-DB work
   - Missing `IMMEDIATE` mode for write operations

3. **Missing PRAGMAs**:
   - `foreign_keys = ON` not enabled
   - `busy_timeout` not configured
   - Suboptimal `journal_mode` and `synchronous` settings

4. **Performance issues**:
   - Repeated `prepare()` calls in loops
   - N+1 query patterns
   - Unbounded result set collection
   - Individual inserts outside transactions

5. **Error handling gaps**:
   - Generic error types (using `String` instead of custom types)
   - Missing error context
   - `unwrap()` on lock acquisition

**Generate refactoring plan** prioritized by impact:
- **CRITICAL**: Deadlocks, missing `PRAGMA foreign_keys`
- **HIGH**: Transaction safety, error standardization
- **MEDIUM**: Performance optimizations, statement caching
- **LOW**: Code cleanup, minor improvements

### 3. Refactoring Patterns

#### Pattern A: Add _with_conn Helper (Deadlock Fix)

**BEFORE:**
```rust
pub fn get_mood_stats(&self, from_date: Option<String>) -> Result<MoodStats, MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // ❌ DEADLOCK: This method also calls conn.lock()
    let correlations = self.get_activity_correlations(from_date)?;

    Ok(MoodStats { correlations, ... })
}

pub fn get_activity_correlations(&self, from_date: Option<String>) -> Result<Vec<...>, MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;  // ← Second lock = DEADLOCK
    // ... query logic ...
}
```

**AFTER:**
```rust
pub fn get_mood_stats(&self, from_date: Option<String>) -> Result<MoodStats, MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // ✅ FIXED: Pass connection to helper
    let correlations = self.get_activity_correlations_with_conn(&conn, from_date)?;

    Ok(MoodStats { correlations, ... })
}

// Keep existing method for backward compatibility
pub fn get_activity_correlations(&self, from_date: Option<String>) -> Result<Vec<...>, MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;
    self.get_activity_correlations_with_conn(&conn, from_date)
}

// NEW: Helper method accepting connection
fn get_activity_correlations_with_conn(
    &self,
    conn: &Connection,  // ← Accepts connection, never locks
    from_date: Option<String>,
) -> Result<Vec<...>, MoodError> {
    // ... query logic using passed connection ...
}
```

**Changes:**
1. Create `{method}_with_conn` helper accepting `&Connection`
2. Move query logic to helper
3. Update public method to keep existing signature (backward compatible)
4. Update callers to use `_with_conn` variant when they already have lock

**Test:**
```rust
#[test]
fn test_get_mood_stats_no_deadlock() {
    // Regression test for deadlock fix
    let repo = setup_test_repo();
    let result = repo.get_mood_stats(None);
    assert!(result.is_ok(), "Method deadlocked!");
}
```

#### Pattern B: Convert to RAII Transaction

**BEFORE:**
```rust
pub fn batch_insert(&self, items: Vec<Item>) -> Result<(), Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // ❌ Manual transaction - no automatic rollback on error
    conn.execute("BEGIN IMMEDIATE", [])?;

    for item in items {
        conn.execute("INSERT INTO items VALUES (?1, ?2)", params![item.id, item.name])?;
        // If this fails, transaction not rolled back!
    }

    conn.execute("COMMIT", [])?;
    Ok(())
}
```

**AFTER:**
```rust
pub fn batch_insert(&self, items: Vec<Item>) -> Result<(), Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // ✅ RAII transaction - auto-rollback on drop if not committed
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;

    {
        let mut stmt = tx.prepare_cached("INSERT INTO items VALUES (?1, ?2)")?;
        for item in items {
            stmt.execute(params![item.id, item.name])?;
            // Error here triggers automatic rollback via Drop
        }
    }

    tx.commit()?;  // Explicit commit on success
    Ok(())
}
```

**Changes:**
1. Replace `BEGIN IMMEDIATE` with `transaction_with_behavior(Immediate)`
2. Replace `COMMIT` with `tx.commit()`
3. Remove `ROLLBACK` handling - automatic via Drop
4. Use `prepare_cached()` for prepared statements inside transaction

**Test:**
```rust
#[test]
fn test_batch_insert_rollback_on_error() {
    let repo = setup_test_repo();
    let invalid_items = vec![
        Item { id: 1, name: "Valid".to_string() },
        Item { id: 2, name: "".to_string() },  // Violates CHECK constraint
    ];

    let result = repo.batch_insert(invalid_items);
    assert!(result.is_err());

    // Verify rollback: First item should NOT be inserted
    let count = repo.count_items().unwrap();
    assert_eq!(count, 0, "Transaction was not rolled back!");
}
```

#### Pattern C: Add PRAGMA Enforcement

**BEFORE (`src-tauri/src/db/mod.rs`):**
```rust
impl Database {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(db_path)?;

        // ❌ MISSING: Critical PRAGMAs not configured
        // Foreign keys disabled by default!

        let db = Arc::new(Mutex::new(conn));
        Ok(Self { conn: db })
    }
}
```

**AFTER:**
```rust
impl Database {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(db_path)?;

        // ✅ CRITICAL: Enable foreign keys and configure connection
        conn.execute_batch("
            PRAGMA foreign_keys = ON;         -- Enable FK constraints (CRITICAL!)
            PRAGMA busy_timeout = 5000;       -- Wait 5s on lock contention
            PRAGMA journal_mode = WAL;        -- Better concurrency than DELETE mode
            PRAGMA synchronous = NORMAL;      -- Safe with WAL mode
            PRAGMA cache_size = -64000;       -- 64MB cache (negative = KB)
            PRAGMA temp_store = MEMORY;       -- Faster temp operations
        ")?;

        let db = Arc::new(Mutex::new(conn));
        Ok(Self { conn: db })
    }
}
```

**Changes:**
1. Add `execute_batch()` call after `Connection::open()`
2. Include all critical PRAGMAs with inline comments
3. Document why each PRAGMA is needed

**Test:**
```rust
#[test]
fn test_foreign_keys_enabled() {
    let db = setup_test_db();
    let conn = db.get_connection();
    let conn = conn.lock().unwrap();

    // Verify foreign_keys pragma is ON
    let fk_enabled: i32 = conn
        .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
        .unwrap();
    assert_eq!(fk_enabled, 1, "Foreign keys not enabled!");
}

#[test]
fn test_foreign_key_constraint_enforced() {
    let (db, _temp) = setup_test_db();
    let repo = ItemRepository::new(db);

    // Try to insert item with non-existent category_id
    let req = CreateItemRequest {
        name: "Test".to_string(),
        category_id: 99999,  // Does not exist
        ..Default::default()
    };

    let result = repo.create_item(req);
    assert!(result.is_err(), "FK constraint not enforced!");
}
```

#### Pattern D: Add Statement Caching

**BEFORE:**
```rust
pub fn get_items_by_ids(&self, ids: Vec<i64>) -> Result<Vec<Item>, Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    let mut items = Vec::new();
    for id in ids {
        // ❌ WASTEFUL: Preparing statement in every iteration
        let mut stmt = conn.prepare("SELECT * FROM items WHERE id = ?")?;
        let item = stmt.query_row([id], |row| { /* map row */ })?;
        items.push(item);
    }
    Ok(items)
}
```

**AFTER:**
```rust
pub fn get_items_by_ids(&self, ids: Vec<i64>) -> Result<Vec<Item>, Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // ✅ OPTIMIZED: Prepare once, execute multiple times
    let mut stmt = conn.prepare_cached("SELECT * FROM items WHERE id = ?")?;

    let mut items = Vec::new();
    for id in ids {
        let item = stmt.query_row([id], |row| { /* map row */ })?;
        items.push(item);
    }
    Ok(items)
}
```

**Changes:**
1. Move `prepare()` outside loop
2. Use `prepare_cached()` instead of `prepare()`
3. Set cache capacity in Database::new() if not already set:
   ```rust
   conn.set_prepared_statement_cache_capacity(100);
   ```

**Benchmark test:**
```rust
#[test]
fn test_statement_caching_performance() {
    let repo = setup_test_repo();

    // Insert test data
    for i in 1..=100 {
        repo.create_item(CreateItemRequest { id: i, ... }).unwrap();
    }

    let ids: Vec<i64> = (1..=100).collect();

    let start = std::time::Instant::now();
    let items = repo.get_items_by_ids(ids).unwrap();
    let duration = start.elapsed();

    assert_eq!(items.len(), 100);
    // With caching, should be under 50ms for 100 queries
    assert!(duration.as_millis() < 50, "Too slow: {}ms", duration.as_millis());
}
```

#### Pattern E: Eliminate N+1 Query

**BEFORE:**
```rust
pub fn get_posts_with_comments(&self) -> Result<Vec<PostWithComments>, Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // ❌ N+1 QUERY: Fetches posts, then separate query per post
    let posts = self.get_all_posts_with_conn(&conn)?;

    let mut result = Vec::new();
    for post in posts {
        let comments = self.get_comments_for_post_with_conn(&conn, post.id)?;
        result.push(PostWithComments { post, comments });
    }

    Ok(result)
}
```

**AFTER:**
```rust
pub fn get_posts_with_comments(&self) -> Result<Vec<PostWithComments>, Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock()?;

    // ✅ SINGLE QUERY: Use JOIN to fetch all data at once
    let mut stmt = conn.prepare("
        SELECT
            p.id, p.title, p.content,
            c.id, c.post_id, c.text
        FROM posts p
        LEFT JOIN comments c ON p.id = c.post_id
        ORDER BY p.id, c.id
    ")?;

    let mut rows = stmt.query([])?;
    let mut result: Vec<PostWithComments> = Vec::new();
    let mut current_post: Option<Post> = None;
    let mut current_comments: Vec<Comment> = Vec::new();

    while let Some(row) = rows.next()? {
        let post_id: i64 = row.get(0)?;

        // Check if we moved to a new post
        if current_post.as_ref().map(|p| p.id) != Some(post_id) {
            // Save previous post if exists
            if let Some(post) = current_post.take() {
                result.push(PostWithComments {
                    post,
                    comments: std::mem::take(&mut current_comments),
                });
            }

            // Start new post
            current_post = Some(Post {
                id: post_id,
                title: row.get(1)?,
                content: row.get(2)?,
            });
        }

        // Add comment if present (LEFT JOIN may have NULL)
        if let Ok(comment_id) = row.get::<_, i64>(3) {
            current_comments.push(Comment {
                id: comment_id,
                post_id: row.get(4)?,
                text: row.get(5)?,
            });
        }
    }

    // Don't forget last post
    if let Some(post) = current_post {
        result.push(PostWithComments { post, comments: current_comments });
    }

    Ok(result)
}
```

**Changes:**
1. Replace loop with single JOIN query
2. Use result set grouping pattern (track current post ID)
3. Handle LEFT JOIN NULLs properly
4. Reduce from N+1 queries to 1 query

**Performance test:**
```rust
#[test]
fn test_n_plus_one_eliminated() {
    let repo = setup_test_repo();

    // Create 100 posts with 10 comments each
    for i in 1..=100 {
        let post = repo.create_post(CreatePostRequest { title: format!("Post {}", i) }).unwrap();
        for j in 1..=10 {
            repo.create_comment(CreateCommentRequest {
                post_id: post.id,
                text: format!("Comment {}", j),
            }).unwrap();
        }
    }

    // Measure query performance
    let start = std::time::Instant::now();
    let result = repo.get_posts_with_comments().unwrap();
    let duration = start.elapsed();

    assert_eq!(result.len(), 100);
    // Single JOIN query should be fast (< 100ms for 100 posts + 1000 comments)
    assert!(duration.as_millis() < 100, "N+1 query not eliminated: {}ms", duration.as_millis());
}
```

#### Pattern F: Standardize Error Types

**BEFORE:**
```rust
// Using String errors
pub fn create_item(&self, req: CreateItemRequest) -> Result<Item, String> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|e| format!("Lock error: {}", e))?;

    conn.execute("INSERT ...", params![...])
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(item)
}
```

**AFTER:**
```rust
use thiserror::Error;

// Custom error type with context
#[derive(Error, Debug)]
pub enum ItemError {
    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Invalid item data: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Lock poisoned - database in inconsistent state")]
    LockPoisoned,
}

pub fn create_item(&self, req: CreateItemRequest) -> Result<Item, ItemError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| ItemError::LockPoisoned)?;

    // rusqlite::Error automatically converts via #[from]
    conn.execute("INSERT ...", params![...])?;

    Ok(item)
}
```

**Changes:**
1. Create custom error enum with thiserror
2. Use `#[from]` for automatic conversion
3. Replace String returns with typed errors
4. Add context to error variants

**Update Tauri commands:**
```rust
#[tauri::command]
pub async fn create_item(
    state: tauri::State<'_, AppState>,
    request: CreateItemRequest,
) -> Result<Item, String> {
    let repo = ItemRepository::new(state.db.clone());
    repo.create_item(request)
        .map_err(|e| e.to_string())  // Convert to String for Tauri
}
```

### 4. Execute Refactoring

For each pattern identified:

1. **Create branch** (if not already on feature branch)
2. **Apply refactoring** following pattern template
3. **Write/update tests** to verify behavior unchanged
4. **Run test suite** to ensure no regressions
5. **Document changes** in commit message

**Incremental approach:**
- One pattern type per commit (e.g., "Add _with_conn helpers to mood repository")
- Test after each change
- Keep commits focused and reviewable

### 5. Generate Refactoring Report

Output Markdown summary:

```markdown
# Database Refactoring Report

**Target:** {Files/scope refactored}
**Date:** {ISO 8601}
**Changes:** {Number of files modified}

## Changes Summary

| File | Pattern Applied | LOC Changed | Tests Added |
|------|----------------|-------------|-------------|
| mood/repository.rs | Add _with_conn helpers | +45, -12 | 2 |
| db/mod.rs | Add PRAGMA enforcement | +10, -0 | 2 |
| assessments/repository.rs | RAII transactions | +15, -20 | 3 |

## Deadlock Fixes

### mood/repository.rs:365

**Issue:** `get_mood_stats()` calling `get_activity_correlations()` caused nested lock acquisition.

**Fix:** Created `get_activity_correlations_with_conn(&conn, ...)` helper.

**Before:**
```rust
pub fn get_mood_stats(&self, ...) -> Result<...> {
    let conn = conn.lock()?;
    self.get_activity_correlations(...)?;  // Deadlock here
}
```

**After:**
```rust
pub fn get_mood_stats(&self, ...) -> Result<...> {
    let conn = conn.lock()?;
    self.get_activity_correlations_with_conn(&conn, ...)?;  // Fixed
}
```

**Test:** `test_get_mood_stats_no_deadlock()`

## Transaction Safety Improvements

[Similar detailed sections for each refactoring]

## PRAGMA Enforcement

**Added to `db/mod.rs:Database::new()`:**
- `PRAGMA foreign_keys = ON` (CRITICAL)
- `PRAGMA busy_timeout = 5000`
- `PRAGMA journal_mode = WAL`
- `PRAGMA synchronous = NORMAL`

**Tests:** `test_foreign_keys_enabled()`, `test_foreign_key_constraint_enforced()`

## Performance Optimizations

[Details on caching, N+1 elimination, etc.]

## Test Results

```
Running 73 tests
test test_get_mood_stats_no_deadlock ... ok
test test_foreign_keys_enabled ... ok
test test_batch_insert_rollback_on_error ... ok
...

test result: ok. 73 passed; 0 failed
```

## Breaking Changes

**None** - All refactorings maintain backward-compatible public APIs.

## Recommendations

1. **Apply to remaining features:**
   - `visualization/repository.rs` needs error type standardization
   - Consider adding _with_conn pattern proactively to new methods

2. **Performance monitoring:**
   - Benchmark statement caching impact in production
   - Monitor WAL checkpoint frequency

3. **Follow-up work:**
   - Add query performance logging in debug builds
   - Consider connection pooling for multi-threaded scenarios
```

## Operating Principles

### Refactoring Guidelines

- **Safety first**: Never compromise correctness for performance
- **Incremental changes**: One pattern type per commit
- **Test coverage**: Every refactoring requires regression test
- **Backward compatibility**: Maintain existing public APIs
- **Document rationale**: Explain why in commit messages and comments

### Testing Requirements

- All existing tests must pass after refactoring
- Add regression tests for bugs fixed (especially deadlocks)
- Add performance tests for optimizations
- Use TempDir pattern for test isolation
- Run full test suite before and after

### Code Review Checklist

Before marking refactoring complete:
- [ ] All tests pass
- [ ] No new clippy warnings
- [ ] Backward-compatible public API
- [ ] Error handling improved
- [ ] Performance not degraded
- [ ] Documentation updated
- [ ] Commit messages explain rationale

## Context

$ARGUMENTS
