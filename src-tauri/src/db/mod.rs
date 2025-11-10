use anyhow::{Context, Result};
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

pub mod migrations;
pub mod query_builder;

/// RAII guard for temporarily setting umask on Unix systems
/// Automatically restores previous umask when dropped
#[cfg(unix)]
struct UmaskGuard {
    old_umask: libc::mode_t,
}

#[cfg(unix)]
impl UmaskGuard {
    fn new(new_umask: libc::mode_t) -> Self {
        let old_umask = unsafe { libc::umask(new_umask) };
        Self { old_umask }
    }
}

#[cfg(unix)]
impl Drop for UmaskGuard {
    fn drop(&mut self) {
        unsafe {
            libc::umask(self.old_umask);
        }
    }
}

/// Database connection manager
pub struct Database {
    conn: Arc<Mutex<Connection>>,
    #[allow(dead_code)] // Used in set_secure_permissions method
    db_path: PathBuf,
}

impl Database {
    /// Initialize database connection and run migrations
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        // Ensure app data directory exists
        std::fs::create_dir_all(&app_data_dir).context("Failed to create app data directory")?;

        let db_path = app_data_dir.join("mental_health_tracker.db");
        info!("Initializing database at: {:?}", db_path);

        // Set secure umask for database file creation on Unix
        // This ensures new files are created with 0600 permissions
        #[cfg(unix)]
        let _umask_guard = {
            // If file exists, fix permissions immediately
            if db_path.exists() {
                Self::set_secure_permissions(&db_path)?;
            }

            // Set restrictive umask for file creation
            // umask(0o077) means created files get 0600 (rw-------)
            Some(UmaskGuard::new(0o077))
        };

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
        // Note: This operation cannot fail - it simply sets an internal capacity value
        conn.set_prepared_statement_cache_capacity(100);

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        };

        // Run migrations
        migrations::run_migrations(&db)?;

        // Ensure secure permissions on all database files (including WAL files)
        #[cfg(unix)]
        {
            Self::set_secure_permissions(&db.db_path)?;

            // Also secure WAL and SHM files if they exist (created by WAL mode)
            let wal_path = db.db_path.with_extension("db-wal");
            let shm_path = db.db_path.with_extension("db-shm");

            if wal_path.exists() {
                Self::set_secure_permissions(&wal_path)?;
            }
            if shm_path.exists() {
                Self::set_secure_permissions(&shm_path)?;
            }
        }

        #[cfg(not(unix))]
        {
            info!(
                "Database stored in user profile directory with inherited ACLs. \
                 On Windows, AppData directory ACLs typically restrict access to the current user. \
                 For additional security, manually verify file permissions in Windows Explorer \
                 (Properties > Security tab)."
            );
        }

        Ok(db)
    }

    /// Set secure file permissions (0600) on a file - Unix only
    #[cfg(unix)]
    fn set_secure_permissions(path: &std::path::Path) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let metadata =
            std::fs::metadata(path).context(format!("Failed to read metadata for {:?}", path))?;
        let mut permissions = metadata.permissions();
        let current_mode = permissions.mode() & 0o777;

        if current_mode != 0o600 {
            permissions.set_mode(0o600);
            std::fs::set_permissions(path, permissions)
                .context(format!("Failed to set permissions on {:?}", path))?;
            info!("Set secure permissions (0600) on {:?}", path);
        }

        Ok(())
    }

    /// Get a clone of the connection Arc for use in commands
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }

    /// Execute a query that returns no results
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        let conn = self.conn.lock();
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
