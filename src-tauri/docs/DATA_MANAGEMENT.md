# Data Management Guide

This document explains how your mental health data is stored, managed, and deleted in this application.

## Data Storage

All your data is stored **locally on your device** in a DuckDB database file. Nothing is transmitted to external servers or cloud services.

### What We Store

1. **Assessment Responses** - Your completed PHQ-9, GAD-7, CES-D, and OASIS assessments
2. **Mood Check-ins** - Your daily mood ratings and associated activities
3. **Custom Activities** - Activities you create for mood tracking
4. **Assessment Schedules** - Your configured reminder settings

### Where Your Data Lives

- **Database Location**: `~/.local/share/mental-health-bar-rs/data.db` (Linux/macOS) or `%APPDATA%\mental-health-bar-rs\data.db` (Windows)
- **Permissions**: File is readable/writable only by your user account (600 permissions)
- **Backups**: We recommend periodically copying this file to a secure backup location

## Data Deletion Policies

Due to the sensitive nature of mental health data, we implement **protective deletion policies** to prevent accidental data loss.

### Assessment Types (PHQ-9, GAD-7, CES-D, OASIS)

**Protection**: Assessment types cannot be deleted if you have completed any assessments of that type.

**Why**: Your assessment history is precious mental health data. We prevent accidental loss.

**What happens if you try to delete**:
```
❌ Error: Cannot delete PHQ-9
   - 15 completed assessments exist
   - 2 active schedules configured

   To proceed: Delete all responses and schedules first,
   or export your data for safekeeping.
```

**Alternative**: Export your data instead of deleting (see Export section below).

### Assessment Responses

**Protection**: Individual assessment responses can be deleted.

**What happens when you delete an assessment response**:
- ✅ The specific response is permanently removed
- ✅ Your total assessment count decreases
- ✅ Charts and trends update to reflect remaining data
- ⚠️ **This action cannot be undone**

**Recommendation**: Export data before deleting if you might need it later.

### Mood Check-ins

**Cascade Behavior**: When you delete a mood check-in, associated activity links are also deleted.

**What happens when you delete a mood check-in**:
- ✅ The mood entry is removed
- ✅ All activity associations for that entry are removed
- ✅ Charts and statistics update automatically
- ⚠️ **This action cannot be undone**

**Important**: The activities themselves are not deleted, only their association with this specific mood check-in.

### Activities

**Soft Delete**: Activities are never permanently deleted to preserve historical accuracy.

**What happens when you "delete" an activity**:
- ✅ Activity disappears from selection lists for new mood check-ins
- ✅ Activity is marked as deleted (timestamp recorded)
- ✅ Historical mood check-ins still show the activity name
- ✅ Historical data displays "(deleted)" badge next to the activity
- ✅ **Can be restored** (future feature planned)

**Why soft delete**: Your past mood check-ins should always show which activities you selected, even if you later delete that activity.

**Example**:
```
Today: Delete "Exercise" activity
Result:
  - New mood check-ins: "Exercise" no longer appears in dropdown
  - Past mood check-ins: Still show "Exercise (deleted)"
  - Your historical data remains accurate and complete
```

### Assessment Schedules

**Protection**: Schedules can be deleted without restriction.

**What happens when you delete a schedule**:
- ✅ Reminders stop immediately
- ✅ No historical data is affected
- ✅ Can be recreated at any time

## Understanding Data Relationships

```
Assessment Types (PHQ-9, GAD-7, etc.)
    ↓
    └── Assessment Responses (your completed assessments)
            ↓
            └── PROTECTED: Cannot delete type if responses exist

Mood Check-ins
    ↓
    └── Activity Associations (which activities you selected)
            ↓
            └── CASCADE: Deleting mood check-in removes associations

Activities
    ↓
    └── Referenced by Mood Check-ins
            ↓
            └── SOFT DELETE: Activity hidden but preserved in history
```

## Complete Data Deletion (GDPR)

**Feature**: Delete all personal data from the application.

**Access**: Settings → Data Management → Delete All Data

**Process**:
1. Type "DELETE" to confirm (safety check)
2. All tables are dropped
3. Database schema is recreated (empty)
4. Application restarts with clean slate

**What is deleted**:
- ✅ All assessment responses
- ✅ All mood check-ins
- ✅ All custom activities
- ✅ All schedules
- ✅ **Everything - this is irreversible**

**What is NOT deleted**:
- ❌ Application settings (preferences, theme)
- ❌ Assessment type definitions (PHQ-9, GAD-7, etc. - these are restored from seed data)

**⚠️ WARNING**: Export your data before using this feature. There is no recovery.

## Data Export (Planned - v0.2.0)

Future versions will support exporting your data to CSV format for:
- Backup purposes
- Sharing with healthcare providers
- Data portability
- Analysis in other tools

**Export will include**:
- All assessment responses with timestamps
- All mood check-ins with activities
- Metadata (assessment types, thresholds)
- Format: Standard CSV (opens in Excel, Google Sheets, etc.)

## Data Retention

**Policy**: Data is retained indefinitely until you explicitly delete it.

**Automatic Deletion**: None. We never automatically delete your data.

**Recommendation**: Periodically review and export your data, especially:
- Before major application updates
- Before system migrations
- At regular intervals (monthly/quarterly) for backup

## Privacy & Security

**Local-Only Storage**:
- ✅ No cloud synchronization
- ✅ No network transmission
- ✅ No third-party access
- ✅ Data never leaves your device

**File Permissions**:
- ✅ Database file readable only by your user account
- ✅ 0600 permissions (Unix) or equivalent (Windows)
- ✅ Startup check verifies permissions

**Encryption** (Planned - v0.2.0):
- Future versions will support at-rest encryption
- Optional passphrase protection
- AES-256 encryption

## Troubleshooting

### "Cannot delete assessment type" error

**Cause**: You have completed assessments of this type.

**Solution**:
1. Export your assessment history first
2. Delete individual responses from Assessment History page
3. Try deleting the type again

**Alternative**: You don't need to delete assessment types - they're just categories.

### Deleted activity still shows in history

**Expected Behavior**: This is intentional. Historical mood check-ins preserve the activity name to maintain data accuracy.

**How to identify**: Look for the "(deleted)" badge next to the activity name.

### Want to recover deleted data

**Not Possible**: Deletion is permanent (except for soft-deleted activities).

**Prevention**:
1. Export data regularly
2. Copy database file to backup location
3. Double-check before confirming deletions

## Backup Recommendations

### Manual Backup

1. Close the application
2. Navigate to data directory (see "Where Your Data Lives" above)
3. Copy `data.db` file to secure location
4. Rename with date: `data-2025-10-20-backup.db`

### Backup Schedule

Recommended: Weekly or after significant data entry

**Before**:
- Major application updates
- System migrations
- Operating system upgrades

### Restoring from Backup

1. Close the application
2. Replace `data.db` with your backup file
3. Restart the application
4. Verify data integrity

## Questions?

- **Technical Details**: See `/docs/duckdb-practices.md` for developer documentation
- **Feature Requests**: Submit GitHub issue for export formats, encryption, etc.
- **Security Concerns**: See `/docs/SECURITY.md` for security checklist

---

**Last Updated**: 2025-10-20
**Version**: v0.1.0
**Related Documents**:
- [DuckDB Best Practices](/src-tauri/docs/duckdb-practices.md) (Developer documentation)
- [Cascading Delete Strategy](/specs/001-mental-health-tracking/plan.md#cascading-delete-strategy) (Architecture)
