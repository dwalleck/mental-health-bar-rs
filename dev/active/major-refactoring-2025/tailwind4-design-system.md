# Tailwind v4 + Catalyst UI Design System Specification

**Last Updated:** 2025-11-07
**Status:** Phase 0 - Specifications
**Related:** REVISED-plan.md, svelte5-architecture.md

**Applies To Tasks:**
- **Week 7, Tasks 7.1-7.6**: Foundation - Create all token files (colors, typography, spacing, shadows)
- **Week 7, Tasks 7.7-7.13**: Enhanced Components - Implement Button, Input, Card, Badge, Alert with `@layer components`
- **Week 8, Tasks 8.1-8.8**: Component Migration - Apply design system to all pages

---

## Overview

This document specifies the complete design system for mental-health-bar-rs based on **Catalyst UI** design patterns. It defines all design tokens, component architecture, and implementation patterns for Tailwind CSS v4.

**Key Decisions:**
- **Color Palette:** Full Catalyst Zinc (50-950) + custom mood/assessment accent colors
- **Typography:** Complete Catalyst scale with text/line-height combinations
- **Spacing:** Full Catalyst spacing tokens including fractional values
- **Dark Mode:** CSS custom properties for dynamic theme switching
- **Component Architecture:** `@layer components` with reusable classes
- **Component Scope:** Dashboard suite (Button, Input, Card, Badge, Alert, Modal)
- **Fidelity Goal:** 90%+ visual match to Catalyst UI

---

## Table of Contents

1. [Design Tokens](#design-tokens)
2. [Color System](#color-system)
3. [Typography System](#typography-system)
4. [Spacing and Sizing](#spacing-and-sizing)
5. [Component Architecture](#component-architecture)
6. [Component Specifications](#component-specifications)
7. [Dark Mode Implementation](#dark-mode-implementation)
8. [Icon System](#icon-system)
9. [Responsive Strategy](#responsive-strategy)
10. [File Organization](#file-organization)

---

## Design Tokens

### Token File Structure

```
src/
├── app.css                    # Main entry, imports all tokens
└── styles/
    └── tokens/
        ├── colors.css         # Color palette (zinc + accents)
        ├── typography.css     # Font sizes, line heights, weights
        ├── spacing.css        # Spacing scale, border radius
        ├── shadows.css        # Box shadow tokens
        └── transitions.css    # Animation tokens
```

**Import Order** (in `app.css`):
```css
@import 'tailwindcss';

/* Token imports */
@import './styles/tokens/colors.css';
@import './styles/tokens/typography.css';
@import './styles/tokens/spacing.css';
@import './styles/tokens/shadows.css';
@import './styles/tokens/transitions.css';

/* Component layer */
@import './styles/components/button.css';
@import './styles/components/input.css';
/* ... other components */
```

---

## Color System

### Primary Neutral Palette (Zinc)

**File:** `src/styles/tokens/colors.css`

```css
@theme {
  /* Zinc Palette - Primary Neutrals (from Catalyst) */
  --color-zinc-50: #fafafa;
  --color-zinc-100: #f4f4f5;
  --color-zinc-200: #e4e4e7;
  --color-zinc-300: #d4d4d8;
  --color-zinc-400: #a1a1aa;
  --color-zinc-500: #71717a;
  --color-zinc-600: #52525b;
  --color-zinc-700: #3f3f46;
  --color-zinc-800: #27272a;
  --color-zinc-900: #18181b;
  --color-zinc-950: #09090b;
}
```

**Usage in Components:**
- **Text:** `text-zinc-950` (headings), `text-zinc-500` (muted), `text-zinc-400` (disabled)
- **Backgrounds:** `bg-zinc-100` (page), `bg-white` (cards), `dark:bg-zinc-900`
- **Borders:** `border-zinc-950/10` (light), `border-zinc-200` (subtle)

---

### Accent Colors (Existing + Catalyst)

```css
@theme {
  /* Blue - Primary Actions (Catalyst) */
  --color-blue-50: #eff6ff;
  --color-blue-100: #dbeafe;
  --color-blue-200: #bfdbfe;
  --color-blue-300: #93c5fd;
  --color-blue-400: #60a5fa;
  --color-blue-500: #3b82f6;
  --color-blue-600: #2563eb;
  --color-blue-700: #1d4ed8;
  --color-blue-800: #1e40af;
  --color-blue-900: #1e3a8a;
  --color-blue-950: #172554;

  /* Red - Destructive Actions (Catalyst) */
  --color-red-50: #fef2f2;
  --color-red-100: #fee2e2;
  --color-red-200: #fecaca;
  --color-red-300: #fca5a5;
  --color-red-400: #f87171;
  --color-red-500: #ef4444;
  --color-red-600: #dc2626;
  --color-red-700: #b91c1c;
  --color-red-800: #991b1b;
  --color-red-900: #7f1d1d;
  --color-red-950: #450a0a;

  /* Green - Success States (Catalyst) */
  --color-green-50: #f0fdf4;
  --color-green-100: #dcfce7;
  --color-green-200: #bbf7d0;
  --color-green-300: #86efac;
  --color-green-400: #4ade80;
  --color-green-500: #22c55e;
  --color-green-600: #16a34a;
  --color-green-700: #15803d;
  --color-green-800: #166534;
  --color-green-900: #14532d;
  --color-green-950: #052e16;

  /* Domain-Specific Colors (Keep Existing) */
  --color-mood-very-bad: #ef4444;      /* red-500 */
  --color-mood-bad: #f97316;           /* orange-500 */
  --color-mood-neutral: #fcd34d;       /* yellow-300 */
  --color-mood-good: #84cc16;          /* lime-500 */
  --color-mood-very-good: #22c55e;     /* green-500 */

  --color-assessment-minimal: #10b981;  /* emerald-500 */
  --color-assessment-mild: #f59e0b;     /* amber-500 */
  --color-assessment-moderate: #f97316; /* orange-500 */
  --color-assessment-severe: #ef4444;   /* red-500 */
}
```

**Semantic Color Aliases:**
```css
@theme {
  /* Semantic Mappings (for clarity) */
  --color-primary: var(--color-blue-600);
  --color-primary-hover: var(--color-blue-700);
  --color-danger: var(--color-red-600);
  --color-danger-hover: var(--color-red-700);
  --color-success: var(--color-green-600);
  --color-success-hover: var(--color-green-700);
}
```

---

## Typography System

### Font Scale (Catalyst Text/Line-Height)

**File:** `src/styles/tokens/typography.css`

```css
@theme {
  /* Font Sizes with Line Heights (Catalyst Convention) */
  /* Format: text-{size}/{line-height} */

  /* xs = 0.75rem (12px) with line-height variants */
  --font-size-xs: 0.75rem;
  --line-height-xs-4: 1rem;        /* text-xs/4 */

  /* sm = 0.875rem (14px) */
  --font-size-sm: 0.875rem;
  --line-height-sm-5: 1.25rem;     /* text-sm/5 */
  --line-height-sm-6: 1.5rem;      /* text-sm/6 (most common) */

  /* base = 1rem (16px) */
  --font-size-base: 1rem;
  --line-height-base-6: 1.5rem;    /* text-base/6 */
  --line-height-base-7: 1.75rem;   /* text-base/7 */

  /* lg = 1.125rem (18px) */
  --font-size-lg: 1.125rem;
  --line-height-lg-7: 1.75rem;     /* text-lg/7 */
  --line-height-lg-8: 2rem;        /* text-lg/8 */

  /* xl = 1.25rem (20px) */
  --font-size-xl: 1.25rem;
  --line-height-xl-7: 1.75rem;     /* text-xl/7 */
  --line-height-xl-8: 2rem;        /* text-xl/8 */

  /* 2xl = 1.5rem (24px) */
  --font-size-2xl: 1.5rem;
  --line-height-2xl-8: 2rem;       /* text-2xl/8 */
  --line-height-2xl-9: 2.25rem;    /* text-2xl/9 */

  /* 3xl = 1.875rem (30px) */
  --font-size-3xl: 1.875rem;
  --line-height-3xl-9: 2.25rem;    /* text-3xl/9 */

  /* Font Weights */
  --font-weight-normal: 400;
  --font-weight-medium: 500;
  --font-weight-semibold: 600;
  --font-weight-bold: 700;
}
```

**Usage Guidelines:**

| Element | Class | Purpose |
|---------|-------|---------|
| Body text | `text-base/6` | Standard paragraph text (16px/24px) |
| Small text | `text-sm/6` | Labels, captions, secondary info (14px/24px) |
| Tiny text | `text-xs/4` | Timestamps, metadata (12px/16px) |
| Headings (H1) | `text-3xl/9 font-semibold` | Page titles (30px/36px) |
| Headings (H2) | `text-2xl/8 font-semibold` | Section titles (24px/32px) |
| Headings (H3) | `text-xl/8 font-medium` | Subsection titles (20px/32px) |
| Button text | `text-sm/6 font-semibold` | Buttons and CTAs (14px/24px) |

**Tailwind v4 Custom Utilities** (for text/line-height combinations):
```css
@layer utilities {
  .text-xs\/4 { font-size: 0.75rem; line-height: 1rem; }
  .text-sm\/5 { font-size: 0.875rem; line-height: 1.25rem; }
  .text-sm\/6 { font-size: 0.875rem; line-height: 1.5rem; }
  .text-base\/6 { font-size: 1rem; line-height: 1.5rem; }
  .text-base\/7 { font-size: 1rem; line-height: 1.75rem; }
  .text-lg\/7 { font-size: 1.125rem; line-height: 1.75rem; }
  .text-lg\/8 { font-size: 1.125rem; line-height: 2rem; }
  .text-xl\/7 { font-size: 1.25rem; line-height: 1.75rem; }
  .text-xl\/8 { font-size: 1.25rem; line-height: 2rem; }
  .text-2xl\/8 { font-size: 1.5rem; line-height: 2rem; }
  .text-2xl\/9 { font-size: 1.5rem; line-height: 2.25rem; }
  .text-3xl\/9 { font-size: 1.875rem; line-height: 2.25rem; }
}
```

---

## Spacing and Sizing

### Spacing Scale (Catalyst Extended)

**File:** `src/styles/tokens/spacing.css`

```css
@theme {
  /* Base Tailwind Scale (Keep) */
  --spacing-0: 0;
  --spacing-0_5: 0.125rem;   /* 2px */
  --spacing-1: 0.25rem;      /* 4px */
  --spacing-1_5: 0.375rem;   /* 6px */
  --spacing-2: 0.5rem;       /* 8px */
  --spacing-2_5: 0.625rem;   /* 10px */
  --spacing-3: 0.75rem;      /* 12px */

  /* Catalyst Additions - Fractional Values */
  --spacing-3_5: 0.875rem;   /* 14px - NEW */

  --spacing-4: 1rem;         /* 16px */
  --spacing-4_5: 1.125rem;   /* 18px - NEW */
  --spacing-5: 1.25rem;      /* 20px */
  --spacing-6: 1.5rem;       /* 24px */
  --spacing-7: 1.75rem;      /* 28px */
  --spacing-8: 2rem;         /* 32px */
  --spacing-9: 2.25rem;      /* 36px */
  --spacing-10: 2.5rem;      /* 40px */
  --spacing-11: 2.75rem;     /* 44px */
  --spacing-12: 3rem;        /* 48px */
  --spacing-14: 3.5rem;      /* 56px */
  --spacing-16: 4rem;        /* 64px */
  --spacing-20: 5rem;        /* 80px */
  --spacing-24: 6rem;        /* 96px */

  /* Border Radius (Catalyst Values) */
  --radius-none: 0;
  --radius-sm: 0.125rem;     /* 2px */
  --radius-DEFAULT: 0.375rem; /* 6px - Catalyst default */
  --radius-md: 0.5rem;       /* 8px */
  --radius-lg: 0.75rem;      /* 12px */
  --radius-xl: 1rem;         /* 16px */
  --radius-2xl: 1.5rem;      /* 24px */
  --radius-full: 9999px;     /* Fully rounded */
}
```

**Usage:**
- **Buttons:** `rounded-md` (8px) or `rounded-lg` (12px)
- **Inputs:** `rounded-lg` (12px)
- **Cards:** `rounded-xl` (16px) or `rounded-2xl` (24px)
- **Avatars/Icons:** `rounded-full`

---

### Shadows (Catalyst Depth)

**File:** `src/styles/tokens/shadows.css`

```css
@theme {
  /* Box Shadows (Catalyst Depth System) */
  --shadow-xs: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-sm: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
  --shadow-DEFAULT: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
  --shadow-md: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
  --shadow-lg: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
  --shadow-xl: 0 25px 50px -12px rgb(0 0 0 / 0.25);

  /* Dark Mode Shadows (Stronger) */
  --shadow-dark-xs: 0 1px 2px 0 rgb(0 0 0 / 0.3);
  --shadow-dark-sm: 0 1px 3px 0 rgb(0 0 0 / 0.3), 0 1px 2px -1px rgb(0 0 0 / 0.3);
  --shadow-dark-DEFAULT: 0 4px 6px -1px rgb(0 0 0 / 0.3), 0 2px 4px -2px rgb(0 0 0 / 0.3);
}
```

**Usage:**
- **Dropdown menus:** `shadow-lg`
- **Cards (elevated):** `shadow-sm`
- **Modals:** `shadow-xl`
- **Buttons (hover):** `shadow-sm`

---

## Component Architecture

### Layer Strategy

**Use `@layer components` for all reusable component classes:**

```css
/* src/styles/components/button.css */
@layer components {
  /* Base button styles */
  .btn {
    @apply inline-flex items-center justify-center gap-2;
    @apply px-3.5 py-2.5;
    @apply text-sm/6 font-semibold;
    @apply rounded-lg;
    @apply transition-colors duration-150;
    @apply focus:outline-none focus:ring-2 focus:ring-offset-2;
  }

  /* Variants */
  .btn-solid-zinc {
    @apply bg-zinc-900 text-white;
    @apply hover:bg-zinc-800;
    @apply focus:ring-zinc-900;
    @apply dark:bg-white dark:text-zinc-900 dark:hover:bg-zinc-100;
  }

  .btn-outline-zinc {
    @apply border border-zinc-950/10 text-zinc-950;
    @apply hover:bg-zinc-950/5;
    @apply dark:border-white/15 dark:text-white dark:hover:bg-white/5;
  }

  /* ... more variants */
}
```

**Why `@layer components`?**
- Reusable across multiple components
- Lower specificity than utilities (can override with classes)
- Organized in separate files
- Better than inline utilities for complex patterns

---

## Component Specifications

### Button Component

**File:** `src/styles/components/button.css`

**Variants:** solid, outline, plain
**Colors:** zinc, blue, red, green

**Full Implementation:**

```css
@layer components {
  /* Base Button */
  .btn {
    @apply inline-flex items-center justify-center gap-2;
    @apply px-3.5 py-2.5;
    @apply text-sm/6 font-semibold;
    @apply rounded-lg;
    @apply transition-colors duration-150;
    @apply focus:outline-none focus:ring-2 focus:ring-offset-2;
    @apply disabled:opacity-50 disabled:cursor-not-allowed;
  }

  /* === SOLID VARIANTS === */

  /* Solid Zinc (Primary) */
  .btn-solid-zinc {
    @apply bg-zinc-900 text-white;
    @apply hover:bg-zinc-800 active:bg-zinc-700;
    @apply focus:ring-zinc-900;
    @apply dark:bg-white dark:text-zinc-900;
    @apply dark:hover:bg-zinc-100 dark:active:bg-zinc-200;
    @apply dark:focus:ring-white;
  }

  /* Solid Blue (Primary Action) */
  .btn-solid-blue {
    @apply bg-blue-600 text-white;
    @apply hover:bg-blue-700 active:bg-blue-800;
    @apply focus:ring-blue-600;
  }

  /* Solid Red (Destructive) */
  .btn-solid-red {
    @apply bg-red-600 text-white;
    @apply hover:bg-red-700 active:bg-red-800;
    @apply focus:ring-red-600;
  }

  /* Solid Green (Success) */
  .btn-solid-green {
    @apply bg-green-600 text-white;
    @apply hover:bg-green-700 active:bg-green-800;
    @apply focus:ring-green-600;
  }

  /* === OUTLINE VARIANTS === */

  /* Outline Zinc (Secondary) */
  .btn-outline-zinc {
    @apply border border-zinc-950/10 text-zinc-950;
    @apply hover:bg-zinc-950/5 active:bg-zinc-950/10;
    @apply focus:ring-zinc-900;
    @apply dark:border-white/15 dark:text-white;
    @apply dark:hover:bg-white/5 dark:active:bg-white/10;
  }

  /* Outline Blue */
  .btn-outline-blue {
    @apply border border-blue-600/30 text-blue-600;
    @apply hover:bg-blue-50 active:bg-blue-100;
    @apply focus:ring-blue-600;
    @apply dark:border-blue-400/30 dark:text-blue-400;
    @apply dark:hover:bg-blue-400/10;
  }

  /* Outline Red */
  .btn-outline-red {
    @apply border border-red-600/30 text-red-600;
    @apply hover:bg-red-50 active:bg-red-100;
    @apply focus:ring-red-600;
    @apply dark:border-red-400/30 dark:text-red-400;
    @apply dark:hover:bg-red-400/10;
  }

  /* === PLAIN VARIANTS === */

  /* Plain Zinc (Minimal) */
  .btn-plain-zinc {
    @apply text-zinc-950;
    @apply hover:bg-zinc-950/5 active:bg-zinc-950/10;
    @apply focus:ring-zinc-900;
    @apply dark:text-white dark:hover:bg-white/5;
  }

  /* Plain Blue */
  .btn-plain-blue {
    @apply text-blue-600;
    @apply hover:bg-blue-50 active:bg-blue-100;
    @apply focus:ring-blue-600;
    @apply dark:text-blue-400 dark:hover:bg-blue-400/10;
  }

  /* Plain Red */
  .btn-plain-red {
    @apply text-red-600;
    @apply hover:bg-red-50 active:bg-red-100;
    @apply focus:ring-red-600;
    @apply dark:text-red-400 dark:hover:bg-red-400/10;
  }
}
```

**Svelte Component Usage:**

```svelte
<!-- Button.svelte -->
<script lang="ts">
  let {
    variant = 'solid',
    color = 'zinc',
    type = 'button',
    disabled = false,
    onclick,
    children
  }: {
    variant?: 'solid' | 'outline' | 'plain'
    color?: 'zinc' | 'blue' | 'red' | 'green'
    type?: 'button' | 'submit' | 'reset'
    disabled?: boolean
    onclick?: () => void
    children: any
  } = $props()

  const className = `btn btn-${variant}-${color}`
</script>

<button
  {type}
  {disabled}
  class={className}
  {onclick}
>
  {@render children()}
</button>
```

---

### Input Component

**File:** `src/styles/components/input.css`

**States:** default, hover, focus, error, disabled

```css
@layer components {
  /* Input Base */
  .input {
    @apply block w-full;
    @apply px-3.5 py-2.5;
    @apply text-base/6 text-zinc-950;
    @apply bg-white;
    @apply border border-zinc-950/10;
    @apply rounded-lg;
    @apply shadow-sm;
    @apply transition-colors duration-150;

    /* Focus State */
    @apply focus:outline-none focus:ring-2 focus:ring-blue-600 focus:border-transparent;

    /* Disabled State */
    @apply disabled:bg-zinc-50 disabled:text-zinc-500 disabled:cursor-not-allowed;

    /* Dark Mode */
    @apply dark:bg-white/5 dark:text-white;
    @apply dark:border-white/15;
    @apply dark:focus:ring-blue-500;
    @apply dark:disabled:bg-white/5;
  }

  /* Input with Error */
  .input-error {
    @apply border-red-600 focus:ring-red-600;
    @apply dark:border-red-500 dark:focus:ring-red-500;
  }

  /* Input Label */
  .input-label {
    @apply block text-sm/6 font-medium text-zinc-950 mb-2;
    @apply dark:text-white;
  }

  /* Input Helper Text */
  .input-helper {
    @apply mt-2 text-sm/6 text-zinc-500;
    @apply dark:text-zinc-400;
  }

  /* Input Error Message */
  .input-error-message {
    @apply mt-2 text-sm/6 text-red-600 font-medium;
    @apply dark:text-red-400;
  }
}
```

**Svelte Component:**

```svelte
<!-- Input.svelte -->
<script lang="ts">
  let {
    label,
    error,
    helper,
    disabled = false,
    ...rest
  }: {
    label?: string
    error?: string
    helper?: string
    disabled?: boolean
    [key: string]: any
  } = $props()

  const inputClass = error ? 'input input-error' : 'input'
</script>

{#if label}
  <label class="input-label">
    {label}
  </label>
{/if}

<input
  class={inputClass}
  {disabled}
  {...rest}
/>

{#if error}
  <p class="input-error-message">{error}</p>
{:else if helper}
  <p class="input-helper">{helper}</p>
{/if}
```

---

### Card Component

**File:** `src/styles/components/card.css`

```css
@layer components {
  /* Card Base */
  .card {
    @apply bg-white rounded-2xl shadow-sm;
    @apply border border-zinc-950/10;
    @apply dark:bg-white/5 dark:border-white/15;
  }

  /* Card Padding Variants */
  .card-compact {
    @apply p-4;
  }

  .card-default {
    @apply p-6;
  }

  .card-comfortable {
    @apply p-8;
  }

  /* Card Header */
  .card-header {
    @apply border-b border-zinc-950/10 pb-4 mb-4;
    @apply dark:border-white/15;
  }

  /* Card Title */
  .card-title {
    @apply text-xl/8 font-semibold text-zinc-950;
    @apply dark:text-white;
  }

  /* Card Description */
  .card-description {
    @apply text-sm/6 text-zinc-500 mt-1;
    @apply dark:text-zinc-400;
  }

  /* Card Footer */
  .card-footer {
    @apply border-t border-zinc-950/10 pt-4 mt-4;
    @apply dark:border-white/15;
  }
}
```

---

### Badge Component

**File:** `src/styles/components/badge.css`

```css
@layer components {
  /* Badge Base */
  .badge {
    @apply inline-flex items-center gap-1.5;
    @apply px-2.5 py-0.5;
    @apply text-xs/4 font-medium;
    @apply rounded-md;
  }

  /* Badge Colors */
  .badge-zinc {
    @apply bg-zinc-100 text-zinc-700;
    @apply dark:bg-zinc-800 dark:text-zinc-300;
  }

  .badge-blue {
    @apply bg-blue-100 text-blue-700;
    @apply dark:bg-blue-900/50 dark:text-blue-300;
  }

  .badge-green {
    @apply bg-green-100 text-green-700;
    @apply dark:bg-green-900/50 dark:text-green-300;
  }

  .badge-red {
    @apply bg-red-100 text-red-700;
    @apply dark:bg-red-900/50 dark:text-red-300;
  }

  .badge-yellow {
    @apply bg-yellow-100 text-yellow-700;
    @apply dark:bg-yellow-900/50 dark:text-yellow-300;
  }
}
```

---

### Alert Component

**File:** `src/styles/components/alert.css`

```css
@layer components {
  /* Alert Base */
  .alert {
    @apply flex gap-3 p-4 rounded-lg;
  }

  /* Alert Variants */
  .alert-info {
    @apply bg-blue-50 border border-blue-200;
    @apply dark:bg-blue-900/20 dark:border-blue-800;
  }

  .alert-success {
    @apply bg-green-50 border border-green-200;
    @apply dark:bg-green-900/20 dark:border-green-800;
  }

  .alert-warning {
    @apply bg-yellow-50 border border-yellow-200;
    @apply dark:bg-yellow-900/20 dark:border-yellow-800;
  }

  .alert-error {
    @apply bg-red-50 border border-red-200;
    @apply dark:bg-red-900/20 dark:border-red-800;
  }

  /* Alert Title */
  .alert-title {
    @apply text-sm/6 font-semibold;
  }

  .alert-info .alert-title { @apply text-blue-900 dark:text-blue-200; }
  .alert-success .alert-title { @apply text-green-900 dark:text-green-200; }
  .alert-warning .alert-title { @apply text-yellow-900 dark:text-yellow-200; }
  .alert-error .alert-title { @apply text-red-900 dark:text-red-200; }

  /* Alert Description */
  .alert-description {
    @apply text-sm/6;
  }

  .alert-info .alert-description { @apply text-blue-700 dark:text-blue-300; }
  .alert-success .alert-description { @apply text-green-700 dark:text-green-300; }
  .alert-warning .alert-description { @apply text-yellow-700 dark:text-yellow-300; }
  .alert-error .alert-description { @apply text-red-700 dark:text-red-300; }
}
```

---

### Modal Component (Headless UI Pattern)

**File:** `src/styles/components/modal.css`

```css
@layer components {
  /* Modal Overlay */
  .modal-overlay {
    @apply fixed inset-0 bg-zinc-950/25 backdrop-blur-sm;
    @apply dark:bg-zinc-950/50;
  }

  /* Modal Container */
  .modal-container {
    @apply fixed inset-0 flex items-center justify-center p-4;
  }

  /* Modal Panel */
  .modal-panel {
    @apply bg-white rounded-2xl shadow-xl;
    @apply max-w-lg w-full max-h-[90vh] overflow-y-auto;
    @apply dark:bg-zinc-900;
  }

  /* Modal Header */
  .modal-header {
    @apply flex items-start justify-between p-6 border-b border-zinc-950/10;
    @apply dark:border-white/15;
  }

  /* Modal Title */
  .modal-title {
    @apply text-xl/8 font-semibold text-zinc-950;
    @apply dark:text-white;
  }

  /* Modal Body */
  .modal-body {
    @apply p-6;
  }

  /* Modal Footer */
  .modal-footer {
    @apply flex gap-3 justify-end p-6 border-t border-zinc-950/10;
    @apply dark:border-white/15;
  }
}
```

---

## Dark Mode Implementation

### CSS Custom Properties Strategy

**Why CSS Variables?**
- Dynamic theme switching without class recompilation
- Easier customization per user preference
- Better performance than class toggling on large DOMs
- Future-proof for additional themes

**Implementation:**

```css
/* src/app.css */
@import 'tailwindcss';

/* Define light theme (default) */
:root {
  /* Surface Colors */
  --surface-primary: theme('colors.white');
  --surface-secondary: theme('colors.zinc.100');
  --surface-tertiary: theme('colors.zinc.50');

  /* Text Colors */
  --text-primary: theme('colors.zinc.950');
  --text-secondary: theme('colors.zinc.500');
  --text-tertiary: theme('colors.zinc.400');

  /* Border Colors */
  --border-primary: theme('colors.zinc.950' / 10%);
  --border-secondary: theme('colors.zinc.200');

  /* Interactive Colors */
  --interactive-primary: theme('colors.blue.600');
  --interactive-primary-hover: theme('colors.blue.700');
  --interactive-danger: theme('colors.red.600');
  --interactive-danger-hover: theme('colors.red.700');
}

/* Define dark theme */
.dark {
  /* Surface Colors */
  --surface-primary: theme('colors.zinc.950');
  --surface-secondary: theme('colors.zinc.900');
  --surface-tertiary: theme('colors.zinc.800');

  /* Text Colors */
  --text-primary: theme('colors.white');
  --text-secondary: theme('colors.zinc.400');
  --text-tertiary: theme('colors.zinc.500');

  /* Border Colors */
  --border-primary: theme('colors.white' / 15%);
  --border-secondary: theme('colors.zinc.800');

  /* Interactive Colors */
  --interactive-primary: theme('colors.blue.500');
  --interactive-primary-hover: theme('colors.blue.400');
  --interactive-danger: theme('colors.red.500');
  --interactive-danger-hover: theme('colors.red.400');
}
```

**Usage in Components:**

```css
.my-component {
  background-color: var(--surface-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
}

.my-component:hover {
  background-color: var(--surface-secondary);
}
```

**Store Integration (Keep Existing):**

```typescript
// lib/stores/theme.ts (existing store - keep as-is)
import { writable } from 'svelte/store'
import { browser } from '$app/environment'

export type Theme = 'light' | 'dark' | 'system'

const theme = writable<Theme>('system')

if (browser) {
  theme.subscribe((value) => {
    localStorage.setItem('theme', value)
    const isDark = value === 'dark' ||
      (value === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)

    document.documentElement.classList.toggle('dark', isDark)
  })
}

export { theme }
```

---

## Icon System

### Heroicon Size Classes (Catalyst Convention)

**Tailwind v4 Custom Utilities:**

```css
@layer utilities {
  /* Icon Sizes (Catalyst) */
  .size-3 { width: 0.75rem; height: 0.75rem; }   /* 12px */
  .size-4 { width: 1rem; height: 1rem; }         /* 16px */
  .size-5 { width: 1.25rem; height: 1.25rem; }   /* 20px */
  .size-6 { width: 1.5rem; height: 1.5rem; }     /* 24px */
  .size-7 { width: 1.75rem; height: 1.75rem; }   /* 28px */
  .size-8 { width: 2rem; height: 2rem; }         /* 32px */
}
```

**Usage Guidelines:**

| Context | Size Class | Actual Size | Use Case |
|---------|------------|-------------|----------|
| Button icon | `size-4` | 16px | Icons in buttons (with text) |
| Button icon-only | `size-5` | 20px | Icon-only buttons |
| Input prefix/suffix | `size-5` | 20px | Icons inside inputs |
| Card header icon | `size-6` | 24px | Decorative icons in headers |
| Large feature icons | `size-8` | 32px | Empty states, feature highlights |

**Svelte Usage:**

```svelte
<script lang="ts">
  import { Icon } from 'svelte-hero-icons'
  import { Heart } from 'svelte-hero-icons/outline'
</script>

<!-- In a button -->
<button class="btn btn-solid-blue">
  <Icon src={Heart} class="size-4" />
  Like
</button>

<!-- Icon-only button -->
<button class="btn btn-plain-zinc">
  <Icon src={Heart} class="size-5" />
</button>

<!-- In a card header -->
<div class="card-header flex items-center gap-3">
  <Icon src={Heart} class="size-6 text-red-500" />
  <h2 class="card-title">Favorites</h2>
</div>
```

---

## Responsive Strategy

### Desktop-First Approach

**Breakpoints (Tailwind Default):**
- `sm: 640px` - Small tablets
- `md: 768px` - Tablets
- `lg: 1024px` - Laptops
- `xl: 1280px` - Desktops
- `2xl: 1536px` - Large desktops

**Strategy:** Optimize for desktop (1024px+), scale down for mobile.

**Example:**

```svelte
<!-- Desktop-optimized card grid -->
<div class="grid grid-cols-3 gap-6 lg:gap-8">
  <!-- 3 columns on desktop, scale down below -->
  <div class="col-span-3 md:col-span-1">
    <!-- Full width mobile, 1/3 desktop -->
  </div>
</div>

<!-- Desktop-optimized form -->
<form class="space-y-6 max-w-2xl">
  <!-- Optimized for desktop reading width -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Stack on mobile, side-by-side on desktop -->
    <Input label="First Name" />
    <Input label="Last Name" />
  </div>
</form>
```

**Layout Pattern:**

```css
/* Desktop-optimized container */
.container-desktop {
  @apply max-w-7xl mx-auto px-6 lg:px-8;
}

/* Sidebar layout (desktop-first) */
.layout-sidebar {
  @apply flex flex-col lg:flex-row gap-6;
}

.layout-sidebar-nav {
  @apply w-full lg:w-64 shrink-0;
}

.layout-sidebar-content {
  @apply flex-1 min-w-0;
}
```

---

## File Organization

### Recommended Structure

```
src/
├── app.css                         # Main entry point
├── app.html
└── styles/
    ├── tokens/
    │   ├── colors.css              # Color palette
    │   ├── typography.css          # Font scale
    │   ├── spacing.css             # Spacing/radius
    │   ├── shadows.css             # Box shadows
    │   └── transitions.css         # Animation tokens
    └── components/
        ├── button.css              # Button component
        ├── input.css               # Input component
        ├── card.css                # Card component
        ├── badge.css               # Badge component
        ├── alert.css               # Alert component
        └── modal.css               # Modal component
```

### Main Entry File

**`src/app.css`:**

```css
@import 'tailwindcss';

/* Custom variant for dark mode */
@custom-variant dark (&:is(.dark *));

/* Token imports */
@import './styles/tokens/colors.css';
@import './styles/tokens/typography.css';
@import './styles/tokens/spacing.css';
@import './styles/tokens/shadows.css';
@import './styles/tokens/transitions.css';

/* Component imports */
@import './styles/components/button.css';
@import './styles/components/input.css';
@import './styles/components/card.css';
@import './styles/components/badge.css';
@import './styles/components/alert.css';
@import './styles/components/modal.css';

/* Base styles */
@layer base {
  /* Border color compatibility (Tailwind v3 → v4) */
  *,
  ::after,
  ::before,
  ::backdrop,
  ::file-selector-button {
    border-color: var(--border-primary, currentcolor);
  }

  /* Typography defaults */
  body {
    @apply text-base/6 text-zinc-950 bg-zinc-100;
    @apply dark:text-white dark:bg-zinc-950;
  }
}
```

---

## Implementation Checklist

### Week 7: Foundation (6-8 hours)

- [ ] **Create token files** (colors, typography, spacing, shadows)
  - [ ] `src/styles/tokens/colors.css` - Full zinc palette + accents
  - [ ] `src/styles/tokens/typography.css` - Catalyst text/line-height scale
  - [ ] `src/styles/tokens/spacing.css` - Fractional spacing values
  - [ ] `src/styles/tokens/shadows.css` - Catalyst shadow depth system
  - [ ] `src/styles/tokens/transitions.css` - Animation tokens

- [ ] **Update `app.css`** with imports and dark mode CSS variables

- [ ] **Test dark mode** - Verify all tokens work in both themes

### Week 7: Core Components (10-12 hours)

- [ ] **Button component** - All variants (solid, outline, plain × 4 colors)
  - [ ] Create `src/styles/components/button.css`
  - [ ] Create `src/lib/components/ui/Button.svelte`
  - [ ] Write component tests

- [ ] **Input component** - All states (hover, focus, error, disabled)
  - [ ] Create `src/styles/components/input.css`
  - [ ] Create `src/lib/components/ui/Input.svelte`
  - [ ] Write component tests

- [ ] **Card component** - Header, body, footer
  - [ ] Create `src/styles/components/card.css`
  - [ ] Create `src/lib/components/ui/Card.svelte`
  - [ ] Write component tests

### Week 8: Extended Components (8-10 hours)

- [ ] **Badge component** - All color variants
- [ ] **Alert component** - Info, success, warning, error
- [ ] **Modal component** - Full overlay + panel structure
- [ ] **Select component** (optional) - Styled dropdown

### Week 8: Migration (6-8 hours)

- [ ] Migrate assessment forms to new components
- [ ] Migrate all buttons to `Button` component
- [ ] Migrate dashboard cards to `Card` component
- [ ] Update activity management UI
- [ ] Test visual consistency across all pages

---

## Testing Strategy

### Visual Regression Testing

**Tools:** Playwright component testing or Chromatic

**Test Cases:**
- [ ] Button variants in light/dark mode
- [ ] Input states (default, hover, focus, error, disabled)
- [ ] Card layouts with different content
- [ ] Badge colors
- [ ] Alert types
- [ ] Modal overlay + panel

### Accessibility Testing

**Tools:** Axe DevTools, Lighthouse

**Checklist:**
- [ ] All interactive elements have focus indicators
- [ ] Color contrast meets WCAG AA (4.5:1 for text)
- [ ] Keyboard navigation works for all components
- [ ] Screen reader compatibility (aria-labels)

---

## Success Metrics

### Visual Fidelity
- **Target:** 90%+ match to Catalyst UI demo
- **Measure:** Side-by-side screenshot comparison

### Performance
- **Target:** No performance degradation from design system
- **Measure:** Lighthouse performance score remains >90

### Developer Experience
- **Target:** Consistent component API across all components
- **Measure:** Component reusability (used in 3+ pages)

### Accessibility
- **Target:** Lighthouse accessibility score >95
- **Measure:** Axe DevTools finds 0 critical issues

---

## References

- **Catalyst UI Demo:** https://catalyst-demo.tailwindui.com/
- **Tailwind CSS v4 Docs:** https://tailwindcss.com/docs
- **Tailwind v4 @theme:** https://tailwindcss.com/docs/theme
- **Heroicons:** https://heroicons.com/
- **Project Plan:** `REVISED-plan.md` (Week 7-8 Catalyst UI Refresh)
- **Svelte Architecture:** `svelte5-architecture.md` (component patterns)
