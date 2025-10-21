// Assessment feature module
// Vertical slice: models, commands, queries, repository

pub mod commands;
pub mod content;
pub mod models;
pub mod queries;
pub mod repository;

// Re-export commonly used types
pub use models::{
    AssessmentError, AssessmentQuestion, AssessmentResponse, AssessmentType,
    SubmitAssessmentRequest,
};
