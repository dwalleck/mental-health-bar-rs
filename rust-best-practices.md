# Rust Best Practices: Safety & Error Handling

*Based on mental-health-bar-rs codebase patterns*

## 1. Structured Error Handling

**Use `thiserror` for domain errors, not `anyhow`**

```rust
#[derive(Error, Debug)]
pub enum AssessmentError {
    #[error("Assessment not found: {0}")]
    NotFound(i32),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),  // Auto-conversion with #[from]
}
```

**Why:** `thiserror` provides typed errors for libraries/features, while `anyhow` is for application-level error propagation. Typed errors enable pattern matching and precise error handling.

## 2. Command Error Pattern (Tauri/API Layer)

**Create structured API responses with retryability hints:**

```rust
pub struct CommandError {
    pub message: String,        // User-friendly
    pub error_type: ErrorType,  // Machine-readable enum
    pub retryable: bool,        // Client guidance
}

impl CommandError {
    pub fn retryable(msg: impl Into<String>, error_type: ErrorType) -> Self {
        Self { message: msg.into(), error_type, retryable: true, details: None }
    }

    pub fn permanent(msg: impl Into<String>, error_type: ErrorType) -> Self {
        Self { message: msg.into(), error_type, retryable: false, details: None }
    }
}
```

**Convert feature errors to command errors:**

```rust
impl ToCommandError for AssessmentError {
    fn to_command_error(&self) -> CommandError {
        match self {
            AssessmentError::NotFound(id) =>
                CommandError::permanent(self.to_string(), ErrorType::NotFound),
            AssessmentError::Database(e) =>
                CommandError::from_rusqlite_error(e),  // Smart classification
        }
    }
}
```

**Why:** Separates domain errors from API concerns. Frontend can retry transient errors (database locks) but not validation errors.

## 3. Input Validation

**Validate at boundaries using `validator` crate:**

```rust
#[derive(Validate)]
pub struct SubmitAssessmentRequest {
    #[validate(length(max = 10), custom(function = "validate_alphanumeric"))]
    pub assessment_type_code: String,

    #[validate(length(max = 10000))]
    pub notes: Option<String>,
}

// In handler:
request.validate()
    .map_err(|e| CommandError::permanent(format!("Validation failed: {}", e), ErrorType::Validation))?;
```

**Why:** Fail fast at system boundaries. Never trust external input—validate length, format, and content before processing.

## 4. Safe Concurrent Database Access

**Use `parking_lot::Mutex` + `Arc` for shared connections:**

```rust
pub struct Database {
    conn: Arc<Mutex<Connection>>,  // Thread-safe shared access
}

impl Database {
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)  // Cheap clone of Arc
    }
}

// In repository:
pub fn save(&self, data: Data) -> Result<i32> {
    let conn = self.db.get_connection();
    let conn = conn.lock();  // Lock ONCE per public method
    // ... use &conn for all operations
    Ok(id)
}
```

**Why:** `Arc` enables safe shared ownership across threads. `Mutex` prevents data races. `parking_lot` is faster than `std::sync::Mutex`.

## 5. Database Safety Best Practices

**Always configure critical PRAGMAs:**

```rust
conn.execute_batch("
    PRAGMA foreign_keys = ON;      -- CRITICAL: Enforce referential integrity
    PRAGMA busy_timeout = 5000;    -- Prevent immediate SQLITE_BUSY
    PRAGMA journal_mode = WAL;     -- Better concurrency
")?;
```

**100% parameterized queries (never string interpolation):**

```rust
// ✅ SAFE:
conn.execute("INSERT INTO users (name) VALUES (?)", [&user_name])?;

// ❌ UNSAFE (SQL injection):
conn.execute(&format!("INSERT INTO users (name) VALUES ('{}')", user_name), [])?;
```

**Why:** Foreign keys OFF by default in SQLite. Parameterized queries prevent SQL injection.

## 6. Ownership & Borrowing

**Prefer borrowing over cloning:**

```rust
// ✅ GOOD: Borrow for read-only access
fn calculate_score(responses: &[i32]) -> Result<i32> {
    Ok(responses.iter().sum())
}

// ❌ WASTEFUL: Unnecessary clone
fn calculate_score(responses: Vec<i32>) -> Result<i32> {
    Ok(responses.iter().sum())
}
```

**Return owned types from functions:**

```rust
// ✅ GOOD: Caller owns the result
pub fn get_assessment(&self, id: i32) -> Result<Assessment> { ... }

// ❌ BAD: Lifetime complexity
pub fn get_assessment(&self, id: i32) -> Result<&Assessment> { ... }
```

**Why:** Borrowing avoids allocation. Returning owned types simplifies lifetimes for API boundaries.

## 7. Match-Based Error Handling

**Use exhaustive matching for error classification:**

```rust
match err {
    rusqlite::Error::SqliteFailure(err, _) => match err.code {
        ErrorCode::DatabaseBusy => CommandError::retryable(...),
        ErrorCode::ConstraintViolation => CommandError::permanent(...),
        _ => CommandError::permanent(...),
    },
    _ => CommandError::permanent(...),
}
```

**Why:** Compiler enforces exhaustive handling. Never silently swallow errors—classify and propagate appropriately.

## 8. Testing Strategy

**Write unit tests for all error paths:**

```rust
#[test]
fn test_incomplete_responses_error() {
    let result = calculate_phq9_score(&vec![1, 2, 3]);  // Only 3, needs 9
    assert!(result.is_err());
}
```

**Why:** Safety comes from testing failure modes, not just happy paths. Rust's Result type makes error testing explicit.

## Key Takeaways

- **Type safety:** Use enums and structs instead of strings/primitives
- **Fail fast:** Validate at boundaries, propagate typed errors
- **Never unwrap:** Use `?` operator or explicit match for error handling
- **Shared state:** `Arc<Mutex<T>>` for thread-safe shared access
- **Security:** Validate inputs, parameterize queries, check permissions
- **Test errors:** Every error path deserves a unit test
