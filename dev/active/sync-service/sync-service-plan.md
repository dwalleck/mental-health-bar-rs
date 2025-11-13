# Sync Service Plan

Last Updated: 2025-11-10

## 1. Executive Summary

Design and implement an optional, privacy-respecting sync architecture that:

- Keeps the existing Tauri + SQLite desktop app as the primary, offline-first store.
- Introduces a separate Axum + Postgres sync service for users who explicitly opt into multi-device sync or self-hosting.
- Uses shared Rust domain types and strong typing end-to-end.
- Provides deterministic, testable sync semantics with minimal complexity.

## 2. Goals and Non-Goals

### Goals

- Maintain offline-first, local-by-default behavior.
- Enable:
  - Multi-device sync for a single user.
  - Optional self-hosted sync backend.
- Keep protocol:
  - Simple.
  - Auditable.
  - Backed by explicit types and tests.
- Preserve clinical correctness and data integrity.

### Non-Goals (for first iteration)

- No real-time collaboration between multiple end-users.
- No complex conflict resolution beyond clearly defined, deterministic rules.
- No automatic cloud backup for all users; sync is opt-in.

## 3. Current State (Brief)

- Tauri 2 + Rust backend, SQLite for persistence.
- SvelteKit SPA frontend.
- Strong domain modeling for assessments, mood, activities, schedules.
- No concepts for:
  - Global identity.
  - Cross-device versioning.
  - Remote transport.

This foundation is compatible with a local-first sync design.

## 4. Proposed Architecture (High-Level)

### Components

1. Desktop App (unchanged core)
   - Local SQLite DB remains source of truth for that device.
   - New sync client module:
     - Tracks changes.
     - Calls sync service REST endpoints.
   - Sync is optional and configurable in Settings.

2. Sync Service (new)
   - Rust (Axum) HTTP API.
   - Postgres as backing store.
   - Endpoints for:
     - Authentication / API keys.
     - Uploading local changes.
     - Downloading remote changes.
   - Stateless app servers backed by persistent DB.

3. Shared Domain Crate
   - New crate used by:
     - Tauri app.
     - Sync service.
   - Contains:
     - Core types (AssessmentEntry, MoodEntry, Activity, Schedule, etc.).
     - Sync DTOs and validation logic.
     - Assessment scoring rules (if appropriate).

### Sync Model (Conceptual)

- Local-first:
  - Changes are applied to local SQLite immediately.
  - Sync is periodic/on-demand.
- Data model:
  - Each record:
    - Has stable UUID.
    - Timestamps: created_at, updated_at.
    - Soft-delete via deleted_at.
- Protocol:
  - Client sends changes since last_sync_cursor.
  - Server merges and returns changes newer than client’s cursor.
- Conflict resolution:
  - First iteration: last-write-wins on (updated_at, device_id) with deterministic tie-breaker.
  - Clinical forms & mood entries are append-like; conflicts are rare and manageable.

## 5. Implementation Phases

### Phase 1: Foundations

- Extract shared domain crate.
- Introduce sync-friendly metadata to local DB.
- Define preliminary sync types.

### Phase 2: Sync Service Core

- Create Axum + Postgres service.
- Implement:
  - User model.
  - API key auth.
  - Basic health and meta endpoints.

### Phase 3: Sync Protocol

- Define upload/download endpoints.
- Implement deterministic merge logic, cursors, and tests.

### Phase 4: Client Integration

- Implement sync module in Tauri backend.
- Wire up:
  - Manual sync.
  - Basic background/triggered sync.
- Add Settings UI.

### Phase 5: Hardening

- Security review.
- Observability.
- Documentation and self-hosting guides.

## 6. Acceptance Criteria (High-Level)

- Existing local-only functionality remains unchanged when sync is disabled.
- With sync enabled on 2+ devices using same account:
  - Creating/ updating / deleting assessments, moods, activities, schedules syncs correctly.
  - Behavior is deterministic and covered by automated tests.
- Sync service is self-hostable with documented migrations and configuration.
