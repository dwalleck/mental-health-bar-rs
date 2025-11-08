# Svelte 5 Best Practices: Avoiding Common Linting Errors

*Based on mental-health-bar-rs codebase patterns*

## 1. State Management: Runes, Not Stores

**Use Svelte 5 runes for component state:**

```svelte
<script lang="ts">
  // ✅ CORRECT: Use runes for reactive state
  let count = $state(0)
  let doubled = $derived(count * 2)

  // ❌ WRONG: Don't use let for reactive values
  let count = 0  // Won't trigger reactivity
  $: doubled = count * 2  // Old Svelte 4 syntax
</script>
```

**$bindable for two-way binding:**

```svelte
<script lang="ts">
  interface Props {
    selectedItems?: T[]
    sortKey?: string
  }

  let {
    selectedItems = $bindable([]),  // Parent can bind to this
    sortKey = $bindable(''),
  }: Props = $props()
</script>
```

**Why:** Svelte 5 runes provide better TypeScript support and clearer reactivity semantics than stores or `$:` labels.

## 2. Cannot Bind to Derived Values

**Common ESLint Error:** `Cannot bind to constant`

```svelte
<script lang="ts">
  let items = $state([1, 2, 3])

  // selectAll is DERIVED - computed from items
  const selectAll = $derived(items.length > 0 && items.every(i => i > 0))
</script>

<!-- ❌ WRONG: bind:checked on derived value -->
<input type="checkbox" bind:checked={selectAll} />

<!-- ✅ CORRECT: Use checked (one-way) and onchange -->
<input
  type="checkbox"
  checked={selectAll}
  onchange={toggleSelectAll}
/>
```

**Why:** Derived values are read-only. Use one-way binding with event handlers for computed state.

## 3. Snippets Replace Slots

**Common ESLint Warning:** `Using <slot> is deprecated. Use {@render ...} tags instead`

```svelte
<!-- ❌ OLD (Svelte 4): -->
<div>
  <slot />
  <slot name="footer" />
</div>

<!-- ✅ NEW (Svelte 5): -->
<script lang="ts">
  interface Props {
    children?: import('svelte').Snippet
    footer?: import('svelte').Snippet
  }

  let { children, footer }: Props = $props()
</script>

<div>
  {@render children?.()}
  {@render footer?.()}
</div>
```

**Defining reusable snippets:**

```svelte
{#snippet SidebarNav(isMobile: boolean)}
  <nav>
    <!-- Markup here -->
  </nav>
{/snippet}

<!-- Use it: -->
{@render SidebarNav(true)}
{@render SidebarNav(false)}
```

**Why:** Snippets provide better type safety and more flexible composition than slots.

## 4. Snippet Naming Rules

**Common ESLint Error:** `Expected token (` when using hyphens

```svelte
<!-- ❌ WRONG: Hyphens not allowed in snippet names -->
{#snippet section-0()}
  <div>Content</div>
{/snippet}

<!-- ✅ CORRECT: Use underscores or camelCase -->
{#snippet section_0()}
  <div>Content</div>
{/snippet}

{#snippet sectionZero()}
  <div>Content</div>
{/snippet}
```

**Dynamic snippet access:**

```svelte
<script lang="ts">
  interface SectionSlots {
    [key: `section_${number}`]: import('svelte').Snippet | undefined
  }

  let { ...slots }: SectionSlots = $props()
</script>

{@render slots[`section_${i}`]?.()}
```

**Why:** JavaScript identifiers cannot contain hyphens. Use underscores for dynamic access patterns.

## 5. TypeScript: No `any` Types

**Common ESLint Error:** `@typescript-eslint/no-explicit-any`

```svelte
<script lang="ts">
  // ❌ WRONG: Using any
  function handleClick(item: any) {
    console.log(item.id)
  }

  const data: any[] = []

  // ✅ CORRECT: Define proper interfaces
  interface TableItem {
    id: number
    name: string
    status: string
  }

  function handleClick(item: TableItem) {
    console.log(item.id)  // Type-safe!
  }

  const data: TableItem[] = []
</script>
```

**Using generics for reusable components:**

```svelte
<script lang="ts" generics="T extends Record<string, unknown>">
  interface Props<T> {
    items: T[]
    onSelect: (item: T) => void
  }

  let { items, onSelect }: Props<T> = $props()
</script>
```

**Why:** TypeScript catches bugs at compile time. `any` disables all type checking.

## 6. @const Placement Rules

**Common ESLint Error:** `{@const} must be immediate child of {#snippet}, {#if}, {#each}, etc.`

```svelte
<!-- ❌ WRONG: @const directly in <div> -->
<div>
  {@const isActive = item.status === 'active'}
  <span>{isActive}</span>
</div>

<!-- ✅ CORRECT: Wrap in {#if true} block -->
<div>
  {#if true}
    {@const isActive = item.status === 'active'}
    <span>{isActive}</span>
  {/if}
</div>

<!-- ✅ BETTER: Use in proper control flow -->
{#each items as item}
  {@const isActive = item.status === 'active'}
  <div class={isActive ? 'active' : ''}>{item.name}</div>
{/each}
```

**Why:** `@const` is scoped to control flow blocks, not arbitrary markup.

## 7. Event Handlers Must Return void

**Common ESLint Error:** Event handler should not return boolean

```svelte
<script lang="ts">
  // ❌ WRONG: Returning boolean
  function handleClick(e: Event) {
    console.log('clicked')
    return false  // ESLint error!
  }

  // ✅ CORRECT: Return void, use preventDefault if needed
  function handleClick(e: Event) {
    e.preventDefault()
    console.log('clicked')
  }
</script>

<button onclick={handleClick}>Click</button>
```

**Inline handlers:**

```svelte
<!-- ❌ WRONG -->
<button onclick={() => { doSomething(); return false }}>

<!-- ✅ CORRECT -->
<button onclick={() => doSomething()}>
<button onclick={(e) => { e.preventDefault(); doSomething() }}>
```

**Why:** Returning values from event handlers is unexpected behavior in modern frameworks.

## 8. Accessibility Violations

**Common ESLint Warnings from svelte-check:**

```svelte
<!-- ❌ WRONG: Click handler without keyboard support -->
<div onclick={() => open()}>Click me</div>

<!-- ✅ CORRECT: Use button or add keyboard handler -->
<button onclick={() => open()}>Click me</button>

<!-- Or if you must use div: -->
<div
  role="button"
  tabindex="0"
  onclick={() => open()}
  onkeydown={(e) => e.key === 'Enter' && open()}
>
  Click me
</div>
```

**ARIA labels for icon-only buttons:**

```svelte
<!-- ❌ WRONG: No accessible label -->
<button onclick={toggleSidebar}>
  <svg>...</svg>
</button>

<!-- ✅ CORRECT: Add aria-label -->
<button onclick={toggleSidebar} aria-label="Toggle sidebar">
  <svg aria-hidden="true">...</svg>
</button>
```

**Form labels must be associated:**

```svelte
<!-- ❌ WRONG: Label without for attribute -->
<label>Username</label>
<input type="text" />

<!-- ✅ CORRECT: Use for + id -->
<label for="username">Username</label>
<input type="text" id="username" />

<!-- ✅ ALSO CORRECT: Wrap input in label -->
<label>
  Username
  <input type="text" />
</label>
```

**Why:** Accessibility is not optional. Screen readers and keyboard navigation depend on proper semantics.

## 9. $effect Cleanup Pattern

**Always clean up effects that use async operations:**

```svelte
<script lang="ts">
  let data = $state([])

  $effect(() => {
    let isMounted = true  // Track component lifecycle

    async function loadData() {
      const result = await fetchData()

      if (!isMounted) return  // Don't update if unmounted

      data = result
    }

    loadData()

    // Cleanup function
    return () => {
      isMounted = false
    }
  })
</script>
```

**Why:** Prevents "Cannot set state on unmounted component" errors and memory leaks.

## 10. Event Handler Naming

**Use lowercase event attributes (Svelte 5 convention):**

```svelte
<!-- ❌ WRONG: PascalCase (React-style) -->
<button onClick={handleClick}>

<!-- ✅ CORRECT: lowercase (Svelte convention) -->
<button onclick={handleClick}>
```

**All event handlers:**
- `onclick` not `onClick`
- `onchange` not `onChange`
- `onsubmit` not `onSubmit`
- `onkeydown` not `onKeyDown`

**Why:** Svelte uses lowercase to match DOM standards. PascalCase is a React convention.

## 11. Conditional Classes with Ternaries

**Prefer class directive for single conditions, ternary for multiple:**

```svelte
<!-- ✅ GOOD: Single condition -->
<div class:active={isActive}>

<!-- ✅ GOOD: Multiple conditions/values -->
<div class="base {isActive ? 'active' : 'inactive'}">

<!-- ❌ AVOID: Overly complex -->
<div class="{condition1 ? 'a' : ''} {condition2 ? 'b' : ''} {condition3 ? 'c' : ''}">

<!-- ✅ BETTER: -->
<div class="base" class:a={condition1} class:b={condition2} class:c={condition3}>
```

**Why:** `class:` directive is cleaner for boolean flags. Ternaries for mutually exclusive states.

## 12. Component Prop Destructuring

**Always use $props() for prop access:**

```svelte
<script lang="ts">
  interface Props {
    title: string
    count?: number
  }

  // ✅ CORRECT: Destructure with $props()
  let { title, count = 0 }: Props = $props()

  // ❌ WRONG: Direct export (Svelte 4 syntax)
  export let title: string
  export let count = 0
</script>
```

**Rest props pattern:**

```svelte
<script lang="ts">
  let { title, ...restProps }: Props = $props()
</script>

<div {...restProps}>
  {title}
</div>
```

**Why:** `$props()` provides better TypeScript integration and matches Svelte 5's runes system.

## Quick Reference Checklist

Before committing, verify:

- [ ] No `bind:` on `$derived` values
- [ ] All snippets use underscores/camelCase (no hyphens)
- [ ] Replaced `<slot>` with snippets + `{@render}`
- [ ] No `any` types (use interfaces or generics)
- [ ] Event handlers lowercase (`onclick` not `onClick`)
- [ ] Event handlers return void (no `return false`)
- [ ] `@const` inside control flow blocks
- [ ] Click handlers have keyboard alternatives
- [ ] Icon-only buttons have `aria-label`
- [ ] Form labels properly associated with inputs
- [ ] `$effect` has cleanup function for async ops
- [ ] Using `$state`, `$derived`, `$props` (not old syntax)

## Run Checks Locally

```bash
# Type check
npm run check

# Lint
npm run lint

# Format
npm run format
```

**Why:** Catch issues before CI/CD. Fast feedback loop improves development speed.
