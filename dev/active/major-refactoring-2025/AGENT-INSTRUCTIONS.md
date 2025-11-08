# Agent Instructions - Major Refactoring 2025

**Last Updated**: 2025-11-07
**Project**: Mental Health Tracking Application - Major Refactoring

---

## Overview

This document provides step-by-step instructions for agents working on the major refactoring project. Follow this workflow to select tasks, understand requirements, and implement features correctly.

---

## Architecture Documentation

Before starting any task, familiarize yourself with the relevant architecture documents:

### Core Planning Documents

**📋 REVISED-tasks.md** - **PRIMARY SOURCE OF TRUTH**
- Master task checklist organized by week and phase
- Each task section includes references to relevant architecture docs
- Mark tasks complete with `[X]` as you finish them
- **Always start here** to select your next task

**📖 REVISED-plan.md** - Development Plan
- High-level overview of the 10-week plan
- Phase descriptions and user value delivered
- Release strategy (v0.1, v0.2, v0.3, v0.4, v1.0)
- Context and rationale for major decisions

### Architecture Specifications

These documents provide detailed technical specifications. REVISED-tasks.md will tell you which ones to reference for each task.

**🏗️ svelte5-architecture.md** - State Management Patterns
- **When to use**: Week 1, 3, 5, 7-8 (any frontend component work)
- Runes vs stores decision tree
- Component patterns (`$state()`, `$derived()`, `$effect()`, `$props()`)
- File structure and naming conventions
- Migration strategy

**🎨 tailwind4-design-system.md** - Design Token Specifications
- **When to use**: Week 7-8 (UI component implementation)
- Complete Catalyst Zinc palette (50-950)
- Typography scale with text/line-height utilities
- Spacing tokens, shadows, border radius
- Dark mode CSS variables
- Complete CSS for Button, Input, Card, Badge, Alert, Modal

**🧩 catalyst-integration-spec.md** - Practical Integration Guide
- **When to use**: Week 7-8 (Catalyst component migration)
- Component priority matrix (build vs Melt UI vs defer)
- React → Svelte translation patterns
- Melt UI integration examples
- 70-80% visual parity checklist
- Page-by-page migration strategy

**🔧 component-architecture.md** - Component Patterns & Testing
- **When to use**: Week 1, 3, 4, 7, 10 (any component work)
- Directory structure (feature-based organization)
- Composition patterns (slots vs snippets vs props)
- Props validation (TypeScript + Zod)
- Testing conventions (describe blocks, mocking Tauri)
- Accessibility patterns (WCAG AA, keyboard nav, ARIA)

**📊 data-structures.md** - TypeScript Type Specifications
- **When to use**: Week 1 (prerequisite), then referenced throughout all weeks
- UI state types (FormState, ModalState, AsyncData)
- Domain types with UI metadata (ActivityGroupUI, GoalProgressUI)
- Component prop types (ButtonProps, InputProps)
- Chart data types (MoodTrendData, ActivityFrequencyData)

**🧮 algorithms.md** - Calculation Specifications
- **When to use**: Week 2 (goal progress), Week 4 (reporting dashboard)
- Goal progress calculation (days_per_period, percent_improvement)
- Activity frequency calculation (unique days, days per week)
- Activity trend analysis (percent change)
- Mood correlation with activities
- Report aggregation
- Chart data transformation

### Supporting Documents

**⚡ PRAGMATIC-PRINCIPLES.md** - Development Philosophy
- Ship early, ship often
- Validate risks before building
- TDD approach (write tests first)
- YAGNI principle

**📝 DECISION-LOG.md** / **DECISIONS.md** - Decision Tracking
- Critical decisions made during planning
- Rationale for technical choices
- Reference when questions arise

---

## Task Selection Workflow

### Step 1: Review Current Phase

Open `REVISED-tasks.md` and identify the current phase/week:

```markdown
## Phase 1 (v0.1): Activity Groups (Weeks 1-4) → SHIP IT

### Week 1: Database Schema and Repository Layer (22-25 hours)
```

Check the **Progress Tracking** section at the bottom to see what's already complete.

### Step 2: Select Next Incomplete Task

Tasks are organized sequentially within each week. Find the first unchecked task:

```markdown
- [ ] 1.1 Create migration file `003_activity_groups.sql`
```

**Important**:
- Tasks within a week may have dependencies - check if earlier tasks must be done first
- Read the entire week's task section to understand context
- Look for **"📖 Architecture Reference"** header at the top of each week

### Step 3: Read Architecture References

Before implementing, read the architecture docs referenced in that week's section:

```markdown
**📖 Architecture Reference**: See `data-structures.md` for TypeScript type specifications:
- `ActivityGroupFormData`, `ActivityFormData`, `ActivityGoalFormData`
- `FormState<T>`, `AsyncData<T>`, `ModalState<T>`
```

Open those files and review the relevant sections. They contain:
- Exact TypeScript type definitions
- Implementation patterns
- Code examples
- Testing conventions

### Step 4: Understand Task Requirements

Each task description includes:
- **What to build**: Component, function, or feature
- **Where to build it**: File paths specified
- **Acceptance criteria**: How to verify it's complete
- **Testing requirements**: Tests to write (TDD approach)

Example:
```markdown
- [ ] 1.11 Create `features/activities/models.rs` - Add ActivityGroup struct
```

This tells you:
- Create a new file: `src-tauri/src/features/activities/models.rs`
- Define the `ActivityGroup` struct
- Follow Rust naming conventions from CLAUDE.md

---

## Development Workflow

### Before Starting ANY Work

#### 1. Create Feature Branch

**CRITICAL**: Always create a new branch before starting development.

```bash
# From the main branch (or 001-mental-health-tracking)
git checkout 001-mental-health-tracking  # Ensure you're on the correct base branch

# Create feature branch with descriptive name
git checkout -b feature/week-1-database-schema

# Alternative naming patterns:
git checkout -b feature/activity-groups-db
git checkout -b feature/week-2-goals-reporting
git checkout -b feature/catalyst-ui-migration
```

**Branch Naming Convention**:
- Prefix: `feature/`
- Description: Use week number OR feature name
- Examples:
  - `feature/week-1-database-schema`
  - `feature/week-3-activity-ui`
  - `feature/activity-goals-repository`
  - `feature/catalyst-button-component`

#### 2. Verify Current State

```bash
# Check current branch
git branch

# Verify clean working directory
git status

# Pull latest changes (if working with others)
git pull origin 001-mental-health-tracking
```

### During Development

#### 1. Follow TDD Approach

Per project constitution and PRAGMATIC-PRINCIPLES.md:

```
Red → Green → Refactor
```

**For Backend (Rust)**:
1. Write integration test FIRST (red - test fails)
2. Implement repository method (green - test passes)
3. Refactor if needed
4. Verify test still passes

**For Frontend (Svelte)**:
1. Write component test FIRST
2. Implement component
3. Verify test passes
4. Test in browser manually

#### 2. Reference Architecture Docs

Keep architecture docs open while coding:
- Check `data-structures.md` for exact TypeScript types
- Check `component-architecture.md` for component patterns
- Check `algorithms.md` for calculation logic
- Check `tailwind4-design-system.md` for CSS classes

#### 3. Follow Project Guidelines

**Always reference**: `/CLAUDE.md` for:
- Rust coding standards
- Database patterns (connection management, deadlock prevention)
- Svelte 5 component guidelines
- Error handling patterns (`CommandError`, `ToCommandError`)
- Testing requirements

#### 4. Mark Tasks Complete

As you finish each task, update REVISED-tasks.md:

```markdown
- [X] 1.1 Create migration file `003_activity_groups.sql`
```

**Commit this change** so progress is tracked:

```bash
git add dev/active/major-refactoring-2025/REVISED-tasks.md
git commit -m "Mark task 1.1 complete - create migration file"
```

### After Completing Tasks

#### 1. Run Tests

```bash
# Backend tests
cargo test

# Frontend tests (if applicable)
npm run test:unit

# Build verification
cargo build
npm run build
```

#### 2. Verify Code Quality

```bash
# Rust
cargo clippy -- -D warnings
cargo fmt --check

# Frontend
npm run lint
npm run check
```

#### 3. Commit Your Work

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: implement activity groups database schema

- Create migration 003_activity_groups.sql
- Add activity_groups, activities, activity_logs, activity_goals tables
- Define foreign key relationships with CASCADE
- Add CHECK constraints and partial indexes
- Write rollback script

Closes task 1.1-1.10"
```

**Commit Message Format**:
- Prefix: `feat:`, `fix:`, `test:`, `docs:`, `refactor:`
- Summary: Brief description (50 chars)
- Body: Detailed explanation, reference task numbers
- Footer: Close related tasks

#### 4. Push Branch (Optional)

```bash
# Push feature branch to remote
git push -u origin feature/week-1-database-schema
```

---

## Common Workflows by Task Type

### Database Migration Tasks

**Files to reference**:
- `CLAUDE.md` § Database Development Guidelines
- `REVISED-tasks.md` for task details

**Steps**:
1. Create migration file in `src-tauri/migrations/`
2. Write SQL with proper constraints (CHECK, FK, NOT NULL)
3. Add partial indexes for soft deletes (`WHERE deleted_at IS NULL`)
4. Enable `PRAGMA foreign_keys=ON` in connection setup
5. Write rollback script
6. Test migration with `cargo test`

### Repository Method Tasks

**Files to reference**:
- `CLAUDE.md` § Database Access (DuckDB) [Note: Actually SQLite, but patterns apply]
- `data-structures.md` for return types
- `algorithms.md` if calculation logic involved

**Steps (TDD)**:
1. Write integration test in `#[cfg(test)]` module
2. Verify test FAILS (red)
3. Implement repository method using `_with_conn` pattern
4. Use parameterized queries (NEVER string interpolation)
5. Verify test PASSES (green)
6. Refactor if needed

### Tauri Command Tasks

**Files to reference**:
- `CLAUDE.md` § Error Handling
- `data-structures.md` for request/response types

**Steps**:
1. Implement command in `features/*/commands.rs`
2. Return `Result<T, CommandError>`
3. Implement `ToCommandError` for feature errors
4. Generate TypeScript bindings: `cargo test`
5. Write command tests for error handling

### Frontend Component Tasks

**Files to reference**:
- `svelte5-architecture.md` for patterns
- `component-architecture.md` for testing
- `data-structures.md` for prop types
- `tailwind4-design-system.md` for styling (Week 7-8)

**Steps**:
1. Create component file in `src/lib/components/`
2. Use `$state()` for local state, `$props()` for props
3. Follow accessibility guidelines (ARIA labels, keyboard nav)
4. Write component test with Testing Library
5. Test manually in browser

### UI Migration Tasks (Catalyst)

**Files to reference**:
- `tailwind4-design-system.md` for CSS classes
- `catalyst-integration-spec.md` for React→Svelte patterns
- `component-architecture.md` for composition

**Steps**:
1. Identify component to migrate (per priority matrix)
2. Extract Catalyst CSS to `src/styles/components/`
3. Create Svelte component with `$props()` pattern
4. Test all variants (solid/outline/plain × colors)
5. Migrate existing usages
6. Verify visual parity (70-80% checklist)

---

## Task Completion Checklist

Before marking a task complete, verify:

- [ ] **Code written** following project guidelines (CLAUDE.md)
- [ ] **Tests written** BEFORE implementation (TDD)
- [ ] **Tests passing** (`cargo test` or `npm run test:unit`)
- [ ] **Architecture docs followed** (referenced patterns used)
- [ ] **Type safety verified** (TypeScript/Rust compile without errors)
- [ ] **Linting passing** (`cargo clippy`, `npm run lint`)
- [ ] **Manual testing done** (if UI component)
- [ ] **Task marked complete** in REVISED-tasks.md
- [ ] **Changes committed** to feature branch

---

## Release Workflow

When completing a phase (e.g., end of Week 1 for v0.1):

### Pre-Release Checklist

- [ ] All tasks for the week/phase marked complete
- [ ] All tests passing
- [ ] Build successful (`cargo build && npm run build`)
- [ ] Manual testing of user workflows
- [ ] Update REVISED-tasks.md progress tracking section

### Create Release Branch

```bash
# Create release branch from feature branch
git checkout -b release/v0.1

# Merge all completed feature branches
git merge feature/week-1-database-schema

# Tag release
git tag -a v0.1 -m "Release v0.1: Activity Groups Database Schema"

# Push release
git push origin release/v0.1
git push origin v0.1
```

### Shippable Criteria

Verify the release meets criteria from REVISED-tasks.md:

**Example (v0.1)**:
- ✅ Users can create activity groups and activities
- ✅ Users can log activities with notes
- ✅ Tests pass, no P0 bugs
- ✅ Performance: Activity list loads <200ms

---

## Troubleshooting

### "I don't know which task to do next"
→ Open REVISED-tasks.md, find the current week, select first unchecked task

### "I don't understand the task requirements"
→ Read the architecture docs referenced in the week's header section

### "I don't know how to implement this"
→ Check CLAUDE.md for coding patterns, then architecture docs for specific patterns

### "Tests are failing"
→ Follow TDD: Write test first (should fail), implement (should pass), refactor

### "I forgot to create a feature branch"
→ Stash changes, create branch, pop stash:
```bash
git stash
git checkout -b feature/my-feature
git stash pop
```

### "Which architecture doc do I need?"
→ Look at the **"📖 Architecture Reference"** header in REVISED-tasks.md for that week

---

## Quick Reference

| Task Type | Primary Reference | Secondary Reference |
|-----------|------------------|---------------------|
| Database Migration | CLAUDE.md § Database | REVISED-tasks.md |
| Repository Method | CLAUDE.md § Database | data-structures.md, algorithms.md |
| Tauri Command | CLAUDE.md § Error Handling | data-structures.md |
| UI Component | svelte5-architecture.md | component-architecture.md |
| UI Migration | tailwind4-design-system.md | catalyst-integration-spec.md |
| Form Component | data-structures.md (FormState) | component-architecture.md |
| Chart Component | algorithms.md | data-structures.md |

---

## Summary

**The Agent Workflow**:

1. **Read** REVISED-tasks.md → Find current week → Select task
2. **Review** architecture docs referenced in that week's header
3. **Create** feature branch (`feature/week-X-description`)
4. **Implement** following TDD (test first, then code)
5. **Verify** tests pass, code quality checks pass
6. **Mark** task complete in REVISED-tasks.md
7. **Commit** changes with descriptive message
8. **Repeat** for next task

**Remember**:
- REVISED-tasks.md is the PRIMARY source of truth
- Architecture docs provide implementation details
- CLAUDE.md provides coding standards
- TDD is mandatory (tests written FIRST)
- Feature branches required BEFORE development
- Mark tasks complete as you go

---

## Questions?

If you encounter ambiguity or blockers:
1. Check DECISION-LOG.md for past decisions
2. Review PRAGMATIC-PRINCIPLES.md for philosophy
3. Ask user for clarification if still unclear

**Good luck shipping value! 🚀**
