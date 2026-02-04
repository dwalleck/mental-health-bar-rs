//! Shared type definitions for improved type safety.
//!
//! These types replace stringly-typed fields with proper enums and newtypes,
//! providing compile-time validation and better error messages.

pub mod activity;
pub mod assessment;
pub mod mood;

// Re-export commonly used types
pub use activity::{GoalTarget, GoalType, HexColor};
pub use assessment::{AssessmentCode, AssessmentStatus, SeverityLevel};
pub use mood::MoodRating;
