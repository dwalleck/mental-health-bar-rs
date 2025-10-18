// Assessment feature module
// Vertical slice: models, commands, queries, repository

pub mod models;
pub mod commands;
pub mod queries;
pub mod repository;
pub mod content;

// Re-export commonly used types
pub use models::{
    AssessmentType, AssessmentResponse, AssessmentQuestion,
    SubmitAssessmentRequest, AssessmentError,
};
