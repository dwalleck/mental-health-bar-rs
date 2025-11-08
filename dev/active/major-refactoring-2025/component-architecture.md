# Component Architecture Specification

**Last Updated:** 2025-11-07
**Status:** Phase 0 - Specifications
**Related:** svelte5-architecture.md, tailwind4-design-system.md, catalyst-integration-spec.md

**Applies To Tasks:**
- **Week 1, Tasks 1.11-1.31**: Repository tests - follow testing conventions (describe blocks, mocking patterns)
- **Week 3, Tasks 3.1-3.23**: Component creation - use composition patterns (slots vs snippets), implement accessibility
- **Week 4, Tasks 4.1-4.16**: Integration tests - follow integration testing patterns
- **Week 7, Tasks 7.11-7.13**: Component tests - test rendering, interactions, accessibility for all Catalyst components
- **Week 10, Tasks 10.1-10.6**: Component testing - comprehensive test coverage (80%+ target)

---

## Overview

This document specifies component architecture patterns, testing conventions, and accessibility requirements for the mental-health-bar-rs Svelte application. It provides practical guidance on:

- Where to put components and tests
- When to use slots vs snippets vs props
- How to validate props and handle errors
- How to write consistent, maintainable tests
- How to implement accessible components

**Goal**: Ensure all components follow consistent patterns, are thoroughly tested, and meet WCAG AA accessibility standards.

---

## Table of Contents

1. [Directory Structure](#directory-structure)
2. [Composition Patterns](#composition-patterns)
3. [Props Validation](#props-validation)
4. [Testing Conventions](#testing-conventions)
5. [Accessibility Requirements](#accessibility-requirements)
6. [Component Lifecycle](#component-lifecycle)
7. [Error Handling](#error-handling)

---

## Directory Structure

### Feature-Based Organization

**Principle**: Components are organized by feature, not by type.

```
src/lib/components/
├── ui/                          # Reusable design system components
│   ├── Button.svelte
│   ├── Button.test.ts           # Co-located tests
│   ├── Input.svelte
│   ├── Input.test.ts
│   ├── Card.svelte
│   └── Badge.svelte
│
├── assessments/                 # Assessment feature components
│   ├── AssessmentList.svelte
│   ├── AssessmentList.test.ts
│   ├── AssessmentForm.svelte
│   ├── AssessmentForm.test.ts
│   ├── AssessmentResults.svelte
│   └── QuestionCard.svelte      # Feature-specific component
│
├── mood/                        # Mood tracking feature
│   ├── MoodScaleInput.svelte
│   ├── MoodScaleInput.test.ts
│   ├── ActivitySelector.svelte
│   ├── ActivityForm.svelte
│   └── MoodHistoryList.svelte
│
├── activity-groups/             # Activity Groups feature (NEW)
│   ├── ActivityGroupList.svelte
│   ├── ActivityGroupList.test.ts
│   ├── ActivityGroupForm.svelte
│   ├── ActivityGroupCard.svelte
│   ├── GoalSettingModal.svelte
│   └── GoalProgressIndicator.svelte
│
├── charts/                      # Data visualization
│   ├── MoodTrendChart.svelte
│   ├── ActivityReportCard.svelte
│   └── GoalProgressChart.svelte
│
└── layout/                      # Layout components
    ├── Navigation.svelte
    ├── Header.svelte
    └── Footer.svelte
```

**Test Co-Location Rules:**
- ✅ Component tests live next to components (`Button.svelte` → `Button.test.ts`)
- ✅ Integration tests in `tests/` directory at project root
- ✅ E2E tests in `tests/e2e/` directory

---

### File Naming Conventions

| Type | Pattern | Example |
|------|---------|---------|
| **Component** | `PascalCase.svelte` | `ActivityGroupList.svelte` |
| **Component Test** | `PascalCase.test.ts` | `ActivityGroupList.test.ts` |
| **Route Page** | `+page.svelte` | `routes/mood/+page.svelte` |
| **Route Layout** | `+layout.svelte` | `routes/+layout.svelte` |
| **Store** | `camelCase.ts` | `stores/theme.ts` |
| **Utility** | `camelCase.ts` | `utils/formatDate.ts` |
| **Type Definitions** | `PascalCase.types.ts` | `components/ui/Button.types.ts` |

---

## Composition Patterns

### Decision Framework: Slots vs Snippets vs Props

| Use Case | Pattern | Rationale |
|----------|---------|-----------|
| **Simple content injection** | Slots | Clean syntax, most common |
| **Multiple content areas** | Named slots | Clear structure |
| **Dynamic/conditional content** | Snippets | Runtime control |
| **Simple data** | Props | Type safety |
| **Functions/callbacks** | Props | Clear intent |

---

### Pattern 1: Default Slot (Simple Content)

**Use when:** Component wraps single piece of content

```svelte
<!-- Button.svelte -->
<script lang="ts">
  let { variant = 'solid', color = 'zinc', onclick, children } = $props()
</script>

<button class="btn btn-{variant}-{color}" {onclick}>
  {@render children()}
</button>

<!-- Usage -->
<Button variant="solid" color="blue">
  Save Changes
</Button>
```

**Pros:**
- ✅ Simple, clean syntax
- ✅ Most familiar pattern

**Cons:**
- ❌ Only one content area
- ❌ No conditional rendering

---

### Pattern 2: Named Slots (Multiple Content Areas)

**Use when:** Component has multiple distinct content regions

```svelte
<!-- Card.svelte -->
<script lang="ts">
  let { header, body, footer, class: className = '' } = $props()
</script>

<div class="card {className}">
  {#if header}
    <div class="card-header">
      {@render header()}
    </div>
  {/if}

  <div class="card-body">
    {@render body()}
  </div>

  {#if footer}
    <div class="card-footer">
      {@render footer()}
    </div>
  {/if}
</div>

<!-- Usage -->
<Card>
  {#snippet header()}
    <h2 class="card-title">Activity Groups</h2>
    <p class="card-description">Manage your activity categories</p>
  {/snippet}

  {#snippet body()}
    <ActivityGroupList groups={$activityGroups} />
  {/snippet}

  {#snippet footer()}
    <Button variant="solid" color="blue">Add Group</Button>
  {/snippet}
</Card>
```

**Pros:**
- ✅ Clear structure
- ✅ Optional sections
- ✅ Type-safe snippets

**Cons:**
- ❌ More verbose than default slot

---

### Pattern 3: Snippets with Parameters (Dynamic Content)

**Use when:** Parent needs to control rendering based on item data

```svelte
<!-- DataTable.svelte -->
<script lang="ts">
  let {
    items,
    renderRow
  }: {
    items: any[]
    renderRow: (item: any, index: number) => any
  } = $props()
</script>

<table class="table">
  <tbody>
    {#each items as item, index}
      <tr>
        {@render renderRow(item, index)}
      </tr>
    {/each}
  </tbody>
</table>

<!-- Usage -->
<DataTable items={activities}>
  {#snippet renderRow(activity, index)}
    <td>{activity.name}</td>
    <td><Badge color={activity.color}>{activity.group}</Badge></td>
    <td>{activity.lastLogged}</td>
  {/snippet}
</DataTable>
```

**Pros:**
- ✅ Full control over rendering
- ✅ Access to item data
- ✅ Conditional rendering

**Cons:**
- ❌ More complex
- ❌ Can be overused

---

### Pattern 4: Props for Data (Simple Values)

**Use when:** Component needs simple configuration

```svelte
<!-- Badge.svelte -->
<script lang="ts">
  let {
    color = 'zinc',
    size = 'md',
    children
  }: {
    color?: 'zinc' | 'blue' | 'green' | 'red' | 'yellow'
    size?: 'sm' | 'md' | 'lg'
    children: any
  } = $props()
</script>

<span class="badge badge-{color} badge-{size}">
  {@render children()}
</span>

<!-- Usage -->
<Badge color="green" size="sm">Active</Badge>
```

**Pros:**
- ✅ Type-safe
- ✅ Clear API
- ✅ Easy to validate

**Cons:**
- ❌ Not flexible for complex content

---

### Best Practices

**✅ DO:**
- Use default slot for simple content injection
- Use named slots for distinct content regions (header, body, footer)
- Use snippets when parent needs control over rendering
- Use props for configuration and simple data
- Provide sensible defaults for optional props

**❌ DON'T:**
- Mix too many patterns in one component (confusing API)
- Use snippets when a simple slot would work
- Force users to provide snippets for optional content
- Use slots for data that should be props

---

## Props Validation

### TypeScript Validation (Compile-Time)

**Pattern:** Define prop types inline with `$props()`

```svelte
<script lang="ts">
  // ✅ GOOD: Type-safe props with defaults
  let {
    variant = 'solid',
    color = 'zinc',
    size = 'md',
    disabled = false,
    onclick,
    children
  }: {
    variant?: 'solid' | 'outline' | 'plain'
    color?: 'zinc' | 'blue' | 'red' | 'green'
    size?: 'sm' | 'md' | 'lg'
    disabled?: boolean
    onclick?: () => void
    children: any
  } = $props()
</script>
```

**Extract to type file for reusability:**

```typescript
// Button.types.ts
export type ButtonVariant = 'solid' | 'outline' | 'plain'
export type ButtonColor = 'zinc' | 'blue' | 'red' | 'green'
export type ButtonSize = 'sm' | 'md' | 'lg'

export interface ButtonProps {
  variant?: ButtonVariant
  color?: ButtonColor
  size?: ButtonSize
  disabled?: boolean
  onclick?: () => void
  children: any
}

// Button.svelte
<script lang="ts">
  import type { ButtonProps } from './Button.types'

  let props: ButtonProps = $props()
  let { variant = 'solid', color = 'zinc', size = 'md', disabled = false, onclick, children } = props
</script>
```

---

### Runtime Validation (For User Input)

**Use Zod for runtime validation of complex props:**

```typescript
// ActivityGroupForm.svelte
<script lang="ts">
  import { z } from 'zod'

  // Define schema
  const activityGroupSchema = z.object({
    name: z.string()
      .min(1, 'Name is required')
      .max(100, 'Name must be 100 characters or less'),
    description: z.string()
      .max(500, 'Description must be 500 characters or less')
      .optional(),
    color: z.enum(['zinc', 'blue', 'green', 'red', 'yellow'])
  })

  type ActivityGroupInput = z.infer<typeof activityGroupSchema>

  // Component state
  let formData = $state<ActivityGroupInput>({
    name: '',
    description: '',
    color: 'zinc'
  })

  let errors = $state<Record<string, string>>({})

  function validateForm(): boolean {
    try {
      activityGroupSchema.parse(formData)
      errors = {}
      return true
    } catch (err) {
      if (err instanceof z.ZodError) {
        errors = err.errors.reduce((acc, e) => {
          acc[e.path[0]] = e.message
          return acc
        }, {} as Record<string, string>)
      }
      return false
    }
  }

  async function handleSubmit() {
    if (!validateForm()) return

    // Submit validated data
    await invoke('create_activity_group', { input: formData })
  }
</script>

<form onsubmit={handleSubmit}>
  <Input
    label="Group Name"
    bind:value={formData.name}
    error={errors.name}
    required
  />

  <Input
    label="Description"
    bind:value={formData.description}
    error={errors.description}
  />

  <Button type="submit">Create Group</Button>
</form>
```

**When to use runtime validation:**
- ✅ User input (forms)
- ✅ External data (API responses)
- ✅ Complex business rules
- ❌ Simple props (TypeScript is enough)
- ❌ Internal component state

---

### Prop Validation Checklist

- [ ] **Required props** - No default value, TypeScript enforces
- [ ] **Optional props** - Provide sensible defaults
- [ ] **Enums** - Use union types for fixed options
- [ ] **Numbers** - Validate ranges at runtime if needed
- [ ] **Strings** - Validate length/format at runtime for user input
- [ ] **Objects** - Use Zod schema for complex validation
- [ ] **Functions** - Type signature with TypeScript

---

## Testing Conventions

### Test File Structure

**Pattern:** One test file per component, co-located

```
Button.svelte
Button.test.ts       ← Tests for Button.svelte
Button.types.ts      ← Type definitions (optional)
```

---

### Test Naming Convention

**Pattern:** `describe` block per component, `test` or `it` for test cases

```typescript
// Button.test.ts
import { render, fireEvent } from '@testing-library/svelte'
import { describe, it, expect, vi } from 'vitest'
import Button from './Button.svelte'

describe('Button', () => {
  describe('Rendering', () => {
    it('renders with default props', () => {
      const { container } = render(Button, { props: { children: 'Click me' } })
      expect(container.querySelector('button')).toBeInTheDocument()
    })

    it('renders with solid variant class', () => {
      const { container } = render(Button, {
        props: { variant: 'solid', color: 'blue', children: 'Click' }
      })
      expect(container.querySelector('.btn-solid-blue')).toBeInTheDocument()
    })

    it('renders children content', () => {
      const { getByText } = render(Button, { props: { children: 'Save' } })
      expect(getByText('Save')).toBeInTheDocument()
    })
  })

  describe('Interactions', () => {
    it('calls onclick when clicked', async () => {
      const handleClick = vi.fn()
      const { getByText } = render(Button, {
        props: { onclick: handleClick, children: 'Click' }
      })

      await fireEvent.click(getByText('Click'))
      expect(handleClick).toHaveBeenCalledOnce()
    })

    it('does not call onclick when disabled', async () => {
      const handleClick = vi.fn()
      const { getByText } = render(Button, {
        props: { onclick: handleClick, disabled: true, children: 'Click' }
      })

      const button = getByText('Click').closest('button')
      await fireEvent.click(button!)
      expect(handleClick).not.toHaveBeenCalled()
    })
  })

  describe('Accessibility', () => {
    it('has button role', () => {
      const { container } = render(Button, { props: { children: 'Click' } })
      expect(container.querySelector('button')).toHaveAttribute('role', 'button')
    })

    it('is keyboard accessible', async () => {
      const handleClick = vi.fn()
      const { container } = render(Button, {
        props: { onclick: handleClick, children: 'Click' }
      })

      const button = container.querySelector('button')!
      await fireEvent.keyDown(button, { key: 'Enter' })
      expect(handleClick).toHaveBeenCalled()
    })
  })
})
```

---

### What to Test

**Component Tests (Vitest + Testing Library):**

1. **Rendering**
   - [ ] Renders with default props
   - [ ] Renders with all prop variants
   - [ ] Renders children/slots correctly
   - [ ] Conditional rendering works

2. **Interactions**
   - [ ] Click handlers work
   - [ ] Form submissions work
   - [ ] Two-way binding works (bind:value)
   - [ ] Keyboard interactions work

3. **State**
   - [ ] Local state updates correctly
   - [ ] Derived state computes correctly
   - [ ] Effects run when dependencies change

4. **Accessibility**
   - [ ] Has correct ARIA attributes
   - [ ] Keyboard navigation works
   - [ ] Focus management correct

5. **Edge Cases**
   - [ ] Empty data
   - [ ] Missing props
   - [ ] Invalid input
   - [ ] Error states

---

### Mocking Tauri Commands

**Pattern:** Mock Tauri's `invoke` function

```typescript
// ActivityGroupList.test.ts
import { render, waitFor } from '@testing-library/svelte'
import { vi } from 'vitest'
import ActivityGroupList from './ActivityGroupList.svelte'

// Mock Tauri
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn()
}))

import { invoke } from '@tauri-apps/api/tauri'

describe('ActivityGroupList', () => {
  it('loads and displays activity groups', async () => {
    // Mock successful API response
    vi.mocked(invoke).mockResolvedValue([
      { id: 1, name: 'Exercise', description: 'Physical activities' },
      { id: 2, name: 'Social', description: 'Social connections' }
    ])

    const { getByText } = render(ActivityGroupList)

    // Wait for async load
    await waitFor(() => {
      expect(getByText('Exercise')).toBeInTheDocument()
      expect(getByText('Social')).toBeInTheDocument()
    })

    // Verify invoke was called
    expect(invoke).toHaveBeenCalledWith('get_activity_groups')
  })

  it('displays error when loading fails', async () => {
    // Mock API error
    vi.mocked(invoke).mockRejectedValue(new Error('Network error'))

    const { getByText } = render(ActivityGroupList)

    await waitFor(() => {
      expect(getByText(/error/i)).toBeInTheDocument()
    })
  })
})
```

---

### Integration Tests

**Location:** `tests/integration/`

**Pattern:** Test feature flows across multiple components

```typescript
// tests/integration/activity-group-workflow.test.ts
import { render, fireEvent, waitFor } from '@testing-library/svelte'
import { vi } from 'vitest'
import ActivityGroupPage from '$routes/activity-groups/+page.svelte'

vi.mock('@tauri-apps/api/tauri')
import { invoke } from '@tauri-apps/api/tauri'

describe('Activity Group Workflow', () => {
  it('allows user to create, edit, and delete activity group', async () => {
    // Setup mocks
    vi.mocked(invoke).mockImplementation((cmd, args) => {
      if (cmd === 'get_activity_groups') {
        return Promise.resolve([])
      }
      if (cmd === 'create_activity_group') {
        return Promise.resolve({ id: 1, name: args.name })
      }
      if (cmd === 'delete_activity_group') {
        return Promise.resolve()
      }
    })

    const { getByText, getByLabelText } = render(ActivityGroupPage)

    // Create group
    await fireEvent.click(getByText('Add Group'))
    await fireEvent.input(getByLabelText('Group Name'), {
      target: { value: 'Exercise' }
    })
    await fireEvent.click(getByText('Create'))

    await waitFor(() => {
      expect(getByText('Exercise')).toBeInTheDocument()
    })

    // Delete group
    await fireEvent.click(getByText('Delete'))
    await fireEvent.click(getByText('Confirm')) // Confirmation modal

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('delete_activity_group', { id: 1 })
    })
  })
})
```

---

### Test Coverage Targets

**Per Component:**
- Unit tests: **80%+ coverage**
- All variants tested
- All interactions tested
- Edge cases covered

**Per Feature:**
- Integration tests: **70%+ coverage**
- Happy path tested
- Error paths tested
- User flows tested

**Project-Wide:**
- Overall coverage: **80%+**
- No untested critical paths

---

## Accessibility Requirements

### WCAG AA Compliance Checklist

**All interactive components must:**

- [ ] **Keyboard Navigation** - Accessible via Tab, Enter, Escape, Arrow keys
- [ ] **Focus Indicators** - Visible focus ring (2px, blue-600, 2px offset)
- [ ] **ARIA Attributes** - Correct roles, states, properties
- [ ] **Color Contrast** - 4.5:1 for text, 3:1 for UI elements
- [ ] **Screen Reader** - All controls have labels
- [ ] **Touch Targets** - Min 44×44px for mobile
- [ ] **Error Identification** - Clear error messages associated with inputs

---

### Pattern 1: Keyboard Navigation

**Button Example:**

```svelte
<!-- Already accessible - native <button> handles keyboard -->
<button
  class="btn"
  onclick={handleClick}
  disabled={isDisabled}
>
  Save
</button>
```

**Custom Interactive Element:**

```svelte
<script lang="ts">
  let { onclick } = $props()

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault()
      onclick?.()
    }
  }
</script>

<div
  role="button"
  tabindex="0"
  onclick={onclick}
  onkeydown={handleKeyDown}
  class="custom-button"
>
  Click me
</div>
```

**Keyboard Navigation Patterns:**

| Element | Tab | Enter | Space | Escape | Arrow Keys |
|---------|-----|-------|-------|--------|------------|
| Button | Focus | Activate | Activate | - | - |
| Link | Focus | Navigate | - | - | - |
| Checkbox | Focus | Toggle | Toggle | - | - |
| Radio | Focus | Select | Select | - | Navigate group |
| Select | Focus | Open | Open | Close | Navigate options |
| Modal | Trap focus | - | - | Close | - |
| Menu | Focus | Open/Select | - | Close | Navigate items |

---

### Pattern 2: Focus Management

**Modal Focus Trap:**

```svelte
<script lang="ts">
  import { createDialog } from '@melt-ui/svelte'

  let { open = $bindable(false) } = $props()

  const {
    elements: { overlay, content, title, close },
    states: { open: isOpen }
  } = createDialog({
    open,
    onOpenChange: ({ next }) => {
      open = next
      return next
    }
  })
</script>

{#if $isOpen}
  <!-- Overlay -->
  <div use:melt={$overlay} class="modal-overlay" />

  <!-- Modal with focus trap -->
  <div use:melt={$content} class="modal-panel">
    <h2 use:melt={$title}>Confirm Delete</h2>
    <p>Are you sure you want to delete this activity group?</p>

    <div class="modal-footer">
      <Button variant="outline" onclick={() => open = false}>
        Cancel
      </Button>
      <Button variant="solid" color="red" onclick={handleConfirm}>
        Delete
      </Button>
    </div>

    <!-- Close button (keyboard accessible) -->
    <button use:melt={$close} aria-label="Close" class="modal-close">
      <Icon src={XMark} class="size-5" />
    </button>
  </div>
{/if}
```

**Focus Return Pattern:**

```svelte
<script lang="ts">
  let previousFocus: HTMLElement | null = null

  function openModal() {
    // Save current focus
    previousFocus = document.activeElement as HTMLElement
    isOpen = true
  }

  function closeModal() {
    isOpen = false
    // Return focus to previously focused element
    previousFocus?.focus()
  }
</script>
```

---

### Pattern 3: ARIA Attributes

**Form Input with Error:**

```svelte
<script lang="ts">
  let { label, error, id = crypto.randomUUID(), ...rest } = $props()
  const errorId = `${id}-error`
</script>

<label for={id} class="input-label">
  {label}
</label>

<input
  {id}
  class="input {error ? 'input-error' : ''}"
  aria-invalid={!!error}
  aria-describedby={error ? errorId : undefined}
  {...rest}
/>

{#if error}
  <p id={errorId} class="input-error-message" role="alert">
    {error}
  </p>
{/if}
```

**Live Region for Dynamic Updates:**

```svelte
<script lang="ts">
  let statusMessage = $state('')

  async function saveChanges() {
    statusMessage = 'Saving...'
    await save()
    statusMessage = 'Changes saved successfully'
  }
</script>

<!-- Announces to screen readers when message changes -->
<div role="status" aria-live="polite" aria-atomic="true">
  {statusMessage}
</div>
```

**Common ARIA Patterns:**

| Pattern | ARIA Attributes | Example |
|---------|----------------|---------|
| **Button** | `role="button"` | Custom interactive element |
| **Toggle** | `aria-pressed="true/false"` | Like button |
| **Checkbox** | `aria-checked="true/false/mixed"` | Custom checkbox |
| **Expandable** | `aria-expanded="true/false"` | Accordion, dropdown |
| **Modal** | `role="dialog" aria-modal="true"` | Modal overlay |
| **Alert** | `role="alert"` or `role="status"` | Error/success messages |
| **Tabs** | `role="tablist/tab/tabpanel"` | Tab navigation |
| **Menu** | `role="menu/menuitem"` | Context menu |

---

### Pattern 4: Color Contrast

**Check contrast ratios:**

```css
/* ✅ GOOD: 4.75:1 contrast */
.text-primary {
  color: theme('colors.zinc.950');      /* #09090b */
  background: theme('colors.white');    /* #ffffff */
}

/* ❌ BAD: 2.5:1 contrast (fails WCAG AA) */
.text-muted {
  color: theme('colors.zinc.400');      /* #a1a1aa */
  background: theme('colors.white');    /* #ffffff */
}

/* ✅ FIXED: 4.63:1 contrast */
.text-muted {
  color: theme('colors.zinc.600');      /* #52525b */
  background: theme('colors.white');
}
```

**Use color contrast checker:**
- WebAIM: https://webaim.org/resources/contrastchecker/
- Chrome DevTools: Inspect element → Accessibility → Contrast

---

### Pattern 5: Screen Reader Labels

**Icon-Only Buttons:**

```svelte
<button class="btn-plain-zinc" aria-label="Delete activity group">
  <Icon src={Trash} class="size-5" />
</button>
```

**Visually Hidden Labels:**

```svelte
<style>
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }
</style>

<button>
  <span class="sr-only">Search</span>
  <Icon src={MagnifyingGlass} class="size-5" />
</button>
```

---

## Component Lifecycle

### Initialization Pattern

```svelte
<script lang="ts">
  import { onMount } from 'svelte'

  let data = $state([])
  let loading = $state(true)
  let error = $state<string | null>(null)

  // Load data on mount
  onMount(async () => {
    try {
      data = await invoke('get_activity_groups')
    } catch (err) {
      error = String(err)
    } finally {
      loading = false
    }
  })
</script>

{#if loading}
  <LoadingSpinner />
{:else if error}
  <Alert type="error">{error}</Alert>
{:else}
  <ActivityGroupList groups={data} />
{/if}
```

---

### Cleanup Pattern

```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte'

  let interval: number

  onMount(() => {
    // Start interval
    interval = setInterval(() => {
      console.log('Tick')
    }, 1000)
  })

  onDestroy(() => {
    // Clean up interval
    clearInterval(interval)
  })
</script>
```

**With $effect (Svelte 5):**

```svelte
<script lang="ts">
  let count = $state(0)

  $effect(() => {
    const interval = setInterval(() => {
      count++
    }, 1000)

    // Cleanup function (runs on unmount or before next effect)
    return () => clearInterval(interval)
  })
</script>

<p>Count: {count}</p>
```

---

## Error Handling

### Error Boundary Pattern (Component-Level)

```svelte
<!-- ErrorBoundary.svelte -->
<script lang="ts">
  let { children, fallback } = $props()
  let error = $state<Error | null>(null)

  function handleError(err: Error) {
    error = err
    console.error('Component error:', err)
  }
</script>

{#if error}
  {#if fallback}
    {@render fallback(error)}
  {:else}
    <Alert type="error">
      <strong>Something went wrong</strong>
      <p>{error.message}</p>
    </Alert>
  {/if}
{:else}
  {@render children()}
{/if}

<!-- Usage -->
<ErrorBoundary>
  {#snippet fallback(error)}
    <Alert type="error">Failed to load activity groups: {error.message}</Alert>
  {/snippet}

  <ActivityGroupList />
</ErrorBoundary>
```

---

### Form Validation Errors

```svelte
<script lang="ts">
  let formData = $state({ name: '', description: '' })
  let errors = $state<Record<string, string>>({})
  let submitError = $state<string | null>(null)

  async function handleSubmit() {
    // Clear previous errors
    errors = {}
    submitError = null

    // Validate
    if (!formData.name) {
      errors.name = 'Name is required'
    }
    if (formData.name.length > 100) {
      errors.name = 'Name must be 100 characters or less'
    }

    if (Object.keys(errors).length > 0) return

    // Submit
    try {
      await invoke('create_activity_group', { input: formData })
      // Success - redirect or show success message
    } catch (err) {
      submitError = String(err)
    }
  }
</script>

<form onsubmit={handleSubmit}>
  <Input
    label="Group Name"
    bind:value={formData.name}
    error={errors.name}
    required
  />

  {#if submitError}
    <Alert type="error">{submitError}</Alert>
  {/if}

  <Button type="submit">Create Group</Button>
</form>
```

---

## Quick Reference

### Component Checklist

When creating a new component:

- [ ] **File Structure** - Component + test co-located
- [ ] **Props** - TypeScript types with defaults
- [ ] **Composition** - Slots/snippets for flexibility
- [ ] **Validation** - Runtime validation if needed (Zod)
- [ ] **Tests** - Rendering, interactions, accessibility, edge cases
- [ ] **Accessibility** - Keyboard nav, ARIA, focus management, color contrast
- [ ] **Error Handling** - Graceful degradation
- [ ] **Documentation** - JSDoc comments for props

---

## Resources

- **Testing Library Svelte:** https://testing-library.com/docs/svelte-testing-library/intro/
- **Vitest Docs:** https://vitest.dev/
- **WCAG Guidelines:** https://www.w3.org/WAI/WCAG21/quickref/
- **ARIA Practices:** https://www.w3.org/WAI/ARIA/apg/
- **Melt UI Accessibility:** https://melt-ui.com/docs/accessibility
- **Zod Validation:** https://zod.dev/
