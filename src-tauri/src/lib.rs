use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use tauri_specta::{collect_commands, Builder};

mod config;
pub mod db;
mod errors;
pub mod features;

// Re-export for easier access
pub use config::AppConfig;
pub use db::Database;
pub use errors::{AppError, AppResult};

/// Application state managed by Tauri
pub struct AppState {
    pub db: Arc<Database>,
    pub config: Arc<Mutex<AppConfig>>,
}

// Example command - keep for now
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            greet,
            features::assessments::commands::submit_assessment,
            features::assessments::commands::delete_assessment,
            features::assessments::queries::get_assessment_types,
            features::assessments::queries::get_assessment_questions,
            features::assessments::queries::get_assessment_history,
            features::assessments::queries::get_assessment_response,
            features::assessments::queries::get_latest_assessment,
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(specta_typescript::Typescript::default(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            // Initialize database
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            let db = Database::new(app_data_dir)
                .expect("Failed to initialize database");

            // Check database permissions
            db.check_permissions()
                .expect("Failed to check database permissions");

            // Load configuration
            let config = AppConfig::load()
                .expect("Failed to load configuration");

            // Setup managed state
            app.manage(AppState {
                db: Arc::new(db),
                config: Arc::new(Mutex::new(config)),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test to generate TypeScript bindings
    /// Run with: cargo test generate_types -- --exact --nocapture
    #[test]
    fn generate_types() {
        let builder = Builder::<tauri::Wry>::new()
            .commands(collect_commands![
                greet,
                features::assessments::commands::submit_assessment,
                features::assessments::commands::delete_assessment,
                features::assessments::queries::get_assessment_types,
                features::assessments::queries::get_assessment_questions,
                features::assessments::queries::get_assessment_history,
                features::assessments::queries::get_assessment_response,
                features::assessments::queries::get_latest_assessment,
            ]);

        builder
            .export(
                specta_typescript::Typescript::default(),
                "../src/lib/bindings.ts",
            )
            .expect("Failed to export TypeScript bindings");

        println!("TypeScript bindings generated successfully!");
    }
}
