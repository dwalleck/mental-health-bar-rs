# Sync Service Tasks

Last Updated: 2025-11-10

This file is a concrete, trackable checklist to implement the sync service and desktop integration described in `sync-service-plan.md` and `sync-service-context.md`.

Effort legend: S (0.5–1d), M (1–3d), L (3–7d), XL (7d+)

---

## Phase 1: Shared Domain and Local Schema (Foundations)

### 1.1 Create shared core-domain crate (M)

- [ ] Create `crates/core-domain/` with Cargo.toml.
- [ ] Define core entities (types only, no persistence):
  - [ ] `UserId`, `DeviceId` (opaque newtypes).
  - [ ] `AssessmentEntryId`, `MoodEntryId`, `ActivityId`, `ScheduleId` (UUID-based).
  - [ ] `AssessmentEntry` (metadata + payload reference).
  - [ ] `MoodEntry` (score, activities, notes).
  - [ ] `Activity` (id, label, color, metadata).
  - [ ] `Schedule` (assessment type, cadence, next_due, enabled).
- [ ] Add serde derives for all shared types.
- [ ] Add basic validation helpers where appropriate.
- [ ] Integrate crate into `src-tauri`:
  - [ ] Replace ad-hoc types with `core-domain` equivalents where feasible.
- Acceptance:
  - [ ] `cargo test` passes across workspace.
  - [ ] No duplication of core ID/entity shapes between app and service.

### 1.2 Add sync-friendly metadata to local SQLite schema (M)

- [ ] For each synced table, ensure:
  - [ ] `id` is a stable UUID (not just autoincrement).
  - [ ] `created_at` (UTC).
  - [ ] `updated_at` (UTC).
  - [ ] `deleted_at` (nullable, for soft delete).
- [ ] Write migrations to add/transform columns safely.
- [ ] Update Rust DB layer to:
  - [ ] Generate UUIDs via `core-domain` types.
  - [ ] Maintain `updated_at` on changes.
  - [ ] Use `deleted_at` for deletions instead of hard delete (where sync-relevant).
- [ ] Add tests around:
  - [ ] Insert/update/delete maintaining metadata.
- Acceptance:
  - [ ] Existing app behavior unchanged.
  - [ ] All synced entities have stable identity + timestamps.

---

## Phase 2: Sync Service Skeleton (Axum + Postgres)

### 2.1 Scaffold sync-service crate (S)

- [ ] Create `sync-service/` binary crate.
- [ ] Add dependencies:
  - [ ] `axum`, `tower`, `tower-http`
  - [ ] `serde`, `serde_json`
  - [ ] `sqlx` (Postgres feature), `uuid`, `time`
  - [ ] `tracing`, `tracing-subscriber`
- [ ] Wire basic server:
  - [ ] Config via env (DB URL, bind address).
  - [ ] `/health` endpoint.
- Acceptance:
  - [ ] `cargo run -p sync-service` starts.
  - [ ] `/health` returns 200.

### 2.2 Database and migrations (M)

- [ ] Define Postgres schema:
  - [ ] `users` (id, created_at, optional email/metadata).
  - [ ] `api_keys` (id, user_id, token_hash, created_at, revoked_at).
  - [ ] `devices` (id, user_id, label, created_at).
  - [ ] `assessments`, `mood_entries`, `activities`, `schedules` tables mirroring core-domain + sync metadata.
- [ ] Write migrations using `sqlx migrate` (or similar).
- [ ] Add `sqlx` compile-time checks for key queries.
- Acceptance:
  - [ ] `sqlx migrate run` initializes DB correctly in dev.
  - [ ] Basic queries covered by tests.

---

## Phase 3: Auth and Sync Protocol

### 3.1 API key authentication (M)

- [ ] Implement middleware extracting `Authorization: Bearer <token>`.
- [ ] Validate token against `api_keys` table.
- [ ] Attach `UserContext` (user_id) to request extensions.
- [ ] Provide:
  - [ ] Minimal admin/CLI path to create users and API keys.
- Acceptance:
  - [ ] Requests with valid token succeed; invalid → 401/403.
  - [ ] Tests for auth paths and middleware.

### 3.2 Define sync DTOs and protocol (M)

- [ ] In `core-domain`, define:
  - [ ] `SyncChange`:
    - `entity_type`
    - `id`
    - `data` (entity payload or null if deleted)
    - `updated_at`
    - `deleted` (bool or via null data)
  - [ ] `SyncUploadRequest`:
    - `device_id`
    - `changes: Vec<SyncChange>`
    - `client_cursor` (optional)
  - [ ] `SyncUploadResponse`:
    - `server_cursor`
    - `applied: Vec<id>`
    - `conflicts: Vec<ConflictInfo>`
  - [ ] `SyncDownloadRequest`:
    - `device_id`
    - `since: cursor`
  - [ ] `SyncDownloadResponse`:
    - `server_cursor`
    - `changes: Vec<SyncChange>`
- Acceptance:
  - [ ] DTOs compile and are used by both service and client.

### 3.3 Implement /v1/sync/upload (M)

- [ ] Handler logic:
  - [ ] Authenticate user.
  - [ ] For each change:
    - [ ] Load server record (if exists).
    - [ ] Compare `updated_at`:
      - If client newer → apply change.
      - If server newer → record conflict (do not override).
  - [ ] Update server cursor for this user.
- [ ] Write tests:
  - [ ] New records.
  - [ ] Updates.
  - [ ] Deletes.
  - [ ] Conflict cases.
- Acceptance:
  - [ ] Deterministic last-write-wins behavior with coverage.

### 3.4 Implement /v1/sync/download (M)

- [ ] Handler logic:
  - [ ] Authenticate user.
  - [ ] Query all changes with `updated_at > since_cursor`.
  - [ ] Return as `SyncChange` list + new `server_cursor`.
- [ ] Tests:
  - [ ] No changes → empty list.
  - [ ] Some changes → correct subset.
- Acceptance:
  - [ ] Symmetric with upload; forms a coherent protocol.

---

## Phase 4: Tauri Client Integration

### 4.1 Local change tracking (M)

- Option A (preferred for v1): cursor + updated_at scan.
- [ ] Store per-user:
  - [ ] `last_sync_cursor` in a local `sync_state` table.
- [ ] For upload:
  - [ ] Select rows where `updated_at > last_pushed_at` or equivalent.
- [ ] For apply:
  - [ ] Upsert rows based on `id` and `updated_at`.
- Acceptance:
  - [ ] No separate changelog table required for v1.
  - [ ] Stable behavior under repeated syncs.

### 4.2 Tauri sync module (M)

- [ ] New module: `src-tauri/src/sync/`.
- [ ] Implement:
  - [ ] `configure_sync(endpoint_url, api_key, device_id)`:
    - Persist in secure config.
  - [ ] `sync_once()`:
    - Build `SyncUploadRequest`, call `/v1/sync/upload`.
    - Call `/v1/sync/download` with latest cursor.
    - Apply incoming changes to local DB.
    - Update `last_sync_cursor`.
  - [ ] Error handling:
    - Network failures must not corrupt local data.
- [ ] Expose Tauri commands:
  - [ ] `enable_sync`
  - [ ] `disable_sync`
  - [ ] `sync_now`
  - [ ] `get_sync_status`
- Acceptance:
  - [ ] Manual sync from app works end-to-end against dev server.
  - [ ] Failures are visible but non-destructive.

### 4.3 Frontend UI integration (S)

- [ ] Add Settings UI:
  - [ ] Fields for server URL, API key.
  - [ ] Display device_id and last sync time.
  - [ ] Button: “Sync now”.
  - [ ] Status indicator (idle, in-progress, last result).
- [ ] Wire UI to Tauri commands.
- Acceptance:
  - [ ] Users can configure sync without touching config files.
  - [ ] Clear indication when sync is enabled/disabled.

---

## Phase 5: Hardening, Privacy, and Ops

### 5.1 Security & privacy review (M)

- [ ] Ensure all sync traffic uses HTTPS in production.
- [ ] Confirm:
  - [ ] No sensitive data logged in:
    - Axum service.
    - Tauri app.
- [ ] Update docs:
  - [ ] `SECURITY.md` / README section for sync:
    - Threat model.
    - Data stored remotely.
    - Self-hosting guidance.
- Acceptance:
  - [ ] Written guarantees align with implementation.

### 5.2 Observability and robustness (S)

- [ ] Add structured logging using `tracing`.
- [ ] Add:
  - [ ] Request IDs.
  - [ ] Basic metrics hooks (if desired).
- [ ] Document:
  - [ ] Health checks and readiness endpoints.
- Acceptance:
  - [ ] Ops can monitor sync-service health.

### 5.3 Self-hosting & deployment docs (S)

- [ ] Provide:
  - [ ] `docker-compose.yml` example (Postgres + sync-service).
  - [ ] Example systemd service.
  - [ ] Env var configuration reference.
- Acceptance:
  - [ ] A technical user can self-host with copy-pasteable examples.

---

## Phase 6: Validation

### 6.1 End-to-end test scenarios (M)

- [ ] Scenario: Single device, sync disabled:
  - [ ] All existing flows unchanged.
- [ ] Scenario: Two devices, same user:
  - [ ] Create records on device A, sync, then device B syncs → sees data.
  - [ ] Concurrent edits resolved via last-write-wins.
  - [ ] Deletes propagate correctly.
- [ ] Record automated tests where possible:
  - [ ] Integration tests for sync-service.
  - [ ] Tauri integration/integration-style tests for sync client (if feasible).
- Acceptance:
  - [ ] All core sync scenarios are green.
