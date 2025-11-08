# Svelte 5 Architecture Specification

**Last Updated:** 2025-11-07
**Status:** Phase 0 - Specifications
**Related:** REVISED-plan.md, REVISED-tasks.md

**Applies To Tasks:**
- **Week 1** (Tasks 1.11-1.31): Repository layer - use runes for local state, keep stores for global
- **Week 3** (Tasks 3.1-3.23): Frontend UI - runes for component state, context for feature-scoped state
- **Week 7** (Tasks 7.7-7.13): Catalyst components - presentation components with `$props()` only
- **Week 8** (Tasks 8.1-8.8): Component migration - follow presentation vs container pattern

---

## Overview

This document defines architectural patterns for Svelte 5 development in the mental-health-bar-rs project. It provides clear decision criteria for when to use runes vs stores, component patterns, state management strategies, and migration guidelines.

**Key Principles:**
- Runes for component reactivity
- Stores for global state management
- Both patterns are valid and complementary
- Choose based on scope, persistence, and usage patterns

---

## Table of Contents

1. [State Management Decision Tree](#state-management-decision-tree)
2. [Runes Patterns](#runes-patterns)
3. [Store Patterns](#store-patterns)
4. [Component Architecture](#component-architecture)
5. [Migration Strategy](#migration-strategy)
6. [Anti-Patterns](#anti-patterns)
7. [Examples](#examples)

---

## State Management Decision Tree

### Decision Flowchart

```
Is this state needed outside a component?
├─ NO → Use $state() (local component state)
└─ YES → Is it used across unrelated components?
    ├─ YES → Use Svelte stores (global state)
    └─ NO → Is it shared within a parent-child tree?
        ├─ YES → Use runes + context (feature-scoped state)
        └─ NO → Does it need persistence?
            ├─ YES → Use Svelte stores (localStorage/sessionStorage)
            └─ NO → Is it called from utilities/commands?
                ├─ YES → Use Svelte stores (external access)
                └─ NO → Use $state() (component-local)
```

### Quick Reference Table

| Use Case | Pattern | Rationale |
|----------|---------|-----------|
| Component-local counter, toggle, form input | `$state()` | No external access needed |
| Computed value from local state | `$derived()` | Reactivity within component |
| Component props | `$props()` | Standard Svelte 5 pattern |
| Side effects (API calls, subscriptions) | `$effect()` | Lifecycle-aware effects |
| Global theme, toast notifications | `writable/readable` | Used everywhere, cross-cutting |
| Persisted preferences | `writable` + subscription | localStorage integration |
| Feature-specific shared state (e.g., form wizard) | Runes + context | Scoped to component tree |
| State accessed from utilities | `writable/readable` | External function access |
| Complex derived state (multiple sources) | `derived` store | Clarity for complex logic |

---

## Runes Patterns

### $state() - Local Component State

**Use for:** Data that lives and dies with the component

**Pattern:**
```svelte
<script lang="ts">
  // ✅ Local state
  let count = $state(0)
  let isOpen = $state(false)
  let formData = $state({ name: '', email: '' })

  function increment() {
    count++  // Reactive update
  }
</script>

<button onclick={increment}>Count: {count}</button>
```

**When to use:**
- UI state (modals, toggles, accordions)
- Form inputs (before submission)
- Temporary computed values
- Component-specific flags

**When NOT to use:**
- State shared across unrelated components
- State that needs to persist across navigation
- State accessed from non-component code

---

### $derived() - Computed Values

**Use for:** Values that depend on other reactive state

**Pattern:**
```svelte
<script lang="ts">
  let count = $state(0)
  let doubled = $derived(count * 2)  // ✅ Auto-updates when count changes

  // Complex derivation
  let validationMessage = $derived(() => {
    if (count < 0) return 'Must be positive'
    if (count > 100) return 'Must be ≤100'
    return ''
  })
</script>

<p>Doubled: {doubled}</p>
<p>{validationMessage}</p>
```

**When to use:**
- Single-source derived values
- Simple computations
- Validation messages
- Filtered/sorted local data

**When NOT to use:**
- Complex multi-source derivations (prefer `derived` store)
- Expensive computations (consider memoization)
- Derived state needed outside component

---

### $props() - Component Props

**Use for:** ALL component inputs (replaces `export let`)

**Pattern:**
```svelte
<script lang="ts">
  // ✅ Basic props
  let { title, count }: { title: string; count: number } = $props()

  // ✅ Optional props with defaults
  let { size = 'md' }: { size?: 'sm' | 'md' | 'lg' } = $props()

  // ✅ Rest props (spread to child)
  let { class: className, ...rest }: { class?: string; [key: string]: unknown } = $props()
</script>

<div class={className} {...rest}>
  <h2>{title}</h2>
  <p>Count: {count}</p>
</div>
```

**When to use:**
- ALWAYS for component inputs
- Props with TypeScript types
- Optional props with defaults
- Rest props for pass-through

**When NOT to use:**
- Never use `export let` in Svelte 5
- Not for two-way binding (use `$bindable()` instead)

---

### $bindable() - Two-Way Binding

**Use for:** Props that can be modified by child component

**Pattern:**
```svelte
<!-- Parent.svelte -->
<script lang="ts">
  let value = $state('')
</script>

<InputField bind:value />

<!-- InputField.svelte -->
<script lang="ts">
  let { value = $bindable('') }: { value?: string } = $props()
</script>

<input type="text" bind:value />
```

**When to use:**
- Form inputs with parent state
- Modal open/close state
- Selected items in lists
- Any parent-controlled child state

**When NOT to use:**
- One-way data flow (prefer props + events)
- When events provide better clarity

---

### $effect() - Side Effects

**Use for:** Code that runs when reactive values change

**Pattern:**
```svelte
<script lang="ts">
  let count = $state(0)

  // ✅ Effect with cleanup
  $effect(() => {
    console.log('Count changed:', count)

    const timer = setTimeout(() => {
      console.log('Delayed log')
    }, 1000)

    // Cleanup function (runs before next effect or on unmount)
    return () => clearTimeout(timer)
  })

  // ✅ Effect with dependencies
  $effect(() => {
    if (count > 10) {
      fetchData(count)
    }
  })
</script>
```

**When to use:**
- API calls based on reactive values
- Subscriptions (websockets, intervals)
- DOM manipulation (focus, scroll)
- External library integration
- Logging/analytics

**When NOT to use:**
- Synchronous state updates (use `$derived` instead)
- One-time initialization (use `onMount` if needed)
- Complex async flows (extract to utility functions)

**Common Patterns:**
```svelte
<script lang="ts">
  // ✅ Load data when ID changes
  let { assessmentId } = $props()
  let data = $state(null)

  $effect(() => {
    loadData(assessmentId).then(result => {
      data = result
    })
  })

  // ✅ Subscribe to store
  import { theme } from '$lib/stores/theme'
  let currentTheme = $state($theme)

  $effect(() => {
    const unsubscribe = theme.subscribe(value => {
      currentTheme = value
    })
    return unsubscribe
  })
</script>
```

---

## Store Patterns

### writable() - Global Mutable State

**Use for:** State shared across unrelated components or accessed externally

**Pattern:**
```typescript
// lib/stores/theme.ts
import { writable } from 'svelte/store'

export type Theme = 'light' | 'dark' | 'system'

export const theme = writable<Theme>('system')

// Usage in components:
import { theme } from '$lib/stores/theme'

// Read value
$theme  // Auto-subscribes in component

// Update value
theme.set('dark')
theme.update(current => current === 'light' ? 'dark' : 'light')
```

**When to use:**
- Global application state (theme, user, locale)
- Toast/notification system
- State accessed from utilities/commands
- State that needs persistence

**Persistence Pattern:**
```typescript
// lib/stores/theme.ts
import { writable } from 'svelte/store'
import { browser } from '$app/environment'

function createPersistedStore<T>(key: string, initial: T) {
  const stored = browser ? localStorage.getItem(key) : null
  const initialValue = stored ? JSON.parse(stored) : initial

  const store = writable<T>(initialValue)

  if (browser) {
    store.subscribe(value => {
      localStorage.setItem(key, JSON.stringify(value))
    })
  }

  return store
}

export const theme = createPersistedStore<Theme>('theme', 'system')
```

---

### readable() - Global Read-Only State

**Use for:** State that updates from external source (time, websocket, etc.)

**Pattern:**
```typescript
// lib/stores/time.ts
import { readable } from 'svelte/store'

export const time = readable(new Date(), (set) => {
  const interval = setInterval(() => {
    set(new Date())
  }, 1000)

  return () => clearInterval(interval)
})

// Usage:
<p>Current time: {$time.toLocaleTimeString()}</p>
```

**When to use:**
- External data sources (websockets, SSE)
- Timers and intervals
- Browser APIs (geolocation, media queries)

---

### derived() - Complex Computed State

**Use for:** State computed from multiple stores

**Pattern:**
```typescript
// lib/stores/user.ts
import { writable, derived } from 'svelte/store'

export const firstName = writable('John')
export const lastName = writable('Doe')

// ✅ Derived from multiple stores
export const fullName = derived(
  [firstName, lastName],
  ([$first, $last]) => `${$first} ${$last}`
)

// ✅ Complex derivation
export const stats = derived(
  [activities, logs, goals],
  ([$activities, $logs, $goals]) => {
    // Complex calculation
    return {
      totalActivities: $activities.length,
      completedGoals: $goals.filter(g => g.completed).length,
      // ...
    }
  }
)
```

**When to use:**
- Multiple source dependencies
- Complex transformations
- Shared computed values
- Performance-critical calculations (derived stores memoize)

---

## Component Architecture

### File Structure

```
src/lib/components/
├── ui/                      # Reusable UI components
│   ├── Button.svelte
│   ├── Input.svelte
│   └── Card.svelte
├── {feature}/               # Feature-specific components
│   ├── FeatureList.svelte
│   ├── FeatureForm.svelte
│   └── FeatureCard.svelte
└── shared/                  # Shared business components
    ├── ErrorBoundary.svelte
    └── LoadingOverlay.svelte
```

### Component Naming Conventions

- **PascalCase.svelte** for all components
- Descriptive names: `ActivityGroupList.svelte` (not `List.svelte`)
- Suffix pattern: `{Entity}{Action}.svelte` (e.g., `AssessmentForm.svelte`)

### Component Patterns

#### 1. Presentation Component (UI-only)

**Characteristics:** No business logic, only props and events

```svelte
<!-- Button.svelte -->
<script lang="ts">
  let {
    variant = 'solid',
    onClick,
    children
  }: {
    variant?: 'solid' | 'outline' | 'plain'
    onClick?: () => void
    children: any
  } = $props()
</script>

<button
  class="btn btn-{variant}"
  onclick={onClick}
>
  {@render children()}
</button>

<style>
  .btn { /* Base styles */ }
  .btn-solid { /* Variant styles */ }
</style>
```

#### 2. Container Component (Business Logic)

**Characteristics:** Manages state, API calls, data transformation

```svelte
<!-- ActivityGroupList.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import type { ActivityGroup } from '$lib/bindings'
  import ActivityGroupCard from './ActivityGroupCard.svelte'

  let groups = $state<ActivityGroup[]>([])
  let loading = $state(true)
  let error = $state<string | null>(null)

  $effect(() => {
    loadGroups()
  })

  async function loadGroups() {
    try {
      loading = true
      groups = await invoke('get_activity_groups')
    } catch (err) {
      error = String(err)
    } finally {
      loading = false
    }
  }

  async function deleteGroup(id: number) {
    await invoke('delete_activity_group', { id })
    await loadGroups()  // Refresh
  }
</script>

{#if loading}
  <LoadingSpinner />
{:else if error}
  <ErrorMessage message={error} />
{:else}
  {#each groups as group}
    <ActivityGroupCard
      {group}
      onDelete={() => deleteGroup(group.id)}
    />
  {/each}
{/if}
```

#### 3. Hybrid Component (State + UI)

**Characteristics:** Small features where separation isn't needed

```svelte
<!-- ThemeToggle.svelte -->
<script lang="ts">
  import { theme, type Theme } from '$lib/stores/theme'

  function toggleTheme() {
    theme.update(current => {
      if (current === 'light') return 'dark'
      if (current === 'dark') return 'system'
      return 'light'
    })
  }

  function getIcon(themeValue: Theme): string {
    if (themeValue === 'light') return '☀️'
    if (themeValue === 'dark') return '🌙'
    return '💻'
  }
</script>

<button
  onclick={toggleTheme}
  class="theme-toggle"
  aria-label="Toggle theme ({$theme})"
>
  {getIcon($theme)}
</button>
```

---

## Migration Strategy

### Existing Stores - Keep or Convert?

| Store | Decision | Rationale |
|-------|----------|-----------|
| `theme.ts` | **KEEP** | Global, persistent, accessed everywhere |
| `toast.ts` | **KEEP** | Called from utilities and error handlers |
| `mood.ts` | **EVALUATE** | If only used in mood feature tree, convert to context |

### Evaluation Criteria for `mood.ts`

**Questions to ask:**
1. Is it used outside `/routes/mood/**` pages?
2. Is it accessed from non-component code (utilities)?
3. Does it need persistence?

If **NO** to all three → **Convert to runes + context**
If **YES** to any → **Keep as store**

### Conversion Example: Store → Runes + Context

**Before (Store):**
```typescript
// lib/stores/mood.ts
import { writable } from 'svelte/store'

export const selectedActivities = writable<number[]>([])
export const moodRating = writable<number | null>(null)
```

**After (Runes + Context):**
```svelte
<!-- routes/mood/+layout.svelte -->
<script lang="ts">
  import { setContext } from 'svelte'

  // Create state
  let selectedActivities = $state<number[]>([])
  let moodRating = $state<number | null>(null)

  // Share via context
  setContext('moodState', {
    get selectedActivities() { return selectedActivities },
    set selectedActivities(value) { selectedActivities = value },
    get moodRating() { return moodRating },
    set moodRating(value) { moodRating = value }
  })
</script>

<slot />

<!-- routes/mood/+page.svelte (child) -->
<script lang="ts">
  import { getContext } from 'svelte'

  const moodState = getContext<MoodState>('moodState')

  function selectActivity(id: number) {
    moodState.selectedActivities = [...moodState.selectedActivities, id]
  }
</script>
```

**When to migrate:**
- Not urgent - do during feature work
- Convert when refactoring related code
- Don't mass-migrate everything

---

## Anti-Patterns

### ❌ Don't Mix Paradigms Within Components

**Bad:**
```svelte
<script lang="ts">
  // ❌ Mixing $state and export let
  export let title: string  // Old Svelte 4 pattern
  let count = $state(0)     // New Svelte 5 pattern
</script>
```

**Good:**
```svelte
<script lang="ts">
  // ✅ Use $props() for all props
  let { title }: { title: string } = $props()
  let count = $state(0)
</script>
```

---

### ❌ Don't Use $state for Global State

**Bad:**
```svelte
<!-- ComponentA.svelte -->
<script lang="ts">
  // ❌ Can't share this with ComponentB
  let theme = $state('dark')
</script>
```

**Good:**
```typescript
// lib/stores/theme.ts
// ✅ Accessible from any component
export const theme = writable('dark')
```

---

### ❌ Don't Use Stores for Local State

**Bad:**
```svelte
<script lang="ts">
  import { writable } from 'svelte/store'

  // ❌ Overkill for component-local state
  const isOpen = writable(false)

  function toggle() {
    isOpen.update(v => !v)
  }
</script>

<button onclick={toggle}>Toggle</button>
{#if $isOpen}
  <div>Content</div>
{/if}
```

**Good:**
```svelte
<script lang="ts">
  // ✅ Simple and clear
  let isOpen = $state(false)

  function toggle() {
    isOpen = !isOpen
  }
</script>

<button onclick={toggle}>Toggle</button>
{#if isOpen}
  <div>Content</div>
{/if}
```

---

### ❌ Don't Use $effect for Derived State

**Bad:**
```svelte
<script lang="ts">
  let count = $state(0)
  let doubled = $state(0)

  // ❌ Overcomplicating simple derivation
  $effect(() => {
    doubled = count * 2
  })
</script>
```

**Good:**
```svelte
<script lang="ts">
  let count = $state(0)
  // ✅ $derived is for computed values
  let doubled = $derived(count * 2)
</script>
```

---

### ❌ Don't Forget Effect Cleanup

**Bad:**
```svelte
<script lang="ts">
  let count = $state(0)

  // ❌ Timer never cleaned up (memory leak)
  $effect(() => {
    setInterval(() => {
      count++
    }, 1000)
  })
</script>
```

**Good:**
```svelte
<script lang="ts">
  let count = $state(0)

  // ✅ Cleanup prevents memory leak
  $effect(() => {
    const interval = setInterval(() => {
      count++
    }, 1000)

    return () => clearInterval(interval)
  })
</script>
```

---

## Examples

### Example 1: Form with Local State (Runes)

```svelte
<!-- ActivityForm.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  let { groupId }: { groupId: number } = $props()

  // ✅ Local form state
  let name = $state('')
  let icon = $state('')
  let submitting = $state(false)

  // ✅ Validation
  let isValid = $derived(name.length >= 1 && name.length <= 50)
  let errorMessage = $derived(() => {
    if (name.length === 0) return 'Name is required'
    if (name.length > 50) return 'Name must be ≤50 characters'
    return ''
  })

  async function handleSubmit() {
    if (!isValid) return

    submitting = true
    try {
      await invoke('create_activity', {
        name,
        icon: icon || null,
        groupId
      })
      // Reset form
      name = ''
      icon = ''
    } finally {
      submitting = false
    }
  }
</script>

<form onsubmit={handleSubmit}>
  <label>
    Name:
    <input type="text" bind:value={name} maxlength="50" />
    {#if errorMessage}
      <span class="error">{errorMessage}</span>
    {/if}
  </label>

  <label>
    Icon:
    <input type="text" bind:value={icon} placeholder="heart" />
  </label>

  <button type="submit" disabled={!isValid || submitting}>
    {submitting ? 'Saving...' : 'Create Activity'}
  </button>
</form>
```

---

### Example 2: Global Toast System (Store)

```typescript
// lib/stores/toast.ts
import { writable } from 'svelte/store'

export type Toast = {
  id: string
  message: string
  type: 'success' | 'error' | 'info'
  duration?: number
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([])

  return {
    subscribe,
    show(message: string, type: Toast['type'] = 'info', duration = 3000) {
      const id = crypto.randomUUID()
      const toast: Toast = { id, message, type, duration }

      update(toasts => [...toasts, toast])

      if (duration > 0) {
        setTimeout(() => {
          this.dismiss(id)
        }, duration)
      }
    },
    dismiss(id: string) {
      update(toasts => toasts.filter(t => t.id !== id))
    }
  }
}

export const toastStore = createToastStore()

// Usage from anywhere (even utilities):
import { toastStore } from '$lib/stores/toast'

toastStore.show('Activity created!', 'success')
```

---

### Example 3: Feature-Scoped State (Context)

```svelte
<!-- routes/mood/check-in/+layout.svelte -->
<script lang="ts">
  import { setContext } from 'svelte'

  // ✅ State scoped to check-in flow
  let moodRating = $state<number | null>(null)
  let selectedActivities = $state<number[]>([])
  let notes = $state('')

  const checkInState = {
    get moodRating() { return moodRating },
    set moodRating(value) { moodRating = value },
    get selectedActivities() { return selectedActivities },
    set selectedActivities(value) { selectedActivities = value },
    get notes() { return notes },
    set notes(value) { notes = value }
  }

  setContext('checkIn', checkInState)
</script>

<slot />

<!-- routes/mood/check-in/step1/+page.svelte -->
<script lang="ts">
  import { getContext } from 'svelte'
  import type { CheckInState } from '../types'

  const state = getContext<CheckInState>('checkIn')

  function selectMood(rating: number) {
    state.moodRating = rating
  }
</script>

<MoodSelector
  value={state.moodRating}
  onChange={selectMood}
/>
```

---

## Summary

### Quick Decision Guide

**"Should I use runes or stores?"**

1. **Is it component-local?** → Use `$state()`
2. **Is it a prop?** → Use `$props()`
3. **Is it computed?** → Use `$derived()`
4. **Is it a side effect?** → Use `$effect()`
5. **Is it global/persistent/external?** → Use Svelte stores
6. **Is it shared in a feature tree?** → Use runes + context

### Implementation Guidelines

1. **Start with runes** for new components (assume local until proven otherwise)
2. **Keep existing stores** for global concerns (theme, toast, user)
3. **Evaluate feature stores** case-by-case (convert if scoped to feature tree)
4. **Don't mass-migrate** - convert during feature work
5. **Document decisions** in code comments when patterns aren't obvious

### For Activity Groups Implementation (Phase 1)

**Recommended approach:**
- Activity Group management UI: **Runes** (local state for forms)
- Activity selection state: **Evaluate** (context if only used in check-in flow)
- Goal progress: **Runes** (local to dashboard components)
- Notifications: **Keep store** (toast.ts already works)

---

## References

- [Svelte 5 Runes Documentation](https://svelte-5-preview.vercel.app/docs/runes)
- [Svelte Stores Documentation](https://svelte.dev/docs/svelte-store)
- Project patterns: `/src/lib/components/**/*.svelte`
- Existing stores: `/src/lib/stores/*.ts`
