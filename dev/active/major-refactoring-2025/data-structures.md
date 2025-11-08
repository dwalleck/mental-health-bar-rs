# Data Structures Specification

**Last Updated:** 2025-11-07
**Status:** Phase 0 - Specifications
**Related:** svelte5-architecture.md, component-architecture.md

**Applies To Tasks:**
- **Week 1, Task 1.0** (Prerequisite): Create `src/lib/types/` with FormState, ModalState, AsyncData helpers
- **Week 1, Tasks 1.11-1.31**: Use typed interfaces for repository operations (ActivityGroupUI, ActivityUI)
- **Week 2, Tasks 2.1-2.17**: Use GoalProgressUI for goal calculations and display
- **Week 3, Tasks 3.1-3.23**: Use FormState for all forms, ModalState for dialogs, AsyncData for API calls
- **Week 5, Tasks 5.1-5.23**: Use MoodCheckinUI with 7-point scale helpers
- **Week 7, Tasks 7.7-7.13**: Define component prop types (ButtonProps, InputProps, etc.)

---

## Overview

This document specifies **all TypeScript interfaces and types** for the mental-health-bar-rs frontend before implementation. Following the catalyst-cli approach, we define data structures upfront to ensure:

- Type safety across the application
- Consistent data shapes
- Clear API contracts
- Reduced refactoring during implementation

**Scope:**
- UI state types (forms, modals, loading states)
- Frontend-specific domain types (with UI metadata)
- Component prop interfaces
- Chart data structures
- Utility types (common patterns)

**Note:** Backend types are auto-generated via `tauri-specta` in `src/lib/bindings.ts`. This spec covers **frontend-only** types.

---

## Table of Contents

1. [UI State Types](#ui-state-types)
2. [Domain Types (Frontend)](#domain-types-frontend)
3. [Component Prop Types](#component-prop-types)
4. [Chart Data Types](#chart-data-types)
5. [Utility Types](#utility-types)
6. [Type Organization](#type-organization)

---

## UI State Types

### Form State

**Purpose:** Standardize form state management across all forms

```typescript
// types/forms.ts

/**
 * Generic form state with data, errors, and submission status
 */
export interface FormState<T> {
  /** Form data */
  data: T
  /** Field-level errors (key = field name, value = error message) */
  errors: Record<string, string>
  /** Form-level error (e.g., API error) */
  submitError: string | null
  /** Whether form is currently submitting */
  isSubmitting: boolean
  /** Whether form has been modified */
  isDirty: boolean
  /** Whether form has been submitted (for validation timing) */
  isTouched: boolean
}

/**
 * Helper to create initial form state
 */
export function createFormState<T>(initialData: T): FormState<T> {
  return {
    data: initialData,
    errors: {},
    submitError: null,
    isSubmitting: false,
    isDirty: false,
    isTouched: false
  }
}

/**
 * Activity Group form data
 */
export interface ActivityGroupFormData {
  name: string
  description?: string
  color?: 'zinc' | 'blue' | 'green' | 'red' | 'yellow'
}

/**
 * Activity form data
 */
export interface ActivityFormData {
  name: string
  groupId: number
  icon?: string  // Heroicon name
}

/**
 * Activity Goal form data
 */
export interface ActivityGoalFormData {
  activityId?: number
  groupId?: number
  goalType: 'days_per_period' | 'percent_improvement'
  targetValue: number
  periodDays: number
}

/**
 * Mood Check-In form data
 */
export interface MoodCheckInFormData {
  moodRating: number  // 1-7 scale
  activityIds: number[]
  notes?: string
}
```

**Usage Example:**
```svelte
<script lang="ts">
  import type { FormState, ActivityGroupFormData } from '$lib/types/forms'
  import { createFormState } from '$lib/types/forms'

  let formState = $state<FormState<ActivityGroupFormData>>(
    createFormState({ name: '', description: '', color: 'zinc' })
  )

  function updateField(field: keyof ActivityGroupFormData, value: any) {
    formState.data[field] = value
    formState.isDirty = true
  }
</script>
```

---

### Modal State

**Purpose:** Standardize modal/dialog state management

```typescript
// types/modals.ts

/**
 * Generic modal state
 */
export interface ModalState<T = void> {
  /** Whether modal is open */
  isOpen: boolean
  /** Modal mode (create, edit, view, delete, etc.) */
  mode: 'create' | 'edit' | 'view' | 'delete' | 'confirm'
  /** Data associated with modal (e.g., item being edited) */
  data: T | null
}

/**
 * Helper to create initial modal state
 */
export function createModalState<T = void>(): ModalState<T> {
  return {
    isOpen: false,
    mode: 'create',
    data: null
  }
}

/**
 * Activity Group modal state
 */
export type ActivityGroupModalState = ModalState<ActivityGroup>

/**
 * Delete confirmation modal state
 */
export interface DeleteConfirmationState {
  isOpen: boolean
  itemType: string  // "activity group", "activity", etc.
  itemName: string
  itemId: number
  onConfirm: () => void | Promise<void>
}
```

**Usage Example:**
```svelte
<script lang="ts">
  import type { ActivityGroupModalState } from '$lib/types/modals'
  import { createModalState } from '$lib/types/modals'

  let modalState = $state<ActivityGroupModalState>(createModalState())

  function openEditModal(group: ActivityGroup) {
    modalState = {
      isOpen: true,
      mode: 'edit',
      data: group
    }
  }

  function closeModal() {
    modalState = createModalState()
  }
</script>
```

---

### Loading State

**Purpose:** Standardize async operation state management

```typescript
// types/loading.ts

/**
 * Generic loading state for async operations
 */
export type LoadingState = 'idle' | 'loading' | 'success' | 'error'

/**
 * Async data state with loading/error handling
 */
export interface AsyncData<T> {
  /** Current state */
  state: LoadingState
  /** Data (null if not loaded) */
  data: T | null
  /** Error message (null if no error) */
  error: string | null
}

/**
 * Helper to create initial async data state
 */
export function createAsyncData<T>(): AsyncData<T> {
  return {
    state: 'idle',
    data: null,
    error: null
  }
}

/**
 * Helper to set loading state
 */
export function setLoading<T>(state: AsyncData<T>): AsyncData<T> {
  return { state: 'loading', data: null, error: null }
}

/**
 * Helper to set success state
 */
export function setSuccess<T>(data: T): AsyncData<T> {
  return { state: 'success', data, error: null }
}

/**
 * Helper to set error state
 */
export function setError<T>(error: string): AsyncData<T> {
  return { state: 'error', data: null, error }
}
```

**Usage Example:**
```svelte
<script lang="ts">
  import type { AsyncData } from '$lib/types/loading'
  import { createAsyncData, setLoading, setSuccess, setError } from '$lib/types/loading'
  import { onMount } from 'svelte'

  let activityGroups = $state<AsyncData<ActivityGroup[]>>(createAsyncData())

  onMount(async () => {
    activityGroups = setLoading(activityGroups)
    try {
      const data = await invoke('get_activity_groups')
      activityGroups = setSuccess(data)
    } catch (err) {
      activityGroups = setError(String(err))
    }
  })
</script>

{#if activityGroups.state === 'loading'}
  <LoadingSpinner />
{:else if activityGroups.state === 'error'}
  <Alert type="error">{activityGroups.error}</Alert>
{:else if activityGroups.state === 'success' && activityGroups.data}
  <ActivityGroupList groups={activityGroups.data} />
{/if}
```

---

## Domain Types (Frontend)

### Activity Groups (with UI metadata)

**Purpose:** Extend backend types with UI-specific properties

```typescript
// types/activity-groups.ts
import type { ActivityGroup as BackendActivityGroup } from '$lib/bindings'

/**
 * Activity Group with UI metadata
 */
export interface ActivityGroupUI extends BackendActivityGroup {
  /** Whether group is expanded in UI */
  isExpanded?: boolean
  /** Number of activities in group (computed) */
  activityCount?: number
  /** Number of logs today (computed) */
  todayLogCount?: number
  /** Display color for UI */
  displayColor?: string
}

/**
 * Activity with UI metadata
 */
export interface ActivityUI extends Activity {
  /** Icon name (Heroicon) */
  icon?: string
  /** Whether activity was logged today (computed) */
  loggedToday?: boolean
  /** Last logged date (computed) */
  lastLoggedDate?: string
  /** Group name (denormalized for display) */
  groupName?: string
}

/**
 * Activity Log with UI metadata
 */
export interface ActivityLogUI extends ActivityLog {
  /** Activity name (denormalized) */
  activityName?: string
  /** Group name (denormalized) */
  groupName?: string
  /** Relative time display (e.g., "2 hours ago") */
  relativeTime?: string
}
```

---

### Goal Progress (with UI metadata)

**Purpose:** Goal progress data with display-ready calculations

```typescript
// types/goals.ts
import type { ActivityGoal as BackendGoal } from '$lib/bindings'

/**
 * Goal progress with UI display data
 */
export interface GoalProgressUI {
  /** Goal details */
  goal: BackendGoal
  /** Current progress (0-100+) */
  percentage: number
  /** Current value (e.g., 5 days) */
  currentValue: number
  /** Target value (e.g., 7 days) */
  targetValue: number
  /** Whether goal is achieved */
  isAchieved: boolean
  /** Progress status */
  status: 'not_started' | 'in_progress' | 'achieved' | 'exceeded'
  /** Display label */
  label: string  // "5 / 7 days"
  /** Progress bar color */
  color: 'zinc' | 'blue' | 'green' | 'yellow' | 'red'
}

/**
 * Helper to compute goal progress UI
 */
export function computeGoalProgressUI(
  goal: BackendGoal,
  currentValue: number
): GoalProgressUI {
  const percentage = (currentValue / goal.targetValue) * 100
  const isAchieved = percentage >= 100

  let status: GoalProgressUI['status']
  if (currentValue === 0) status = 'not_started'
  else if (percentage < 100) status = 'in_progress'
  else if (percentage === 100) status = 'achieved'
  else status = 'exceeded'

  let color: GoalProgressUI['color']
  if (percentage < 50) color = 'red'
  else if (percentage < 75) color = 'yellow'
  else if (percentage < 100) color = 'blue'
  else color = 'green'

  const label = goal.goalType === 'days_per_period'
    ? `${currentValue} / ${goal.targetValue} days`
    : `${percentage.toFixed(0)}%`

  return {
    goal,
    percentage,
    currentValue,
    targetValue: goal.targetValue,
    isAchieved,
    status,
    label,
    color
  }
}
```

---

### Mood Check-In (with UI metadata)

**Purpose:** Mood check-in data with display helpers

```typescript
// types/mood.ts
import type { MoodCheckin as BackendMoodCheckin } from '$lib/bindings'

/**
 * Mood rating with UI display
 */
export interface MoodRatingUI {
  /** Rating value (1-7) */
  value: number
  /** Display label */
  label: string  // "Terrible", "Bad", "Poor", "Neutral", "Good", "Great", "Excellent"
  /** Emoji representation */
  emoji: string
  /** Color for UI */
  color: string  // CSS color class
}

/**
 * Mood rating scale (1-7)
 */
export const MOOD_SCALE: MoodRatingUI[] = [
  { value: 1, label: 'Terrible', emoji: '😭', color: 'red-600' },
  { value: 2, label: 'Bad', emoji: '😢', color: 'orange-600' },
  { value: 3, label: 'Poor', emoji: '😟', color: 'yellow-600' },
  { value: 4, label: 'Neutral', emoji: '😐', color: 'zinc-600' },
  { value: 5, label: 'Good', emoji: '🙂', color: 'lime-600' },
  { value: 6, label: 'Great', emoji: '😊', color: 'green-600' },
  { value: 7, label: 'Excellent', emoji: '😄', color: 'emerald-600' }
]

/**
 * Mood Check-In with UI metadata
 */
export interface MoodCheckinUI extends BackendMoodCheckin {
  /** Rating UI details */
  ratingUI?: MoodRatingUI
  /** Activity names (denormalized) */
  activityNames?: string[]
  /** Relative time (e.g., "2 hours ago") */
  relativeTime?: string
}

/**
 * Helper to get mood rating UI
 */
export function getMoodRatingUI(value: number): MoodRatingUI {
  return MOOD_SCALE.find(r => r.value === value) || MOOD_SCALE[3]  // Default to neutral
}
```

---

## Component Prop Types

### Button Props

```typescript
// components/ui/Button.types.ts

export type ButtonVariant = 'solid' | 'outline' | 'plain'
export type ButtonColor = 'zinc' | 'blue' | 'red' | 'green'
export type ButtonSize = 'sm' | 'md' | 'lg'

export interface ButtonProps {
  /** Button variant */
  variant?: ButtonVariant
  /** Button color */
  color?: ButtonColor
  /** Button size */
  size?: ButtonSize
  /** Button type */
  type?: 'button' | 'submit' | 'reset'
  /** Disabled state */
  disabled?: boolean
  /** Click handler */
  onclick?: () => void
  /** Button content */
  children: any
}
```

---

### Input Props

```typescript
// components/ui/Input.types.ts

export type InputType = 'text' | 'email' | 'password' | 'number' | 'tel' | 'url'

export interface InputProps {
  /** Input label */
  label?: string
  /** Input type */
  type?: InputType
  /** Input value */
  value?: string | number
  /** Placeholder text */
  placeholder?: string
  /** Disabled state */
  disabled?: boolean
  /** Required field */
  required?: boolean
  /** Error message */
  error?: string
  /** Helper text */
  helper?: string
  /** Max length */
  maxlength?: number
  /** Min value (for number inputs) */
  min?: number
  /** Max value (for number inputs) */
  max?: number
}
```

---

### Card Props

```typescript
// components/ui/Card.types.ts

export interface CardProps {
  /** Card padding size */
  padding?: 'compact' | 'default' | 'comfortable'
  /** Custom class */
  class?: string
  /** Card header slot */
  header?: any
  /** Card body slot */
  body: any
  /** Card footer slot */
  footer?: any
}
```

---

### Badge Props

```typescript
// components/ui/Badge.types.ts

export type BadgeColor = 'zinc' | 'blue' | 'green' | 'red' | 'yellow'
export type BadgeSize = 'sm' | 'md' | 'lg'

export interface BadgeProps {
  /** Badge color */
  color?: BadgeColor
  /** Badge size */
  size?: BadgeSize
  /** Badge content */
  children: any
}
```

---

### Alert Props

```typescript
// components/ui/Alert.types.ts

export type AlertType = 'info' | 'success' | 'warning' | 'error'

export interface AlertProps {
  /** Alert type */
  type: AlertType
  /** Alert title */
  title?: string
  /** Alert message */
  children: any
  /** Dismissible */
  dismissible?: boolean
  /** Dismiss handler */
  onDismiss?: () => void
}
```

---

## Chart Data Types

### Chart.js Data Structures

```typescript
// types/charts.ts
import type { ChartData, ChartOptions } from 'chart.js'

/**
 * Mood trend chart data
 */
export interface MoodTrendData {
  /** Chart data */
  chartData: ChartData<'line'>
  /** Chart options */
  chartOptions: ChartOptions<'line'>
}

/**
 * Activity frequency chart data
 */
export interface ActivityFrequencyData {
  /** Chart data */
  chartData: ChartData<'bar'>
  /** Chart options */
  chartOptions: ChartOptions<'bar'>
}

/**
 * Goal progress chart data
 */
export interface GoalProgressData {
  /** Chart data */
  chartData: ChartData<'doughnut'>
  /** Chart options */
  chartOptions: ChartOptions<'doughnut'>
}

/**
 * Helper to create mood trend chart data
 */
export function createMoodTrendChartData(
  checkins: MoodCheckin[]
): MoodTrendData {
  const labels = checkins.map(c => new Date(c.checkedInAt).toLocaleDateString())
  const data = checkins.map(c => c.moodRating)

  return {
    chartData: {
      labels,
      datasets: [{
        label: 'Mood Rating',
        data,
        borderColor: 'rgb(59, 130, 246)',  // blue-500
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
        tension: 0.3
      }]
    },
    chartOptions: {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        y: {
          min: 1,
          max: 7,
          ticks: {
            stepSize: 1
          }
        }
      }
    }
  }
}

/**
 * Helper to create activity frequency chart data
 */
export function createActivityFrequencyChartData(
  frequencies: { activityName: string; days: number }[]
): ActivityFrequencyData {
  const labels = frequencies.map(f => f.activityName)
  const data = frequencies.map(f => f.days)

  return {
    chartData: {
      labels,
      datasets: [{
        label: 'Days per Week',
        data,
        backgroundColor: 'rgb(34, 197, 94)',  // green-500
      }]
    },
    chartOptions: {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        y: {
          min: 0,
          max: 7,
          ticks: {
            stepSize: 1
          }
        }
      }
    }
  }
}
```

---

## Utility Types

### Common Patterns

```typescript
// types/utils.ts

/**
 * Make specific keys required
 */
export type WithRequired<T, K extends keyof T> = T & Required<Pick<T, K>>

/**
 * Make specific keys optional
 */
export type WithOptional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>

/**
 * Nullable type
 */
export type Nullable<T> = T | null

/**
 * Optional type
 */
export type Optional<T> = T | undefined

/**
 * ID type (for entity IDs)
 */
export type ID = number

/**
 * Timestamp type (ISO 8601 string)
 */
export type Timestamp = string

/**
 * Color type (CSS color)
 */
export type Color = string

/**
 * Pagination state
 */
export interface PaginationState {
  /** Current page (0-indexed) */
  page: number
  /** Items per page */
  pageSize: number
  /** Total items */
  total: number
  /** Total pages */
  totalPages: number
}

/**
 * Sort state
 */
export interface SortState<T = string> {
  /** Sort field */
  field: T
  /** Sort direction */
  direction: 'asc' | 'desc'
}

/**
 * Filter state (generic)
 */
export interface FilterState<T> {
  /** Active filters */
  filters: T
  /** Whether filters are applied */
  isActive: boolean
}

/**
 * Selection state (for lists)
 */
export interface SelectionState<T> {
  /** Selected items */
  selected: Set<T>
  /** Whether all items selected */
  isAllSelected: boolean
}

/**
 * Helper to create selection state
 */
export function createSelectionState<T>(): SelectionState<T> {
  return {
    selected: new Set(),
    isAllSelected: false
  }
}
```

---

### Date/Time Utilities

```typescript
// types/datetime.ts

/**
 * Date range
 */
export interface DateRange {
  /** Start date (ISO 8601) */
  start: string
  /** End date (ISO 8601) */
  end: string
}

/**
 * Time period
 */
export type TimePeriod = 'today' | 'yesterday' | 'last_week' | 'last_month' | 'last_quarter' | 'custom'

/**
 * Helper to get date range for period
 */
export function getDateRangeForPeriod(period: TimePeriod): DateRange {
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())

  switch (period) {
    case 'today':
      return {
        start: today.toISOString(),
        end: now.toISOString()
      }
    case 'yesterday': {
      const yesterday = new Date(today)
      yesterday.setDate(yesterday.getDate() - 1)
      return {
        start: yesterday.toISOString(),
        end: today.toISOString()
      }
    }
    case 'last_week': {
      const weekAgo = new Date(today)
      weekAgo.setDate(weekAgo.getDate() - 7)
      return {
        start: weekAgo.toISOString(),
        end: now.toISOString()
      }
    }
    case 'last_month': {
      const monthAgo = new Date(today)
      monthAgo.setMonth(monthAgo.getMonth() - 1)
      return {
        start: monthAgo.toISOString(),
        end: now.toISOString()
      }
    }
    case 'last_quarter': {
      const quarterAgo = new Date(today)
      quarterAgo.setMonth(quarterAgo.getMonth() - 3)
      return {
        start: quarterAgo.toISOString(),
        end: now.toISOString()
      }
    }
    default:
      return {
        start: today.toISOString(),
        end: now.toISOString()
      }
  }
}
```

---

## Type Organization

### File Structure

```
src/lib/types/
├── index.ts                  # Re-export all types
├── forms.ts                  # Form state types
├── modals.ts                 # Modal state types
├── loading.ts                # Loading/async state types
├── activity-groups.ts        # Activity Groups domain types
├── goals.ts                  # Goal-related types
├── mood.ts                   # Mood check-in types
├── charts.ts                 # Chart data types
├── utils.ts                  # Utility types
└── datetime.ts               # Date/time types

src/lib/components/ui/
├── Button.types.ts           # Button prop types
├── Input.types.ts            # Input prop types
├── Card.types.ts             # Card prop types
├── Badge.types.ts            # Badge prop types
└── Alert.types.ts            # Alert prop types
```

---

### Barrel Export (index.ts)

```typescript
// src/lib/types/index.ts

// UI State
export * from './forms'
export * from './modals'
export * from './loading'

// Domain
export * from './activity-groups'
export * from './goals'
export * from './mood'

// Charts
export * from './charts'

// Utils
export * from './utils'
export * from './datetime'

// Component Props (re-export from components/ui)
export type { ButtonProps, ButtonVariant, ButtonColor } from '$lib/components/ui/Button.types'
export type { InputProps, InputType } from '$lib/components/ui/Input.types'
export type { CardProps } from '$lib/components/ui/Card.types'
export type { BadgeProps, BadgeColor } from '$lib/components/ui/Badge.types'
export type { AlertProps, AlertType } from '$lib/components/ui/Alert.types'
```

**Usage:**
```svelte
<script lang="ts">
  // Single import for all types
  import type {
    FormState,
    ActivityGroupFormData,
    AsyncData,
    GoalProgressUI
  } from '$lib/types'
</script>
```

---

## Implementation Checklist

### Before Week 1 (Activity Groups)

- [ ] Create `src/lib/types/` directory
- [ ] Create all type files:
  - [ ] `forms.ts` - Form state types
  - [ ] `modals.ts` - Modal state types
  - [ ] `loading.ts` - Async data types
  - [ ] `activity-groups.ts` - Activity Groups UI types
  - [ ] `goals.ts` - Goal progress UI types
  - [ ] `utils.ts` - Utility types
  - [ ] `index.ts` - Barrel exports

### During Week 7 (Catalyst UI)

- [ ] Create component prop type files:
  - [ ] `Button.types.ts`
  - [ ] `Input.types.ts`
  - [ ] `Card.types.ts`
  - [ ] `Badge.types.ts`
  - [ ] `Alert.types.ts`

### Future (as needed)

- [ ] `charts.ts` - Chart data types (when building reporting)
- [ ] `mood.ts` - Mood UI types (when upgrading to 7-point scale)
- [ ] `datetime.ts` - Date/time utilities (when adding backdating)

---

## Naming Conventions

| Type Category | Pattern | Example |
|---------------|---------|---------|
| **State types** | `{Feature}State` | `FormState`, `ModalState` |
| **UI types** | `{Entity}UI` | `ActivityGroupUI`, `GoalProgressUI` |
| **Form data** | `{Entity}FormData` | `ActivityGroupFormData` |
| **Props** | `{Component}Props` | `ButtonProps`, `InputProps` |
| **Enums** | `{Feature}{Type}` | `ButtonVariant`, `AlertType` |
| **Helpers** | `create{Type}` | `createFormState`, `createModalState` |

---

## Best Practices

**✅ DO:**
- Define types before implementation
- Use descriptive, unambiguous names
- Provide helper functions for complex types
- Export from barrel file for easy imports
- Document types with JSDoc comments

**❌ DON'T:**
- Create types after implementation (leads to `any`)
- Use vague names (`Data`, `State`, `Props`)
- Duplicate types across files
- Export everything from every file (use barrel)
- Leave types undocumented

---

## Resources

- **TypeScript Handbook:** https://www.typescriptlang.org/docs/handbook/intro.html
- **TypeScript Utility Types:** https://www.typescriptlang.org/docs/handbook/utility-types.html
- **Zod Documentation:** https://zod.dev/ (for runtime validation)
- **Chart.js Types:** https://www.chartjs.org/docs/latest/getting-started/typescript.html
