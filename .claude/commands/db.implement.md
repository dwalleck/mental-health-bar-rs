---
description: Generate database implementation code following project patterns - repository methods, migrations, tests, and Tauri commands.
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Goal

Generate production-ready database implementation code following codebase architecture patterns:
- **Repository methods**: Following `Arc<Database>` + `_with_conn` pattern
- **SQL migrations**: With proper constraints and indexes
- **Integration tests**: Using TempDir pattern for isolation
- **Tauri commands**: With proper error handling and type safety

Ensure all generated code:
- Prevents deadlocks through `_with_conn` pattern
- Uses 100% parameterized queries
- Includes comprehensive error handling
- Follows TDD with tests written first
- Matches existing code style and conventions

## Knowledge Base References

Load these knowledge base documents for context:
- `.claude/knowledge/database-patterns.md` - Codebase-specific patterns and architecture
- `.claude/knowledge/sqlite-reference.md` - SQLite/rusqlite API reference
- `CLAUDE.md` - Project coding guidelines and conventions

## Execution Steps

### 1. Parse Implementation Request

Extract from $ARGUMENTS:
- **Feature name**: What database entity/feature to implement
- **Operations**: CRUD operations needed (Create, Read, Update, Delete, List, etc.)
- **Schema requirements**: Tables, columns, relationships, constraints
- **Query requirements**: Filtering, sorting, pagination, aggregations

If requirements are unclear, ask clarifying questions before proceeding.

### 2. Plan Implementation Architecture

**Determine feature placement:**
- Existing feature? Extend `src-tauri/src/features/{name}/repository.rs`
- New feature? Create new module under `src-tauri/src/features/{name}/`

**Plan file structure:**
```
src-tauri/src/features/{name}/
├── mod.rs              # Public exports
├── models.rs           # Domain types (Request/Response DTOs)
├── commands.rs         # Tauri commands (mutations)
├── queries.rs          # Tauri queries (reads)
└── repository.rs       # Database access layer (NEW/MODIFIED)

src-tauri/src/db/migrations/
└── {NNN}_{description}.sql  # New migration

src-tauri/tests/
└── test_{feature}.rs   # Integration tests
```

**Design method signatures:**
1. Public method: Takes parameters, acquires lock, calls `_with_conn` helper
2. Helper method: Accepts `&Connection`, performs database operations

Example:
```rust
// Public method
pub fn get_items(&self, filter: Option<String>) -> Result<Vec<Item>, Error> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| Error::LockPoisoned)?;
    self.get_items_with_conn(&conn, filter)
}

// Helper method (prevents deadlocks)
fn get_items_with_conn(
    &self,
    conn: &Connection,
    filter: Option<String>,
) -> Result<Vec<Item>, Error> {
    // Query implementation
}
```

### 3. Generate Migration First (TDD + Schema First)

**Create migration file:**
- Naming: `{NNN}_{snake_case_description}.sql` where NNN is next sequential number
- Location: `src-tauri/src/db/migrations/`

**Include in migration:**

```sql
-- Table creation with proper types
CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK(length(name) > 0),
    description TEXT,
    category_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT,  -- Soft delete support

    -- Foreign key constraints
    FOREIGN KEY (category_id) REFERENCES categories(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- Indexes for query performance
CREATE INDEX IF NOT EXISTS idx_items_category_id
    ON items(category_id);

-- Partial index for active records (soft delete pattern)
CREATE UNIQUE INDEX IF NOT EXISTS idx_items_name_active
    ON items(name) WHERE deleted_at IS NULL;

-- Trigger for updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_items_timestamp
AFTER UPDATE ON items
FOR EACH ROW
BEGIN
    UPDATE items SET updated_at = datetime('now')
    WHERE id = OLD.id;
END;
```

**Migration best practices:**
- Use `INTEGER` for booleans (0/1) with CHECK constraint
- Use `TEXT` for timestamps (ISO 8601 format)
- Always add `ON DELETE CASCADE/RESTRICT` to foreign keys
- Index foreign key columns
- Use partial indexes for soft deletes
- Include CHECK constraints for validation
- Add migration comment explaining purpose

### 4. Generate Repository Implementation

**Repository struct and constructor:**

```rust
use crate::db::Database;
use std::sync::Arc;

pub struct ItemRepository {
    db: Arc<Database>,
}

impl ItemRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}
```

**Implement CRUD operations following patterns:**

**CREATE:**
```rust
pub fn create_item(&self, req: CreateItemRequest) -> Result<Item, ItemError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| ItemError::LockPoisoned)?;

    // Use RETURNING for atomic insert+select
    let item = conn.query_row(
        "INSERT INTO items (name, description, category_id)
         VALUES (?1, ?2, ?3)
         RETURNING id, name, description, category_id, created_at, updated_at",
        params![req.name, req.description, req.category_id],
        |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                category_id: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        },
    )?;

    Ok(item)
}
```

**READ (with filtering):**
```rust
pub fn get_items(&self, filter: ItemFilter) -> Result<Vec<Item>, ItemError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| ItemError::LockPoisoned)?;
    self.get_items_with_conn(&conn, filter)
}

fn get_items_with_conn(
    &self,
    conn: &Connection,
    filter: ItemFilter,
) -> Result<Vec<Item>, ItemError> {
    let mut query = String::from(
        "SELECT id, name, description, category_id, created_at, updated_at
         FROM items
         WHERE deleted_at IS NULL"
    );
    let mut params: Vec<Box<dyn ToSql>> = Vec::new();

    // Safe dynamic query building (parameterized)
    if let Some(category_id) = filter.category_id {
        query.push_str(" AND category_id = ?");
        params.push(Box::new(category_id));
    }

    if let Some(name_search) = filter.name_search {
        query.push_str(" AND name LIKE ?");
        params.push(Box::new(format!("%{}%", name_search)));
    }

    query.push_str(" ORDER BY created_at DESC");

    if let Some(limit) = filter.limit {
        query.push_str(" LIMIT ?");
        params.push(Box::new(limit));
    }

    let mut stmt = conn.prepare(&query)?;
    let items = stmt
        .query_map(params.as_slice(), |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                category_id: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(items)
}
```

**UPDATE:**
```rust
pub fn update_item(&self, id: i64, req: UpdateItemRequest) -> Result<Item, ItemError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| ItemError::LockPoisoned)?;

    let updated = conn.execute(
        "UPDATE items
         SET name = ?1, description = ?2, category_id = ?3
         WHERE id = ?4 AND deleted_at IS NULL",
        params![req.name, req.description, req.category_id, id],
    )?;

    if updated == 0 {
        return Err(ItemError::NotFound(id.to_string()));
    }

    // Fetch updated record
    self.get_item_by_id_with_conn(&conn, id)
}
```

**DELETE (soft delete):**
```rust
pub fn delete_item(&self, id: i64) -> Result<(), ItemError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| ItemError::LockPoisoned)?;

    let deleted = conn.execute(
        "UPDATE items SET deleted_at = datetime('now') WHERE id = ?1 AND deleted_at IS NULL",
        params![id],
    )?;

    if deleted == 0 {
        return Err(ItemError::NotFound(id.to_string()));
    }

    Ok(())
}
```

**Aggregation queries with _with_conn:**
```rust
pub fn get_item_stats(&self, category_id: Option<i64>) -> Result<ItemStats, ItemError> {
    let conn = self.db.get_connection();
    let conn = conn.lock().map_err(|_| ItemError::LockPoisoned)?;
    self.get_item_stats_with_conn(&conn, category_id)
}

fn get_item_stats_with_conn(
    &self,
    conn: &Connection,
    category_id: Option<i64>,
) -> Result<ItemStats, ItemError> {
    // Use passed connection - no deadlock risk
    let mut query = String::from(
        "SELECT COUNT(*) as total,
                COUNT(DISTINCT category_id) as categories
         FROM items
         WHERE deleted_at IS NULL"
    );

    let params: Vec<Box<dyn ToSql>> = if let Some(cat_id) = category_id {
        query.push_str(" AND category_id = ?");
        vec![Box::new(cat_id)]
    } else {
        vec![]
    };

    let stats = conn.query_row(&query, params.as_slice(), |row| {
        Ok(ItemStats {
            total: row.get(0)?,
            categories: row.get(1)?,
        })
    })?;

    Ok(stats)
}
```

**Error type (using thiserror):**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ItemError {
    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Lock poisoned")]
    LockPoisoned,
}
```

### 5. Generate Integration Tests (TDD)

**Create test file** `src-tauri/tests/test_{feature}.rs`:

```rust
use mental_health_bar::db::Database;
use mental_health_bar::features::items::{ItemRepository, CreateItemRequest, ItemFilter};
use std::sync::Arc;
use tempfile::TempDir;

// Test helper: Create isolated database
fn setup_test_db() -> (Arc<Database>, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Arc::new(Database::new(db_path.to_str().unwrap()).unwrap());
    (db, temp_dir)
}

#[test]
fn test_create_item_success() {
    let (db, _temp_dir) = setup_test_db();
    let repo = ItemRepository::new(db);

    let req = CreateItemRequest {
        name: "Test Item".to_string(),
        description: Some("Description".to_string()),
        category_id: 1,
    };

    let item = repo.create_item(req).expect("Failed to create item");
    assert_eq!(item.name, "Test Item");
    assert!(item.id > 0);
}

#[test]
fn test_create_item_empty_name_fails() {
    let (db, _temp_dir) = setup_test_db();
    let repo = ItemRepository::new(db);

    let req = CreateItemRequest {
        name: "".to_string(),  // Invalid: empty name
        description: None,
        category_id: 1,
    };

    let result = repo.create_item(req);
    assert!(result.is_err(), "Should fail with empty name");
}

#[test]
fn test_get_items_with_filter() {
    let (db, _temp_dir) = setup_test_db();
    let repo = ItemRepository::new(db);

    // Create test data
    repo.create_item(CreateItemRequest {
        name: "Item 1".to_string(),
        description: None,
        category_id: 1,
    }).unwrap();

    repo.create_item(CreateItemRequest {
        name: "Item 2".to_string(),
        description: None,
        category_id: 2,
    }).unwrap();

    // Filter by category
    let filter = ItemFilter {
        category_id: Some(1),
        name_search: None,
        limit: None,
    };

    let items = repo.get_items(filter).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "Item 1");
}

#[test]
fn test_update_item_not_found() {
    let (db, _temp_dir) = setup_test_db();
    let repo = ItemRepository::new(db);

    let req = UpdateItemRequest {
        name: "Updated".to_string(),
        description: None,
        category_id: 1,
    };

    let result = repo.update_item(99999, req);
    assert!(matches!(result, Err(ItemError::NotFound(_))));
}

#[test]
fn test_soft_delete_item() {
    let (db, _temp_dir) = setup_test_db();
    let repo = ItemRepository::new(db);

    // Create and delete
    let item = repo.create_item(CreateItemRequest {
        name: "To Delete".to_string(),
        description: None,
        category_id: 1,
    }).unwrap();

    repo.delete_item(item.id).unwrap();

    // Should not appear in list
    let items = repo.get_items(ItemFilter::default()).unwrap();
    assert_eq!(items.len(), 0);

    // But record still exists in DB (soft delete)
    let conn = db.get_connection();
    let conn = conn.lock().unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM items WHERE id = ?", [item.id], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn test_get_stats_no_deadlock() {
    // Regression test: Ensure stats method doesn't deadlock
    let (db, _temp_dir) = setup_test_db();
    let repo = ItemRepository::new(db);

    // Create test data
    for i in 1..=10 {
        repo.create_item(CreateItemRequest {
            name: format!("Item {}", i),
            description: None,
            category_id: i % 3 + 1,
        }).unwrap();
    }

    // This should not deadlock
    let stats = repo.get_item_stats(None).expect("Stats query deadlocked!");
    assert_eq!(stats.total, 10);
}
```

### 6. Generate Tauri Commands (Optional)

If Tauri integration needed, generate commands in `commands.rs`:

```rust
use crate::features::items::{ItemRepository, CreateItemRequest, ItemFilter, ItemError};
use crate::AppState;
use anyhow::Context;

#[tauri::command]
pub async fn create_item(
    state: tauri::State<'_, AppState>,
    request: CreateItemRequest,
) -> Result<Item, String> {
    let repo = ItemRepository::new(state.db.clone());
    repo.create_item(request)
        .context("Failed to create item")
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_items(
    state: tauri::State<'_, AppState>,
    filter: ItemFilter,
) -> Result<Vec<Item>, String> {
    let repo = ItemRepository::new(state.db.clone());
    repo.get_items(filter)
        .context("Failed to fetch items")
        .map_err(|e| e.to_string())
}
```

### 7. Generate Implementation Summary

Output Markdown summary:

```markdown
# Database Implementation: {Feature Name}

## Files Created/Modified

- ✅ `src-tauri/src/db/migrations/{NNN}_{description}.sql`
- ✅ `src-tauri/src/features/{name}/repository.rs` (NEW/MODIFIED)
- ✅ `src-tauri/src/features/{name}/models.rs`
- ✅ `src-tauri/tests/test_{name}.rs`
- ✅ `src-tauri/src/features/{name}/commands.rs` (if Tauri commands generated)

## Repository Methods Implemented

| Method | Purpose | Deadlock Safe |
|--------|---------|---------------|
| `create_item()` | Insert new record | ✅ |
| `get_items()` | List with filtering | ✅ Uses _with_conn helper |
| `get_item_by_id()` | Fetch single record | ✅ |
| `update_item()` | Update existing record | ✅ |
| `delete_item()` | Soft delete record | ✅ |
| `get_item_stats()` | Aggregation query | ✅ Uses _with_conn helper |

## Migration Details

**Migration:** `{NNN}_{description}.sql`

**Tables Added:**
- `items`: Main entity table

**Indexes Added:**
- `idx_items_category_id`: Foreign key performance
- `idx_items_name_active`: Unique constraint for active records

**Constraints:**
- Foreign key to `categories` with CASCADE
- CHECK constraint on `name` length
- Soft delete support with `deleted_at`

## Test Coverage

**Tests Implemented:** N tests

- ✅ Create success path
- ✅ Create validation (empty name fails)
- ✅ Read with filtering
- ✅ Update not found error
- ✅ Soft delete verification
- ✅ Aggregation deadlock regression test

## Code Quality Checklist

- [x] 100% parameterized queries (no SQL injection)
- [x] All public methods use _with_conn helpers (no deadlocks)
- [x] Error handling with thiserror
- [x] TempDir pattern for test isolation
- [x] Foreign key constraints in schema
- [x] Soft delete support
- [x] Comprehensive test coverage

## Next Steps

1. Run migration: `cargo test` (migrations run automatically)
2. Run tests: `cargo test test_{name}`
3. Review generated code for business logic accuracy
4. Integrate Tauri commands into frontend (if applicable)
5. Add any domain-specific validation logic
```

## Operating Principles

### Code Generation Guidelines

- **Follow existing patterns**: Match codebase architecture exactly
- **Deadlock prevention**: Always generate _with_conn helpers for nested calls
- **Security first**: 100% parameterized queries, no string interpolation
- **TDD approach**: Generate tests alongside implementation
- **Complete implementations**: Fully functional code, not scaffolding
- **Error handling**: Use thiserror for repository, anyhow for commands
- **Documentation**: Include inline comments for complex logic

### Quality Standards

- All generated code must compile without warnings
- Tests must pass immediately after generation
- Follow naming conventions from `CLAUDE.md`
- Match indentation and formatting of existing code
- Include edge case tests (empty input, not found, constraints)
- Document any deviations from standard patterns

### Interaction Guidelines

- Ask clarifying questions if requirements unclear
- Provide implementation summary after generation
- Suggest related work (indexes, optimizations, validations)
- Reference relevant knowledge base sections
- Offer to run tests and fix any issues

## Context

$ARGUMENTS
