// Criterion benchmarks for Activity Logs operations
// Task 4.16: Test with 500 activity logs
//
// These benchmarks measure:
// - Logging activity performance
// - Retrieving logs with various filters
// - Scaling with 100, 500, and 1000 logs
// - Performance targets: <200ms for log list operations

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

/// Helper to create test data: 1 group with 1 activity and N logs
fn setup_with_n_logs(n: usize) -> (ActivityRepository, TempDir, i32) {
    let (repo, temp_dir) = setup_repo();

    let group = repo
        .create_activity_group("Test Group", None)
        .expect("Failed to create group");

    let activity = repo
        .create_activity(group.id, "Test Activity", Some("#4CAF50"), Some("🏃"))
        .expect("Failed to create activity");

    // Create N logs spread over time
    for i in 0..n {
        let days_ago = (i / 10) as i64; // ~10 logs per day
        let logged_at = (chrono::Utc::now() - chrono::Duration::days(days_ago)).to_rfc3339();
        repo.log_activity(activity.id, &logged_at, None)
            .expect("Failed to log activity");
    }

    (repo, temp_dir, activity.id)
}

fn bench_log_single_activity(c: &mut Criterion) {
    c.bench_function("log_single_activity", |b| {
        b.iter_batched(
            || {
                let (repo, temp_dir) = setup_repo();
                let group = repo
                    .create_activity_group("Test Group", None)
                    .expect("Failed to create group");
                let activity = repo
                    .create_activity(group.id, "Test Activity", None, None)
                    .expect("Failed to create activity");
                (repo, temp_dir, activity)
            },
            |(repo, _temp_dir, activity)| {
                let now = chrono::Utc::now().to_rfc3339();
                black_box(
                    repo.log_activity(activity.id, &now, None)
                        .expect("Failed to log activity"),
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_log_activity_with_notes(c: &mut Criterion) {
    c.bench_function("log_activity_with_notes", |b| {
        b.iter_batched(
            || {
                let (repo, temp_dir) = setup_repo();
                let group = repo
                    .create_activity_group("Test Group", None)
                    .expect("Failed to create group");
                let activity = repo
                    .create_activity(group.id, "Test Activity", None, None)
                    .expect("Failed to create activity");
                (repo, temp_dir, activity)
            },
            |(repo, _temp_dir, activity)| {
                let now = chrono::Utc::now().to_rfc3339();
                black_box(
                    repo.log_activity(
                        activity.id,
                        &now,
                        Some("This is a test note with some content"),
                    )
                    .expect("Failed to log activity"),
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_get_all_logs_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_all_logs_scaling");

    for size in [100, 500, 1000].iter() {
        let (repo, _temp_dir, _activity_id) = setup_with_n_logs(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                black_box(
                    repo.get_activity_logs(None, None, None)
                        .expect("Failed to get logs"),
                );
            });
        });
    }

    group.finish();
}

fn bench_get_logs_by_activity(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_logs_by_activity");

    for size in [100, 500, 1000].iter() {
        let (repo, _temp_dir, activity_id) = setup_with_n_logs(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                black_box(
                    repo.get_activity_logs(Some(activity_id), None, None)
                        .expect("Failed to get logs"),
                );
            });
        });
    }

    group.finish();
}

fn bench_get_logs_with_date_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_logs_with_date_filter");

    for size in [100, 500, 1000].iter() {
        let (repo, _temp_dir, _activity_id) = setup_with_n_logs(*size);

        // Filter to last 7 days
        let from_date = (chrono::Utc::now() - chrono::Duration::days(7)).to_rfc3339();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                black_box(
                    repo.get_activity_logs(None, Some(&from_date), None)
                        .expect("Failed to get logs"),
                );
            });
        });
    }

    group.finish();
}

fn bench_get_logs_recent_with_limit(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_logs_recent_with_limit");

    for size in [100, 500, 1000].iter() {
        let (repo, _temp_dir, _activity_id) = setup_with_n_logs(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                // Get all logs (no limit parameter available)
                black_box(
                    repo.get_activity_logs(None, None, None)
                        .expect("Failed to get logs"),
                );
            });
        });
    }

    group.finish();
}

fn bench_batch_log_activities(c: &mut Criterion) {
    // Note: This benchmark tests individual log_activity calls (no batching).
    // In production, bulk operations should use transactions for better performance.
    // See CLAUDE.md "Transaction Pattern (RAII)" for implementation guidelines.
    let mut group = c.benchmark_group("batch_log_activities");

    for batch_size in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let (repo, temp_dir) = setup_repo();
                        let test_group = repo
                            .create_activity_group("Test Group", None)
                            .expect("Failed to create group");
                        let activity = repo
                            .create_activity(test_group.id, "Test Activity", None, None)
                            .expect("Failed to create activity");
                        (repo, temp_dir, activity)
                    },
                    |(repo, _temp_dir, activity)| {
                        let now = chrono::Utc::now().to_rfc3339();
                        for _ in 0..size {
                            black_box(
                                repo.log_activity(activity.id, &now, None)
                                    .expect("Failed to log"),
                            );
                        }
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_log_single_activity,
    bench_log_activity_with_notes,
    bench_get_all_logs_scaling,
    bench_get_logs_by_activity,
    bench_get_logs_with_date_filter,
    bench_get_logs_recent_with_limit,
    bench_batch_log_activities
);
criterion_main!(benches);
