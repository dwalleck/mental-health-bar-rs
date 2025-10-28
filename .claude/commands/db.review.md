---
description: Comprehensive SQLite/rusqlite database code review detecting deadlocks, SQL injection risks, and best practice violations.
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Goal

Perform comprehensive database code review across all repository layers, focusing on:
- **Deadlock detection**: Identify nested lock acquisition patterns and missing `_with_conn` helpers
- **Query security**: Verify 100% parameterization and detect SQL injection risks
- **Best practices**: Check PRAGMA enforcement, transaction patterns, error handling
- **Migration safety**: Analyze schema changes for data loss risks and constraint violations

## Knowledge Base References

Load these knowledge base documents for context:
- `.claude/knowledge/database-patterns.md` - Codebase-specific patterns and architecture
- `.claude/knowledge/sqlite-reference.md` - SQLite/rusqlite API reference
- `.claude/knowledge/sqlite-anti-patterns.md` - Common mistakes to avoid

## Execution Steps

### 1. Initialize Review Scope

If user provided specific files/paths in $ARGUMENTS:
- Focus review on those files only
- Still check for cross-file deadlock patterns

If no arguments provided:
- Review all database-related code:
  - `src-tauri/src/db/**/*.rs`
  - `src-tauri/src/features/**/repository.rs`
  - `src-tauri/src/db/migrations/*.sql`
  - `src-tauri/tests/test_*.rs` (database test files)

### 2. Deadlock Pattern Detection

**Scan for nested lock acquisition:**

```rust
// CRITICAL: Nested lock - causes deadlock
pub fn outer(&self) -> Result<T> {
    let conn = self.db.get_connection().lock()?;  // First lock
    self.inner()?;  // Calls method that locks again
}

fn inner(&self) -> Result<T> {
    let conn = self.db.get_connection().lock()?;  // Second lock - DEADLOCK!
}
```

**Check for missing `_with_conn` helpers:**
1. Find all methods that call `conn.lock()`
2. For each, check if it calls other repository methods
3. If nested call exists and callee also locks, flag as CRITICAL

**Verify `_with_conn` pattern compliance:**
- Methods ending in `_with_conn` must accept `&Connection` parameter
- Methods ending in `_with_conn` must NEVER call `conn.lock()`
- Public methods should use helpers like `helper_with_conn(&conn, ...)`

**Reference:** Real deadlock bug fixed at `mood/repository.rs:365` - see `database-patterns.md`

### 3. Query Security Analysis

**Verify 100% parameterization:**

```rust
// ❌ CRITICAL: SQL injection risk
let query = format!("SELECT * FROM users WHERE name = '{}'", user_input);
conn.query_row(&query, [], |row| ...)?;

// ✅ CORRECT: Parameterized query
conn.query_row("SELECT * FROM users WHERE name = ?", [user_input], |row| ...)?;
```

**Check for:**
- Any usage of `format!()` or string interpolation with SQL
- Dynamic table/column names (requires careful validation)
- Raw SQL in string literals containing variables

**Report finding if ANY non-parameterized query found** (CRITICAL severity)

### 4. Best Practices Audit

**PRAGMA Enforcement Check:**

Verify `db/mod.rs` enables critical PRAGMAs after connection open:
```rust
// REQUIRED: Must be present
conn.execute_batch("
    PRAGMA foreign_keys = ON;      -- CRITICAL: Enable FK constraints
    PRAGMA busy_timeout = 5000;    -- Prevent immediate SQLITE_BUSY
    PRAGMA journal_mode = WAL;     -- Recommended for concurrency
    PRAGMA synchronous = NORMAL;   -- Safe with WAL
")?;
```

**Missing PRAGMA severity:**
- `foreign_keys = ON`: **CRITICAL** (constraints not enforced)
- `busy_timeout`: **HIGH** (lock contention failures)
- `journal_mode = WAL`: **MEDIUM** (performance/concurrency)
- Others: **LOW** (optimization opportunities)

**Transaction Pattern Review:**

Current pattern (manual):
```rust
conn.execute("BEGIN IMMEDIATE", [])?;
// ... operations ...
conn.execute("COMMIT", [])?;
```

Recommended pattern (RAII):
```rust
let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
// ... operations ...
tx.commit()?;  // Auto-rollback on drop if not committed
```

Flag manual BEGIN/COMMIT as **MEDIUM** - suggest RAII refactoring

**Error Handling Review:**

Check for:
- `unwrap()` on lock acquisition (should use `map_err`)
- Generic error messages losing context
- Missing error type conversions (`#[from]` derives)
- Lock poisoning not handled properly

**Statement Caching Opportunities:**

Identify hot paths that would benefit from caching:
```rust
// Flag for optimization if in loop
for id in ids {
    let mut stmt = conn.prepare("SELECT ...")?;  // Wasteful
}

// Recommend:
let mut stmt = conn.prepare_cached("SELECT ...")?;
```

### 5. Migration Safety Analysis

For each `.sql` file in `src-tauri/src/db/migrations/`:

**Check for data loss risks:**
- `DROP TABLE/COLUMN` without backup strategy
- `ALTER TABLE` changing data types
- Adding `NOT NULL` to existing column without `DEFAULT`

**Verify constraint safety:**
- Foreign key references exist
- CHECK constraints are testable
- Unique constraints won't conflict with existing data

**Recommend:**
- Add migration tests verifying constraints
- Document rollback strategy for destructive changes

### 6. Performance Anti-Pattern Detection

**N+1 Query Detection:**
```rust
// ❌ Flag as MEDIUM
let posts = conn.prepare("SELECT * FROM posts")?.query_map(...)?;
for post in posts {
    let comments = conn.prepare("SELECT * FROM comments WHERE post_id = ?")?.query_map([post.id], ...)?;
}

// ✅ Recommend JOIN
```

**Unbounded Result Sets:**
```rust
// ❌ Flag as MEDIUM if table can be large
let all_users: Vec<User> = conn.prepare("SELECT * FROM users")?
    .query_map([], |row| ...)?.collect()?;
```

**Individual Inserts Without Transaction:**
```rust
// ❌ Flag as HIGH if in loop
for user in users {
    conn.execute("INSERT ...", params![user])?;  // Slow!
}
```

### 7. Produce Review Report

Output structured Markdown report:

```markdown
# Database Code Review Report

**Scope:** [list files reviewed]
**Timestamp:** [ISO 8601]
**Status:** [PASS | NEEDS_ATTENTION | CRITICAL_ISSUES]

## Summary Statistics

- Total files reviewed: N
- Critical issues: N
- High severity: N
- Medium severity: N
- Low severity: N

## Critical Issues

| Severity | Category | Location | Description | Fix |
|----------|----------|----------|-------------|-----|
| CRITICAL | Deadlock | repo.rs:123 | Nested lock in `get_stats()` calling `get_correlations()` | Add `get_correlations_with_conn(&conn, ...)` helper |

## High Severity Issues

[Same table format]

## Medium Severity Issues

[Same table format]

## Low Severity Issues / Optimization Opportunities

[Same table format]

## Best Practices Compliance

- [x] 100% query parameterization
- [ ] PRAGMA foreign_keys enabled ← **CRITICAL: Missing**
- [ ] PRAGMA busy_timeout configured ← **HIGH: Missing**
- [x] Error types use thiserror
- [ ] RAII transactions (currently manual) ← **MEDIUM: Refactor recommended**

## Migration Safety

[Per-migration analysis if applicable]

## Recommendations

1. **Immediate Actions** (CRITICAL/HIGH):
   - [Ordered list of must-fix items]

2. **Suggested Improvements** (MEDIUM):
   - [Ordered list of refactoring opportunities]

3. **Optimizations** (LOW):
   - [Ordered list of performance improvements]

## Next Steps

[Specific commands or actions user should take]
```

## Operating Principles

### Review Guidelines

- **Evidence-based findings**: Every issue must include file:line reference
- **Severity consistency**: Use documented severity heuristics
- **Actionable recommendations**: Provide specific fix with code example
- **Context from knowledge base**: Reference patterns/anti-patterns documents
- **No false positives**: Verify pattern before flagging
- **Prioritize safety**: Deadlocks and SQL injection are always CRITICAL

### Detection Priorities

1. **CRITICAL** (blocks production use):
   - Deadlock patterns (nested locks without _with_conn)
   - SQL injection risks (non-parameterized queries)
   - Missing `PRAGMA foreign_keys = ON`
   - Lock poisoning not handled

2. **HIGH** (causes failures under load):
   - Missing `PRAGMA busy_timeout`
   - Manual transactions without rollback handling
   - Generic error messages hiding root cause

3. **MEDIUM** (maintainability/performance):
   - Manual BEGIN/COMMIT instead of RAII
   - N+1 query patterns
   - Missing statement caching in hot paths
   - Terminology inconsistencies

4. **LOW** (optimization opportunities):
   - Missing WAL mode
   - Unbounded result set collection
   - Individual inserts outside transaction

### Output Format

- Use tables for structured findings
- Include code examples for complex issues
- Provide file:line references for navigation
- Group by severity for triage efficiency
- Keep total report under 2000 lines

## Context

$ARGUMENTS
