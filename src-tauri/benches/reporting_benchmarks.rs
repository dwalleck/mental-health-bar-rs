// Criterion benchmarks for Reporting Queries
// Task 4.17: Test reporting queries with large datasets (>1000 logs)
//
// These benchmarks measure:
// - Activity frequency calculations
// - Activity trend analysis
// - Goal progress calculations
// - Complex aggregation queries
// - Performance targets: <500ms for reporting queries

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::activities::repository::ActivityRepository;
use tempfile::TempDir;

fn setup_repo() -> (ActivityRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db =
        Arc::new(Database::new(temp_dir.path().to_path_buf()).expect("Failed to create database"));
    (ActivityRepository::new(db), temp_dir)
}

/// Helper to create realistic reporting test data
/// Returns (repo, temp_dir, activity_id)
fn setup_reporting_data(num_logs: usize) -> (ActivityRepository, TempDir, i32) {
    let (repo, temp_dir) = setup_repo();

    let group = repo
        .create_activity_group("Test Group", None)
        .expect("Failed to create group");

    let activity = repo
        .create_activity(group.id, "Test Activity", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Create logs spread over realistic time period
    // E.g., 1200 logs = ~10 per day over 120 days (4 months)
    let days_to_spread = (num_logs / 10).max(1);
    for i in 0..num_logs {
        let days_ago = (i / 10) as i64;
        if days_ago < days_to_spread as i64 {
            let logged_at = (chrono::Utc::now() - chrono::Duration::days(days_ago)).to_rfc3339();
            repo.log_activity(activity.id, &logged_at, None)
                .expect("Failed to log activity");
        }
    }

    (repo, temp_dir, activity.id)
}

fn bench_activity_frequency_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("activity_frequency_scaling");

    for size in [100, 500, 1200].iter() {
        let (repo, _temp_dir, activity_id) = setup_reporting_data(*size);

        let start_date = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
        let end_date = chrono::Utc::now().to_rfc3339();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                black_box(
                    repo.get_activity_frequency(activity_id, &start_date, &end_date)
                        .expect("Failed to get frequency"),
                );
            });
        });
    }

    group.finish();
}

fn bench_activity_trend_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("activity_trend_scaling");

    for size in [100, 500, 1200].iter() {
        let (repo, _temp_dir, activity_id) = setup_reporting_data(*size);

        let end_date = chrono::Utc::now().to_rfc3339();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                black_box(
                    repo.get_activity_trend(activity_id, 30, &end_date)
                        .expect("Failed to get trend"),
                );
            });
        });
    }

    group.finish();
}

fn bench_check_goal_progress_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_goal_progress_scaling");

    for size in [100, 500, 1200].iter() {
        let (repo, _temp_dir, activity_id) = setup_reporting_data(*size);

        // Set a goal for the activity
        let goal = repo
            .set_activity_goal(Some(activity_id), None, "days_per_period", 20, 30)
            .expect("Failed to set goal");

        let end_date = chrono::Utc::now().to_rfc3339();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                black_box(
                    repo.check_goal_progress(goal.id, &end_date)
                        .expect("Failed to check progress"),
                );
            });
        });
    }

    group.finish();
}

fn bench_frequency_different_periods(c: &mut Criterion) {
    let mut group = c.benchmark_group("frequency_different_periods");
    let (repo, _temp_dir, activity_id) = setup_reporting_data(1200);

    for period_days in [7, 30, 90, 365].iter() {
        let start_date = (chrono::Utc::now() - chrono::Duration::days(*period_days)).to_rfc3339();
        let end_date = chrono::Utc::now().to_rfc3339();

        group.bench_with_input(
            BenchmarkId::from_parameter(period_days),
            period_days,
            |b, &_size| {
                b.iter(|| {
                    black_box(
                        repo.get_activity_frequency(activity_id, &start_date, &end_date)
                            .expect("Failed to get frequency"),
                    );
                });
            },
        );
    }

    group.finish();
}

fn bench_trend_different_periods(c: &mut Criterion) {
    let mut group = c.benchmark_group("trend_different_periods");
    let (repo, _temp_dir, activity_id) = setup_reporting_data(1200);

    let end_date = chrono::Utc::now().to_rfc3339();

    for period_days in [7, 30, 90, 365].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(period_days),
            period_days,
            |b, &_size| {
                b.iter(|| {
                    black_box(
                        repo.get_activity_trend(activity_id, *period_days, &end_date)
                            .expect("Failed to get trend"),
                    );
                });
            },
        );
    }

    group.finish();
}

fn bench_goal_progress_both_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("goal_progress_both_types");
    let (repo, _temp_dir, activity_id) = setup_reporting_data(1200);

    let end_date = chrono::Utc::now().to_rfc3339();

    // Benchmark days_per_period goal type
    let goal_days = repo
        .set_activity_goal(Some(activity_id), None, "days_per_period", 20, 30)
        .expect("Failed to set days goal");

    group.bench_function("days_per_period", |b| {
        b.iter(|| {
            black_box(
                repo.check_goal_progress(goal_days.id, &end_date)
                    .expect("Failed to check progress"),
            );
        });
    });

    // Delete the days goal and set a percent improvement goal
    repo.delete_activity_goal(goal_days.id)
        .expect("Failed to delete goal");

    let goal_percent = repo
        .set_activity_goal(Some(activity_id), None, "percent_improvement", 25, 30)
        .expect("Failed to set percent goal");

    group.bench_function("percent_improvement", |b| {
        b.iter(|| {
            black_box(
                repo.check_goal_progress(goal_percent.id, &end_date)
                    .expect("Failed to check progress"),
            );
        });
    });

    group.finish();
}

fn bench_multiple_activities_reporting(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_activities_reporting");

    // Create realistic scenario: 20 activities with 60 logs each = 1200 total logs
    let (repo, _temp_dir) = setup_repo();
    let test_group = repo
        .create_activity_group("Test Group", None)
        .expect("Failed to create group");

    let mut activity_ids = Vec::new();
    for i in 1..=20 {
        let activity = repo
            .create_activity(test_group.id, &format!("Activity {}", i), None, None)
            .expect("Failed to create activity");
        activity_ids.push(activity.id);

        // Create 60 logs for this activity over 60 days
        for j in 0..60 {
            let logged_at = (chrono::Utc::now() - chrono::Duration::days(j)).to_rfc3339();
            repo.log_activity(activity.id, &logged_at, None)
                .expect("Failed to log");
        }
    }

    let start_date = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
    let end_date = chrono::Utc::now().to_rfc3339();

    // Benchmark: Get frequency for all activities (simulates dashboard view)
    group.bench_function("frequency_all_activities", |b| {
        b.iter(|| {
            for &activity_id in &activity_ids {
                black_box(
                    repo.get_activity_frequency(activity_id, &start_date, &end_date)
                        .expect("Failed to get frequency"),
                );
            }
        });
    });

    // Benchmark: Get trend for all activities
    group.bench_function("trend_all_activities", |b| {
        b.iter(|| {
            for &activity_id in &activity_ids {
                black_box(
                    repo.get_activity_trend(activity_id, 30, &end_date)
                        .expect("Failed to get trend"),
                );
            }
        });
    });

    group.finish();
}

fn bench_group_level_goals(c: &mut Criterion) {
    let (repo, _temp_dir) = setup_repo();
    let test_group = repo
        .create_activity_group("Test Group", None)
        .expect("Failed to create group");

    // Create 5 activities in the group
    for i in 1..=5 {
        let activity = repo
            .create_activity(test_group.id, &format!("Activity {}", i), None, None)
            .expect("Failed to create activity");

        // Create 240 logs (10 per day over 24 days)
        for j in 0..240 {
            let days_ago = (j / 10) as i64;
            let logged_at = (chrono::Utc::now() - chrono::Duration::days(days_ago)).to_rfc3339();
            repo.log_activity(activity.id, &logged_at, None)
                .expect("Failed to log");
        }
    }

    // Set a group-level goal
    let goal = repo
        .set_activity_goal(None, Some(test_group.id), "days_per_period", 50, 30)
        .expect("Failed to set group goal");

    let end_date = chrono::Utc::now().to_rfc3339();

    c.bench_function("group_level_goal_progress", |b| {
        b.iter(|| {
            black_box(
                repo.check_goal_progress(goal.id, &end_date)
                    .expect("Failed to check group progress"),
            );
        });
    });
}

criterion_group!(
    benches,
    bench_activity_frequency_scaling,
    bench_activity_trend_scaling,
    bench_check_goal_progress_scaling,
    bench_frequency_different_periods,
    bench_trend_different_periods,
    bench_goal_progress_both_types,
    bench_multiple_activities_reporting,
    bench_group_level_goals
);
criterion_main!(benches);
