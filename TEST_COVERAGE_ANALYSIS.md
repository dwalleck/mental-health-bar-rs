# Test Coverage Gap Analysis

**Generated**: 2025-10-27
**Purpose**: Identify missing test coverage in the Rust backend

## Summary

**Existing Tests**: ~95 tests across 5 files (1,678 total lines)
**Coverage Areas**: Assessments (7 tests), Mood (11 tests), Activities (17 tests), Visualization (18 tests), Repository Integration (9 tests)

## Critical Gaps Identified

### 1. **Command-Level Validation NOT Tested**

#### Assessment Commands (`submit_assessment`)
- ❌ Notes exceeding MAX_NOTES_LENGTH (10,000 chars)
- ❌ Notes containing invalid control characters (non-printable chars except \n, \t, \r)
- ❌ Assessment type code exceeding MAX_TYPE_CODE_LENGTH (10 chars)
- ❌ Assessment type code with non-alphanumeric characters
- ❌ Empty assessment type code
- ❌ Unknown assessment type after validation passes

**Current**: Only 1 test for scoring validation errors
**Needed**: ~6 additional tests for input validation edge cases

#### Mood Commands
- ❌ Notes exceeding 5000 characters
- ❌ Mood rating boundary values (0, 6, negative, very large)
- ❌ Activity IDs array with non-existent IDs (multiple at once)
- ❌ Empty activity IDs array (already tested implicitly)
- ❌ Very large activity IDs array (e.g., 100+ activities)

**Current**: 4 validation tests (invalid rating, invalid activity ID)
**Needed**: ~4 additional edge case tests

#### Activity Commands
- ❌ Icon field validation (max 20 chars) - NOT TESTED
- ❌ Color format validation edge cases:
  - Valid: `#RGB`, `#RRGGBB`, `#RRGGBBAA`
  - Invalid: `RGB`, `#GGG`, `#12345`, non-hex chars
- ❌ Update activity with all fields null
- ❌ Update activity to duplicate name of another active activity
- ❌ Concurrent activity creation with same name

**Current**: 17 tests (good coverage of CRUD + soft delete)
**Needed**: ~5 additional tests for validation edge cases

### 2. **Query Edge Cases NOT Tested**

#### Assessment Queries
- ❌ `get_assessment_history` with invalid date formats
- ❌ `get_assessment_history` with future dates
- ❌ `get_assessment_history` with limit=0 or negative
- ❌ `get_assessment_history` with limit exceeding data size
- ❌ `get_assessment_response` with negative ID
- ❌ `get_latest_assessment` when no assessments exist
- ❌ `get_latest_assessment` for specific type with no data

**Current**: 4 query tests (basic history, filtering, deletion)
**Needed**: ~7 additional edge case tests

#### Mood Queries
- ❌ `get_mood_stats` with no data
- ❌ `get_mood_stats` with single check-in
- ❌ `get_mood_history` with invalid date formats
- ❌ `get_mood_history` with limit=0
- ❌ Activity correlations with only 1 check-in per activity

**Current**: 3 tests (history filtering, get by ID, not found)
**Needed**: ~5 additional tests

### 3. **Error Handling NOT Tested**

#### Database Errors
- ❌ Connection/lock failures (difficult to test but critical)
- ❌ Transaction rollback scenarios
- ❌ Constraint violation errors
- ❌ Foreign key violations

**Current**: None
**Needed**: ~4 tests with mocked database failures

#### Concurrent Operations
- ❌ Simultaneous assessment submissions
- ❌ Simultaneous activity updates to same name
- ❌ Deleting activity while creating mood check-in
- ❌ Reading during write operations

**Current**: None
**Needed**: ~4 concurrency tests

### 4. **Boundary Conditions NOT Tested**

#### Numeric Boundaries
- ❌ Assessment scores at exact threshold boundaries
  - PHQ-9: 5, 10, 15, 20 (threshold edges)
  - GAD-7: 5, 10, 15 (threshold edges)
  - CES-D: 16, 22, 37 (threshold edges)
  - OASIS: 8, 15 (threshold edges)
- ❌ Mood ratings at boundaries (1, 5)
- ❌ Very large IDs (INT_MAX scenarios)

**Current**: Some boundary tests in models
**Needed**: ~8 additional boundary tests

#### String Length Boundaries
- ❌ Activity name at exactly 100 characters
- ❌ Activity name at 99, 100, 101 characters (boundary)
- ❌ Notes at exactly 10,000 characters (assessment)
- ❌ Notes at exactly 5,000 characters (mood)
- ❌ Icon at exactly 20 characters

**Current**: 1 test (name too long)
**Needed**: ~5 additional boundary tests

### 5. **Data Integrity NOT Tested**

#### Cascade Behavior
- ✅ Mood check-in deletion → junction table (TESTED in T093a)
- ✅ Assessment type deletion blocked when responses exist (TESTED)
- ✅ Assessment type deletion blocked when schedules exist (TESTED)
- ❌ Soft-deleted activities still referenced in historical mood check-ins (PARTIALLY TESTED)
- ❌ Activity deletion while mood check-in is being created

**Current**: 3 tests
**Needed**: ~2 additional tests

#### Data Consistency
- ❌ Assessment responses maintain correct timestamp order
- ❌ Mood check-ins maintain correct timestamp order
- ❌ Activity soft-delete timestamp accuracy
- ❌ Duplicate prevention (same assessment submitted twice quickly)

**Current**: None
**Needed**: ~4 tests

### 6. **Visualization Feature - MISSING TESTS**

#### Chart Data Queries (T118-T119, T136-T137)
- ❌ **CRITICAL**: Test file created but NOT INTEGRATED into build
  - File exists: `tests/test_visualization.rs`
  - NOT listed in `src/lib.rs` or `tests/` module
  - Tests never run by `cargo test`

**Action Required**:
1. Verify test file is discovered by cargo
2. Run tests to verify they pass
3. Fix any failing tests

**Current**: 18 tests written but NOT EXECUTED
**Needed**: Ensure tests are run and passing

### 7. **Model Validation Functions NOT Tested**

#### Mood Models
- ✅ `validate_mood_rating` (tested in integration tests)
- ✅ `validate_activity_name` (tested in activity tests)
- ✅ `validate_color` (tested partially)
- ❌ `validate_notes` - NOT DIRECTLY TESTED
- ❌ Icon validation function (if it exists) - NOT TESTED

**Current**: Implicit testing through integration
**Needed**: ~2 direct unit tests for validation functions

#### Assessment Models
- ✅ Scoring algorithms (PHQ-9, GAD-7, CES-D, OASIS) - WELL TESTED
- ✅ Severity level functions - TESTED
- ❌ Edge cases for invalid assessment types
- ❌ Response array validation edge cases

**Current**: Good coverage (17 tests in models.rs)
**Needed**: ~3 additional edge case tests

## Test Coverage Metrics

| Feature | Lines | Tests | Coverage Estimate | Priority |
|---------|-------|-------|------------------|----------|
| Assessments | ~1,500 | 7 integration + 17 unit | ~60% | HIGH |
| Mood | ~1,200 | 11 integration + 5 unit | ~70% | MEDIUM |
| Activities | ~800 | 17 integration | ~80% | LOW |
| Visualization | ~600 | 18 (NOT RUN) | 0% | **CRITICAL** |
| Commands | ~400 | 0 unit | ~30% (via integration) | HIGH |
| Queries | ~300 | 0 unit | ~40% (via integration) | MEDIUM |

**Overall Backend Coverage**: Estimated ~55-60%
**Target Coverage**: 80%+ for production readiness

## High-Priority Missing Tests (Add to tasks.md)

### Immediate (Critical)
1. **Fix visualization tests** - Ensure test_visualization.rs runs
2. **Command validation tests** - Notes length, control chars, type code format
3. **Query edge cases** - Invalid dates, boundary limits, empty results

### Short-Term (Important)
4. **Boundary value tests** - Exact threshold scores, max string lengths
5. **Error handling tests** - Database failures, transaction rollbacks
6. **Data consistency tests** - Timestamp ordering, duplicate prevention

### Long-Term (Nice to Have)
7. **Concurrency tests** - Simultaneous operations
8. **Performance tests** - Large datasets (1000+ records)
9. **Integration tests** - End-to-end command → query flows

## Recommended Test Additions to tasks.md

See next section for specific task additions.
