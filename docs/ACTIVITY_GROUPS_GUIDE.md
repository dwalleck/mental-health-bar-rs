# Activity Groups User Guide

## Overview

Activity Groups help you organize and track related activities, set goals, and monitor progress across multiple dimensions of your life. This guide covers everything from basic organization to advanced goal tracking and reporting.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Creating Activity Groups](#creating-activity-groups)
3. [Managing Activities](#managing-activities)
4. [Setting Goals](#setting-goals)
5. [Tracking Progress](#tracking-progress)
6. [Reporting & Analytics](#reporting--analytics)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)

## Getting Started

### What are Activity Groups?

Activity Groups allow you to categorize activities into meaningful collections. For example:

- **🏃 Exercise**: Running, Gym, Yoga, Swimming
- **👥 Social**: Friends, Family Time, Community Events
- **💆 Self-Care**: Meditation, Reading, Journaling
- **💼 Work**: Deep Work, Meetings, Learning

### Why Use Activity Groups?

- **Organization**: Keep related activities together
- **Goal Setting**: Track progress across multiple activities
- **Reporting**: See aggregate statistics for activity categories
- **Simplified Logging**: Filter activities by group during mood check-ins

## Creating Activity Groups

### Step 1: Navigate to Groups

1. Open the application
2. Click **"Activities"** in the sidebar
3. Select **"Manage Groups"**

### Step 2: Create a New Group

1. Click **"+ New Group"** button
2. Enter a **name** (required, max 100 characters)
   - Good examples: "Exercise", "Social Activities", "Self-Care Routines"
   - Bad examples: "Stuff", "Group1", "Other"
3. Add a **description** (optional, max 500 characters)
   - Describe the purpose or types of activities in this group
   - Example: "Physical activities that promote cardiovascular health"
4. Click **"Save"**

### Tips for Naming Groups

- **Be specific but inclusive**: "Outdoor Activities" > "Things I Do Outside"
- **Use consistent themes**: "Physical Health", "Mental Health", "Social Health"
- **Consider your goals**: Name groups based on what you want to track

## Managing Activities

### Assigning Activities to Groups

**Method 1: During Activity Creation**
1. Go to **Activities → Create New**
2. Enter activity details (name, color, icon)
3. Select group from **"Group"** dropdown
4. Save

**Method 2: Edit Existing Activity**
1. Click on any existing activity
2. Click **"Edit"**
3. Change the **"Group"** dropdown
4. Save changes

**Method 3: Bulk Assignment** (if implemented)
1. Go to **Activities → Manage**
2. Select multiple activities (checkbox or Ctrl+Click)
3. Choose **"Assign to Group"** from actions menu
4. Select target group

### Moving Activities Between Groups

1. Click on the activity
2. Select **"Edit"**
3. Change the group dropdown to the new group
4. Save

**Note**: An activity can only belong to one group at a time. Moving an activity removes it from its previous group.

### Ungrouped Activities

Activities without a group assignment are shown in the "Ungrouped" filter. This is useful for:
- New activities you haven't categorized yet
- One-off activities that don't fit into existing groups
- Activities you're still deciding how to organize

## Setting Goals

### Goal Types

**1. Days per Period** - Track consistency
- Example: "Exercise 5 days per week"
- Example: "Social activity 10 days per month"
- Best for: Building habits, maintaining routines

**2. Percent Improvement** - Track growth
- Example: "Increase meditation by 25% compared to last period"
- Example: "Do 20% more social activities than previous month"
- Best for: Progressive improvement, challenging yourself

### Setting Activity-Level Goals

**When to use**: Track progress for a specific activity

1. Navigate to **Activities**
2. Select an activity (e.g., "Running")
3. Click **"Set Goal"**
4. Choose goal type:
   - **Days per Period**: Set target days (e.g., 5) and period (e.g., 7 days)
   - **Percent Improvement**: Set target percentage (e.g., 25%) and comparison period (e.g., 30 days)
5. Click **"Save Goal"**

**Example - Days per Period**:
```
Activity: Running
Goal Type: Days per Period
Target: 3 days
Period: 7 days
Result: Track "Run at least 3 times per week"
```

**Example - Percent Improvement**:
```
Activity: Meditation
Goal Type: Percent Improvement
Target: 20%
Period: 30 days
Result: "Meditate 20% more often than last 30 days"
```

### Setting Group-Level Goals

**When to use**: Track aggregate progress across multiple activities

1. Navigate to **Activity Groups**
2. Select a group (e.g., "Exercise")
3. Click **"Set Group Goal"**
4. Choose goal type and set parameters
5. Save

**Example**:
```
Group: Social Activities
Members: Friends, Family Time, Events, Volunteer Work
Goal: 15 days per 30 days
Result: Track "Do ANY social activity 15 times per month"
```

**Benefits of Group Goals**:
- Track variety (any activity in the group counts)
- More flexible than single-activity goals
- See aggregate progress across related activities

### Updating or Deleting Goals

**Update a Goal**:
1. Click on the activity or group with the goal
2. Click **"Edit Goal"**
3. Modify target value or period
4. Save changes

**Delete a Goal**:
1. Click on the activity or group
2. Click **"Delete Goal"**
3. Confirm deletion

**Note**: Deleting a goal doesn't delete historical progress data. You can re-create the goal later and continue tracking.

## Tracking Progress

### Viewing Goal Progress

**Dashboard View**:
- Navigate to **Charts → Activities Tab**
- See all active goals with visual progress bars
- Color coding:
  - 🟢 Green: Goal achieved (100%+)
  - 🟡 Yellow: In progress (50-99%)
  - 🔴 Red: Needs attention (<50%)

**Individual Goal View**:
1. Click on any activity or group
2. View detailed progress:
   - Current value (e.g., "12 days")
   - Target value (e.g., "15 days")
   - Percentage (e.g., "80%")
   - Status indicator

### Real-Time Progress Updates

Progress updates automatically when you:
- Log a new activity
- Edit or delete an activity log
- Change the goal parameters

No manual refresh needed!

### Progress Calculation

**Days per Period**:
```
Progress = (Days logged in period / Target days) × 100%

Example:
Target: 5 days per 7 days
Logged: 4 days this week
Progress: (4 / 5) × 100% = 80%
```

**Percent Improvement**:
```
Progress = (Current frequency / Previous frequency - 1) × 100%

Example:
Previous 30 days: 10 times
Current 30 days: 12 times
Improvement: (12 / 10 - 1) × 100% = 20%
Target: 25%
Progress: (20% / 25%) × 100% = 80% of goal
```

## Reporting & Analytics

### Activity Frequency Report

**Access**: Charts → Activities → Select Activity → View Frequency

**Shows**:
- Days per week (e.g., "4.2 days/week")
- Bar chart visualization
- Historical trends

**Use cases**:
- Monitor consistency over time
- Identify patterns (e.g., "I exercise more in summer")
- Validate goals (e.g., "Am I really exercising 5x/week?")

### Activity Trend Report

**Access**: Charts → Activities → Select Activity → View Trend

**Shows**:
- Percentage change (e.g., "+15%", "–8%", "→ stable")
- Arrow indicators:
  - ↑ Improving (positive change)
  - → Stable (change within ±5%)
  - ↓ Declining (negative change)
- Comparison periods: 7, 30, 90, 365 days

**Use cases**:
- See if you're improving over time
- Identify declining patterns early
- Celebrate progress with data

### Goal Progress Dashboard

**Access**: Charts → Activities → Goal Progress Tab

**Shows all active goals**:
- Activity name / Group name
- Goal type and target
- Current progress with visual bar
- Status indicators

**Features**:
- Sort by progress percentage
- Filter by status (achieved, in progress, needs attention)
- Quick navigation to individual activities

### Time Range Filters

All reports support multiple time ranges:
- **7 days**: Short-term patterns, weekly goals
- **30 days**: Monthly habits, medium-term trends
- **90 days**: Quarterly progress, seasonal patterns
- **365 days**: Annual review, long-term changes
- **All time**: Complete history

## Best Practices

### Organizing Groups

**Keep groups focused**:
✅ Good: "Cardio Exercise" (Running, Cycling, Swimming)
❌ Bad: "Exercise and Some Other Stuff" (Running, Cooking, Reading)

**Use 5-10 groups maximum**:
- Too few: Hard to organize activities
- Too many: Overwhelming, defeats the purpose

**Name groups by theme, not frequency**:
✅ Good: "Social Activities"
❌ Bad: "Things I Do Every Week"

### Setting Effective Goals

**Start with achievable targets**:
- Don't set "Exercise 7 days/week" if you currently do 2
- Aim for 20-30% improvement, not 200%

**Use specific periods**:
- Days per week goals: Use 7-day periods
- Monthly goals: Use 30-day periods
- Avoid arbitrary periods like 13 days

**Combine activity and group goals**:
- Activity goal: "Run 3x/week"
- Group goal: "Any exercise 5x/week"
- This allows flexibility while maintaining structure

### Logging Best Practices

**Log activities consistently**:
- Set a daily reminder
- Log immediately after completing activities
- Use the "Recent" filter to quickly find common activities

**Add notes for context**:
- Note unusual factors: "Felt tired", "Great energy"
- Track intensity: "Light jog", "Intense workout"
- Record environment: "Gym", "Outdoor", "Home"

**Review weekly**:
- Check progress every Sunday
- Adjust goals if consistently over/under-achieving
- Celebrate wins, identify improvement areas

## Troubleshooting

### "I don't see my group in the dropdown"

**Possible causes**:
1. Group was recently deleted
2. You're in the wrong section
3. Filtering is active

**Solutions**:
- Refresh the page (Ctrl/Cmd + R)
- Check "Manage Groups" to verify group exists
- Clear any active filters

### "Goal progress shows 0% but I've logged activities"

**Possible causes**:
1. Activities logged outside the goal period
2. Goal was just created (no historical data)
3. Wrong activity selected for group goal

**Solutions**:
- Check goal period vs. log dates
- Wait for new logs within the current period
- Verify which activities are in the group

### "Can't delete a group"

**Possible cause**: Group has activities assigned to it

**Solution**:
1. Reassign all activities to other groups
2. Or, delete activities first
3. Then delete the empty group

**Note**: Groups with activities cannot be deleted to prevent data loss. This is a safety feature.

### "Progress percentage seems wrong"

**Check**:
1. Goal type matches your intention
2. Period is correct (7 days vs. 30 days)
3. Historical data exists for comparison (percent improvement goals)

**For percent improvement goals**:
- Need data from previous period for comparison
- If you just started tracking, use "days per period" instead

## Performance

Activity Groups are highly optimized for performance (measured with Criterion benchmarks, 100 samples):

- **Group creation/update**: ~50 µs ± 3 µs (95% CI)
- **Activity retrieval (100 groups)**: ~34 µs ± 2 µs (95% CI)
- **Log retrieval (1000 logs)**: ~328 µs ± 15 µs (95% CI)
- **Reporting queries (1200 logs)**: ~160 µs ± 8 µs (95% CI)
- **Goal progress calculation**: ~198 µs ± 10 µs (95% CI)

All operations are **1000x-6000x faster** than target thresholds (<200ms for lists, <500ms for reporting).

Even with years of data, all operations remain instant and responsive.

### Benchmark Reports

Detailed performance analysis with charts and regression detection is available after running:
```bash
cd src-tauri && cargo bench
```

Reports are generated in:
- `src-tauri/target/criterion/*/report/index.html`
- Open any `index.html` file in your browser for interactive charts

The Criterion framework provides:
- Statistical analysis with confidence intervals
- Regression detection (flags performance degradations)
- Historical trend tracking
- Visual comparison charts

## FAQ

**Q: Can an activity belong to multiple groups?**
A: No, each activity belongs to exactly one group (or no group). This keeps organization simple and unambiguous.

**Q: What happens to logs if I delete an activity?**
A: Activity logs are soft-deleted and can be recovered. The data remains in the database with a `deleted_at` timestamp.

**Q: Can I have goals without using groups?**
A: Yes! You can set goals on individual activities without creating any groups.

**Q: How far back does historical data go?**
A: All data since you started using the app. There's no automatic data deletion or archiving.

**Q: Can I export my activity and goal data?**
A: Data export is planned for v0.2.0. Currently, data is stored locally in the SQLite database.

**Q: What happens if I change a goal mid-period?**
A: Progress resets based on the new goal parameters. Historical progress under the old goal is not retained.

## Support

- **Documentation**: [README.md](../README.md)
- **Database schema**: See [CLAUDE.md](../CLAUDE.md) for technical details
- **Issues**: Report bugs or request features via GitHub Issues
- **Performance**: See benchmark results in `target/criterion/` after running `cargo bench`

## Related Documentation

- [README.md](../README.md) - Quick start and overview
- [TESTING.md](../src-tauri/TESTING.md) - Test coverage for Activity Groups
- [DATA_MANAGEMENT.md](../src-tauri/docs/DATA_MANAGEMENT.md) - Database practices

---

**Version**: 1.0 (v0.1.0 release)
**Last Updated**: 2025-11-13
