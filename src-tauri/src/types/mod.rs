//! Shared type definitions for improved type safety.
//!
//! These types replace stringly-typed fields with proper enums and newtypes,
//! providing compile-time validation and better error messages.
//!
//! # Design Decisions
//!
//! ## Typed vs String Fields
//!
//! This module provides newtypes for fields that benefit from validation:
//! - [`MoodRating`]: Enforces 1-7 range at construction time
//! - [`HexColor`]: Validates #RGB, #RRGGBB, or #RRGGBBAA format
//! - [`GoalType`], [`AssessmentCode`], [`AssessmentStatus`]: Enums replacing magic strings
//!
//! ## Timestamps as Strings (Intentional)
//!
//! Timestamps (`created_at`, `deleted_at`, `logged_at`) are kept as `String` rather
//! than using `chrono::DateTime<Utc>` or `time::OffsetDateTime`. This is intentional:
//!
//! **Rationale:**
//! - **Boundary types**: Timestamps flow `SQLite (TEXT) → Rust → JSON → Frontend`
//!   with minimal manipulation in Rust. They're transport values, not business logic.
//! - **SQLite compatibility**: SQLite stores timestamps as ISO 8601 TEXT. String
//!   representation matches storage format directly without conversion overhead.
//! - **Frontend simplicity**: TypeScript receives strings and parses with `new Date()`.
//!   Typed timestamps would serialize to identical strings anyway.
//! - **Minimal dependencies**: Avoids chrono/time crate dependency for boundary types.
//!
//! **When to reconsider:**
//! If the codebase requires significant date arithmetic (duration calculations,
//! relative date filtering, timezone conversions), consider parsing to `DateTime`
//! within specific functions while keeping `String` at API boundaries.
//!
//! **Format:** All timestamps use ISO 8601 format: `"2024-01-15T10:30:00"`

pub mod activity;
pub mod assessment;
pub mod mood;

// Re-export commonly used types
pub use activity::{Activity, GoalTarget, GoalType, HexColor};
pub use assessment::{AssessmentCode, AssessmentStatus, SeverityLevel};
pub use mood::MoodRating;
