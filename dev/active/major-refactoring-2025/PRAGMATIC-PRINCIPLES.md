# Pragmatic Development Principles
## Major Refactoring 2025 - Sam Rivera's Approach

**Created**: 2025-11-06
**Based On**: AGENT-PERSONA-PRAGMATIC-DEVELOPER.md (Sam Rivera)

---

## Core Philosophy

> **"Shipping beats planning. Real feedback beats theoretical design."**

This refactoring follows pragmatic development principles to deliver user value incrementally while managing technical risk.

---

## Key Principles Applied

### 1. Ship Early, Ship Often
**Principle**: Version 0.1 doesn't need to be perfect.

**Applied**:
- **Week 1**: Ship Dashboard v0.1 (basic charts) - users see value immediately
- **Week 5**: Ship Activity Groups v0.2 - complete but validated through usage
- **Week 6**: Ship Check-In v0.3 - improved UX iteration
- **Week 8**: Ship UI v0.4 - visual refresh with feedback

**Not**: Wait 10 weeks to ship "complete" system.

---

### 2. Validate Risks Before Building
**Principle**: If you're not sure it works, spike it first.

**Applied**:
- **Week 0**: Validation Sprint
  - Prove Chart.js works with Svelte 5 + Tauri (build example)
  - Upgrade Tailwind v4 beta NOW, validate or rollback
  - Commit to Heroicons (remove decision paralysis)
  - Build end-to-end proof-of-concept

**Not**: Discover Chart.js doesn't work in Week 2, or Tailwind v4 breaks in Week 7.

---

### 3. Make It Work, Make It Right, Make It Fast
**Principle**: In that order. Don't optimize before it works.

**Applied**:
- **v0.1**: Dashboard works with basic charts (make it work)
- **v0.2-v0.3**: Refine UX based on feedback (make it right)
- **v0.4-v1.0**: Optimize performance, polish UI (make it fast)

**Not**: Spend Week 1 designing perfect architecture without building.

---

### 4. Progressive Disclosure
**Principle**: Simple things simple, complex things possible.

**Applied**:
- **Activity Groups**: Start with complete feature (per requirements), but ship when done
- **Dashboard**: Basic charts first, advanced filtering later
- **Goals**: Included in v0.2 but optional UI pattern (power users discover it)

**Not**: Require users to learn complex goal-setting before logging an activity.

---

### 5. TDD Means Tests First
**Principle**: If you're writing tests after code, you're not doing TDD.

**Applied**:
- **Every feature**: Write test → verify fail (red) → implement → verify pass (green)
- Tests integrated into feature work (Week 3-10)
- No separate "catch-up on tests" phase

**Not**: Week 2 dedicated to writing tests for existing code.

---

### 6. YAGNI (You Aren't Gonna Need It)
**Principle**: Don't build features until someone asks.

**Applied**:
- **Activity Groups**: Full feature with goals/reporting (explicitly requested in new-features.md)
- **Dashboard**: Standard charts only (no custom visualizations yet)
- **UI Components**: Button, Input, Select, Card (what we use), not entire Catalyst library

**Not**: Build plugin architecture, API versioning, or advanced features "for later."

---

### 7. Developer Experience is a Feature
**Principle**: If it's hard to build, it's wrong.

**Applied**:
- **Clear tasks**: "Users can view trends" (user outcome) not "Create API endpoint #47"
- **Validation first**: Prove approach works before committing 20 hours
- **Feature flags**: Easy rollback if Tailwind v4 or Catalyst has issues

**Not**: Complex abstractions that slow down development.

---

### 8. If You Can't Explain It in the README, You've Failed
**Principle**: Complexity is the enemy of shipping.

**Applied**:
- **Each release**: Clear one-sentence milestone ("Dashboard v0.1 shipped")
- **Shippable criteria**: Defined per release (what makes v0.1 "done"?)
- **Simple architecture**: Vertical slices (models → repository → commands → UI)

**Not**: Phases described as "22 tasks complete" or "backend infrastructure ready."

---

## How This Changes The Plan

### Before (Waterfall)
```
Week 1-2: Build dashboard (no shipping)
Week 3-5: Build activity groups (no shipping)
Week 6: Migrate check-in (no shipping)
Week 7-8: Migrate UI (no shipping)
Week 9-10: Test everything, THEN ship v1.0
```
**Problem**: No user value until Week 10. Risks discovered late.

### After (Iterative)
```
Week 0: Validate risks (Chart.js, Tailwind v4, proof-of-concept)
Week 1: SHIP Dashboard v0.1 (users get value NOW)
Week 3-5: Build Activity Groups v0.2, SHIP when complete
Week 6: SHIP Check-In v0.3 (iteration on existing)
Week 7-8: SHIP UI v0.4 (visual refresh)
Week 9-10: Polish, SHIP v1.0 (production-ready)
```
**Benefit**: Users see value every 1-2 weeks. Risks validated Week 0.

---

## Shippable Release Criteria

### v0.1: Dashboard (Week 1)
**Definition of Done**:
- ✅ Users can view assessment history (PHQ-9, GAD-7, CES-D, OASIS)
- ✅ Users can see trend charts (line charts for scores over time)
- ✅ Basic date filtering works (last 30/60/90 days)
- ✅ Tests pass, no critical bugs
- ✅ Performance: Chart rendering <500ms

**User Value**: Users see their mental health trends visualized for first time.

---

### v0.2: Activity Groups (Week 5)
**Definition of Done**:
- ✅ Users can create activity groups (e.g., "Exercise", "Hobbies")
- ✅ Users can add activities to groups (e.g., "Hiking" → "Exercise")
- ✅ Users can log activities with notes
- ✅ Users can set goals (x days in y period, % improvement)
- ✅ Users see progress toward goals
- ✅ Reporting: days/week, % change from previous week
- ✅ Tests pass, no critical bugs
- ✅ Performance: Activity list loads <200ms

**User Value**: Users can track habits and see progress over time.

---

### v0.3: Check-In v2.0 (Week 6)
**Definition of Done**:
- ✅ Mood scale upgraded to 1-7 (from 1-5)
- ✅ Activities displayed grouped by Activity Group
- ✅ Activities shown with icons (not just names)
- ✅ Multi-select activities in check-in flow
- ✅ Tests pass, backward compatible with old check-ins
- ✅ Performance: Check-in form <100ms response

**User Value**: More granular mood tracking, better activity selection UX.

---

### v0.4: UI Refresh (Week 8)
**Definition of Done**:
- ✅ All components use Catalyst-inspired design
- ✅ Button, Input, Select, Card components migrated
- ✅ Tailwind v4 working across all pages
- ✅ Dark mode functional
- ✅ Accessibility score >90 (Lighthouse)
- ✅ Tests pass, visual consistency verified
- ✅ Performance: No regressions from UI changes

**User Value**: Modern, professional UI that matches design standards.

---

### v1.0: Production-Ready (Week 10)
**Definition of Done**:
- ✅ All features from v0.1-v0.4 integrated
- ✅ E2E tests pass (all user flows work)
- ✅ No P0/P1 bugs
- ✅ Documentation complete
- ✅ Performance targets met (<100ms UI, <500ms charts)
- ✅ Accessibility validated
- ✅ Ready for real users

**User Value**: Stable, polished mental health tracking application.

---

## Red Flags We're Avoiding

### ❌ Planning Without Building
**Sam would say**: "Why plan 22 dashboard tasks when you haven't proven Chart.js works?"

**We're doing**: Week 0 validation sprint - build working example FIRST.

---

### ❌ No Shippable Value Until The End
**Sam would say**: "10 weeks without shipping? That's waterfall, not agile."

**We're doing**: Ship v0.1 in Week 1, iterate every 1-2 weeks.

---

### ❌ Tests as an Afterthought
**Sam would say**: "If you're writing tests in Week 2 for Week 1 code, you're not doing TDD."

**We're doing**: Tests written AS features are built (red → green → refactor).

---

### ❌ Deferring Risk Validation
**Sam would say**: "Upgrading to Tailwind v4 beta in Week 7? What if it breaks? 6 weeks wasted."

**We're doing**: Validate in Week 0 - upgrade, test, commit or rollback.

---

### ❌ Building Complex Before Simple
**Sam would say**: "You're designing a 4-table schema before validating anyone wants activity tracking."

**We're doing**: Complete Activity Groups feature (per explicit requirements), but ship as complete unit.

---

## When To Ship vs. When To Iterate

### Ship It When:
- ✅ Core functionality works (even if basic)
- ✅ No P0 bugs (critical failures)
- ✅ Tests pass
- ✅ User can accomplish goal (view trends, track activities, etc.)
- ✅ Performance acceptable (doesn't feel broken)

### Don't Block Shipping For:
- ❌ "But it could be more polished"
- ❌ "We should add one more feature"
- ❌ "Let's refactor this first"
- ❌ "The UI could be prettier"
- ❌ "We need more test coverage" (if basics pass)

**Rule of Thumb**: Ship at 80%, iterate based on feedback.

---

## Quick Decision Framework

When faced with a choice, ask:

1. **Does this block shipping?**
   - Yes → Fix it now
   - No → Defer to next iteration

2. **Have we validated this works?**
   - Yes → Proceed
   - No → Spike first (2-4 hours max)

3. **Will users notice this?**
   - Yes → Prioritize
   - No → Defer or cut

4. **Can we iterate later?**
   - Yes → Ship minimal version now
   - No → Build it right the first time

---

## Validation Checklist (Week 0)

Before starting any feature work, validate:

- [ ] **Chart.js + Svelte 5 + Tauri**: Build working example (1 chart component)
- [ ] **Tailwind v4 beta**: Upgrade, test build, verify no breakage
- [ ] **Heroicons**: Install, render test icons, confirm integration
- [ ] **End-to-End Flow**: 1 assessment → 1 database entry → 1 chart display
- [ ] **Performance Baseline**: Measure current metrics (baseline for comparison)

**Time Budget**: 8 hours total (1 day)
**Outcome**: Confidence in technical approach OR pivot before wasting time

---

## Iteration Cadence

### Weekly Rhythm
- **Monday**: Plan week's work (what are we shipping?)
- **Tuesday-Thursday**: Build, test, iterate
- **Friday**: Demo, retrospective, document learnings

### Release Rhythm
- **Week 1**: v0.1 (Dashboard)
- **Week 5**: v0.2 (Activity Groups) - longer cycle due to complexity
- **Week 6**: v0.3 (Check-In)
- **Week 8**: v0.4 (UI Refresh)
- **Week 10**: v1.0 (Production)

---

## Success Metrics

### Shipping Velocity
- ✅ **Target**: Ship new feature every 1-5 weeks
- ✅ **Measure**: Count of v0.X releases
- ✅ **Goal**: 5 releases in 10 weeks

### User Value Delivered
- ✅ **Week 1**: Users can visualize trends (new capability)
- ✅ **Week 5**: Users can track activities (new capability)
- ✅ **Week 6**: Users get better check-in UX (improvement)
- ✅ **Week 8**: Users see modern UI (polish)

### Technical Quality
- ✅ **Tests**: Written with features (TDD approach)
- ✅ **Performance**: Targets met per release
- ✅ **Stability**: No P0 bugs at release time

---

## FAQ: Applying Pragmatic Principles

### Q: "Should I add this feature?"
**A**: Is it in the spec? Is it blocking a release? If no to both, defer it.

### Q: "Should I refactor this code?"
**A**: Does it block shipping? Does it cause bugs? If no, ship it and refactor later.

### Q: "Should I write more tests?"
**A**: Do critical paths pass? If yes, ship it. Add tests in next iteration if bugs found.

### Q: "Should I optimize this?"
**A**: Does it meet performance targets? If yes, ship it. Optimize in v0.4 (Week 8).

### Q: "Should I polish this UI?"
**A**: Does it work? If yes, ship it. Polish in v0.4 UI refresh.

---

## Pragmatic Mantras

1. **"Make it work"** before "make it pretty"
2. **"Ship it at 80%"** and iterate
3. **"Build one, throw away"** - prototypes teach lessons
4. **"Validate before you build"** - spike first
5. **"Tests are not optional"** - but they don't block v0.1
6. **"Users give better feedback than plans"** - ship early
7. **"Simple first"** - complex later (if needed)

---

## References

- **Persona**: AGENT-PERSONA-PRAGMATIC-DEVELOPER.md (Sam Rivera)
- **Plan**: major-refactoring-2025-plan.md
- **Context**: major-refactoring-2025-context.md
- **Tasks**: major-refactoring-2025-tasks.md

---

## Quick Reference Card

```
┌─────────────────────────────────────────┐
│  PRAGMATIC DEVELOPMENT CHECKLIST       │
├─────────────────────────────────────────┤
│ Before Starting:                        │
│  □ Validate riskiest assumptions        │
│  □ Build working example                │
│  □ Define "done" criteria               │
│                                         │
│ While Building:                         │
│  □ Write tests first (TDD)              │
│  □ Focus on user outcomes               │
│  □ Measure performance                  │
│                                         │
│ Before Shipping:                        │
│  □ Core functionality works             │
│  □ Critical tests pass                  │
│  □ No P0 bugs                           │
│  □ Meets performance targets            │
│                                         │
│ Decision Framework:                     │
│  • Blocks shipping? → Fix now          │
│  • Validated? → Proceed                │
│  • User-facing? → Prioritize           │
│  • Can iterate? → Ship minimal         │
│                                         │
│ Remember:                               │
│  "Ship it at 80% and iterate"          │
└─────────────────────────────────────────┘
```

---

**Last Updated**: 2025-11-06
**Next Review**: After each release (retrospective)
