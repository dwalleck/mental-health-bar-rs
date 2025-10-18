# mental-health-bar-rs Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-10-15

## Active Technologies
- Rust (latest stable) + TypeScript/JavaScript (ES2022) (001-mental-health-tracking)

## Project Structure
```
src/
tests/
```

## Commands
cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style
Rust (latest stable) + TypeScript/JavaScript (ES2022): Follow standard conventions

## Recent Changes
- 001-mental-health-tracking: Added Rust (latest stable) + TypeScript/JavaScript (ES2022)

<!-- MANUAL ADDITIONS START -->

## Rust Coding Guidelines

### Style & Formatting
- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix all warnings
- Use `rustfmt.toml` configuration (if present)
- Maximum line length: 100 characters

### Error Handling
- Use `thiserror` for feature-level error types (models, repository, domain logic)
- Use `anyhow` for Tauri commands and application-level errors
- Always provide context with `.context()` or `.with_context()`
- Example:
  ```rust
  // Feature error type
  #[derive(Error, Debug)]
  pub enum AssessmentError {
      #[error("Assessment not found: {0}")]
      NotFound(String),
      #[error("Database error: {0}")]
      Database(#[from] duckdb::Error),
  }

  // Command usage
  #[tauri::command]
  pub async fn submit_assessment(req: Request) -> Result<Response, String> {
      commands::submit(req)
          .await
          .context("Failed to submit assessment")
          .map_err(|e| e.to_string())
  }
  ```

### Naming Conventions
- **Modules**: `snake_case` (e.g., `mood_checkin`, `assessment_types`)
- **Structs/Enums**: `PascalCase` (e.g., `AssessmentResponse`, `MoodRating`)
- **Functions/Variables**: `snake_case` (e.g., `get_assessment_history`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_ACTIVITY_NAME_LENGTH`)
- **Type Parameters**: Single uppercase letter or `PascalCase` (e.g., `T`, `AppState`)

### Architecture Patterns
- **Vertical Slice**: Each feature owns its complete stack (models, commands, queries, repository)
- **CQRS Lite**: Separate commands (mutations) from queries (reads) in separate files
- **Repository Pattern**: Encapsulate all database access in `repository.rs`
- Feature module structure:
  ```
  features/
  └── assessments/
      ├── mod.rs          # Public exports
      ├── models.rs       # Domain types
      ├── commands.rs     # Tauri commands (mutations)
      ├── queries.rs      # Tauri queries (reads)
      └── repository.rs   # Database access
  ```

### Testing
- **TDD Mandatory**: Write tests before implementation per project constitution
- Unit tests: In same file with `#[cfg(test)]` module
- Integration tests: In `tests/` directory
- Test naming: `test_<scenario>_<expected_behavior>`
- Example:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_incomplete_assessment_saves_as_draft() {
          // Arrange, Act, Assert
      }
  }
  ```

### Database Access (DuckDB)
- All queries in repository layer only
- Use parameterized queries (never string interpolation)
- Validate input before database operations
- Handle connection errors gracefully

### Dependencies
- Minimize external crates - justify additions
- Prefer maintained crates with 1.0+ versions
- Use workspace dependencies in `Cargo.toml`

---

## Svelte Coding Guidelines

### Style & Formatting
- Use Prettier for formatting (if configured)
- Use ESLint with Svelte plugin
- Prefer `<script lang="ts">` for TypeScript
- Maximum line length: 100 characters

### Component Structure
- Order: `<script>`, `<style>`, then markup
- Use SvelteKit file-based routing conventions
- Component file naming: `PascalCase.svelte` for components, `+page.svelte` for routes
- Example structure:
  ```svelte
  <script lang="ts">
    // Imports
    import { invoke } from '@tauri-apps/api/tauri';

    // Props
    export let assessmentType: string;

    // State
    let score: number | null = null;

    // Functions
    async function submitAssessment() { ... }
  </script>

  <div class="assessment">
    <!-- Markup -->
  </div>

  <style>
    /* Scoped styles */
  </style>
  ```

### State Management
- Use Svelte stores for shared state (`lib/stores/`)
- Writable stores for mutable state
- Derived stores for computed values
- Keep stores focused and single-purpose
- Example:
  ```typescript
  // lib/stores/assessments.ts
  import { writable } from 'svelte/store';

  export const currentAssessment = writable<Assessment | null>(null);
  export const assessmentHistory = writable<AssessmentResponse[]>([]);
  ```

### Tauri Integration
- Use auto-generated types from `tauri-specta` (in `lib/bindings.ts`)
- Wrap Tauri commands in utility functions for error handling
- Example:
  ```typescript
  // lib/utils/api.ts
  import { invoke } from '@tauri-apps/api/tauri';
  import type { AssessmentResponse } from '$lib/bindings';

  export async function submitAssessment(req: SubmitAssessmentRequest): Promise<AssessmentResponse> {
    try {
      return await invoke('submit_assessment', { request: req });
    } catch (error) {
      console.error('Failed to submit assessment:', error);
      throw error;
    }
  }
  ```

### Naming Conventions
- **Components**: `PascalCase.svelte` (e.g., `AssessmentCard.svelte`)
- **Routes**: `+page.svelte`, `+layout.svelte`
- **Stores**: `camelCase.ts` (e.g., `assessmentStore.ts`)
- **Utilities**: `camelCase.ts` (e.g., `formatDate.ts`)
- **Variables/Functions**: `camelCase`
- **Constants**: `SCREAMING_SNAKE_CASE`

### Accessibility
- Always include `aria-label` for icon buttons
- Use semantic HTML (`<button>`, `<nav>`, `<main>`, etc.)
- Ensure keyboard navigation works (test with Tab/Enter/Escape)
- Maintain color contrast ratios (use TailwindCSS utilities)
- Add focus indicators for all interactive elements

### TailwindCSS Usage
- Use utility classes in markup (avoid custom CSS when possible)
- Extract common patterns into components, not `@apply` directives
- Use Tailwind's color palette for consistency
- Responsive design: Mobile-first with `sm:`, `md:`, `lg:` breakpoints

### Testing
- Component tests with Vitest + Testing Library
- Test user interactions, not implementation details
- Example:
  ```typescript
  import { render, fireEvent } from '@testing-library/svelte';
  import AssessmentCard from './AssessmentCard.svelte';

  test('displays assessment score', () => {
    const { getByText } = render(AssessmentCard, { props: { score: 15 } });
    expect(getByText('Score: 15')).toBeInTheDocument();
  });
  ```

### Performance
- Use `{#key}` blocks to force re-renders when needed
- Avoid expensive computations in reactive statements
- Use `onMount` for initialization, `onDestroy` for cleanup
- Lazy load heavy components with dynamic imports

---

## General Project Guidelines

### Git Workflow
- Commit messages: Follow conventional commits (`feat:`, `fix:`, `docs:`, `test:`)
- Branch naming: `feature/description`, `fix/description`
- Small, focused commits (each commit should pass tests)

### Documentation
- Document public APIs with rustdoc (`///`) and JSDoc (`/**`)
- Keep README.md updated with setup instructions
- Add inline comments for complex logic only

### Performance Targets (from spec.md)
- UI responsiveness: <100ms for all interactions
- Chart rendering: <500ms
- Assessment submission: <2s end-to-end

---

## Extended Guidelines (from GitHub Awesome Copilot)

### Self-Explanatory Code Commenting

**Core Principle**: Write code that speaks for itself. Comment only when necessary to explain WHY, not WHAT.

**Comments to Avoid**:
- **Obvious Comments**: Don't state what the code clearly shows ("Initialize counter to zero", "Increment counter by one")
- **Redundant Comments**: Avoid repeating the code's meaning in prose form
- **Outdated Comments**: Never let documentation drift from actual implementation

**Comments Worth Writing**:
- **Complex Business Logic**: Clarify non-obvious calculations or domain-specific rules
- **Algorithm Choices**: Explain why you selected a particular algorithm
  - Example: "Using Floyd-Warshall for all-pairs shortest paths because we need distances between all nodes"
- **Regex Patterns**: Describe what complex regular expressions match in plain language
- **API Constraints**: Document external limitations
  - Example: "GitHub API rate limit: 5000 requests/hour for authenticated users"

**Decision Framework** (before commenting):
1. Is the code self-explanatory?
2. Would better naming eliminate the need?
3. Does this explain WHY, not WHAT?
4. Will future maintainers benefit?

**Special Cases**:
- **Public APIs**: Use structured documentation (rustdoc `///`, JSDoc `/**`)
- **Constants**: Explain reasoning ("Based on network reliability studies")
- **Annotations**: Use standard markers: TODO, FIXME, HACK, NOTE, WARNING, PERF, SECURITY, BUG, REFACTOR, DEPRECATED

**Anti-Patterns**:
- Don't comment out code; use version control instead
- Never maintain change history in comments
- Avoid decorative divider lines

---

### Rust - Extended Guidelines (GitHub Awesome Copilot)

**Overview**: Follow idiomatic Rust practices based on The Rust Book, Rust API Guidelines, RFC 430, and community standards.

**General Instructions**:
- Prioritize readability, safety, and maintainability throughout
- Leverage strong typing and Rust's ownership system for memory safety
- Decompose complex functions into smaller, manageable units
- Include explanations for algorithm-related code
- Handle errors gracefully using `Result<T, E>` with meaningful messages
- Document external dependencies and their purposes
- Follow RFC 430 naming conventions consistently
- Ensure code compiles without warnings

**Ownership, Borrowing, and Lifetimes**:
- Prefer borrowing (`&T`) over cloning unless ownership transfer is necessary
- Use `&mut T` when modifying borrowed data
- Explicitly annotate lifetimes when the compiler cannot infer them
- Use `Rc<T>` for single-threaded reference counting; `Arc<T>` for thread-safe scenarios
- Use `RefCell<T>` for interior mutability in single-threaded contexts; `Mutex<T>` or `RwLock<T>` for multi-threaded

**Patterns to Follow**:
- Use modules (`mod`) and public interfaces (`pub`) for encapsulation
- Handle errors properly with `?`, `match`, or `if let`
- Employ `serde` for serialization and `thiserror`/`anyhow` for custom errors
- Implement traits to abstract services or dependencies
- Structure async code using `async/await` with `tokio` or `async-std`
- Prefer enums over flags for type safety
- Use builders for complex object creation
- Separate binary and library code for testability
- Use `rayon` for data parallelism
- Prefer iterators over index-based loops
- Use `&str` instead of `String` for function parameters when ownership isn't needed
- Favor borrowing and zero-copy operations

---

### TypeScript 5.x / ES2022 - Extended Guidelines (GitHub Awesome Copilot)

**Core Principles**:
- Respect existing architecture and coding standards
- Prioritize readable, explicit solutions over clever shortcuts
- Extend current abstractions before creating new ones
- Focus on maintainability, clarity, and clean code

**General Requirements**:
- Target TypeScript 5.x compiling to ES2022 JavaScript baseline
- Use pure ES modules exclusively; avoid `require`, `module.exports`, or CommonJS
- Rely on project's build, lint, and test scripts
- Document design trade-offs when intent isn't obvious

**Project Organization**:
- Follow repository's folder structure and responsibility layout
- Use kebab-case filenames (e.g., `user-session.ts`, `data-service.ts`)
- Keep tests, types, and helpers near implementation for discoverability
- Reuse or extend shared utilities before adding new ones

**Naming Conventions**:
- PascalCase for classes, interfaces, enums, and type aliases
- camelCase for variables, functions, and properties
- Avoid interface prefixes like `I`; use descriptive names instead
- Name entities for their behavior or domain meaning, not implementation

**Code Style & Formatting**:
- Run project's lint/format scripts before submission
- Match project's indentation, quote style, and trailing comma rules
- Keep functions focused; extract helpers when logic branches grow
- Favor immutable data and pure functions when practical

**Type System**:
- Avoid `any` (implicit or explicit); prefer `unknown` with type narrowing
- Use discriminated unions for realtime events and state machines
- Centralize shared contracts instead of duplicating type shapes
- Leverage TypeScript utility types (`Readonly`, `Partial`, `Record`)

**Async & Error Handling**:
- Use `async/await` wrapped in try/catch with structured error handling
- Guard edge cases early to prevent deep nesting
- Route errors through project's logging/telemetry utilities
- Surface user-facing errors via repository's notification pattern
- Debounce configuration-driven updates; dispose resources deterministically

**Architecture & Patterns**:
- Follow repository's dependency injection or composition pattern
- Keep modules single-purpose
- Observe existing initialization and disposal sequences

---

### Svelte 5 - Extended Guidelines (GitHub Awesome Copilot)

**Project Foundation**:
- Svelte 5.x with runes system (`$state`, `$derived`, `$effect`, `$props`, `$bindable`)
- SvelteKit for full-stack applications
- TypeScript integration
- Component-scoped styling with CSS custom properties

**Core Architecture Principles**:
- Organize components by feature or domain for scalability
- Separate presentation components from logic-heavy components
- Extract reusable logic into composable functions
- Implement proper component composition with slots and snippets
- Use SvelteKit's file-based routing with proper load functions

**TypeScript Standards**:
- Enable strict mode in `tsconfig.json` for maximum type safety
- Define interfaces for component props using the `$props()` syntax
- Type event handlers and refs
- Leverage generated `$types.ts` files
- Implement proper type checking with `svelte-check`

**Reactivity with Runes**:
- Use `$state()` for reactive local state management
- Implement `$derived()` for computed values and expensive calculations
- Use `$effect()` for side effects with proper cleanup
- Define component props with `$props()` and destructuring
- Use `$bindable()` for two-way data binding between components

**Component Design**:
- Follow single responsibility principle
- Keep components small and focused on one concern
- Implement proper prop validation with TypeScript
- Use slots and snippets for flexible composition
- Design components to be testable and reusable

**State Management Approach**:
- Use `$state()` for local component state
- Implement context API with `setContext`/`getContext` for shared state
- Use SvelteKit stores for global application state when needed
- Keep state normalized for complex data structures
- Implement proper state persistence for client-side data

**SvelteKit Patterns**:
- Use `+page.svelte` for page components with proper SEO
- Implement `+layout.svelte` for shared layouts and navigation
- Use `+page.server.ts` for server-side data loading and API calls
- Implement form actions in `+page.server.ts` for data mutations
- Use `+server.ts` for API endpoints and server-side logic

**Styling Conventions**:
- Use component-scoped styles with `<style>` blocks
- Implement CSS custom properties for theming
- Follow mobile-first responsive design patterns
- Use proper CSS specificity and avoid `!important`
- Leverage Tailwind utilities when configured

<!-- MANUAL ADDITIONS END -->
