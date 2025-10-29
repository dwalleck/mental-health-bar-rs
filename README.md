# Mental Health Assessment & Tracking

A privacy-focused desktop application for tracking mental health through validated clinical assessments (PHQ-9, GAD-7, CES-D, OASIS), daily mood check-ins, and data visualization.

## ✨ Features

- 📋 **Clinical Assessments** - PHQ-9, GAD-7, CES-D, OASIS with accurate scoring
- 📊 **Data Visualization** - Track assessment trends and mood patterns over time
- 🎯 **Daily Mood Check-Ins** - Quick 1-5 mood rating with activity tracking
- 📈 **Dashboard Score Overview** - At-a-glance view of your current mental health status with color-coded progress bars showing severity levels for all assessment types
- 🔔 **Smart Reminders** - Configurable assessment schedules
- 🔒 **Privacy First** - All data stored locally, no cloud sync
- 📱 **Cross-Platform** - Linux, macOS, Windows

## 🚀 Quick Start (< 5 Minutes)

### Prerequisites

1. **Node.js** (v20+)
2. **Rust** (latest stable)
3. **System dependencies**: [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd mental-health-bar-rs

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Your First Assessment (2 minutes)

1. **Launch the app** - The application opens to the dashboard
2. **Select "Assessments"** from the sidebar
3. **Choose "PHQ-9"** (9-question depression screening)
4. **Answer all 9 questions** using the 0-3 scale
5. **Submit** - View your score with clinical interpretation

**Result**: You'll see your total score (0-27) with severity level (minimal/mild/moderate/severe) and interpretation guidance.

### Log Your Mood (30 seconds)

1. **Click "Mood Check-In"** from sidebar
2. **Rate your mood** - 1 (Very Bad) to 5 (Very Good)
3. **Select activities** (optional) - e.g., "Exercise", "Work"
4. **Submit** - Timestamp and mood saved

### View Your Trends

**Dashboard Quick View**:
1. **Open the application** - Dashboard displays automatically
2. **Check current scores** - See your most recent assessment scores with color-coded severity indicators
3. **Click any score bar** - Navigate directly to detailed charts for that assessment

**Detailed Chart View**:
1. **Navigate to "Charts"**
2. **Select assessment type** (PHQ-9, GAD-7, etc.)
3. **Choose time range** (7 days, 30 days, 90 days, all time)
4. **Interact with data points** - Hover for exact scores and dates

## 📖 Usage Examples

### Complete a GAD-7 Assessment

```typescript
// The app handles this via UI, but here's the data flow:
// 1. Load GAD-7 (7 questions about anxiety)
// 2. Answer each question (0-3 scale)
// 3. System calculates total score (0-21)
// 4. Display severity: 0-4 minimal, 5-9 mild, 10-14 moderate, 15-21 severe
```

### Create Custom Activities

1. Go to **Mood → Manage Activities**
2. Click **"Create New Activity"**
3. Enter name (max 100 chars) - e.g., "Meditation"
4. Choose color (hex format) and icon/emoji (optional)
5. Save - Activity available in mood check-ins

### Schedule Regular Assessments

1. **Settings → Assessment Schedules**
2. Select assessment type (e.g., PHQ-9)
3. Set frequency: Daily, Weekly, Biweekly, Monthly
4. Enable notifications
5. Save - Reminders sent automatically

## 🏗️ Architecture

- **Backend**: Rust (Tauri 2.x) with SQLite (rusqlite) for local persistence
- **Frontend**: Svelte 5 + SvelteKit with TailwindCSS
- **Charts**: Chart.js with threshold annotations
- **Testing**: Vitest (frontend), cargo test (backend) - 233 total tests
- **Type Safety**: tauri-specta for Rust ↔ TypeScript bindings

## 📊 Assessments Included

| Assessment | Questions | Scale | Purpose |
|------------|-----------|-------|---------|
| PHQ-9 | 9 | 0-27 | Depression screening |
| GAD-7 | 7 | 0-21 | Anxiety screening |
| CES-D | 20 | 0-60 | Depression (detailed) |
| OASIS | 5 | 0-20 | Anxiety severity |

All assessments use validated clinical scoring algorithms and standard interpretation thresholds.

## 🔒 Privacy & Security

- ✅ **Local-only storage** - No data leaves your device
- ✅ **No cloud sync** - Complete offline capability
- ✅ **Secure file permissions** - Database file restricted to user-only access (0600)
- ✅ **GDPR compliant** - Full data deletion available
- ✅ **No tracking** - No analytics or telemetry

**Note**: Database encryption at rest is planned for v0.2.0. Current version uses OS-level file permissions.

## 🧪 Development

### Run Tests

```bash
# Frontend tests (Vitest)
npm test

# Backend tests (Rust)
cd src-tauri && cargo test

# All tests
npm run test:all
```

### Build for Production

```bash
# Create production builds for all platforms
npm run tauri build

# Output:
# Linux: .deb, .AppImage
# macOS: .dmg, .app
# Windows: .msi, .exe
```

### Code Quality

```bash
# Lint frontend
npm run lint

# Format code
npm run format

# Check Rust code
cd src-tauri && cargo clippy -- -D warnings
```

## 📝 Success Criteria

- ✅ Complete PHQ-9 assessment in < 5 minutes (SC-001)
- ✅ Log mood check-in in < 30 seconds (SC-002)
- ✅ Create activity in < 15 seconds (SC-003)
- ✅ View charts in < 3 seconds (SC-005, SC-006)
- ✅ 100% accurate clinical scoring (SC-004)

## 🗺️ Roadmap

**v0.1.0 (Current)** - Core assessments, mood tracking, visualization
**v0.2.0** - Data export, backup/restore, enhanced charts
**v0.3.0** - Multi-language support, accessibility improvements

## 📄 License

[Your License Here]

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## 🆘 Support

- 📖 [Full Documentation](./specs/001-mental-health-tracking/quickstart.md)
- 🐛 [Report Issues](https://github.com/your-repo/issues)
- 💬 [Discussions](https://github.com/your-repo/discussions)

---

**⚠️ Disclaimer**: This application is a tracking tool only. It does not provide clinical diagnosis or treatment recommendations. Consult healthcare professionals for medical advice.
