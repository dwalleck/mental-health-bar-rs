# Catalyst UI Integration Specification

**Last Updated:** 2025-11-07
**Status:** Phase 0 - Specifications
**Related:** tailwind4-design-system.md, svelte5-architecture.md, REVISED-plan.md (Week 7-8)

**Applies To Tasks:**
- **Week 7, Tasks 7.7-7.13**: Build 5 core components (Button, Input, Card, Badge, Alert) - see Priority Matrix
- **Week 8, Tasks 8.1-8.8**: Build extended components + integrate Melt UI (Select, Checkbox, Radio)
- **Week 8, Tasks 8.1-8.8**: Migrate pages following migration strategy and checklist
- **Week 10, Task 10.6**: Test Catalyst UI components for visual parity (70-80%)

---

## Overview

This document provides the **practical integration strategy** for adopting Catalyst UI design patterns in the mental-health-bar-rs Svelte application. While `tailwind4-design-system.md` specifies the design tokens and CSS, this document answers:

- **What** Catalyst components to extract
- **How** to translate React/Headless UI patterns to Svelte
- **When** to use Melt UI vs custom implementation
- **Why** we're building some components and skipping others

**Integration Goal**: 70-80% visual parity with Catalyst demo for core UI components, creating a professional health tracking interface.

---

## Table of Contents

1. [Catalyst Component Inventory](#catalyst-component-inventory)
2. [Component Priority Matrix](#component-priority-matrix)
3. [React → Svelte Translation Guide](#react--svelte-translation-guide)
4. [Melt UI Integration Strategy](#melt-ui-integration-strategy)
5. [Visual Parity Metrics](#visual-parity-metrics)
6. [Phased Rollout Plan](#phased-rollout-plan)
7. [Migration Strategy](#migration-strategy)
8. [Testing and Validation](#testing-and-validation)

---

## Catalyst Component Inventory

### Complete Catalyst UI Component List

Based on Catalyst UI (Tailwind UI's React component library), here's what's available:

#### Form Components
- **Button** - Solid, outline, plain variants
- **Input** - Text, email, password, number
- **Textarea** - Multi-line text input
- **Select** - Native and custom dropdowns
- **Checkbox** - Single and group checkboxes
- **Radio Group** - Radio button groups
- **Switch** - Toggle switches
- **Fieldset** - Form field groupings
- **Field** - Label + input wrapper

#### Navigation
- **Navbar** - Top navigation bar
- **Sidebar** - Collapsible side navigation
- **Dropdown Menu** - Context menus
- **Tabs** - Horizontal tab navigation
- **Breadcrumbs** - Navigation breadcrumbs
- **Pagination** - Page navigation

#### Data Display
- **Table** - Data tables with sorting/filtering
- **Badge** - Status/tag badges
- **Avatar** - User profile images
- **Divider** - Section separators
- **Description List** - Key-value pairs
- **Stat Card** - Metric displays

#### Feedback
- **Alert** - Info, success, warning, error messages
- **Toast** - Notification popups
- **Progress** - Progress bars/spinners
- **Skeleton** - Loading placeholders

#### Overlay
- **Dialog/Modal** - Modal overlays
- **Popover** - Floating popovers
- **Tooltip** - Hover tooltips
- **Sheet** - Slide-out panels

#### Layout
- **Card** - Content containers
- **Stack** - Vertical/horizontal stacks
- **Container** - Max-width wrappers
- **Grid** - Responsive grids

---

## Component Priority Matrix

### Decision Framework

| Criteria | Extract & Build | Use Melt UI | Skip/Defer |
|----------|----------------|-------------|------------|
| **Complexity** | Simple (CSS only) | Complex (state, a11y, keyboard) | Very complex |
| **Usage Frequency** | High (10+ uses) | Medium (5-10 uses) | Low (<5 uses) |
| **Customization Need** | High | Medium | Low |
| **Melt UI Availability** | No equivalent | Good equivalent | Perfect equivalent |

### Week 7: Core Components (Extract & Build)

**Why build these?** High frequency, Catalyst provides clear CSS patterns, minimal interactivity.

| Component | Complexity | Usage | Decision | Rationale |
|-----------|------------|-------|----------|-----------|
| **Button** | Low | Very High (50+ uses) | ✅ **BUILD** | Simple CSS, used everywhere, needs customization |
| **Input** | Low | Very High (30+ uses) | ✅ **BUILD** | Forms are core, need tight control |
| **Card** | Low | High (20+ uses) | ✅ **BUILD** | Layout primitive, simple structure |
| **Badge** | Low | Medium (15+ uses) | ✅ **BUILD** | Status indicators, just CSS |
| **Alert** | Low | Medium (10+ uses) | ✅ **BUILD** | User feedback, simple structure |

**Week 7 Total**: 5 components

---

### Week 8: Extended Components (Mix of Build + Melt UI)

| Component | Complexity | Usage | Decision | Rationale |
|-----------|------------|-------|----------|-----------|
| **Textarea** | Low | Medium (8 uses) | ✅ **BUILD** | Just styled `<textarea>`, follows Input pattern |
| **Checkbox** | Medium | Medium (10 uses) | 🔷 **MELT UI** | Complex a11y, indeterminate state |
| **Radio Group** | Medium | Low (5 uses) | 🔷 **MELT UI** | Keyboard navigation, ARIA roles |
| **Select** | High | Medium (8 uses) | 🔷 **MELT UI** | Dropdown state, keyboard, accessibility |
| **Avatar** | Low | Medium (12 uses) | ✅ **BUILD** | Image + fallback, simple |
| **Divider** | Low | High (15 uses) | ✅ **BUILD** | Just styled `<hr>` |

**Week 8 Total**: 6 components (3 build, 3 Melt UI)

---

### Future/Deferred (v1.1+)

| Component | Complexity | Usage | Decision | Rationale |
|-----------|------------|-------|----------|-----------|
| **Table** | High | Low (3 uses) | 🔷 **MELT UI** | Sorting, filtering = complex state |
| **Dialog/Modal** | High | Medium (6 uses) | 🔷 **MELT UI** | Focus trap, overlay, animations |
| **Dropdown Menu** | High | Medium (8 uses) | 🔷 **MELT UI** | Nested menus, keyboard nav |
| **Tabs** | Medium | Low (4 uses) | 🔷 **MELT UI** | Keyboard navigation, ARIA |
| **Toast** | Medium | High (already exists) | ✅ **KEEP EXISTING** | Custom store works well |
| **Popover** | High | Low (2 uses) | 🔷 **MELT UI** | Positioning, focus management |
| **Tooltip** | Medium | Low (5 uses) | 🔷 **MELT UI** | Hover delay, positioning |
| **Switch** | Medium | Low (3 uses) | 🔷 **MELT UI** | Toggle animations, a11y |
| **Pagination** | Medium | Low (2 uses) | ⏸️ **DEFER** | Not needed yet |
| **Breadcrumbs** | Low | Low (1 use) | ⏸️ **DEFER** | Not needed yet |
| **Progress** | Low | Medium (5 uses) | ⏸️ **DEFER** | Add when needed |
| **Skeleton** | Low | Low (3 uses) | ⏸️ **DEFER** | Nice-to-have |

---

## React → Svelte Translation Guide

### Pattern Mapping

Catalyst uses **React + Headless UI**. Here's how to translate to **Svelte 5 + Melt UI**:

#### 1. Simple Components (No State)

**React/Catalyst Pattern:**
```jsx
// Catalyst Button (React)
import { Button } from '@catalyst/react'

<Button variant="solid" color="blue" onClick={handleClick}>
  Save
</Button>
```

**Svelte Translation:**
```svelte
<!-- Our Button.svelte -->
<script lang="ts">
  let { variant = 'solid', color = 'blue', onclick, children } = $props()
</script>

<button class="btn btn-{variant}-{color}" {onclick}>
  {@render children()}
</button>
```

**Key Differences:**
- React: Props as component props
- Svelte: `$props()` destructuring
- React: `children` prop
- Svelte: `{@render children()}` or slots

---

#### 2. Form Inputs (Controlled State)

**React/Catalyst Pattern:**
```jsx
// Catalyst Input (React)
import { Input } from '@catalyst/react'

const [value, setValue] = useState('')

<Input
  value={value}
  onChange={(e) => setValue(e.target.value)}
  label="Email"
  error={errors.email}
/>
```

**Svelte Translation:**
```svelte
<!-- Our Input.svelte -->
<script lang="ts">
  let { label, error, ...rest } = $props()
</script>

{#if label}
  <label class="input-label">{label}</label>
{/if}

<input
  class="input {error ? 'input-error' : ''}"
  {...rest}
/>

{#if error}
  <p class="input-error-message">{error}</p>
{/if}

<!-- Parent usage: -->
<script lang="ts">
  let email = $state('')
  let errors = $state({})
</script>

<Input bind:value={email} label="Email" error={errors.email} />
```

**Key Differences:**
- React: Controlled via `value` + `onChange`
- Svelte: Two-way binding with `bind:value`
- React: State with `useState`
- Svelte: State with `$state()`

---

#### 3. Complex Interactive (Headless UI)

**React/Catalyst Pattern (Dropdown):**
```jsx
// Catalyst uses Headless UI
import { Menu } from '@headlessui/react'

<Menu>
  <Menu.Button>Options</Menu.Button>
  <Menu.Items>
    <Menu.Item>
      {({ active }) => (
        <a className={active ? 'bg-blue-500' : ''}>Edit</a>
      )}
    </Menu.Item>
  </Menu.Items>
</Menu>
```

**Svelte Translation (Melt UI):**
```svelte
<!-- Use Melt UI's Select/Menu builder -->
<script lang="ts">
  import { createSelect } from '@melt-ui/svelte'

  const {
    elements: { trigger, menu, option },
    states: { open }
  } = createSelect()
</script>

<button use:melt={$trigger}>Options</button>

{#if $open}
  <div use:melt={$menu}>
    <div use:melt={$option({ value: 'edit' })}>
      Edit
    </div>
  </div>
{/if}
```

**Key Differences:**
- React Headless UI: Render props pattern
- Svelte Melt UI: Action directives (`use:melt`)
- React: `Menu.Items` component
- Svelte: Conditional rendering with `{#if $open}`

---

### Translation Checklist

When translating a Catalyst component:

- [ ] **Props** → `$props()` destructuring
- [ ] **useState** → `$state()`
- [ ] **useEffect** → `$effect()`
- [ ] **children** → `{@render children()}` or slots
- [ ] **className** → `class` (Svelte)
- [ ] **onClick** → `onclick` (lowercase in Svelte)
- [ ] **Controlled inputs** → `bind:value`
- [ ] **Headless UI** → Melt UI builders
- [ ] **Render props** → Svelte actions or conditionals

---

## Melt UI Integration Strategy

### When to Use Melt UI

**Use Melt UI when:**
✅ Component requires complex keyboard navigation (Tab, Arrow keys, Escape)
✅ Component needs ARIA attributes (role, aria-expanded, aria-controls)
✅ Component has focus management (focus trap, return focus)
✅ Component needs positioning (tooltips, popovers, dropdowns)
✅ Component has animations/transitions tied to state
✅ Melt UI provides a well-maintained builder

**Don't use Melt UI when:**
❌ Component is purely CSS (Button, Badge, Card)
❌ Component is a simple wrapper (Input, Textarea)
❌ You need tight control over markup structure
❌ Melt UI builder doesn't exist or is immature

---

### Melt UI Component Mapping

| Need | Melt UI Builder | Use For | Catalyst Equivalent |
|------|----------------|---------|---------------------|
| Dropdown/Select | `createSelect()` | Select, Combobox | `<Listbox>` |
| Menu | `createDropdownMenu()` | Context menus | `<Menu>` |
| Dialog | `createDialog()` | Modals | `<Dialog>` |
| Tooltip | `createTooltip()` | Hover tooltips | `<Tooltip>` (custom) |
| Popover | `createPopover()` | Click popovers | `<Popover>` |
| Tabs | `createTabs()` | Tab navigation | `<TabGroup>` |
| Radio | `createRadioGroup()` | Radio buttons | `<RadioGroup>` |
| Checkbox | `createCheckbox()` | Checkboxes | Custom checkbox |
| Switch | `createSwitch()` | Toggles | `<Switch>` |

**Installation:**
```bash
npm install @melt-ui/svelte
```

---

### Melt UI Integration Pattern

**Example: Select Dropdown (Week 8)**

```svelte
<!-- SelectInput.svelte -->
<script lang="ts">
  import { createSelect, melt } from '@melt-ui/svelte'
  import { Check, ChevronDown } from 'lucide-svelte'

  let {
    options,
    value = $bindable(),
    label,
    placeholder = 'Select option...'
  }: {
    options: { value: string; label: string }[]
    value?: string
    label?: string
    placeholder?: string
  } = $props()

  const {
    elements: { trigger, menu, option },
    states: { open, selected }
  } = createSelect({
    defaultSelected: value ? { value, label: '' } : undefined,
    onSelectedChange: ({ next }) => {
      value = next?.value
      return next
    }
  })
</script>

{#if label}
  <label class="input-label">{label}</label>
{/if}

<!-- Trigger Button (Catalyst style) -->
<button
  use:melt={$trigger}
  class="input flex items-center justify-between"
>
  <span>{$selected?.label ?? placeholder}</span>
  <ChevronDown class="size-5 text-zinc-500" />
</button>

<!-- Dropdown Menu (Catalyst style) -->
{#if $open}
  <div
    use:melt={$menu}
    class="absolute z-10 mt-2 w-full rounded-lg bg-white shadow-lg border border-zinc-950/10 dark:bg-zinc-900 dark:border-white/15"
  >
    {#each options as opt}
      <div
        use:melt={$option({ value: opt.value, label: opt.label })}
        class="flex items-center justify-between px-3.5 py-2.5 text-sm/6 hover:bg-zinc-100 dark:hover:bg-zinc-800"
      >
        <span>{opt.label}</span>
        {#if $selected?.value === opt.value}
          <Check class="size-4 text-blue-600" />
        {/if}
      </div>
    {/each}
  </div>
{/if}
```

**Usage:**
```svelte
<script lang="ts">
  let selectedPeriod = $state('last-week')
</script>

<SelectInput
  label="Period"
  bind:value={selectedPeriod}
  options={[
    { value: 'last-week', label: 'Last week' },
    { value: 'last-month', label: 'Last month' },
    { value: 'last-quarter', label: 'Last quarter' }
  ]}
/>
```

**Benefits:**
- ✅ Full keyboard navigation (Arrow Up/Down, Enter, Escape)
- ✅ ARIA attributes automatically added
- ✅ Focus management handled
- ✅ Catalyst visual styling maintained
- ✅ Svelte 5 reactive with `$bindable()`

---

## Visual Parity Metrics

### 70-80% Parity Definition

**What it means:**
- **70%**: Core visual elements match (colors, spacing, typography, border radius)
- **80%**: Subtle details match (hover states, focus rings, transitions)
- **90%+**: Pixel-perfect match (shadows, micro-interactions, animations)

**We're targeting 70-80%**, meaning:
- ✅ Colors, fonts, spacing exactly match Catalyst
- ✅ Component structure matches (button has same padding, input same height)
- ✅ Dark mode works correctly
- ✅ Hover/focus states visually similar
- ⚠️ Animations may differ (simpler transitions OK)
- ⚠️ Micro-interactions may differ (ripple effects, etc.)
- ⚠️ Advanced shadows/effects may be approximated

---

### Measurement Criteria

**Visual Checklist (per component):**

- [ ] **Colors match** - Zinc palette, blue/red accents identical
- [ ] **Typography matches** - Font size, line height, weight correct
- [ ] **Spacing matches** - Padding, margin, gap values match
- [ ] **Border radius matches** - Rounded corners same
- [ ] **Hover state** - Background change on hover
- [ ] **Focus state** - Blue ring on focus (2px, offset 2px)
- [ ] **Disabled state** - Reduced opacity, cursor disabled
- [ ] **Dark mode** - All states work in dark mode

**Success Metrics:**

| Component | Visual Match Target | Acceptance |
|-----------|-------------------|------------|
| Button | 80% | All 8 checklist items ✅ |
| Input | 80% | All 8 checklist items ✅ |
| Card | 70% | 6/8 checklist items ✅ |
| Badge | 75% | 7/8 checklist items ✅ |
| Alert | 75% | 7/8 checklist items ✅ |

**Testing Method:**
1. Open Catalyst demo side-by-side with your app
2. Compare Button solid/outline/plain in light/dark mode
3. Take screenshots, overlay in image editor
4. Count matching visual elements vs. total elements
5. Percentage = matches / total

---

## Phased Rollout Plan

### Phase 1: Week 7 - Foundation + Core Components

**Goal**: Design tokens in place, 5 core components ready

**Tasks:**
1. ✅ Create all token files (colors, typography, spacing, shadows)
2. ✅ Update `app.css` with CSS variables for dark mode
3. ✅ Build 5 core components:
   - Button (12 variants)
   - Input (all states)
   - Card (header, body, footer)
   - Badge (5 colors)
   - Alert (4 types)
4. ✅ Write component tests
5. ✅ Visual parity check against Catalyst demo

**Deliverables:**
- `src/styles/tokens/*.css` files
- `src/lib/components/ui/Button.svelte`
- `src/lib/components/ui/Input.svelte`
- `src/lib/components/ui/Card.svelte`
- `src/lib/components/ui/Badge.svelte`
- `src/lib/components/ui/Alert.svelte`

**Exit Criteria:**
- ✅ All 5 components match Catalyst visually (70-80%)
- ✅ Dark mode works correctly
- ✅ Component tests pass

---

### Phase 2: Week 8 - Extended Components + Migration

**Goal**: Add 3 Melt UI components, migrate existing pages

**Tasks:**
1. ✅ Install and configure Melt UI
   ```bash
   npm install @melt-ui/svelte
   ```

2. ✅ Build 3 additional components:
   - Textarea (simple build)
   - Avatar (simple build)
   - Divider (simple build)

3. ✅ Integrate 3 Melt UI components:
   - Select (Melt UI Select builder)
   - Checkbox (Melt UI Checkbox builder)
   - Radio Group (Melt UI Radio builder)

4. ✅ Migrate existing pages:
   - Assessment forms → new Input/Textarea
   - All buttons → new Button component
   - Dashboard → new Card component
   - Activity tags → new Badge component
   - Validation messages → new Alert component

5. ✅ Update icon sizing to Catalyst conventions

**Deliverables:**
- 6 additional components
- All pages using new design system
- Visual consistency across app

**Exit Criteria:**
- ✅ All pages migrated to new components
- ✅ No visual regressions
- ✅ Lighthouse accessibility >90
- ✅ Lighthouse performance >90

---

### Phase 3: Future (v1.1+) - Advanced Components

**Goal**: Add complex components as needed

**Candidates:**
- Modal/Dialog (Melt UI)
- Dropdown Menu (Melt UI)
- Tabs (Melt UI)
- Table with sorting
- Tooltip (Melt UI)
- Popover (Melt UI)

**Trigger**: Only when feature requires them

---

## Migration Strategy

### Page-by-Page Migration

**Order of migration (Week 8):**

1. **Assessment Forms** (easiest, most forms)
   - `/assessments/[type]/+page.svelte`
   - Replace `<input>` with `<Input>`
   - Replace `<textarea>` with `<Textarea>`
   - Replace `<button>` with `<Button>`

2. **Dashboard** (high visibility)
   - `/routes/+page.svelte`
   - Replace cards with `<Card>` component
   - Replace buttons with `<Button>`
   - Replace badges with `<Badge>`

3. **Activity Management** (complex forms)
   - `/routes/mood/activities/+page.svelte`
   - All form inputs → new components
   - Activity tags → `<Badge>`

4. **Charts Page** (data visualization)
   - `/routes/charts/+page.svelte`
   - Stat cards → `<Card>`
   - Buttons → `<Button>`

5. **Navigation** (global)
   - Update last to avoid breaking other pages
   - Navigation buttons → `<Button>`

---

### Migration Checklist (per page)

- [ ] Replace `<button>` → `<Button variant="..." color="...">`
- [ ] Replace `<input type="text">` → `<Input label="..." />`
- [ ] Replace `<textarea>` → `<Textarea label="..." />`
- [ ] Replace custom cards → `<Card>`
- [ ] Replace status badges → `<Badge color="...">`
- [ ] Replace alerts → `<Alert type="...">`
- [ ] Update icon classes → `size-4`, `size-5`, `size-6`
- [ ] Test in light mode
- [ ] Test in dark mode
- [ ] Test responsive (mobile, tablet, desktop)
- [ ] Test keyboard navigation
- [ ] Test screen reader (basic check)

---

### Backwards Compatibility

**Strategy**: Gradual migration, not breaking changes

**Approach:**
1. Keep old components during migration
2. Migrate page-by-page
3. Delete old components only when 100% migrated
4. Don't break existing functionality

**Example:**
```svelte
<!-- Old button (keep during migration) -->
<button class="old-button">Click me</button>

<!-- New Button (use in migrated pages) -->
<Button variant="solid" color="blue">Click me</Button>

<!-- After migration: delete old-button class from CSS -->
```

---

## Testing and Validation

### Visual Regression Testing

**Tools:**
- Playwright component testing
- Chromatic (optional, costs money)
- Manual screenshot comparison

**Test Cases:**
1. **Button variants** (12 total)
   - Screenshot each variant in light/dark mode
   - Compare to Catalyst demo screenshots

2. **Input states** (5 states)
   - Default, hover, focus, error, disabled
   - Light + dark mode

3. **Card layouts**
   - With header, without header
   - With footer, without footer
   - Light + dark mode

4. **Badge colors** (5 colors)
   - Screenshot each color
   - Light + dark mode

5. **Alert types** (4 types)
   - Info, success, warning, error
   - Light + dark mode

**Process:**
```bash
# Take screenshots of components
npm run test:visual

# Compare with baseline
npm run test:visual:compare
```

---

### Accessibility Testing

**Tools:**
- Axe DevTools (Chrome extension)
- Lighthouse (Chrome DevTools)
- Manual keyboard testing

**Checklist:**
- [ ] **Keyboard navigation** - Tab, Enter, Escape, Arrow keys work
- [ ] **Focus indicators** - Visible focus ring on all interactive elements
- [ ] **Color contrast** - WCAG AA 4.5:1 for text, 3:1 for UI elements
- [ ] **Screen reader** - All interactive elements have labels
- [ ] **ARIA attributes** - Melt UI components have correct ARIA

**Target:**
- Lighthouse accessibility score: **>95**
- Axe DevTools: **0 critical issues**

---

### Functional Testing

**Test Categories:**

1. **Component isolation** (Vitest + Testing Library)
   ```typescript
   // Button.test.ts
   import { render, fireEvent } from '@testing-library/svelte'
   import Button from './Button.svelte'

   test('Button renders with correct variant class', () => {
     const { container } = render(Button, {
       props: { variant: 'solid', color: 'blue', children: 'Click' }
     })
     expect(container.querySelector('.btn-solid-blue')).toBeInTheDocument()
   })

   test('Button calls onclick when clicked', () => {
     const handleClick = vi.fn()
     const { getByText } = render(Button, {
       props: { onclick: handleClick, children: 'Click' }
     })
     fireEvent.click(getByText('Click'))
     expect(handleClick).toHaveBeenCalledOnce()
   })
   ```

2. **Integration testing** (Playwright)
   ```typescript
   // assessment-form.spec.ts
   test('User can fill out assessment form', async ({ page }) => {
     await page.goto('/assessments/phq-9')

     // New Input components should work
     await page.fill('input[name="question1"]', '2')
     await page.fill('input[name="question2"]', '3')

     // New Button component should work
     await page.click('button:has-text("Submit")')

     // Should navigate to results
     await expect(page).toHaveURL(/\/result/)
   })
   ```

3. **E2E testing** (User flows)
   - Create activity → Log activity → View report (all new components)
   - Mood check-in → Select activities → Submit (new form components)
   - Assessment form → Save draft → Resume → Complete (new inputs/buttons)

---

### Performance Testing

**Metrics to track:**

| Metric | Before Catalyst | After Catalyst | Target |
|--------|----------------|----------------|--------|
| Lighthouse Performance | Baseline | ? | >90 |
| First Contentful Paint | Baseline | ? | <1.5s |
| Time to Interactive | Baseline | ? | <3s |
| Bundle size (CSS) | Baseline | ? | <50KB gzipped |
| Bundle size (JS) | Baseline | ? | No increase |

**Test Process:**
1. Run Lighthouse audit before migration (baseline)
2. Run Lighthouse audit after each page migration
3. If performance drops >5 points, investigate
4. Optimize if needed (purge CSS, tree-shake Melt UI)

---

## Success Criteria

### Week 7 Success

- ✅ All design tokens implemented (colors, typography, spacing, shadows)
- ✅ 5 core components built and tested
- ✅ Visual parity 70-80% with Catalyst demo
- ✅ Dark mode works correctly
- ✅ Component tests pass

### Week 8 Success

- ✅ 6 additional components (3 build, 3 Melt UI)
- ✅ All pages migrated to new design system
- ✅ No visual regressions
- ✅ Lighthouse accessibility >90
- ✅ Lighthouse performance >90
- ✅ All E2E tests pass

### v0.4 Ship Criteria

- ✅ All components using Catalyst-inspired design
- ✅ Visual consistency across all pages
- ✅ Dark mode functional everywhere
- ✅ Accessibility validated
- ✅ Performance maintained or improved
- ✅ Tests passing (unit, integration, E2E)

---

## Troubleshooting Guide

### Common Issues

#### 1. Melt UI not working

**Symptom:** Dropdown doesn't open, keyboard nav broken

**Fix:**
```svelte
<!-- Make sure you're using the melt action correctly -->
<button use:melt={$trigger}>  <!-- ✅ Correct -->
<button melt={$trigger}>      <!-- ❌ Wrong -->

<!-- Make sure you imported melt -->
import { createSelect, melt } from '@melt-ui/svelte'
```

---

#### 2. Dark mode not applying

**Symptom:** Components look wrong in dark mode

**Fix:**
```css
/* Make sure CSS variables are defined for dark mode */
.dark {
  --surface-primary: theme('colors.zinc.950');  /* ✅ Correct */
}

/* Make sure dark variant is included in component CSS */
@apply dark:bg-zinc-900 dark:text-white;  /* ✅ Correct */
@apply bg-white text-black;  /* ❌ Missing dark variant */
```

---

#### 3. Focus ring not visible

**Symptom:** Can't see focus indicator when tabbing

**Fix:**
```css
/* Make sure focus ring has enough contrast */
@apply focus:ring-2 focus:ring-blue-600 focus:ring-offset-2;  /* ✅ Good contrast */
@apply focus:ring-1 focus:ring-zinc-300;  /* ❌ Low contrast */

/* Make sure offset is visible in dark mode */
@apply focus:ring-offset-white dark:focus:ring-offset-zinc-900;
```

---

#### 4. Component not matching Catalyst

**Symptom:** Button looks different from demo

**Checklist:**
- [ ] Colors exact match? (Check hex values)
- [ ] Padding exact match? (Check px values)
- [ ] Border radius exact match?
- [ ] Font size/weight exact match?
- [ ] Line height exact match?
- [ ] Transitions included?

**Fix:** Compare side-by-side with Catalyst demo, adjust values

---

## Quick Reference

### File Locations

```
src/
├── app.css                         # Main CSS entry
├── styles/
│   ├── tokens/
│   │   ├── colors.css              # Color palette
│   │   ├── typography.css          # Font scale
│   │   ├── spacing.css             # Spacing/radius
│   │   └── shadows.css             # Shadows
│   └── components/
│       ├── button.css              # Button styles
│       ├── input.css               # Input styles
│       ├── card.css                # Card styles
│       ├── badge.css               # Badge styles
│       └── alert.css               # Alert styles
└── lib/
    └── components/
        └── ui/
            ├── Button.svelte       # Button component
            ├── Input.svelte        # Input component
            ├── Card.svelte         # Card component
            ├── Badge.svelte        # Badge component
            ├── Alert.svelte        # Alert component
            ├── SelectInput.svelte  # Melt UI Select
            └── ... (more components)
```

---

## Resources

- **Catalyst Demo:** https://catalyst-demo.tailwindui.com/
- **Melt UI Docs:** https://melt-ui.com/
- **Melt UI Examples:** https://melt-ui.com/docs/builders
- **Tailwind v4 Docs:** https://tailwindcss.com/docs
- **Design Token Spec:** `tailwind4-design-system.md`
- **Svelte Architecture:** `svelte5-architecture.md`
- **Project Plan:** `REVISED-plan.md` (Week 7-8)

---

## Appendix: Component Examples

### Button Usage Examples

```svelte
<!-- Primary action -->
<Button variant="solid" color="blue">Save Changes</Button>

<!-- Secondary action -->
<Button variant="outline" color="zinc">Cancel</Button>

<!-- Danger action -->
<Button variant="solid" color="red">Delete</Button>

<!-- Icon button -->
<Button variant="plain" color="zinc">
  <Icon src={Heart} class="size-5" />
</Button>

<!-- Button with icon -->
<Button variant="solid" color="blue">
  <Icon src={Plus} class="size-4" />
  Add Activity
</Button>
```

---

### Input Usage Examples

```svelte
<!-- Basic input -->
<Input
  label="Email"
  type="email"
  placeholder="you@example.com"
  bind:value={email}
/>

<!-- Input with error -->
<Input
  label="Activity Name"
  bind:value={activityName}
  error={errors.activityName}
  helper="Max 50 characters"
/>

<!-- Disabled input -->
<Input
  label="User ID"
  value={userId}
  disabled
/>
```

---

### Card Usage Examples

```svelte
<!-- Simple card -->
<Card class="card-default">
  <h2 class="card-title">Recent Activity</h2>
  <p>Your mood this week...</p>
</Card>

<!-- Card with header and footer -->
<Card class="card-default">
  <div class="card-header">
    <h2 class="card-title">Assessment Results</h2>
    <p class="card-description">PHQ-9 Score</p>
  </div>

  <div>
    <!-- Body content -->
  </div>

  <div class="card-footer">
    <Button variant="outline" color="zinc">View History</Button>
  </div>
</Card>
```

---

### Melt UI Select Example

```svelte
<SelectInput
  label="Select Period"
  bind:value={selectedPeriod}
  options={[
    { value: 'week', label: 'Last Week' },
    { value: 'month', label: 'Last Month' },
    { value: 'quarter', label: 'Last Quarter' }
  ]}
/>
```
