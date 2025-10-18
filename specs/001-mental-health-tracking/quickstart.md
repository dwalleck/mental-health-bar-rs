# Quickstart Guide: Mental Health Tracker

**Feature**: Mental Health Assessment and Tracking Application
**Stack**: Tauri (Rust + Svelte) + DuckDB
**Time to Hello World**: <5 minutes

## Prerequisites

- Rust (latest stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js 18+ and npm: `node --version` and `npm --version`
- Git: `git --version`

## Project Setup

### 1. Clone and Install

```bash
# Clone the repository
git clone https://github.com/yourusername/mental-health-bar-rs.git
cd mental-health-bar-rs

# Install frontend dependencies
npm install

# Install Rust dependencies (happens automatically on first build)
cd src-tauri
cargo build
cd ..
```

### 2. Run Development Server

```bash
# Start Tauri app in development mode
npm run tauri dev
```

The app should open in a native window. You're now running the mental health tracker!

---

## Feature Walkthrough

### Take Your First Assessment

**Goal**: Complete a PHQ-9 assessment in under 5 minutes

**Steps**:
1. Click "Assessments" in the sidebar
2. Select "PHQ-9" from the list
3. Answer all 9 questions using the 0-3 scale:
   - 0 = Not at all
   - 1 = Several days
   - 2 = More than half the days
   - 3 = Nearly every day
4. Click "Submit"
5. View your calculated score and severity level

**Expected Result**:
- Score displayed (0-27)
- Severity interpretation shown (e.g., "Mild depression")
- Assessment saved to history

**Code Flow**:
1. **Frontend** (`src/routes/assessments/[type]/+page.svelte`):
   - Displays questions from assessment content
   - Collects user responses in array
   - Calls Tauri command on submit

2. **Backend** (`src-tauri/src/features/assessments/commands.rs`):
   ```rust
   #[tauri::command]
   pub async fn submit_assessment(request: SubmitAssessmentRequest) -> Result<AssessmentResponse, String> {
       // Validate responses
       // Calculate score using assessment-specific algorithm
       // Save to DuckDB
       // Return response with score and severity
   }
   ```

3. **Database** (`assessment_responses` table):
   ```sql
   INSERT INTO assessment_responses (assessment_type_id, responses, total_score, severity_level)
   VALUES (1, '[0,1,2,1,0,1,2,3,1]', 11, 'moderate');
   ```

---

### Log Your First Mood Check-In

**Goal**: Log mood and activities in under 30 seconds

**Steps**:
1. Click "Mood" in the sidebar
2. Rate your current mood: 1 (Very Bad) to 5 (Very Good)
3. Select activities you've been doing (optional):
   - Click "+" to create a new activity (e.g., "Exercise", "Work")
   - Select existing activities by clicking them
4. Add optional notes
5. Click "Log Mood"

**Expected Result**:
- Mood check-in saved with timestamp
- Activities linked to check-in
- Success message shown

**Code Flow**:
1. **Frontend** (`src/routes/mood/+page.svelte`):
   ```svelte
   <script>
   import { invoke } from '@tauri-apps/api/core';

   let mood_rating = 3;
   let selectedActivities = [];

   async function submitMood() {
       const request = {
           mood_rating,
           activity_ids: selectedActivities.map(a => a.id),
           notes: $moodNotes
       };

       const result = await invoke('log_mood', { request });
       console.log('Mood logged:', result);
   }
   </script>

   <!-- Mood scale buttons (1-5) -->
   <!-- Activity selection -->
   <!-- Notes textarea -->
   <button on:click={submitMood}>Log Mood</button>
   ```

2. **Backend** (`src-tauri/src/features/mood/commands.rs`):
   ```rust
   #[tauri::command]
   pub async fn log_mood(request: LogMoodRequest, state: State<'_, AppState>) -> Result<MoodCheckin, String> {
       validate_mood_rating(request.mood_rating)?;

       let checkin_id = repository::create_mood_checkin(&request).await?;
       repository::link_activities(checkin_id, &request.activity_ids).await?;

       Ok(repository::get_mood_checkin(checkin_id).await?)
   }
   ```

---

### View Your Data in Charts

**Goal**: Visualize assessment trends in under 3 seconds

**Steps**:
1. Complete at least 2 assessments (same type, different dates)
2. Click "Charts" in the sidebar
3. Select assessment type (e.g., PHQ-9)
4. Choose time range (Week, Month, Quarter, Year, All Time)
5. View line chart with:
   - Scores plotted over time
   - Threshold lines for severity levels
   - Statistics (average, min, max, trend)

**Expected Result**:
- Line chart renders with all data points
- Hover over points to see exact score and date
- Threshold lines show severity boundaries (minimal/mild/moderate/severe)
- Trend indicator: "Improving", "Worsening", or "Stable"

**Code Flow**:
1. **Frontend** (`src/routes/charts/+page.svelte`):
   ```svelte
   <script>
   import { Line } from 'svelte-chartjs';
   import { invoke } from '@tauri-apps/api/core';

   let chartData;

   async function loadChartData() {
       const result = await invoke('get_assessment_chart_data', {
           code: 'PHQ9',
           timeRange: 'quarter',
           fromDate: null,
           toDate: null
       });

       chartData = {
           labels: result.data_points.map(p => new Date(p.timestamp)),
           datasets: [{
               label: result.assessment_type.name,
               data: result.data_points.map(p => p.value),
               borderColor: '#2196F3',
               tension: 0.3
           }]
       };
   }

   onMount(loadChartData);
   </script>

   <Line data={chartData} options={chartOptions} />
   ```

2. **Backend** (`src-tauri/src/features/visualization/queries.rs`):
   ```rust
   #[tauri::command]
   pub async fn get_assessment_chart_data(
       code: String,
       time_range: TimeRange,
       from_date: Option<String>,
       to_date: Option<String>,
       state: State<'_, AppState>
   ) -> Result<AssessmentChartData, String> {
       let (start, end) = resolve_time_range(time_range, from_date, to_date)?;

       let data_points = repository::get_assessment_history(&code, start, end).await?;
       let thresholds = repository::get_assessment_thresholds(&code).await?;
       let statistics = calculate_statistics(&data_points);

       Ok(AssessmentChartData {
           assessment_type: repository::get_assessment_type(&code).await?,
           data_points,
           thresholds,
           statistics
       })
   }
   ```

3. **Database Query** (DuckDB):
   ```sql
   SELECT
       completed_at,
       total_score,
       severity_level
   FROM assessment_responses
   WHERE assessment_type_id = (SELECT id FROM assessment_types WHERE code = 'PHQ9')
     AND completed_at >= '2025-07-01'
     AND completed_at <= '2025-10-15'
   ORDER BY completed_at ASC;
   ```

---

### Set Up Recurring Reminders

**Goal**: Configure weekly PHQ-9 reminder in under 2 minutes

**Steps**:
1. Click "Settings" in the sidebar
2. Navigate to "Schedules" tab
3. Click "Add Schedule"
4. Fill out form:
   - Assessment: PHQ-9
   - Frequency: Weekly
   - Day: Monday
   - Time: 09:00 AM
5. Click "Save"

**Expected Result**:
- Schedule created and shown in list
- Notification will appear on Mondays at 9 AM (when app is running)
- Clicking notification opens app to PHQ-9 assessment page

**Code Flow**:
1. **Frontend** (`src/routes/settings/+page.svelte`):
   ```svelte
   async function createSchedule() {
       const request = {
           assessment_type_code: 'PHQ9',
           frequency: 'weekly',
           time_of_day: '09:00',
           day_of_week: 1,  // Monday
           day_of_month: null
       };

       await invoke('create_schedule', { request });
       await loadSchedules();  // Refresh list
   }
   ```

2. **Backend** (`src-tauri/src/features/scheduling/commands.rs`):
   ```rust
   #[tauri::command]
   pub async fn create_schedule(request: CreateScheduleRequest, state: State<'_, AppState>) -> Result<AssessmentSchedule, String> {
       validate_schedule(&request)?;

       let schedule_id = repository::create_schedule(&request).await?;
       let schedule = repository::get_schedule(schedule_id).await?;

       // Start background scheduler if not running
       start_scheduler(state).await;

       Ok(schedule)
   }
   ```

3. **Background Task** (`src-tauri/src/features/scheduling/scheduler.rs`):
   ```rust
   pub async fn start_scheduler(state: Arc<AppState>) {
       tokio::spawn(async move {
           let mut interval = tokio::time::interval(Duration::from_secs(60));  // Check every minute

           loop {
               interval.tick().await;

               let due_schedules = repository::get_due_schedules(&state).await;
               for schedule in due_schedules {
                   send_notification(&state, &schedule).await;
                   repository::mark_triggered(schedule.id).await;
               }
           }
       });
   }
   ```

---

## Project Structure

```
mental-health-bar-rs/
├── src/                          # Svelte frontend
│   ├── lib/
│   │   ├── bindings.ts           # Auto-generated Tauri types
│   │   ├── components/
│   │   │   ├── ui/               # Shared UI components
│   │   │   ├── assessments/      # Assessment components
│   │   │   ├── mood/             # Mood components
│   │   │   └── charts/           # Chart components
│   │   └── stores/               # Svelte stores
│   └── routes/                   # SvelteKit routes
│       ├── +layout.svelte        # App shell
│       ├── +page.svelte          # Dashboard
│       ├── assessments/          # Assessment pages
│       ├── mood/                 # Mood pages
│       ├── charts/               # Chart pages
│       └── settings/             # Settings pages
│
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri app entry
│   │   ├── lib.rs                # Library exports
│   │   ├── db/                   # DuckDB connection & migrations
│   │   ├── config/               # confy configuration
│   │   ├── features/             # VERTICAL SLICES
│   │   │   ├── assessments/      # Assessment feature
│   │   │   │   ├── models.rs
│   │   │   │   ├── commands.rs
│   │   │   │   ├── queries.rs
│   │   │   │   └── repository.rs
│   │   │   ├── mood/             # Mood feature
│   │   │   ├── scheduling/       # Scheduling feature
│   │   │   └── visualization/    # Visualization feature
│   │   └── errors.rs             # Shared error types
│   └── tests/                    # Rust tests
│
└── specs/                        # Design documentation
    └── 001-mental-health-tracking/
        ├── spec.md               # Feature specification
        ├── plan.md               # This implementation plan
        ├── research.md           # Technology research
        ├── data-model.md         # Database schema
        ├── contracts/            # API contracts
        └── quickstart.md         # This file
```

---

## Development Workflow

### 1. Add a New Assessment Type

**Goal**: Add OASIS (Overall Anxiety Severity and Impairment Scale)

**Steps**:

1. **Add assessment content** (`src-tauri/src/features/assessments/content.rs`):
   ```rust
   pub const OASIS_QUESTIONS: &[(&str, &[&str])] = &[
       ("In the past week, how often have you felt anxious?", &[
           "No anxiety in the past week",
           "Infrequent anxiety",
           "Frequent anxiety",
           "Constant anxiety"
       ]),
       // ... 4 more questions
   ];

   pub fn get_oasis_questions() -> Vec<AssessmentQuestion> {
       OASIS_QUESTIONS.iter().enumerate().map(|(i, (text, options))| {
           AssessmentQuestion {
               number: i + 1,
               text: text.to_string(),
               options: options.iter().map(|s| s.to_string()).collect(),
           }
       }).collect()
   }
   ```

2. **Add scoring logic** (`src-tauri/src/features/assessments/models.rs`):
   ```rust
   pub fn calculate_oasis_score(responses: &[i32]) -> Result<i32, AssessmentError> {
       if responses.len() != 5 {
           return Err(AssessmentError::IncompleteResponses);
       }

       Ok(responses.iter().sum())  // Simple sum (0-20)
   }

   pub fn get_oasis_severity(score: i32) -> &'static str {
       match score {
           0..=7 => "minimal",
           8..=14 => "moderate",
           15..=20 => "severe",
           _ => "unknown"
       }
   }
   ```

3. **Seed database** (already done in migrations, but verify):
   ```sql
   INSERT INTO assessment_types (code, name, description, question_count, min_score, max_score, thresholds)
   VALUES ('OASIS', 'Overall Anxiety Severity and Impairment Scale', 'Brief anxiety assessment', 5, 0, 20,
           '{"minimal": 7, "moderate": 14, "severe": 20}');
   ```

4. **Test**:
   ```bash
   cd src-tauri
   cargo test test_oasis_scoring
   ```

5. **Frontend automatically works** (via tauri-specta type generation):
   - No changes needed! Types are auto-generated
   - OASIS appears in assessment list
   - Questions loaded dynamically from backend

---

### 2. Run Tests

```bash
# Backend tests
cd src-tauri
cargo test

# Frontend tests
cd ..
npm run test

# Integration tests
npm run test:integration
```

**Expected Output**:
```
running 24 tests
test features::assessments::tests::test_phq9_scoring ... ok
test features::mood::tests::test_mood_validation ... ok
test features::scheduling::tests::test_time_validation ... ok
...
test result: ok. 24 passed; 0 failed; 0 ignored
```

---

### 3. Build for Production

```bash
# Build optimized Tauri app
npm run tauri build
```

**Output**:
- **Linux**: `src-tauri/target/release/bundle/appimage/mental-health-tracker_0.1.0_amd64.AppImage`
- **macOS**: `src-tauri/target/release/bundle/dmg/mental-health-tracker_0.1.0_x64.dmg`
- **Windows**: `src-tauri/target/release/bundle/msi/mental-health-tracker_0.1.0_x64.msi`

---

## Common Tasks

### Add a New Tauri Command

1. **Define command** in feature module (`src-tauri/src/features/*/commands.rs`):
   ```rust
   #[tauri::command]
   #[specta::specta]  // Auto-generate TypeScript types
   pub async fn my_new_command(arg: String, state: State<'_, AppState>) -> Result<ReturnType, String> {
       // Implementation
   }
   ```

2. **Register command** (`src-tauri/src/lib.rs`):
   ```rust
   use crate::features::my_feature::commands::my_new_command;

   #[cfg_attr(mobile, tauri::mobile_entry_point)]
   pub fn run() {
       tauri::Builder::default()
           .invoke_handler(tauri::generate_handler![
               // ... existing commands
               my_new_command
           ])
           .run(tauri::generate_context!())
           .expect("error while running tauri application");
   }
   ```

3. **Generate TypeScript types**:
   ```bash
   cd src-tauri
   cargo test generate_types -- --exact
   ```

4. **Use in frontend** (`src/routes/somewhere/+page.svelte`):
   ```svelte
   <script>
   import { invoke } from '@tauri-apps/api/core';

   const result = await invoke('my_new_command', { arg: 'value' });
   </script>
   ```

### Add a New Svelte Page

1. **Create route file** (`src/routes/mypage/+page.svelte`):
   ```svelte
   <script>
   import { onMount } from 'svelte';

   onMount(() => {
       console.log('Page loaded');
   });
   </script>

   <h1>My New Page</h1>
   ```

2. **Add navigation link** (`src/routes/+layout.svelte`):
   ```svelte
   <nav>
       <a href="/">Dashboard</a>
       <a href="/assessments">Assessments</a>
       <a href="/mypage">My Page</a>
   </nav>
   ```

3. **Navigate**:
   - URL: `http://localhost:1420/mypage`
   - Or click link in nav

---

## Troubleshooting

### App Won't Start

**Problem**: `npm run tauri dev` fails

**Solutions**:
1. Check Rust installed: `rustc --version`
2. Check Node.js: `node --version` (need 18+)
3. Clean and rebuild:
   ```bash
   rm -rf node_modules src-tauri/target
   npm install
   npm run tauri dev
   ```

### Database Errors

**Problem**: `Failed to open database: permission denied`

**Solutions**:
1. Check app data directory exists:
   ```bash
   # Linux
   ls ~/.local/share/mental-health-tracker/

   # macOS
   ls ~/Library/Application\ Support/mental-health-tracker/

   # Windows
   dir %APPDATA%\mental-health-tracker\
   ```

2. Check file permissions:
   ```bash
   chmod 600 ~/.local/share/mental-health-tracker/data.db
   ```

3. Reset database (WARNING: deletes all data):
   ```bash
   rm ~/.local/share/mental-health-tracker/data.db
   # Restart app to recreate
   ```

### TypeScript Type Errors

**Problem**: `Property 'my_new_command' does not exist`

**Solution**: Regenerate types:
```bash
cd src-tauri
cargo test generate_types -- --exact
cd ..
npm run check  # Verify types
```

---

## Next Steps

1. **Implement P1 Features** (Core Assessments):
   - Complete PHQ-9 assessment flow
   - Add GAD-7, CES-D, OASIS
   - Test scoring algorithms against published guidelines

2. **Implement P2 Features** (Mood Tracking):
   - Mood check-in form
   - Activity CRUD operations
   - Mood history view

3. **Implement P3 Features** (Visualization & Scheduling):
   - Chart.js integration
   - Time-series queries in DuckDB
   - Background scheduler with notifications

4. **Testing & Polish**:
   - Write integration tests for each user story
   - Test on all platforms (Linux, macOS, Windows)
   - User acceptance testing

5. **Ship v0.1.0**:
   - Build production binaries
   - Create GitHub release
   - Gather feedback from real users

---

## Resources

- **Tauri Docs**: https://tauri.app/v2/
- **Svelte Docs**: https://svelte.dev/docs
- **DuckDB Rust Docs**: https://docs.rs/duckdb/
- **Chart.js Docs**: https://www.chartjs.org/docs/
- **Assessment Sources**:
  - PHQ-9: https://www.phqscreeners.com/
  - GAD-7: https://adaa.org/gad-7
  - CES-D: Public domain
  - OASIS: Public domain

---

## Ship It!

You now have a working mental health tracking application with:
- ✅ Standardized assessments (PHQ-9, GAD-7, CES-D, OASIS)
- ✅ Mood check-ins with activity tracking
- ✅ Data visualization via charts
- ✅ Recurring assessment reminders
- ✅ Local-only data storage (privacy-first)

**Time from clone to working app**: <5 minutes ✅

Follow the development workflow above to implement remaining features. Remember the constitution: **Ship It** → Get feedback → Iterate!
