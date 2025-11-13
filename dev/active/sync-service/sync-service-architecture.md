# Sync Service Architecture & Data Structures

Last Updated: 2025-11-10

This document defines the concrete data structures and architecture for the sync service and its integration with the existing Tauri + SQLite app. It is designed to be directly translatable into Rust types, SQL schemas, and Tauri bindings.

---

## 1. High-Level Architecture

### 1.1 Components

- Desktop App (Tauri + SQLite)
  - Primary, offline-first data store.
  - Exposes Tauri commands for:
    - CRUD on assessments, mood entries, activities, schedules.
    - Sync operations (enable/disable, sync_now, status).
  - Contains:
    - Local DB layer.
    - Sync client module built on top of local DB.

- Sync Service (Axum + Postgres)
  - Stateless HTTP API process.
  - Persists:
    - User accounts / API keys.
    - Device registrations.
    - Synced domain entities with full history semantics.
  - Provides:
    - Authenticated sync endpoints for upload/download.
    - Health and readiness endpoints.

- Shared Domain Crate (core-domain)
  - Rust crate shared by:
    - Desktop app.
    - Sync service.
  - Defines:
    - Core domain entities.
    - Identifiers.
    - Sync DTOs.
    - Validation helpers.
  - Enables generated TS bindings if desired (via specta or OpenAPI).

### 1.2 Data Flow (Summary)

1. User interacts with desktop app:
   - Actions write directly to local SQLite (immediate, offline-capable).

2. When sync is enabled:
   - Desktop app uses stored:
     - endpoint URL
     - api_key
     - device_id
     - last_sync_cursor
   - Sync cycle:
     1. Collect local changes since last_sync_cursor.
     2. POST /v1/sync/upload (to sync-service).
     3. POST /v1/sync/download (to sync-service) with current cursor.
     4. Apply remote changes into local SQLite.
     5. Update last_sync_cursor.

3. Sync service:
   - Authenticates via API key → identifies user.
   - Merges incoming changes into Postgres.
   - Returns authoritative changes to clients.

---

## 2. Core Data Structures (Shared Domain)

All types below live in `crates/core-domain` (names illustrative).

### 2.1 Identifiers

```rust
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssessmentEntryId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MoodEntryId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActivityId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScheduleId(pub Uuid);
```

### 2.2 Common metadata

```rust
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
```

### 2.3 Domain entities (sync-relevant shape)

These mirror your existing entities but ensure stable IDs + metadata.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssessmentEntry {
    pub id: AssessmentEntryId,
    pub assessment_type: String, // e.g. "PHQ-9", "GAD-7"
    pub score: i32,
    pub severity: String,
    pub taken_at: OffsetDateTime,
    pub metadata: Metadata,
    // Optionally: raw answers if you sync them
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoodEntry {
    pub id: MoodEntryId,
    pub value: i32, // 1-5
    pub activities: Vec<ActivityId>,
    pub note: Option<String>,
    pub recorded_at: OffsetDateTime,
    pub metadata: Metadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub id: ActivityId,
    pub label: String,
    pub color_hex: Option<String>,
    pub icon: Option<String>,
    pub metadata: Metadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub id: ScheduleId,
    pub assessment_type: String,
    pub frequency: ScheduleFrequency,
    pub enabled: bool,
    pub next_due_at: Option<OffsetDateTime>,
    pub metadata: Metadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
}
```

(Exact fields can be aligned with your current schema; this is the sync-normalized view.)

---

## 3. Sync Protocol Data Structures

### 3.1 Sync changes

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityType {
    AssessmentEntry,
    MoodEntry,
    Activity,
    Schedule,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncChange {
    pub entity_type: EntityType,
    pub id: Uuid,                 // underlying ID; maps to *_Id newtypes
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
    // When deleted == false, contains serialized entity.
    // When deleted == true, data may be None or minimal.
    pub data: Option<serde_json::Value>,
}
```

Notes:

- `data` is intentionally generic:
  - Server and client know how to deserialize based on `entity_type`.
  - Allows incremental schema evolution.

### 3.2 Cursors

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncCursor(pub String);
// Implementation detail: could be a timestamp, monotonic sequence, or opaque token.
```

### 3.3 Requests and responses

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncUploadRequest {
    pub device_id: DeviceId,
    pub client_cursor: Option<SyncCursor>,
    pub changes: Vec<SyncChange>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub entity_type: EntityType,
    pub id: Uuid,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncUploadResponse {
    pub server_cursor: SyncCursor,
    pub applied_ids: Vec<Uuid>,
    pub conflicts: Vec<ConflictInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncDownloadRequest {
    pub device_id: DeviceId,
    pub since: SyncCursor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncDownloadResponse {
    pub server_cursor: SyncCursor,
    pub changes: Vec<SyncChange>,
}
```

These types live in `core-domain` and are shared between sync-service and Tauri sync client.

---

## 4. Sync Service Architecture (Detailed)

### 4.1 Logical modules

In `sync-service` crate:

- `main.rs`
  - Load config.
  - Initialize tracing.
  - Run Axum router.

- `config.rs`
  - Env-based configuration (DB URL, bind addr, TLS settings).

- `db/`
  - `mod.rs` for connection pool setup.
  - Schema migrations (via `sqlx migrate`).
  - Repository modules:
    - `users.rs`
    - `api_keys.rs`
    - `devices.rs`
    - `assessments.rs`
    - `mood_entries.rs`
    - `activities.rs`
    - `schedules.rs`

- `auth.rs`
  - API key extraction and validation.
  - `UserContext` injection into request extensions.

- `sync/`
  - `dto.rs` (re-exports core-domain sync types if needed).
  - `upload.rs`:
    - Implements /v1/sync/upload.
  - `download.rs`:
    - Implements /v1/sync/download.
  - `merge.rs`:
    - Encapsulates merge/conflict rules.

- `routes.rs`
  - Builds Axum router:
    - /health
    - /v1/sync/*
    - (optional) /v1/auth/api-keys* for management.

### 4.2 Database shaping (Postgres)

Key tables (simplified):

- `users`:
  - `id UUID PK`
  - `created_at TIMESTAMPTZ`

- `api_keys`:
  - `id UUID PK`
  - `user_id UUID FK`
  - `token_hash TEXT`
  - `created_at TIMESTAMPTZ`
  - `revoked_at TIMESTAMPTZ NULL`

- `devices`:
  - `id UUID PK`
  - `user_id UUID FK`
  - `label TEXT`
  - `created_at TIMESTAMPTZ`

- Entity tables (example: `assessments`):
  - `id UUID PK`
  - `user_id UUID FK`
  - `assessment_type TEXT`
  - `score INT`
  - `severity TEXT`
  - `taken_at TIMESTAMPTZ`
  - `created_at TIMESTAMPTZ`
  - `updated_at TIMESTAMPTZ`
  - `deleted_at TIMESTAMPTZ NULL`

Similar structure for `mood_entries`, `activities`, `schedules`.

### 4.3 Merge algorithm (v1)

For each `SyncChange` on upload:

1. Identify target row by `(user_id, entity_type, id)`.
2. Load existing row (if any) with its `updated_at`.
3. Compare:
   - If no row exists:
     - If `deleted == false`: insert.
     - If `deleted == true`: ignore (already deleted).
   - If row exists:
     - If `change.updated_at` > `row.updated_at`:
       - Apply change:
         - If `deleted == true`: set `deleted_at`.
         - Else: update fields + `updated_at`.
     - Else:
       - Treat as conflict:
         - Do not overwrite.
         - Add entry to `conflicts` with reason e.g. "server_newer".
4. After processing all, compute new `server_cursor`:
   - For v1, `server_cursor` can be:
     - Max `updated_at` for this user across all synced tables, encoded as string.

On download:

1. Given `since`:
   - Decode into timestamp (or use sequence).
2. Query all changes for user with `updated_at > since`:
   - Include rows where `deleted_at` is set (as `deleted == true`).
3. Emit as `SyncChange` list.
4. New `server_cursor` = new max `updated_at`.

This algorithm:

- Is deterministic.
- Keeps server as authoritative aggregator.
- Keeps implementation simple and auditable.

---

## 5. Desktop Integration Architecture

### 5.1 Tauri-side sync module

In `src-tauri/src/sync/`:

- `config.rs`
  - Stores:
    - endpoint_url
    - api_key
    - device_id
    - last_sync_cursor
  - Stored in local config table/file.

- `client.rs`
  - Functions:
    - `build_upload_request()`
      - Query local SQLite for entities where `updated_at > last_sync_cursor`.
      - Map to `SyncChange` list.
    - `apply_download_response(resp: SyncDownloadResponse)`
      - For each change:
        - If `deleted`: mark local row deleted (set `deleted_at`) if exists.
        - Else:
          - If no row: insert.
          - If exists:
            - If remote `updated_at` newer: update.
    - `sync_once()`
      - Call upload, then download.
      - Update `last_sync_cursor`.

- `commands.rs`
  - Tauri commands:
    - `enable_sync(...)`
    - `disable_sync()`
    - `sync_now()`
    - `get_sync_status()`

### 5.2 Frontend

- Settings page:
  - Reads status via `get_sync_status`.
  - Allows:
    - Enter/modify endpoint URL + API key.
    - Show device_id.
    - Trigger `sync_now`.
    - Show last success/failure and timestamp.

---

## 6. Invariants and Guarantees

- Local-first:
  - All operations work without sync configured.
- Safety:
  - Sync never hard-deletes immediately on client:
    - Uses soft deletes to ensure idempotence and safety.
- Determinism:
  - Given same sequence of changes, all replicas converge.
- Privacy:
  - No hidden network calls:
    - All sync endpoints and configuration are explicit.
  - API keys and URLs stored securely.
  - No sensitive payloads written to logs.

---

## 7. Next Steps

- Implement `core-domain` types as specified.
- Align SQLite schema with these structures.
- Implement sync-service Postgres schema + Axum routes according to this doc.
- Wire Tauri sync module and Settings UI using these shared types.

This document, together with:
- `sync-service-plan.md`
- `sync-service-context.md`
- `sync-service-tasks.md`

forms the complete specification and blueprint for your sync architecture.
