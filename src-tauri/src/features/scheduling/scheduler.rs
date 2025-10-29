// Background scheduler for assessment reminders (User Story 6)
// T171-T173: Scheduler implementation

use std::sync::Arc;
use std::time::Duration;

use tauri::AppHandle;
use tokio::time::sleep;

use crate::db::Database;

use super::repository::SchedulingRepository;

/// Start the background scheduler
/// Checks for due schedules every minute and sends notifications
pub fn start_scheduler(app_handle: AppHandle, db: Arc<Database>) {
    tokio::spawn(async move {
        let repo = SchedulingRepository::new(db);

        loop {
            // Wait 1 minute before next check
            sleep(Duration::from_secs(60)).await;

            // Check for due schedules and send notifications
            if let Err(e) = check_and_notify(&app_handle, &repo).await {
                eprintln!("Scheduler error: {}", e);
            }
        }
    });
}

/// Check for due schedules and send notifications
async fn check_and_notify(
    app_handle: &AppHandle,
    repo: &SchedulingRepository,
) -> anyhow::Result<()> {
    // Get all due schedules
    let due_schedules = repo.get_due_schedules()?;

    for schedule in due_schedules {
        // Send notification
        send_notification(app_handle, &schedule.assessment_type_name, schedule.id)?;

        // Mark as triggered
        repo.mark_triggered(schedule.id)?;
    }

    Ok(())
}

/// Send a notification using tauri-plugin-notification
fn send_notification(
    app_handle: &AppHandle,
    assessment_name: &str,
    _schedule_id: i32,
) -> anyhow::Result<()> {
    use tauri_plugin_notification::NotificationExt;

    let notification_result = app_handle
        .notification()
        .builder()
        .title("Assessment Reminder")
        .body(format!("Time to complete: {}", assessment_name))
        .show();

    match notification_result {
        Ok(_) => {
            println!("Notification sent for: {}", assessment_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send notification: {}", e);
            Err(anyhow::anyhow!("Failed to send notification: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::features::scheduling::models::CreateScheduleRequest;
    use tempfile::TempDir;

    fn setup_test_repo() -> (SchedulingRepository, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db = Arc::new(Database::new(temp_dir.path()).expect("Failed to create database"));
        (SchedulingRepository::new(db), temp_dir)
    }

    #[test]
    fn test_get_due_schedules_empty() {
        let (repo, _temp_dir) = setup_test_repo();

        let due = repo
            .get_due_schedules()
            .expect("Failed to get due schedules");
        assert_eq!(due.len(), 0);
    }

    #[test]
    fn test_get_due_schedules_with_future_schedule() {
        let (repo, _temp_dir) = setup_test_repo();

        // Create a schedule for tomorrow at this time
        let request = CreateScheduleRequest {
            assessment_type_id: 1, // PHQ-9
            frequency: super::super::models::ScheduleFrequency::Daily,
            time_of_day: "23:59".to_string(), // Late time that hasn't occurred yet today
            day_of_week: None,
            day_of_month: None,
        };

        let _schedule = repo
            .create_schedule(&request)
            .expect("Failed to create schedule");

        let due = repo
            .get_due_schedules()
            .expect("Failed to get due schedules");
        assert_eq!(due.len(), 0, "Future schedule should not be due");
    }
}
