# Repository Layer Best Practices

This document captures best practices for implementing repository layers in this codebase, based on learnings from the Activities and Mood repositories.

## Critical Patterns

### 1. UTF-8 Character Validation

**✅ CORRECT:**
```rust
let name_char_count = name.chars().count();
if name_char_count > 100 {
    return Err(ActivityError::GroupNameTooLong(name_char_count));
}
```

**❌ INCORRECT:**
```rust
let name_len = name.len();  // Counts BYTES, not characters!
if name_len > 100 {
    return Err(ActivityError::GroupNameTooLong(name_len));
}
```

**Why it matters:**
- `.len()` counts bytes, not Unicode characters
- Emoji and international characters are multi-byte (e.g., "🏃" is 4 bytes, but 1 character)
- User enters 100 emoji → `.len()` returns 400 → incorrectly rejected!
- **Action Required**: Audit all existing repositories for this bug

### 2. Structured Logging with Context

**✅ CORRECT:**
```rust
info!(
    group_id = id,
    group_name = name,
    has_description = description.is_some(),
    "Created activity group"
);
```

**❌ LESS USEFUL:**
```rust
info!("Created activity group with name: {}", name);
```

**Benefits:**
- Structured fields enable filtering/aggregation in production
- Easier to query logs: "Show all operations for group_id=5"
- Machine-readable for observability tools
- Context preserved across log lines

**Standard Context Fields:**
- Entity IDs: `group_id`, `activity_id`, `log_id`
- Names: `group_name`, `activity_name`
- Flags: `has_description`, `has_notes`
- Counts: `result_count`, `affected_rows`

### 3. Input Validation and Sanitization

**Best Practice Pattern:**
```rust
pub fn create_activity_group(
    &self,
    name: &str,
    description: Option<&str>,
) -> Result<ActivityGroup, ActivityError> {
    // 1. Trim input
    let name = name.trim();

    // 2. Empty check (after trim!)
    if name.is_empty() {
        return Err(ActivityError::EmptyGroupName);
    }

    // 3. Length validation (use chars().count() for UTF-8)
    let name_char_count = name.chars().count();
    if name_char_count > 100 {
        return Err(ActivityError::GroupNameTooLong(name_char_count));
    }

    // 4. Optional field validation
    if let Some(desc) = description {
        let desc_char_count = desc.chars().count();
        if desc_char_count > 500 {
            return Err(ActivityError::DescriptionTooLong(desc_char_count));
        }
    }

    // 5. Business logic (e.g., duplicate check)
    // ...
}
```

**Validation Order:**
1. Trim whitespace first
2. Check if empty (after trimming)
3. Validate character count (UTF-8 safe)
4. Validate optional fields
5. Business logic validation (duplicates, foreign keys)

### 4. parking_lot Mutex Usage

**Key Difference from std::sync::Mutex:**
```rust
// parking_lot returns guard directly (NOT Result)
let conn = self.db.get_connection();
let conn = conn.lock();  // No .unwrap() or .map_err() needed!

// std::sync::Mutex (for comparison)
let conn = conn.lock().unwrap();  // Returns Result<Guard, PoisonError>
```

**Why We Use parking_lot:**
- Faster than std::sync::Mutex
- Smaller memory footprint
- No lock poisoning (panics unwind locks automatically)
- Simpler API (no Result to handle)

### 5. SQL Query Patterns

**Parameterized Queries (CRITICAL):**
```rust
// ✅ ALWAYS USE:
conn.execute(
    "INSERT INTO activity_groups (name, description) VALUES (?, ?)",
    rusqlite::params![name, description],
)?;

// ❌ NEVER USE:
let query = format!(
    "INSERT INTO activity_groups (name) VALUES ('{}')",
    name  // SQL INJECTION RISK!
);
```

**RETURNING Clause Pattern:**
```rust
let (id, created_at): (i32, String) = conn.query_row(
    "INSERT INTO activity_groups (name, description) VALUES (?, ?)
     RETURNING id, CAST(created_at AS VARCHAR)",
    rusqlite::params![name, description],
    |row| Ok((row.get(0)?, row.get(1)?)),
)?;
```

**Benefits:**
- Single query instead of INSERT + SELECT
- Atomic operation (no race conditions)
- Get database-generated values (id, timestamps)

### 6. Soft Delete Pattern

**Standard Implementation:**
```rust
pub fn delete_activity_group(&self, id: i32) -> Result<(), ActivityError> {
    let conn = self.db.get_connection();
    let conn = conn.lock();

    let rows_affected = conn.execute(
        "UPDATE activity_groups SET deleted_at = CURRENT_TIMESTAMP WHERE id = ? AND deleted_at IS NULL",
        rusqlite::params![id],
    )?;

    if rows_affected == 0 {
        return Err(ActivityError::GroupNotFound(id));
    }

    info!(group_id = id, "Soft-deleted activity group");
    Ok(())
}
```

**Key Points:**
- Check `deleted_at IS NULL` to prevent double-deletion
- Use `rows_affected` to detect if entity exists
- Log the soft-delete operation
- Queries must filter `WHERE deleted_at IS NULL` to exclude deleted records

### 7. Duplicate Name Handling

**Recommended Pattern:**
```rust
// Check for duplicates before insert
let existing: Option<i32> = conn
    .query_row(
        "SELECT id FROM activity_groups WHERE name = ? AND deleted_at IS NULL",
        rusqlite::params![name],
        |row| row.get(0),
    )
    .optional()?;

if existing.is_some() {
    return Err(ActivityError::DuplicateGroupName(name.to_string()));
}
```

**Alternative: Let Database Handle It**
```sql
CREATE UNIQUE INDEX idx_activity_groups_name_active
ON activity_groups(name) WHERE deleted_at IS NULL;
```

**Trade-offs:**
- Application check: Better error messages, no exception handling
- Database constraint: Enforced even if multiple app instances
- Recommendation: Use both for defense-in-depth

### 8. Foreign Key Validation

**Pattern:**
```rust
// Validate foreign key exists before insert
let group_exists: bool = conn
    .query_row(
        "SELECT EXISTS(SELECT 1 FROM activity_groups WHERE id = ? AND deleted_at IS NULL)",
        rusqlite::params![group_id],
        |row| row.get(0),
    )?;

if !group_exists {
    return Err(ActivityError::GroupNotFound(group_id));
}
```

**Why not rely on database FK constraint?**
- Better error message (GroupNotFound vs generic ConstraintViolation)
- Distinguish between "group doesn't exist" vs "group is soft-deleted"
- More user-friendly errors

### 9. Test Setup Pattern

**Standard Test Helper:**
```rust
fn setup_test_repo() -> (ActivityRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db = Arc::new(Database::new(temp_dir.path()).expect("Failed to create database"));
    (ActivityRepository::new(db), temp_dir)
}

#[test]
fn test_create_activity_group() {
    let (repo, _temp_dir) = setup_test_repo();

    // Arrange
    let name = "Exercise";
    let description = Some("Physical activities");

    // Act
    let group = repo
        .create_activity_group(name, description)
        .expect("Failed to create group");

    // Assert
    assert_eq!(group.name, name);
    assert_eq!(group.description, description);
}
```

**Benefits:**
- Isolated test database per test
- Automatic cleanup (TempDir drops)
- Migrations run automatically
- No cross-test contamination

### 10. Comprehensive Test Coverage

**Required Test Categories:**
1. **Happy Path:** Basic CRUD operations work
2. **Validation:** Empty strings, too long, invalid chars
3. **Edge Cases:** Optional fields (None vs Some), boundary values
4. **Error Handling:** Not found, duplicates, foreign key violations
5. **UTF-8 Support:** Emoji, Japanese, multi-byte characters
6. **Query Filtering:** Date ranges, optional filters, ordering
7. **Soft Deletes:** Can't retrieve deleted, can't double-delete

**Example Test Matrix (Activities):**
```
Activity CRUD (8 tests):
✓ Basic creation with all fields
✓ Empty name validation
✓ Name too long (51 chars)
✓ Icon too long (21 chars)
✓ Invalid group_id → GroupNotFound
✓ Get by group (sorted by name)
✓ UTF-8: Emoji in name/icon
✓ UTF-8 validation: 50 emoji OK, 51 rejected

ActivityLog CRUD (6 tests):
✓ Basic logging with notes
✓ Notes too long (501 chars)
✓ Invalid activity_id → ActivityNotFound
✓ Get all logs
✓ Get by activity_id
✓ Get by date range (sorted DESC)
```

## Common Pitfalls

### 1. Byte vs Character Confusion
- **Problem:** Using `.len()` for character counting
- **Fix:** Always use `.chars().count()` for user-facing text
- **Impact:** Found in Mood repository, needs fixing

### 2. Inconsistent Trimming
- **Problem:** Some repos trim input, others don't
- **Fix:** Always trim before validation
- **Reason:** "  name  " should equal "name" for users

### 3. Missing Structured Logging
- **Problem:** String interpolation in log messages
- **Fix:** Use field syntax: `info!(field = value, "message")`
- **Benefit:** Queryable logs in production

### 4. Forgetting deleted_at Filter
- **Problem:** Queries return soft-deleted records
- **Fix:** All SELECT queries need `WHERE deleted_at IS NULL`
- **Test:** Verify deleted entities don't appear in results

### 5. Unclear Parameter Ordering
- **Problem:** Inconsistent parameter order across methods
- **Fix:** Follow convention:
  1. Entity ID (for updates/deletes)
  2. Required foreign keys
  3. Required fields
  4. Optional fields

## Migration Checklist

When implementing a new repository:

- [ ] Use `chars().count()` for all text length validation
- [ ] Add structured logging with context fields
- [ ] Validate foreign keys with helpful error messages
- [ ] Trim input before validation
- [ ] Use `RETURNING` clause for inserts
- [ ] Filter `deleted_at IS NULL` in all queries
- [ ] Write 10+ comprehensive tests (happy path + errors + UTF-8)
- [ ] Document public methods with examples
- [ ] Run `cargo clippy` and fix all warnings
- [ ] Run `cargo fmt` for consistent formatting

## 11. Advanced Patterns

### Batch Validation (Avoid N+1 Queries)

**Problem:** Validating multiple IDs in a loop causes N database queries.

**Example from Mood repository:**
```rust
// ❌ BAD: N+1 queries
for activity_id in activity_ids {
    validate_activity_exists(activity_id)?;  // Query per ID
}

// ✅ GOOD: Single query with IN clause
let placeholders = activity_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
let query = format!(
    "SELECT id FROM activities WHERE id IN ({}) AND deleted_at IS NULL",
    placeholders
);

let mut stmt = conn.prepare(&query)?;
let params: Vec<&dyn rusqlite::ToSql> = activity_ids
    .iter()
    .map(|id| id as &dyn rusqlite::ToSql)
    .collect();

let found_ids: HashSet<i32> = stmt
    .query_map(params.as_slice(), |row| row.get(0))?
    .collect::<Result<_, _>>()?;

// Check all IDs were found
let missing: Vec<i32> = activity_ids
    .iter()
    .filter(|id| !found_ids.contains(id))
    .copied()
    .collect();

if !missing.is_empty() {
    return Err(Error::InvalidActivityIds(missing));
}
```

**Benefits:**
- Single database round-trip instead of N
- Dramatically faster for large batches (10+ IDs)
- Reduces lock contention on database

### Defensive Deletion Pattern

**Problem:** Deleting parent records can orphan children or violate business rules.

**Example from Assessments repository:**
```rust
// Check for children before deleting parent
pub fn delete_assessment_type_with_conn(
    &self,
    conn: &Connection,
    id: i32,
) -> Result<(), AssessmentError> {
    // Count child records atomically within same lock
    let response_count = self.count_assessment_responses_with_conn(conn, id)?;
    let schedule_count = self.count_assessment_schedules_with_conn(conn, id)?;

    // Block deletion if children exist
    if response_count > 0 || schedule_count > 0 {
        return Err(AssessmentError::HasChildren(format!(
            "{} assessment response(s) and {} schedule(s) exist. \
             Delete or export data first.",
            response_count, schedule_count
        )));
    }

    // Safe to delete - no children
    conn.execute("DELETE FROM assessment_types WHERE id = ?", [id])?;
    Ok(())
}
```

**Benefits:**
- Prevents orphaned data
- Provides clear error messages to users
- Ensures data integrity beyond database constraints
- Allows defensive business logic (e.g., "export first")

### Statement Caching in Loops

**Problem:** Re-preparing statements in loops is inefficient.

**Example from Mood and Scheduling repositories:**
```rust
// ❌ BAD: Re-prepares statement N times
for item in items {
    conn.execute("INSERT INTO table VALUES (?)", [item])?;  // Prepare every time
}

// ✅ GOOD: Prepare once, execute many times
let mut stmt = conn.prepare_cached("INSERT INTO table VALUES (?)")?;
for item in items {
    stmt.execute([item])?;  // Uses cached prepared statement
}
```

**Benefits:**
- Avoids repeated SQL parsing and optimization
- 2-5x faster for batch operations
- Automatic caching by rusqlite (`prepare_cached`)
- Especially important inside transactions

### Dynamic Query Building with Constants

**Problem:** Building SQL dynamically risks injection; string interpolation is dangerous.

**Example from Scheduling repository:**
```rust
// Define safe constant clauses
const ENABLED_CLAUSE: &str = "enabled = 1";
const FREQUENCY_DAILY: &str = "frequency = 'daily'";
const TIME_CLAUSE: &str = "time_of_day = ?";

pub fn get_schedules_filtered(
    &self,
    only_enabled: bool,
    daily_only: bool,
    time_filter: Option<&str>,
) -> Result<Vec<Schedule>, Error> {
    let mut clauses = vec![];
    let mut params: Vec<&dyn rusqlite::ToSql> = vec![];

    // Add clauses based on filters
    if only_enabled {
        clauses.push(ENABLED_CLAUSE);  // No params needed
    }
    if daily_only {
        clauses.push(FREQUENCY_DAILY);  // Safe constant
    }
    if let Some(time) = time_filter {
        clauses.push(TIME_CLAUSE);
        params.push(time);  // Parameterized
    }

    // Build WHERE clause from safe constants
    let where_clause = if !clauses.is_empty() {
        format!("WHERE {}", clauses.join(" AND "))
    } else {
        String::new()
    };

    let query = format!("SELECT * FROM schedules {}", where_clause);
    // Execute with params...
}
```

**Benefits:**
- Prevents SQL injection (clauses are constants)
- Safer than string interpolation
- Easier to audit (all SQL clauses visible as constants)
- Maintains flexibility for dynamic queries

### Character vs Grapheme Counting

**Current Limitation:**
We use `.chars().count()` which counts Unicode scalar values, not grapheme clusters.

**Impact:**
```rust
"👨‍👩‍👧‍👦".chars().count()  // Returns 7 (Unicode scalars)
// But user perceives this as 1 character (family emoji)
```

**Future Consideration:**
For apps with heavy emoji usage, consider `unicode-segmentation` crate:
```rust
use unicode_segmentation::UnicodeSegmentation;
text.graphemes(true).count()  // Returns 1 for "👨‍👩‍👧‍👦"
```

**Trade-off:**
- **Current (`.chars().count()`)**: Good for 95% of cases, no extra dependency
- **Future (`graphemes`)**: Handles all emoji correctly, adds ~400KB dependency

**Recommendation:**
Use `.chars().count()` unless users report emoji validation issues. It's vastly better than `.len()` (byte counting) which was the original bug.

## Repository Comparison Summary

**After standardization (this PR):**

| Repository | UTF-8 | Trimming | Logging | Transactions | Advanced Patterns | Grade |
|------------|-------|----------|---------|--------------|-------------------|-------|
| **Activities** | ✅ | ✅ | ✅ | Partial | RETURNING | A |
| **Mood** | ✅ | ✅ | ✅ | ✅ | Batch validation, Statement caching | A+ |
| **Assessments** | N/A | ✅ | ✅ | ✅ | Defensive deletion, RETURNING | A |
| **Scheduling** | N/A | N/A | ✅ | ✅ | Dynamic queries, Statement caching | A |
| **Visualization** | N/A | N/A | N/A | N/A | Read-only queries | N/A |

**Legend:**
- UTF-8: Uses `.chars().count()` for text validation (vs `.len()` which counts bytes)
- Trimming: Trims whitespace from user input
- Logging: Structured logging with context fields
- Transactions: Uses RAII transactions for multi-step operations
- N/A: Not applicable (no user text input or read-only operations)

**Key Improvements Made:**
- ✅ Fixed critical UTF-8 bug in Mood repository (`.len()` → `.chars().count()`)
- ✅ Added structured logging to Assessments and Scheduling
- ✅ Added input trimming to Activities and Assessments
- ✅ Documented advanced patterns from all repositories

## Next Steps

All repositories now follow consistent best practices! Future work:

1. **Monitor for emoji issues** - If users report problems with complex emoji (👨‍👩‍👧‍👦), consider `unicode-segmentation`
2. **Add RAII transactions to Activities** - If multi-step operations are added in future
3. **Update CLAUDE.md** - Reference this document for new repository implementations
