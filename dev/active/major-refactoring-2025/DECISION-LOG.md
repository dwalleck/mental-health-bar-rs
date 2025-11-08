# Decision Log - Plan Review Resolutions

**Date**: 2025-11-07
**Reviewer**: plan-reviewer agent
**Decisions By**: Project Owner

---

## Critical Issues - RESOLVED ✅

### 1. Mood Scale Migration Data Loss Risk (P0) - RESOLVED

**Issue**: 1-5 → 1-7 migration strategy was mathematically flawed

**Decision**: ✅ **NO DATA MIGRATION NEEDED**
- **Rationale**: No users exist yet, no production data to migrate
- **Implementation**: Update CHECK constraint only:
  ```sql
  ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
  ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 7);
  ```
- **Impact**: Simplifies Week 5 tasks, removes migration complexity

**Plan Updates**:
- Remove Task 5.2 (data migration SQL)
- Keep Task 5.3 (constraint update)
- Remove Task 5.4 (test migration with existing data)
- Simplify Task 5.5 (rollback script - just reverse constraint)

---

### 2. Activity Groups Relationship Contradiction (P0) - RESOLVED

**Issue**: Spec said "one-to-one" but schema implemented "many-to-one"

**Decision**: ✅ **MANY-TO-ONE CONFIRMED**
- **Relationship**: Many activities can belong to one Activity Group
- **Cardinality**: An activity can belong to 0 or 1 group (not mandatory)
- **Implementation**:
  ```sql
  ALTER TABLE activities ADD COLUMN group_id INTEGER;
  FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE SET NULL;
  -- Allow NULL for ungrouped activities
  ```

**Plan Updates**:
- Update new-features.md line 14: Change "one-to-one" to "many-to-one"
- Confirm schema design in Task 1.3-1.4 is correct
- No code changes needed

---

### 3. Activity Name Validation Breaking Change (P1) - RESOLVED

**Issue**: Code uses 100 chars, spec requires 30 chars, plan-reviewer suggested data migration

**Decision**: ✅ **50 CHARACTER LIMIT, NO MIGRATION NEEDED**
- **New Limit**: 50 characters (split the difference)
- **Rationale**: No users exist, no production data to migrate
- **Implementation**: Update validation code only

**Plan Updates**:
- Task 6.24: Update to 50 chars (not 30)
- Remove Task 6.24a-6.24b (data migration tasks - not needed)
- Update error message: "Activity name must be 1-50 characters and cannot contain < > & \""

**Code Changes**:
```rust
// src-tauri/src/features/mood/models.rs
if trimmed.len() > 50 {  // Changed from 100 to 50
    return Err(MoodError::ActivityNameTooLong(trimmed.len()));
}
```

---

## Major Concerns - RESOLVED ✅

### 4. Activity Logs Missing Critical Fields (P1) - RESOLVED

**Issue**: activity_logs table missing created_at, deleted_at, notes constraint

**Decision**: ✅ **ADD MISSING FIELDS**

**Updated Schema** (Task 1.5):
```sql
CREATE TABLE activity_logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  activity_id INTEGER NOT NULL,
  logged_at TEXT NOT NULL,           -- When activity occurred
  created_at TEXT NOT NULL DEFAULT (datetime('now')), -- When log was created
  notes TEXT CHECK (length(notes) <= 500 OR notes IS NULL),
  deleted_at TEXT,                   -- Soft delete support
  FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
);

CREATE INDEX idx_activity_logs_activity ON activity_logs(activity_id);
CREATE INDEX idx_activity_logs_logged_at ON activity_logs(logged_at);
CREATE INDEX idx_activity_logs_deleted ON activity_logs(deleted_at);
```

**Plan Updates**:
- Update Task 1.5 with complete schema above
- Add Task 1.5a: Create indexes for performance

---

### 5. Catalyst UI Incompatibility Risk (P1) - RESOLVED

**Issue**: Catalyst is React-only, can't be directly ported to Svelte

**Decision**: ✅ **STYLE AND DESIGN TOKENS ONLY**
- **Scope**: Extract design tokens (colors, typography, spacing) and visual styling
- **Approach**: Recreate simple components in Svelte, use Melt UI for complex patterns
- **Parity Goal**: 70-80% visual similarity (not functional component port)

**Plan Updates**:
- Add clarification to Phase 4 intro:
  > **Note**: Catalyst is React-only. This phase extracts **design tokens and visual styling only**, not component logic. Complex interactive components will use Melt UI (Svelte's Headless UI equivalent).

- Update Week 7 tasks to clarify:
  - Task 7.1-7.5: Extract design tokens (colors, typography, spacing)
  - Task 7.7-7.10: Recreate visual styling in Svelte components (not port React logic)

- Adjust time estimates:
  - Week 7 Foundation: 4-6 hours → **6-8 hours** (research + extraction)
  - Week 7 Components: 10-12 hours → **14-18 hours** (Svelte recreation)

---

### 6. Test Coverage Math Issues (P2) - RESOLVED

**Issue**: Plan-reviewer claimed only ~160 tests, couldn't reach 80%

**Decision**: ✅ **851 TESTS EXIST - COVERAGE IS EXCELLENT**
- **Actual Count**: 851 individual test cases across 36 test files (frontend)
- **Backend Count**: 112 unit tests (#[test] annotations)
- **Total**: ~963 tests
- **Coverage**: Already substantial, Phase 5 expands edge cases only

**Plan Updates**:
- Update REVISED-plan.md Section "Current State Analysis":
  ```markdown
  ### Verified Complete ✅
  - **Testing**: ~963 tests total (851 frontend + 112 backend)
  ```

- Update Phase 5 description:
  > **Goal**: Expand test coverage to 80%+ with edge cases and integration tests. Current coverage is substantial (~963 tests), Phase 5 adds ~40-50 tests for edge cases, error paths, and concurrency.

- No changes to Phase 5 tasks needed - scope is appropriate

---

### 7. Other Concerns - RESOLVED ✅

#### 7a. Foreign Key Cascading Behavior (P2) - RESOLVED

**Decision**: ✅ **CASCADE DELETE**
- **Rationale**: If activity is hard-deleted (rare), logs should follow. Soft deletes (normal case) preserve logs via deleted_at column.
- **Implementation**:
  ```sql
  FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
  ```

**Plan Updates**:
- Task 1.5 already includes this (see decision #4 above)

---

#### 7b. Activity Icon Validation (P2) - RESOLVED

**Decision**: ✅ **OPTION A - HEROICON NAMES ONLY**
- **Format**: Heroicon v5 icon names (e.g., "academic-cap", "beaker", "heart")
- **Required**: Optional (can be NULL)
- **Default**: Display generic activity icon if NULL
- **Validation**: Check against known Heroicon list or allow any string (frontend validates)

**Implementation**:
```rust
// src-tauri/src/features/mood/models.rs
pub fn validate_icon(icon: &str) -> Result<(), MoodError> {
    if icon.len() > 50 {
        return Err(MoodError::ActivityIconTooLong(icon.len()));
    }

    // Allow any non-empty string (frontend validates against Heroicon names)
    if icon.trim().is_empty() {
        return Err(MoodError::InvalidIconFormat("Icon cannot be empty"));
    }

    Ok(())
}
```

**Frontend Validation** (Task 3.11):
```typescript
// IconPicker.svelte - validate against Heroicon v5 icon list
import * as HeroIcons from '@heroicons/svelte/24/outline';
const validIcons = Object.keys(HeroIcons);

function validateIcon(icon: string): boolean {
  return validIcons.includes(icon);
}
```

**Plan Updates**:
- Add Task 1.27a: Update icon validation to 50 char limit
- Add Task 3.11a: Create Heroicon name validator in IconPicker component
- Update Task 5.19: Use Heroicon component lookup (e.g., `HeroIcons[activity.icon]`)

---

#### 7c. Rollback Script Testing (P2) - RESOLVED

**Decision**: ✅ **SKIP ROLLBACK TESTING**
- **Rationale**: No users exist, no production data at risk
- **Approach**: Forward-only migrations for Phase 1-3
- **Documentation**: Note in plan that rollback capability is not tested

**Plan Updates**:
- Remove Task 1.10a (test rollback)
- Remove Task 5.5a (test rollback)
- Add note to Rollback Strategy section:
  > **Note**: Rollback scripts provided for reference only. Since no users exist during development, rollback testing is deferred. Migrations are forward-only until v1.0 ships.

---

#### 7d. Activity Group Limits (Minor) - RESOLVED

**Decision**: ✅ **UNLIMITED FOR NOW**
- **Approach**: No database constraints on group/activity counts
- **Future**: Can add soft limits in UI if performance issues arise
- **Monitoring**: Track performance during Week 4 testing with synthetic data

**Plan Updates**:
- Remove references to max group/activity limits
- Update Week 4 performance testing (Task 4.14-4.16) to test with large datasets (500+ groups/activities)
- Document in plan: "No limits enforced initially. Monitor performance and add UI soft limits if needed."

---

#### 7e. Goal Progress Notifications (Minor) - RESOLVED

**Decision**: ✅ **INCLUDE NOTIFICATIONS**
- **Feature**: Notify users when activity goals are achieved
- **Timing**: Add to Week 2 (backend) and Week 3 (frontend)
- **Implementation**: Use existing tauri-plugin-notification

**Plan Updates**:
- **Add Task 2.17a** (Week 2): Implement goal achievement detection in `check_goal_progress`
  ```rust
  // When goal is achieved (percentage >= 100%), trigger notification
  if progress.is_achieved {
      notification::send("Goal Achieved!",
                        format!("You've reached your {} goal!", activity.name));
  }
  ```

- **Add Task 3.23a** (Week 3): Wire up goal notification UI
  - Show toast notification when goal achieved
  - Option to dismiss or view goal details

**Estimated Addition**: +2 hours (1 hour backend, 1 hour frontend)

---

#### 7f. Drag-and-Drop Reordering (Minor) - RESOLVED

**Decision**: ✅ **DEFER TO v1.1**
- **Rationale**: Unsure how reordering impacts functionality, reduces scope for v0.1
- **Future**: Can add in post-MVP release if users request it

**Plan Updates**:
- Remove Task 3.8 (drag-and-drop reordering)
- Remove any references to `sort_order` column
- Add to "Future Enhancements" section:
  > **v1.1 Features**: Drag-and-drop reordering for activity groups (requires sort_order column and reorder command)

**Time Saved**: ~3-4 hours

---

## Updated Schema Designs

### activity_logs (Task 1.5 - Updated)

```sql
CREATE TABLE activity_logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  activity_id INTEGER NOT NULL,
  logged_at TEXT NOT NULL,           -- When activity occurred (ISO8601)
  created_at TEXT NOT NULL DEFAULT (datetime('now')), -- When log was created
  notes TEXT CHECK (length(notes) <= 500 OR notes IS NULL),
  deleted_at TEXT,                   -- Soft delete support
  FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
);

CREATE INDEX idx_activity_logs_activity ON activity_logs(activity_id);
CREATE INDEX idx_activity_logs_logged_at ON activity_logs(logged_at);
CREATE INDEX idx_activity_logs_deleted ON activity_logs(deleted_at) WHERE deleted_at IS NULL;
```

### activities (Task 1.3-1.4 - Confirmed Correct)

```sql
ALTER TABLE activities ADD COLUMN group_id INTEGER;

ALTER TABLE activities ADD CONSTRAINT fk_activities_group
  FOREIGN KEY (group_id) REFERENCES activity_groups(id) ON DELETE SET NULL;

-- Validates:
-- - Many activities can belong to one group
-- - Activities can be ungrouped (group_id IS NULL)
-- - Deleting a group ungroups its activities (SET NULL)
```

---

## Updated Time Estimates

| Phase | Original | Adjusted | Reason |
|-------|----------|----------|---------|
| Week 1 | 22-25 hours | **22-25 hours** | ✅ No change (schema updates minor) |
| Week 2 | 24-28 hours | **26-30 hours** | + 2 hours (goal notifications) |
| Week 3 | 22-26 hours | **19-23 hours** | - 3 hours (removed drag-and-drop) |
| Week 5 | 22-26 hours | **18-22 hours** | - 4 hours (no data migration) |
| Week 6 | 24-30 hours | **20-26 hours** | - 4 hours (no activity name migration) |
| Week 7-8 | 28-36 hours | **34-44 hours** | + 6 hours (Catalyst extraction effort) |
| **Total** | **200-236 hours** | **198-238 hours** | ±2 hours (essentially unchanged) |

**Timeline**: Still **10 weeks** (no extension needed)

---

## Updated Task List Changes

### Tasks to Add:
- Task 1.5a: Create indexes for activity_logs table
- Task 1.27a: Update icon validation to 50 char limit, allow any non-empty string
- Task 2.17a: Implement goal achievement notification
- Task 3.11a: Create Heroicon name validator in IconPicker
- Task 3.23a: Wire up goal notification UI (toast + view details)

### Tasks to Remove:
- Task 3.8: Drag-and-drop reordering (deferred to v1.1)
- Task 5.2: Mood scale data migration SQL (no users)
- Task 5.4: Test migration with existing data (no users)
- Task 6.24a-6.24b: Activity name data migration (no users)

### Tasks to Update:
- Task 1.5: Use updated activity_logs schema (see above)
- Task 5.3: Simplified constraint update (no data migration)
- Task 5.5: Simplified rollback (just reverse constraint)
- Task 6.24: Change to 50 chars (not 30)
- Task 7.1-7.10: Clarify Catalyst scope (design tokens only)

---

## Plan Status: APPROVED FOR IMPLEMENTATION ✅

All critical issues resolved. Plan is ready for implementation with the following updates:

**Required Before Week 1**:
- [ ] Update new-features.md (clarify many-to-one relationship)
- [ ] Update REVISED-plan.md with decisions above
- [ ] Update REVISED-tasks.md with task additions/removals
- [ ] Review updated schema designs (activity_logs)

**Estimated Update Time**: 1-2 hours to apply all changes to plan documents

**Go/No-Go**: ✅ **GO FOR IMPLEMENTATION**

---

## Next Steps

1. **Update Plan Documents** (1-2 hours)
   - Apply all decisions to REVISED-plan.md
   - Update REVISED-tasks.md with task changes
   - Update new-features.md for clarity

2. **Create Feature Branch**
   ```bash
   git checkout -b feature/activity-groups-2025
   ```

3. **Begin Phase 1, Week 1**
   - Start with Task 1.1: Create migration file 003_activity_groups.sql
   - Follow TDD approach throughout

---

## References

- Plan Review Report: From plan-reviewer agent (2025-11-07)
- Decision Authority: Project Owner
- Updated Documents: REVISED-plan.md, REVISED-tasks.md (pending)
