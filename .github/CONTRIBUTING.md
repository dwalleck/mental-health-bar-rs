# Contributing to Mental Health Tracker

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/mental-health-bar-rs.git
   cd mental-health-bar-rs
   ```
3. **Install dependencies**:
   ```bash
   npm install
   cd src-tauri && cargo build
   ```
4. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Running the Development Server

```bash
npm run tauri dev
```

### Running Tests

**Backend (Rust):**
```bash
cd src-tauri
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

**Frontend (Svelte):**
```bash
npm test
npm run check
npm run lint
```

### Code Style

Please follow the coding guidelines in [CLAUDE.md](../CLAUDE.md):

- **Rust**: Follow rustfmt and clippy recommendations
- **Svelte**: Use Prettier and ESLint configurations
- **TypeScript**: Follow strict TypeScript settings

### Commit Messages

Use conventional commit format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `test:` Adding tests
- `refactor:` Code refactoring
- `perf:` Performance improvement
- `chore:` Maintenance tasks

Example:
```
feat: add PHQ-9 assessment scoring algorithm

- Implement calculate_phq9_score function
- Add severity level mapping
- Include unit tests for edge cases
```

## Pull Request Process

1. **Update documentation** if you're changing functionality
2. **Add tests** for new features or bug fixes
3. **Run all tests** locally before pushing
4. **Update CHANGELOG.md** (if applicable)
5. **Fill out the PR template** completely
6. **Link related issues** using GitHub keywords (Fixes #123, Closes #456)

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass locally
- [ ] No new warnings generated

## CI/CD Pipeline

All pull requests trigger automated checks:

1. **Lint**: Clippy and rustfmt checks
2. **Backend Tests**: Rust unit and integration tests
3. **Frontend Tests**: Vitest component tests
4. **Coverage**: Code coverage report
5. **Build**: Multi-platform builds (Linux, macOS, Windows)

PRs must pass all checks before merging.

## Architecture

This project uses:

- **Vertical Slice Architecture**: Features are self-contained modules
- **CQRS Lite**: Separate commands (mutations) and queries (reads)
- **Repository Pattern**: Database access abstraction
- **TDD**: Write tests before implementation

See [plan.md](../specs/001-mental-health-tracking/plan.md) for detailed architecture documentation.

## Feature Development

When adding a new feature:

1. Review existing features in `src-tauri/src/features/`
2. Create a new feature module following the vertical slice pattern:
   ```
   features/your-feature/
   ├── mod.rs          # Public exports
   ├── models.rs       # Domain types
   ├── commands.rs     # Mutations (Tauri commands)
   ├── queries.rs      # Reads (Tauri queries)
   └── repository.rs   # Database access
   ```
3. Register commands in `src-tauri/src/lib.rs`
4. Generate TypeScript bindings: `cargo test generate_types`
5. Create Svelte components in `src/lib/components/your-feature/`
6. Add routes if needed in `src/routes/your-feature/`

## Testing Guidelines

### Unit Tests (Rust)

Place tests in the same file with `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_behavior() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Component Tests (Svelte)

Use Vitest and Testing Library:

```typescript
import { render, fireEvent } from '@testing-library/svelte';
import Component from './Component.svelte';

test('component renders correctly', () => {
  const { getByText } = render(Component, { props: { value: 'test' } });
  expect(getByText('test')).toBeInTheDocument();
});
```

## Database Migrations

When modifying the database schema:

1. Create a new migration in `src-tauri/src/db/migrations/`
2. Name it sequentially: `002_your_migration.sql`
3. Update `migrations.rs` to apply the new migration
4. Test migration on a fresh database
5. Document breaking changes

## Security

- Never commit sensitive data (.env files, API keys, etc.)
- All user data must remain local (no external network calls)
- Follow secure coding practices
- Report security vulnerabilities privately to maintainers

## Questions?

- Check existing issues and discussions on GitHub
- Review [quickstart.md](../specs/001-mental-health-tracking/quickstart.md)
- Read [research.md](../specs/001-mental-health-tracking/research.md) for technical decisions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to Mental Health Tracker!** 🎉
