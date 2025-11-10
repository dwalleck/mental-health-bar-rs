# Testing Guide

## Running Tests

### Windows

Due to Windows-specific DLL dependencies required by Tauri GUI plugins, running `cargo test` directly may fail with a `STATUS_ENTRYPOINT_NOT_FOUND` error. This is a known limitation with Windows GUI library linking in test environments.

**Recommended approach:**

```bash
cargo test-integration
```

This runs all integration tests (which cover all application functionality) while skipping the lib unit tests that require GUI dependencies.

**Alternative - Run specific test suites:**

```bash
# Run individual test suites
cargo test --test test_mood
cargo test --test test_activities
cargo test --test test_assessments
cargo test --test test_scheduling
cargo test --test test_visualization
cargo test --test test_database
```

### Linux/macOS

Standard cargo test should work without issues:

```bash
cargo test
```

## What's Being Tested

### Integration Tests (146+ tests)
- **Activities**: Activity groups, logging, goals, and reporting (22 tests)
- **Mood Tracking**: Mood check-ins, activities, and statistics (23 tests)
- **Assessments**: PHQ-9/GAD-7 assessments and history (14 tests)
- **Scheduling**: Reminder scheduling and execution (32 tests)
- **Visualization**: Chart data generation and statistics (18 tests)
- **Database**: Schema, migrations, and CRUD operations (11 tests)
- **Repository**: Cross-feature repository integration (7 tests)
- **Property-Based**: Quickcheck-based tests (9 tests)

### Unit Tests (100+ tests)
Located inline in source files, these test:
- Input validation and boundary conditions
- Data model transformations
- Query builder functionality
- Business logic edge cases

**Note**: On Windows, unit tests require special setup due to Tauri GUI dependencies and are skipped by the `test-integration` alias.

## Troubleshooting

### Error: STATUS_ENTRYPOINT_NOT_FOUND (0xc0000139)

This Windows error occurs when the test binary can't find required Windows API functions (specifically `TaskDialogIndirect` from Tauri dialog plugin).

**Solution**: Use `cargo test-integration` instead of `cargo test`.

### Error: TaskDialogIndirect could not be found

Same as above - use the integration test alias.

### Tests hanging or timing out

Check that no other process is locking database files:
```bash
# Kill any lingering test processes
taskkill /F /IM test_*.exe
```

## CI/CD Configuration

For GitHub Actions or other CI systems on Windows, use:

```yaml
- name: Run tests
  run: cargo test-integration
  working-directory: src-tauri
```

For Linux/macOS CI, standard `cargo test` works fine.
