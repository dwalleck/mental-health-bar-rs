// Criterion benchmarks for Activity Groups operations
// Task 4.15: Test with 50 activity groups
//
// These benchmarks measure:
// - CRUD operations on activity groups
// - Scaling with 10, 50, and 100 groups
// - Performance targets: <200ms for list operations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use tauri_sveltekit_modern_lib::db::Database;
use tauri_sveltekit_modern_lib::features::activities::repository::ActivityRepository;
use tempfile::TempDir;

/// Helper to set up fresh repository for each benchmark
fn setup_repo() -> (ActivityRepository, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db =
        Arc::new(Database::new(temp_dir.path().to_path_buf()).expect("Failed to create database"));
    (ActivityRepository::new(db), temp_dir)
}

/// Helper to create N groups
fn create_n_groups(repo: &ActivityRepository, n: usize) {
    for i in 1..=n {
        repo.create_activity_group(&format!("Group {}", i), Some(&format!("Description {}", i)))
            .expect("Failed to create group");
    }
}

fn bench_create_single_group(c: &mut Criterion) {
    c.bench_function("create_single_group", |b| {
        b.iter_batched(
            || setup_repo(),
            |(repo, _temp_dir)| {
                black_box(
                    repo.create_activity_group("Test Group", Some("Test Description"))
                        .expect("Failed to create group"),
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_get_groups_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_groups_scaling");

    for size in [10, 50, 100].iter() {
        let (repo, _temp_dir) = setup_repo();
        create_n_groups(&repo, *size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                black_box(repo.get_activity_groups().expect("Failed to get groups"));
            });
        });
    }

    group.finish();
}

fn bench_update_group(c: &mut Criterion) {
    c.bench_function("update_group", |b| {
        b.iter_batched(
            || {
                let (repo, temp_dir) = setup_repo();
                let group = repo
                    .create_activity_group("Original Name", Some("Original Description"))
                    .expect("Failed to create group");
                (repo, temp_dir, group)
            },
            |(repo, _temp_dir, group)| {
                black_box(
                    repo.update_activity_group(
                        group.id,
                        Some("Updated Name"),
                        Some("Updated Description"),
                    )
                    .expect("Failed to update group"),
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_delete_group(c: &mut Criterion) {
    c.bench_function("delete_group", |b| {
        b.iter_batched(
            || {
                let (repo, temp_dir) = setup_repo();
                let group = repo
                    .create_activity_group("Test Group", None)
                    .expect("Failed to create group");
                (repo, temp_dir, group)
            },
            |(repo, _temp_dir, group)| {
                black_box(
                    repo.delete_activity_group(group.id)
                        .expect("Failed to delete group"),
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_get_activities_by_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_activities_by_group");

    for num_activities in [5, 25, 50].iter() {
        let (repo, _temp_dir) = setup_repo();
        let test_group = repo
            .create_activity_group("Test Group", None)
            .expect("Failed to create group");

        // Create N activities in the group
        for i in 1..=*num_activities {
            repo.create_activity(
                test_group.id,
                &format!("Activity {}", i),
                Some("#4CAF50"),
                Some("✓"),
            )
            .expect("Failed to create activity");
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(num_activities),
            num_activities,
            |b, &_size| {
                b.iter(|| {
                    black_box(
                        repo.get_activities_by_group(test_group.id)
                            .expect("Failed to get activities"),
                    );
                });
            },
        );
    }

    group.finish();
}

fn bench_cascade_delete_group_with_activities(c: &mut Criterion) {
    let mut group = c.benchmark_group("cascade_delete_group_with_activities");

    for num_activities in [5, 25, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_activities),
            num_activities,
            |b, &size| {
                b.iter_batched(
                    || {
                        let (repo, temp_dir) = setup_repo();
                        let test_group = repo
                            .create_activity_group("Test Group", None)
                            .expect("Failed to create group");

                        // Create N activities
                        for i in 1..=size {
                            repo.create_activity(
                                test_group.id,
                                &format!("Activity {}", i),
                                Some("#4CAF50"),
                                None,
                            )
                            .expect("Failed to create activity");
                        }

                        (repo, temp_dir, test_group)
                    },
                    |(repo, _temp_dir, test_group)| {
                        black_box(
                            repo.delete_activity_group(test_group.id)
                                .expect("Failed to delete group"),
                        )
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
    bench_create_single_group,
    bench_get_groups_scaling,
    bench_update_group,
    bench_delete_group,
    bench_get_activities_by_group,
    bench_cascade_delete_group_with_activities
);
criterion_main!(benches);
