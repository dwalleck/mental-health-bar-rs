use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

pub mod migrations;

/// Database connection manager
pub struct Database {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl Database {
    /// Initialize database connection and run migrations
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        // Ensure app data directory exists
        std::fs::create_dir_all(&app_data_dir).context("Failed to create app data directory")?;

        let db_path = app_data_dir.join("mental_health_tracker.db");
        info!("Initializing database at: {:?}", db_path);

        // Set file permissions to 0600 (user-only read/write) on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if db_path.exists() {
                let metadata = std::fs::metadata(&db_path)?;
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o600);
                std::fs::set_permissions(&db_path, permissions)?;
                info!("Database file permissions set to 0600");
            }
        }

        // Open database connection
        let conn = Connection::open(&db_path).context("Failed to open database connection")?;

        // ✅ CRITICAL: Configure SQLite PRAGMAs for data integrity and performance
        // Reference: .claude/knowledge/sqlite-reference.md
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;         -- CRITICAL: Enable FK constraint enforcement
             PRAGMA busy_timeout = 5000;       -- Wait 5s on lock contention before failing
             PRAGMA journal_mode = WAL;        -- Write-Ahead Logging for better concurrency
             PRAGMA synchronous = NORMAL;      -- Safe with WAL mode, faster than FULL
             PRAGMA cache_size = -64000;       -- 64MB cache (negative value = KB)
             PRAGMA temp_store = MEMORY;       -- Store temp tables in memory for speed",
        )
        .context("Failed to configure database PRAGMAs")?;

        // Configure statement cache for prepared statement reuse
        conn.set_prepared_statement_cache_capacity(100);

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        };

        // Run migrations
        migrations::run_migrations(&db)?;

        // Set permissions after creation if file was just created
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(&db.db_path)?;
            let mut permissions = metadata.permissions();
            if permissions.mode() & 0o777 != 0o600 {
                permissions.set_mode(0o600);
                std::fs::set_permissions(&db.db_path, permissions)?;
                info!("Database file permissions set to 0600 after creation");
            }
        }

        #[cfg(not(unix))]
        {
            warn!("File permission enforcement is only supported on Unix systems");
        }

        Ok(db)
    }

    /// Get a clone of the connection Arc for use in commands
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }

    /// Execute a query that returns no results
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Database lock poisoned: {}", e))?;
        conn.execute(sql, params).context("Failed to execute query")
    }

    /// Check database file permissions (Unix only)
    #[cfg(unix)]
    pub fn check_permissions(&self) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let metadata = std::fs::metadata(&self.db_path)?;
        let permissions = metadata.permissions();
        let mode = permissions.mode() & 0o777;

        if mode != 0o600 {
            warn!(
                "Database file has incorrect permissions: {:o} (expected 0600)",
                mode
            );
        } else {
            info!("Database file permissions verified: 0600");
        }

        Ok(())
    }

    #[cfg(not(unix))]
    pub fn check_permissions(&self) -> Result<()> {
        info!("Permission checking not supported on this platform");
        Ok(())
    }

    /// Run database migrations - exposed for testing
    pub fn run_migrations(&self) -> Result<()> {
        migrations::run_migrations(self)
    }
}
