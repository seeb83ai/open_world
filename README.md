# Open World — Text-Based Exploration Game

A persistent, AI-generated, multiplayer text-based exploration game where players explore a graph-based world of interconnected areas, discover items, solve puzzles, and gradually deplete shared resources.

## Quick Start

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Node.js 18+ (for E2E tests with Playwright)
- SQLite3

### Setup

1. **Clone and navigate to the repository:**
```bash
cd open_world
```

2. **Copy environment configuration:**
```bash
cp .env.example .env
```

Edit `.env` and set `JWT_SECRET` to a secure random string.

3. **Install dependencies:**
```bash
cargo build
npm install  # for Playwright
```

4. **Run database migrations:**
```bash
diesel migration run
```

Or use the seed script to initialize the world:
```bash
cargo run --bin seed_initial_world
```

5. **Run the server:**
```bash
cargo run
```

The API will be available at `http://localhost:8000`

## Development Workflow (TDD)

1. **Write a failing test** (red phase)
2. **Implement minimal code** to make it pass (yellow phase)
3. **Ensure test passes** (green phase)
4. **Refactor and optimize** (refactor phase)
5. **Commit and push**

### Running Tests

**Unit and integration tests:**
```bash
cargo test
```

**E2E tests with Playwright:**
```bash
npx playwright test
```

**Run all tests locally:**
```bash
./scripts/test.sh
```

## Project Structure

See `CLAUDE.md` for detailed technical specification and `PROJECT.md` for game design document.

```
src/
  handlers/       - HTTP request handlers
  services/       - Business logic
  models/         - Data structures
  db/             - Database access
  middleware/     - Axum middleware
  auth.rs         - JWT authentication
  config.rs       - Configuration
  error.rs        - Error types

migrations/       - Diesel migrations
tests/            - Integration tests
e2e/              - E2E tests (Playwright)
```

## API Endpoints

See `CLAUDE.md` for complete API specification.

Key endpoints:
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `GET /api/areas` - List areas
- `POST /api/areas/{id}/enter` - Move to area
- `GET /api/player/inventory` - Player inventory

## Configuration

All settings are loaded from `.env` file:

- `DATABASE_URL` - SQLite database path
- `JWT_SECRET` - Secret key for JWT signing
- `JWT_EXPIRATION_HOURS` - Token expiration time
- `LOG_LEVEL` - Logging level (DEBUG, INFO, WARN, ERROR)
- `RATE_LIMIT_IP` - Requests per minute per IP
- `RATE_LIMIT_AUTH` - Requests per minute per user
- `INITIAL_CAPACITY` - Starting inventory capacity

## Documentation

- `PROJECT.md` - Game design and concept
- `CLAUDE.md` - Technical specification
- `GAME.md` - Server configuration (per-instance settings)

## Development Commands

```bash
# Build
cargo build

# Run tests
cargo test
cargo test --lib           # Unit tests only
cargo test --test '*'      # Integration tests only

# Run E2E tests
npx playwright test
npx playwright test --debug

# Check code
cargo check
cargo clippy
cargo fmt

# Database
diesel migration generate <name>
diesel migration run
diesel migration revert

# Development server with auto-reload
cargo watch -x run
```

## License

TBD
