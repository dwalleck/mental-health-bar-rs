# Sync Service Context

Last Updated: 2025-11-10

## 1. Purpose

This document captures key context, decisions, and dependencies for implementing the optional sync service that extends the existing mental-health-bar desktop app with multi-device sync while preserving offline-first and privacy-first guarantees.

## 2. Relevant Existing Files and Areas

- Core app:
  - `src-tauri/src/`
    - Backend commands, DB access, domain logic.
  - `src/`
    - SvelteKit frontend (routes, components, stores).
- Documentation and specs:
  - `README.md`
    - High-level architecture and privacy guarantees.
  - `specs/001-mental-health-tracking/`
    - Requirements, data model, assessments, and workflows.
  - `dev/active/major-refactoring-2025/`
    - Architecture direction, principles, and refactor guidance.
- Quality and practices:
  - `REPOSITORY-BEST-PRACTICES.md`
  - `rust-best-practices.md`
  - `svelte-best-practices.md`
  - `TEST_COVERAGE_ANALYSIS.md`

These documents and modules must remain the source of truth for behavior and standards as sync is added.

## 3. Core Design Decisions

1. Local-first model
   - SQLite on each device remains the primary store.
   - All user actions work fully offline.
   - Sync is additional, never required for core functionality.

2. Separate sync service
   - Implemented as a distinct Rust service using:
     - Axum (HTTP API)
     - Postgres (primary DB)
   - Deployed independently; can be:
     - Hosted by the project.
     - Self-hosted by advanced users.

3. Shared domain model
   - Introduce a shared crate (e.g., `crates/core-domain`) to define:
     - Core entities: assessments, mood entries, activities, schedules.
     - IDs (UUIDs), timestamps, and soft-delete markers.
     - Sync DTOs and validation.
   - Both Tauri app and sync service depend on this crate.

4. Sync semantics
   - Local DB is an eventually-consistent replica of a per-user dataset.
   - Sync operations:
     - Upload local changes (since last cursor).
     - Download remote changes (newer than last cursor).
   - Conflict resolution:
     - Deterministic last-write-wins for v1.
     - Specialized rules only if needed and well documented.

5. Security and privacy
   - Sync is opt-in and explicit in UI.
   - All remote calls over HTTPS.
   - API key or token-based auth.
   - No analytics, tracking, or unnecessary metadata.
   - Logs must avoid sensitive payloads.

## 4. Dependencies and Interfaces

### 4.1. Internal Dependencies

- Tauri app:
  - Must expose a small sync API internally:
    - Read changes from local SQLite.
    - Apply incoming changes.
    - Track sync cursors.
- Sync service:
  - Depends on:
    - `core-domain` for shared types.
    - `sqlx` (or equivalent) for Postgres.
    - Migrations for schema management.

### 4.2. External Interfaces

- Public HTTP API (sync service):
  - `/health`
  - `/v1/auth/api-keys` (management or admin-only)
  - `/v1/sync/upload`
  - `/v1/sync/download`
- Tauri commands (desktop app):
  - `enable_sync`, `disable_sync`
  - `sync_now`
  - `get_sync_status`

Exact signatures will be defined in the plan and tasks.

## 5. Constraints

- Must not break:
  - Existing privacy guarantees for non-sync users.
  - Offline usability.
- Must remain:
  - Auditable, with clear and simple data flows.
  - Backed by tests for correctness and conflict resolution.
- Must support:
  - Self-hosting with minimal steps (env vars, Docker, or systemd unit).

## 6. Open Questions (to be resolved in implementation)

- Identity model:
  - Start with API keys bound to a single logical “account”.
  - Later: optional email/password or OAuth without changing core sync protocol.
- Encryption at rest on server:
  - Not required for v1, but design should not block adding it.
- Granularity of cursors:
  - Global `last_synced_at` vs per-collection/per-entity cursors.
  - v1: single monotonic cursor per user is acceptable if carefully implemented.

This context should be read alongside `sync-service-plan.md` and `sync-service-tasks.md` to understand the why behind the implementation details.
