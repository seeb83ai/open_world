# Testing Guide

This document describes the testing strategy and how to run tests locally and in CI.

## Testing Strategy (TDD)

All code follows Test-Driven Development (TDD) with the Red-Green-Refactor cycle:

1. **Red**: Write a failing test first
2. **Yellow**: Implement minimal code to make the test pass
3. **Green**: Verify test passes
4. **Refactor**: Clean up and optimize

## Test Types

### 1. Unit Tests

**Location**: `src/**/*.rs` (inline `#[test]` modules)

**Purpose**: Test individual functions and modules in isolation

**Examples**:
- `src/error.rs::tests` - Error type handling
- `src/models/mod.rs::tests` - Data structure creation and validation
- `src/config.rs::tests` - Configuration loading
- `src/auth.rs::tests` - JWT encoding/decoding

**Run**:
```bash
cargo test --lib
cargo test --lib error::tests
cargo test --lib models::tests -- --nocapture
```

### 2. Integration Tests

**Location**: `tests/*.rs`

**Purpose**: Test complete flows across modules and endpoints

**Examples**:
- `tests/auth_integration_tests.rs` - Auth flow (register → login)
- `tests/area_integration_tests.rs` - Area exploration and movement
- `tests/item_integration_tests.rs` - Item collection and management
- `tests/action_integration_tests.rs` - Player actions and puzzles

**Run**:
```bash
cargo test --test '*'
cargo test --test auth_integration_tests
cargo test --test area_integration_tests -- --nocapture
```

### 3. E2E Tests

**Location**: `e2e/*.spec.ts`

**Purpose**: Test complete user workflows through HTTP API

**Technology**: Playwright

**Examples**:
- `e2e/auth.spec.ts` - User registration and login flows
- `e2e/exploration.spec.ts` - Exploring areas and checking state
- `e2e/items-and-actions.spec.ts` - Item collection, usage, and actions

**Run**:
```bash
npm run test:e2e
npm run test:e2e:ui          # Interactive UI
npm run test:e2e:debug       # Step through tests
npm run test:e2e:report      # View latest report
```

## Running All Tests

### Locally (Unit + Integration)
```bash
cargo test
```

### All including E2E
```bash
# First, start the server in another terminal:
cargo run

# Then, in another terminal:
npm run test:e2e
```

### Full test suite (unit + integration + lint)
```bash
./scripts/test.sh
```

## CI/CD Pipeline

GitHub Actions runs tests automatically on:
- Push to `main` branch
- Push to `claude/**` branches
- Pull requests to `main`

### Workflows

1. **test.yml** - Unit, integration tests, linting
   - Runs on every push/PR
   - Fast feedback (2-5 minutes)
   - Blocks PR if fails

2. **e2e-tests.yml** - E2E tests with Playwright
   - Runs on every push/PR
   - Builds server, starts it, runs E2E tests
   - Takes longer (5-10 minutes)
   - Uploads Playwright report on failure

## Test Coverage

**Current coverage targets**:
- Unit tests: All public functions and critical logic
- Integration tests: All API endpoints
- E2E tests: Major user workflows

**Coverage reporting** (TBD):
```bash
cargo tarpaulin --out Html
```

## Writing Tests

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange
        let input = setup_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_output);
    }
}
```

### Integration Test Template

```rust
#[tokio::test]
async fn test_api_endpoint() {
    // Setup: Start test server, create user, etc.
    // TODO: Implement

    // Request: Call endpoint
    // let response = client.get("/api/endpoint").await;

    // Assert: Check response
    // assert_eq!(response.status(), 200);
}
```

### E2E Test Template

```typescript
test('should do something', async ({ request }) => {
  // Setup
  const email = `test-${Date.now()}@example.com`;

  // Act
  const response = await request.post(`${BASE_URL}/api/auth/register`, {
    data: { email, name: 'User', password: 'pass' },
  });

  // Assert
  expect(response.status()).toBe(201);
  const data = await response.json();
  expect(data.status).toBe('ok');
});
```

## Common Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run single-threaded (for debugging)
cargo test -- --test-threads=1

# Run tests matching pattern
cargo test auth

# E2E only
npm run test:e2e

# E2E with UI
npm run test:e2e:ui

# Check code without running tests
cargo check

# Format code
cargo fmt

# Lint
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

## Debugging Tests

### Unit/Integration Tests
```bash
# Run with full output
cargo test -- --nocapture --test-threads=1

# Use println! in tests
// In your test:
println!("Debug info: {:?}", variable);

# With rust-gdb
rust-gdb --args target/debug/deps/open_world-xxxxx test_name
```

### E2E Tests
```bash
# Interactive mode
npm run test:e2e:debug

# With logging
PWDEBUG=1 npx playwright test

# View report
npm run test:e2e:report

# Run specific test file
npx playwright test e2e/auth.spec.ts

# Run specific test
npx playwright test -g "should register"
```

## Test Status

Current test files (with TODO placeholders):
- ✅ Unit tests implemented and passing
- 📝 Integration tests (structure in place, cases TBD)
- 📝 E2E tests (structure in place, cases TBD)

Next steps:
1. Implement database fixtures for tests
2. Add test helper functions
3. Fill in integration test implementations
4. Fill in E2E test implementations
5. Set up test database isolation
