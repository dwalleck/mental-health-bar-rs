# Critical Decisions Summary

**Date**: 2025-11-06
**Context**: Major Refactoring 2025 Planning

---

## Overview

This document captures the critical architectural decisions made during the planning phase, based on plan-reviewer feedback.

---

## ✅ Decisions Made

### 1. Database Migration Strategy
**Question**: How to handle existing `activities` table that conflicts with new schema?
**Decision**: **Option C - Drop and recreate** (Back up and drop existing table)

**Rationale**:
- No users exist yet - no data to preserve
- Clean migration without backward compatibility concerns
- Simplifies implementation

**Implementation**:
- Drop existing `activities` table in migration `003_activity_groups.sql`
- Create new schema with `activity_groups` and `activities` tables
- Update all code references

**Impact**:
- Simplified migration (no data preservation logic needed)
- Faster implementation
- No rollback complexity

---

### 2. Tailwind CSS Version
**Question**: Upgrade to Tailwind v4 beta now or wait for stable release?
**Decision**: **Option A - Upgrade immediately to v4 beta (Week 0 Validation)**

**Rationale**:
- **Pragmatic approach**: Validate v4 in Week 0 BEFORE building features
- Use v4 syntax from the start (avoid migration later)
- Matches Catalyst examples exactly
- Accept beta risks in exchange for no future migration work
- "Don't build for 10 weeks only to discover Tailwind v4 breaks everything" - Sam Rivera

**Risk Mitigation**:
- **Week 0 validation prevents late-stage failures**
- Pin specific v4 beta version (e.g., `4.1.13`)
- Tag release before upgrade with `git tag pre-tailwind-v4` (create rollback point)
- Backup `tailwind.config.js` to separate branch
- Test existing components still render
- Implement feature flags for quick disable if issues arise

**Implementation Timeline**:
- **Week 0, Validation Sprint**: First priority task (changed from Week 7)
- Install v4, test build, document rollback procedure
- **Success**: Build passes → proceed with v4 for all features
- **Failure**: Rollback to v3, stay on v3 syntax

**Impact**:
- All new components use v4 syntax from Week 1 onwards
- No syntax migration needed later
- Potential beta instability (accepted risk, validated upfront)
- If v4 fails in Week 0, rollback to v3 with zero wasted effort

---

### 3. Mood Scale Migration Strategy
**Question**: How to migrate mood ratings from 1-5 to 1-7 scale?
**Decision**: **Option A - Linear stretch mapping** (1→1, 2→3, 3→4, 4→5, 5→7)

**Rationale**:
- **No user data exists** - no actual migration needed currently
- Maintains even spacing across the scale (avoids gaps)
- Documented for future reference if migration ever needed

**Implementation**:
- Phase 3 (Week 6): Simple constraint update
- No data migration code needed
- Document mapping formula for future use

**SQL**:
```sql
-- Simple constraint update (no data to migrate)
ALTER TABLE mood_checkins DROP CONSTRAINT mood_rating_check;
ALTER TABLE mood_checkins ADD CONSTRAINT mood_rating_check CHECK (mood_rating BETWEEN 1 AND 7);
```

**Impact**:
- Clean implementation without migration complexity
- Future-proofed with documented migration strategy

---

## ⏳ Decisions Pending

These decisions are still needed but can be made during implementation:

### 4. Icon Library Choice
**Decision Status**: ✅ COMMITTED (Week 0 Validation)
**Decision**: **Heroicons**
**Validation**: Install and test in Week 0 (remove decision paralysis)
**Rationale**:
- Maintained by Tailwind Labs (matches Catalyst ecosystem)
- Good Svelte support with `@heroicons/svelte`
- Pragmatic approach: Commit early, validate it works, move on

### 5. Chart Library Choice
**Decision Status**: ✅ VALIDATED (Week 0 Validation)
**Decision**: **Chart.js**
**Validation**: Build working example in Week 0 (prove before building Dashboard)
**Rationale**:
- Simple API, good for standard charts
- Already familiar from v4.5.1 usage
- Week 0 validation proves Svelte 5 + Chart.js + Tauri integration works

### 6. Activity Group Limits
**Decision Needed By**: Week 3 (schema design)
**Options**: Hard limits, soft limits, no limits, user-tier limits
**Recommendation**: Soft limits via UI (20 groups, 50 activities/group)

### 7. Foreign Key Enforcement
**Decision Needed By**: Week 3 (before migrations)
**Options**: Per-migration PRAGMA, connection initialization, pre-migration check, test setup only
**Recommendation**: Database connection initialization (once, not per migration)

---

## 🚀 Strategic Decisions (Pragmatic Approach)

These decisions fundamentally changed the project approach from waterfall to iterative delivery.

### 8. Iterative Delivery Strategy
**Question**: Ship all features at once (Week 10) or incrementally (v0.1-v1.0)?
**Decision**: **Incremental Delivery with 5 Shippable Releases**

**Options Considered**:
1. **Waterfall Approach** (original plan)
   - Build all features → Test everything → Ship v1.0 in Week 10
   - Pros: Complete feature set, polished
   - Cons: No user value until Week 10, late feedback, high risk
2. **Iterative Approach** (chosen) ✅
   - Ship v0.1 (Week 1), v0.2 (Week 5), v0.3 (Week 5), v0.4 (Week 7), v1.0 (Week 9)
   - Pros: User value every 1-2 weeks, early feedback, risk mitigation
   - Cons: Requires disciplined scope management

**Rationale**:
- "Shipping beats planning. Real feedback beats theoretical design." - Sam Rivera
- Users see value in Week 1 (not Week 10)
- Each release validates assumptions before building more
- Continuous feedback loop improves outcomes

**Impact**:
- 5 shippable releases instead of 1 monolithic release
- Each release has defined "shippable criteria" (see tasks.md)
- Ship at 80%, iterate based on feedback

### 9. Test-Driven Development Approach
**Question**: Write tests after features (Week 2 catch-up) or during features (TDD)?
**Decision**: **True TDD - Tests Written AS Features Are Built**

**Options Considered**:
1. **Test Catch-Up Phase** (original plan)
   - Week 2 dedicated to writing tests for existing code
   - Pros: Ensures coverage eventually
   - Cons: Not true TDD, encourages "write tests later" mindset
2. **True TDD** (chosen) ✅
   - Write test → verify fail (red) → implement → verify pass (green) → refactor
   - Tests integrated into every feature task
   - Pros: Better design, confidence in changes, no technical debt
   - Cons: Requires discipline

**Rationale**:
- "If you're writing tests after code, you're not doing TDD" - Sam Rivera
- Week 2 test phase deleted entirely
- Tests are not an afterthought - they guide design

**Impact**:
- Faster overall delivery (no Week 2 catch-up)
- Better code quality (TDD improves design)
- Tests written AS features are built (no backlog)

### 10. Week 0 Validation Sprint
**Question**: Start building features immediately or validate risky assumptions first?
**Decision**: **Week 0 Validation Sprint - Prove Before Building**

**Rationale**:
- "Don't spend 10 weeks building only to discover Chart.js doesn't work" - Sam Rivera
- Validate riskiest technical assumptions BEFORE committing effort
- 1 day investment to prevent weeks of wasted work

**Week 0 Validation Tasks**:
1. Chart.js + Svelte 5 + Tauri integration (build working example)
2. Tailwind v4 beta upgrade (test build, rollback if needed)
3. Heroicons integration (install, test rendering)
4. End-to-end proof-of-concept (assessment → database → chart)

**Impact**:
- Confidence to proceed OR early pivot
- Risks validated upfront (not discovered in Week 7)
- Decision paralysis eliminated (commit to choices now)

---

## Impact Summary

### Timeline Changes (Pragmatic Approach)
- **Week 0 ADDED**: 1 day validation sprint (8 hours)
- **Week 2 DELETED**: Test coverage phase removed entirely (saves ~16 hours)
- **Week 1**: Dashboard v0.1 ships (user value in Week 1, not Week 10)
- **Weeks 2-5**: Activity Groups v0.2 ships (4 weeks, complete feature)
- **Week 5**: Check-In v0.3 ships (reduced from 30 to 26 hours - no data migration)
- **Weeks 6-7**: UI Refresh v0.4 ships (Tailwind v4 already validated in Week 0)
- **Weeks 8-9**: Production v1.0 ships
- **Overall**: 10 weeks + 1 day, 5 shippable releases (not 1 monolithic release)

### Risk Profile
- **Decreased Risk**: Week 0 validation prevents late-stage failures
- **Decreased Risk**: Iterative delivery allows early pivots
- **Decreased Risk**: No data migration failures possible (no users)
- **Increased Risk**: Tailwind v4 beta instability (validated upfront in Week 0)
- **Mitigation**: Feature flags, pinned versions, documented rollback, Week 0 validation

### Complexity Reduction
- No data backup/restore logic needed (no users)
- No backward compatibility testing
- No migration validation scripts
- No Week 2 test catch-up phase
- Simplified rollback procedures
- Decision paralysis eliminated (commit to choices in Week 0)

### Delivery Velocity Improvement
- **Before**: No user value until Week 10
- **After**: User value every 1-2 weeks (v0.1, v0.2, v0.3, v0.4, v1.0)
- **Before**: 1 monolithic release
- **After**: 5 shippable releases with feedback loops

---

## Key Constraints Acknowledged

### Confirmed: No User Data
- All migration concerns are theoretical/future-proofed
- Can iterate freely on schema design
- No production data to preserve or validate

### Accepted: Beta Software Risk
- Tailwind v4 is in beta (not production-ready per maintainers)
- Accept potential breaking changes
- Plan for quick rollback if critical bugs found

---

## Next Actions (Pragmatic Approach)

1. **Week 0** (Validation Sprint - FIRST PRIORITY):
   - Validate Chart.js + Svelte 5 + Tauri integration
   - Upgrade Tailwind v4 beta, test build, rollback if needed
   - Commit to Heroicons, install and test
   - Build end-to-end proof-of-concept
   - **Outcome**: Confidence to proceed OR early pivot

2. **Week 1** (v0.1 Dashboard - SHIP IT):
   - Build Dashboard with Chart.js (validated in Week 0)
   - Use Tailwind v4 syntax (validated in Week 0)
   - TDD approach: Write tests AS features are built
   - **Ship v0.1** at 80% completeness

3. **Weeks 2-5** (v0.2 Activity Groups - SHIP IT):
   - Execute database migration (drop activities table)
   - Set PRAGMA foreign_keys = ON in connection initialization
   - Define activity group limits (recommend soft limits)
   - TDD throughout
   - **Ship v0.2** when complete

4. **Week 5** (v0.3 Check-In v2.0 - SHIP IT):
   - Upgrade mood scale to 1-7 (simple constraint update)
   - Activity selection with icons (Heroicons validated in Week 0)
   - **Ship v0.3** at 80% completeness

5. **Weeks 6-7** (v0.4 UI Refresh - SHIP IT):
   - Migrate to Catalyst components (Tailwind v4 already validated)
   - Use v4 syntax throughout
   - **Ship v0.4** at 80% completeness

6. **Weeks 8-9** (v1.0 Production - SHIP IT):
   - Integration testing, bug fixes
   - Documentation, deployment prep
   - **Ship v1.0** to production

---

## References

- **Plan Document**: `major-refactoring-2025-plan.md`
- **Context Document**: `major-refactoring-2025-context.md`
- **Tasks Document**: `major-refactoring-2025-tasks.md`
- **Plan Review**: Conducted by `plan-reviewer` agent on 2025-11-06

---

## Revision History

- **2025-11-06 (Version 2.0 - Pragmatic Refactoring)**:
  - **Major Change**: Applied pragmatic development principles (Sam Rivera approach)
  - Updated Decision 2 (Tailwind): Week 0 validation instead of Week 7 upgrade
  - Updated Decision 4 (Heroicons): Committed in Week 0 (removed decision paralysis)
  - Updated Decision 5 (Chart.js): Validated in Week 0 (prove before building)
  - Added Decision 8: Iterative Delivery Strategy (5 shippable releases)
  - Added Decision 9: Test-Driven Development Approach (deleted Week 2 catch-up)
  - Added Decision 10: Week 0 Validation Sprint (validate risks before building)
  - Updated Impact Summary: Delivery velocity improvement, risk reduction
  - Updated Next Actions: Pragmatic approach with 5 releases
  - See: `PRAGMATIC-PRINCIPLES.md` for full philosophy

- **2025-11-06 (Version 1.0 - Initial Planning)**:
  - Initial decisions made (1, 2, 3)
  - Pending decisions documented (4, 5, 6, 7)
