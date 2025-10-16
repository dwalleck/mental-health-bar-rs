# Tauri + SvelteKit Modern Template

A modern, fully-configured Tauri + SvelteKit template with TypeScript, testing, and type-safe Rust bindings.

## Features

- ✅ **Tauri v2** - Latest stable desktop framework
- ✅ **SvelteKit 2** with **Svelte 5** - Modern reactive UI with runes API
- ✅ **Vite 6** - Fast build tooling
- ✅ **TypeScript 5.6** - Type-safe development
- ✅ **ESLint 9** - Code linting with flat config
- ✅ **Prettier** - Consistent code formatting
- ✅ **Vitest** - Fast unit testing with UI
- ✅ **Tauri Specta** - Type-safe Rust ↔ TypeScript bindings

## Prerequisites

1. **Node.js** (v20 or later recommended)
2. **Rust** (latest stable)
3. **System dependencies** for Tauri (see [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/))

## Getting Started

### Install Dependencies

```bash
npm install
```

### Development

Run the app in development mode:

```bash
npm run tauri dev
```

On first run, TypeScript bindings will be auto-generated at `src/lib/bindings.ts`.

### Build

Build the production app:

```bash
npm run tauri build
```

## Available Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start Vite dev server only |
| `npm run tauri dev` | Start Tauri app in dev mode |
| `npm run build` | Build web frontend for production |
| `npm run tauri build` | Build desktop app for production |
| `npm test` | Run Vitest tests |
| `npm run test:ui` | Run Vitest with UI |
| `npm run lint` | Run ESLint |
| `npm run format` | Format code with Prettier |
| `npm run format:check` | Check code formatting |
| `npm run check` | Run Svelte type checking |

## Type-Safe Rust Commands

This template uses [Tauri Specta](https://github.com/oscartbeaumont/tauri-specta) for automatic TypeScript bindings generation.

### Adding a New Command

1. Define your command in `src-tauri/src/lib.rs`:

```rust
#[tauri::command]
#[specta::specta]
fn my_command(input: String) -> Result<String, String> {
    Ok(format!("Processed: {}", input))
}
```

2. Add it to the builder:

```rust
let builder = Builder::<tauri::Wry>::new()
    .commands(collect_commands![greet, my_command]);
```

3. Run the app - TypeScript bindings auto-generate in debug mode

4. Import and use in your Svelte components:

```typescript
import { commands } from '$lib/bindings'

const result = await commands.myCommand('hello')
```

## Project Structure

```
tauri-sveltekit-modern/
├── src/                    # SvelteKit source
│   ├── lib/               # Shared components & utils
│   │   └── bindings.ts    # Auto-generated Tauri bindings
│   └── routes/            # SvelteKit routes
├── src-tauri/             # Rust/Tauri backend
│   ├── src/
│   │   ├── lib.rs        # Tauri commands
│   │   └── main.rs       # App entry point
│   └── Cargo.toml        # Rust dependencies
├── static/                # Static assets
├── eslint.config.js       # ESLint configuration
├── .prettierrc            # Prettier configuration
├── vitest.config.ts       # Vitest configuration
└── svelte.config.js       # SvelteKit configuration
```

## Testing

Write tests in `*.test.ts` files:

```typescript
import { describe, it, expect } from 'vitest'

describe('My Feature', () => {
  it('should work correctly', () => {
    expect(1 + 1).toBe(2)
  })
})
```

Run tests with:
```bash
npm test          # Run in terminal
npm run test:ui   # Run with UI
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

MIT
