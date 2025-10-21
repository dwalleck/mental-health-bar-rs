# DuckDB Best Practices and Gotchas

This document captures DuckDB-specific practices, limitations, and solutions discovered during development.

## Reserved Keywords

### Critical: `AT` is Reserved (DuckDB 1.3.0+)

**Issue**: The `AT` keyword became reserved in DuckDB 1.3.0 (May 2025) for time travel in Iceberg support.

**Impact**: Using `AT` as a table alias causes SQL parser errors:
```sql
-- ❌ FAILS with "Parser Error: syntax error at or near '.'"
SELECT at.id, at.code FROM assessment_types at

-- ✅ WORKS - use different alias
SELECT atype.id, atype.code FROM assessment_types AS atype
```

**Solution**:
- Avoid using `at` as a table alias
- Use descriptive aliases like `atype`, `resp`, etc.
- Use `AS` keyword explicitly for clarity

### Other Reserved Keywords (DuckDB 1.3.0+)
- `LAMBDA` - reserved for lambda syntax
- `AT` - reserved for time travel
- Check with `duckdb_keywords()` function for complete list

**Best Practice**: Use the `duckdb_keywords()` metadata function to check if an identifier is reserved:
```sql
SELECT * FROM duckdb_keywords() WHERE keyword_name = 'AT';
```

## TIMESTAMP Type Handling

### Issue: Rust Type Conversion

DuckDB stores TIMESTAMP as INT64 microseconds since Unix epoch. The Rust duckdb crate can return this as:
- Native `chrono::NaiveDateTime` (with chrono feature)
- Raw integer value
- **NOT directly as String**

### Our Solution: SQL-Side Formatting

Instead of converting timestamps in Rust, we format them in SQL using `strftime()`:

```sql
-- ✅ WORKS - format in SQL query
SELECT
    id,
    strftime(completed_at, '%Y-%m-%d %H:%M:%S') as completed_at
FROM assessment_responses
```

```rust
// Retrieve as String in Rust
let completed_at: String = row.get(5)?;
```

**Benefits**:
- Consistent string format across application
- No Rust chrono deserialization complexity
- Compatible with TypeScript/Svelte frontend (expects strings)
- Simpler error handling

**Format String**: `'%Y-%m-%d %H:%M:%S'` produces ISO-8601-like format: `2025-10-20 14:30:45`

## Foreign Key Constraints

### ON DELETE CASCADE Not Supported

**Critical Limitation**: DuckDB parses `ON DELETE CASCADE` but **does not enforce it**.

```sql
-- ❌ ACCEPTED but CASCADE DOES NOT WORK
CREATE TABLE assessment_responses (
    id INTEGER PRIMARY KEY,
    assessment_type_id INTEGER NOT NULL,
    FOREIGN KEY (assessment_type_id) REFERENCES assessment_types(id) ON DELETE CASCADE
);
```

**Impact**:
- Syntax is valid (no migration errors)
- Deleting parent records with children **fails with constraint violation**
- Manual deletion of children required

**Workaround**:
```rust
// Must manually delete children first
repo.delete_child_records(parent_id)?;
repo.delete_parent_record(parent_id)?;
```

**GitHub Discussion**: [#10851](https://github.com/duckdb/duckdb/discussions/10851)

## Query Parameter Binding

### Dynamic Query Building

When building dynamic queries with optional filters, ensure parameters are bound in the same order they appear:

```rust
let mut query = String::from("SELECT * FROM table WHERE 1=1");
let mut params: Vec<&dyn duckdb::ToSql> = Vec::new();

if let Some(code) = &type_code {
    query.push_str(" AND code = ?");
    params.push(code);  // ✅ Add parameter in order
}

if let Some(from) = &from_date {
    query.push_str(" AND created_at >= ?");
    params.push(from);  // ✅ Add parameter in order
}

stmt.query_map(params.as_slice(), |row| { ... })?;
```

**Anti-pattern**: Building query string with placeholders but forgetting to add to params vector.

## Table Aliases

### Best Practices

1. **Use AS keyword explicitly** for clarity:
   ```sql
   FROM assessment_responses AS resp
   JOIN assessment_types AS atype ON resp.assessment_type_id = atype.id
   ```

2. **Choose descriptive aliases**:
   - `resp` for `assessment_responses`
   - `atype` for `assessment_types`
   - Avoid single letters that might become reserved keywords

3. **DuckDB also supports prefix aliases** (v1.20+):
   ```sql
   SELECT res: col1 + col2 FROM tbl;  -- Alternative syntax
   ```

## Connection and Transaction Management

### Lock Handling

DuckDB uses a single-writer model. When using `Arc<Mutex<Connection>>`:

```rust
// ✅ CORRECT - handle lock poisoning
let conn = self.db.get_connection();
let conn = conn.lock()
    .map_err(|e| anyhow::anyhow!("Database lock poisoned: {}", e))?;
```

**Never use `.unwrap()`** on lock acquisition in production code - handle poisoning gracefully.

### Transactions

```rust
// Begin explicit transaction for data consistency
conn.execute("BEGIN TRANSACTION", [])?;

match conn.query_row(...) {
    Ok(result) => {
        conn.execute("COMMIT", [])?;
        Ok(result)
    }
    Err(e) => {
        let _ = conn.execute("ROLLBACK", []);  // Best effort
        Err(e)
    }
}
```

## Performance Considerations

### Query Limits

Always enforce reasonable limits to prevent excessive memory usage:

```rust
const MIN_QUERY_LIMIT: i32 = 1;
const MAX_QUERY_LIMIT: i32 = 1000;

let safe_limit = requested_limit.clamp(MIN_QUERY_LIMIT, MAX_QUERY_LIMIT);
query.push_str(&format!(" LIMIT {}", safe_limit));
```

**Note**: LIMIT is interpolated (not parameterized) as it's validated/clamped.

## Version-Specific Features

### DuckDB 1.1.3 (Current)
- Foreign keys enforced (without CASCADE)
- TIMESTAMP as microsecond INT64
- `AT` reserved keyword
- JSON type support
- Sequences for auto-increment

### Migration Considerations

When migrating from SQLite:
- `AUTOINCREMENT` → Use `SEQUENCE` + `nextval()`
- `ON DELETE CASCADE` → Manual deletion required
- Date/time functions may differ slightly

## Project-Specific Deletion Patterns

This project implements two deletion patterns to work around DuckDB's lack of CASCADE support.

### Pattern 1: Defensive Deletion (Assessment Types)

**When to use**: Reference data that should rarely or never be deleted, where child data is precious.

**Example**: `assessment_types` table (PHQ-9, GAD-7, CES-D, OASIS)

```rust
// In assessment repository
pub fn delete_assessment_type(&self, id: i32) -> Result<(), AssessmentError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| AssessmentError::LockPoisoned)?;

    // Count child records
    let response_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM assessment_responses WHERE assessment_type_id = ?",
        [id],
        |row| row.get(0),
    )?;

    let schedule_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM assessment_schedules WHERE assessment_type_id = ?",
        [id],
        |row| row.get(0),
    )?;

    // Block deletion if children exist
    if response_count > 0 || schedule_count > 0 {
        return Err(AssessmentError::HasChildren(
            format!("Cannot delete assessment type: {} responses and {} schedules exist. Delete or export data first.",
                    response_count, schedule_count)
        ));
    }

    // Safe to delete - no children
    conn.execute("DELETE FROM assessment_types WHERE id = ?", [id])?;
    Ok(())
}
```

**Benefits**:
- Prevents accidental data loss
- Clear error messages guide user actions
- Fail-safe approach for sensitive mental health data

### Pattern 2: Application-Level Cascade (Mood Check-ins)

**When to use**: Junction tables or dependent data with no independent value.

**Example**: `mood_checkins` → `mood_checkin_activities` (junction table)

```rust
// In mood repository
pub fn delete_mood_checkin(&self, id: i32) -> Result<(), MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

    // Begin transaction for atomicity
    conn.execute("BEGIN TRANSACTION", [])?;

    // Closure for easy error propagation
    let result = (|| {
        // Delete children FIRST (junction table)
        conn.execute(
            "DELETE FROM mood_checkin_activities WHERE mood_checkin_id = ?",
            [id]
        )?;

        // Then delete parent
        conn.execute("DELETE FROM mood_checkins WHERE id = ?", [id])?;

        Ok(())
    })();

    // Commit or rollback
    match result {
        Ok(()) => {
            conn.execute("COMMIT", [])?;
            Ok(())
        }
        Err(e) => {
            // Always attempt rollback on error
            if let Err(rollback_err) = conn.execute("ROLLBACK", []) {
                error!("Failed to rollback transaction: {}", rollback_err);
            }
            Err(e)
        }
    }
}
```

**Benefits**:
- Atomic deletion (both succeed or both fail)
- Explicit deletion order (children before parents)
- Transaction ensures consistency
- Rollback logging for debugging

### Pattern 3: Soft Delete (Already Implemented)

**When to use**: Data referenced by historical records that must be preserved.

**Example**: `activities` table with `deleted_at` column

```rust
// Soft delete - UPDATE instead of DELETE
pub fn delete_activity(&self, id: i32) -> Result<(), MoodError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| MoodError::LockPoisoned)?;

    conn.execute(
        "UPDATE activities SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?",
        [id]
    )?;

    Ok(())
}

// Filter deleted activities in queries (for new mood check-ins)
pub fn get_active_activities(&self) -> Result<Vec<Activity>, MoodError> {
    // ...
    let activities = stmt.query_map([], |row| {
        // Only return activities where deleted_at IS NULL
    })?;
    // ...
}

// Include deleted activities in historical views
pub fn get_mood_checkin_with_activities(&self, id: i32) -> Result<MoodCheckin, MoodError> {
    // JOIN to activities without filtering deleted_at
    // Show "(deleted)" badge in UI if deleted_at IS NOT NULL
}
```

**Benefits**:
- Preserves audit trail and historical accuracy
- No risk of orphaned foreign keys
- Can be "undeleted" if needed
- Historical views show complete picture

### Testing Deletion Patterns

All deletion patterns must have integration tests:

```rust
#[test]
fn test_defensive_deletion_blocks_when_children_exist() {
    let repo = create_test_repo();

    // Create parent with child
    let type_id = repo.create_assessment_type(/* ... */).unwrap();
    repo.submit_assessment(type_id, /* ... */).unwrap();

    // Attempt to delete parent
    let result = repo.delete_assessment_type(type_id);

    // Should fail with HasChildren error
    assert!(matches!(result, Err(AssessmentError::HasChildren(_))));
}

#[test]
fn test_cascade_deletion_removes_children() {
    let repo = create_test_repo();

    // Create parent with children
    let checkin_id = repo.log_mood_checkin(/* ... */).unwrap();
    repo.link_activity(checkin_id, activity_id).unwrap();

    // Delete parent
    repo.delete_mood_checkin(checkin_id).unwrap();

    // Verify children deleted
    let junction_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM mood_checkin_activities WHERE mood_checkin_id = ?",
        [checkin_id],
        |row| row.get(0)
    ).unwrap();

    assert_eq!(junction_count, 0);
}

#[test]
fn test_soft_delete_preserves_historical_data() {
    let repo = create_test_repo();

    // Create activity and link to mood checkin
    let activity_id = repo.create_activity("Exercise").unwrap();
    let checkin_id = repo.log_mood_checkin_with_activity(activity_id).unwrap();

    // Soft delete activity
    repo.delete_activity(activity_id).unwrap();

    // Activity should not appear in "get active activities"
    let active = repo.get_active_activities().unwrap();
    assert!(!active.iter().any(|a| a.id == activity_id));

    // But should still appear in historical mood checkin
    let historical = repo.get_mood_checkin(checkin_id).unwrap();
    assert!(historical.activities.iter().any(|a| a.id == activity_id));
}
```

### Decision Matrix

| Scenario | Pattern | Reason |
|----------|---------|--------|
| Seeded reference data (assessment types) | Defensive | Should never be deleted; fail-safe |
| User historical data (responses, moods) | No deletion allowed | Preserve mental health history |
| Junction tables (mood_checkin_activities) | Cascade | No independent value |
| User-created labels/tags (activities) | Soft delete | Preserve historical references |
| Configuration (schedules) | Defensive or Cascade | Depends on relationship |

**See also**: `/specs/001-mental-health-tracking/plan.md` - Cascading Delete Strategy section for architectural rationale.

## Resources

- [DuckDB Keywords Documentation](https://duckdb.org/docs/stable/sql/dialect/keywords_and_identifiers)
- [DuckDB Timestamp Functions](https://duckdb.org/docs/stable/sql/functions/timestamp)
- [DuckDB Rust Client](https://duckdb.org/docs/stable/clients/rust.html)
- [duckdb crate docs](https://docs.rs/duckdb/)
- [Project Cascading Delete Strategy](/specs/001-mental-health-tracking/plan.md#cascading-delete-strategy)
