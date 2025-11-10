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

## Repository Comparison Summary

| Practice | Activities | Mood | Recommendation |
|----------|-----------|------|----------------|
| UTF-8 validation | ✅ `chars().count()` | ❌ `len()` (bug!) | Fix Mood |
| Structured logging | ✅ Field-based | ❌ String interp | Update Mood |
| Input trimming | ❌ Not implemented | ✅ Trims input | Add to Activities |
| parking_lot usage | ✅ Correct (no unwrap) | ✅ Correct | Keep |
| RETURNING clause | ✅ Used | ✅ Used | Keep |
| Soft deletes | ✅ Implemented | ✅ Implemented | Keep |
| Foreign key checks | ✅ With good errors | ✅ With good errors | Keep |

## Next Steps

1. **Create PR to fix Mood repository UTF-8 bug** - High priority, affects international users
2. **Add input trimming to Activities repository** - Low priority, consistency improvement
3. **Audit other repositories for `.len()` usage** - Search codebase for validation bugs
4. **Update CLAUDE.md** - Add UTF-8 validation requirement to guidelines
