# Backlog: Git Workflow — Web App Refactoring

## Branch Name
`feat/refactor-web-app`

> **Pre-step:** Unstage the 2 currently staged files (`src/api/screener.rs`, `src/core/models/statistics.rs`) so all changes can be committed in logical order:
> ```bash
> git reset HEAD src/api/screener.rs src/core/models/statistics.rs
> ```

## Commit Plan (in order)

### Commit 1: `build: restructure project into library/binary layout`
- **Files:** `src/lib.rs`, `src/bin/generate-hash.rs`, `Cargo.toml`, `Cargo.lock`
- **Purpose:** Establishes the new project architecture — splits the crate into a reusable library (`lib.rs`) and binary targets (`bin/`). Updates dependencies in `Cargo.toml` and resolves them in `Cargo.lock`.
- **Acceptance:** `cargo check` passes with the new library/binary structure. No old CLI/TUI code is referenced yet.

### Commit 2: `refactor: migrate core models and error handling`
- **Files:** `src/core/models/mod.rs`, `src/core/models/contract_type.rs`, `src/core/models/price.rs`, `src/core/models/response.rs`, `src/core/models/symbol.rs`, `src/core/models/statistics.rs`, `src/core/error.rs`
- **Purpose:** Moves all data structs (contract types, prices, symbols, responses, statistics) into the new `core/models/` namespace. Migrates the centralized error type to `core/error.rs`.
- **Acceptance:** All model types compile under `core::models::*`. Error type is usable across core modules.

### Commit 3: `refactor: migrate core utilities, logging, and output`
- **Files:** `src/core/mod.rs`, `src/core/utils/mod.rs`, `src/core/utils/parse.rs`, `src/core/utils/path.rs`, `src/core/output.rs`, `src/core/logging.rs`, `src/core/test_utils.rs`
- **Purpose:** Establishes the core module root (`mod.rs`) and migrates shared utilities (parsing, path helpers), output formatting, logging setup, and test helpers.
- **Acceptance:** `core::utils`, `core::output`, and `core::logging` are accessible. Test utilities compile.

### Commit 4: `refactor: migrate database layer`
- **Files:** `src/core/db/mod.rs`, `src/core/db/connection.rs`, `src/core/db/repository.rs`, `src/core/db/schema.rs`
- **Purpose:** Replaces the old `src/db/` with the refactored database layer under `core/db/`. Includes connection management, repository pattern, and schema definitions.
- **Acceptance:** Database module compiles independently. Connection and repository types are usable.

### Commit 5: `refactor: migrate exchange integration`
- **Files:** `src/core/exchange/mod.rs`, `src/core/exchange/bybit.rs`, `src/core/exchange/exchange_trait.rs`, `src/core/exchange/factory.rs`
- **Purpose:** Migrates the exchange abstraction (trait, Bybit implementation, factory) into `core/exchange/`. This is the data source layer for market data.
- **Acceptance:** Exchange trait and Bybit impl compile. Factory can construct exchange instances.

### Commit 6: `refactor: migrate data fetcher and screener engine`
- **Files:** `src/core/fetcher/mod.rs`, `src/core/screener/mod.rs`, `src/core/screener/calculator.rs`, `src/core/screener/fetcher.rs`, `src/core/screener/mode.rs`, `src/core/screener/output.rs`, `src/api/screener.rs`
- **Purpose:** Migrates the instrument fetcher and the core screener logic (calculator, fetcher, modes, output). Includes `api/screener.rs` which bridges the screener engine to the HTTP layer.
- **Acceptance:** Screener logic compiles. `api/screener.rs` can call into `core::screener`.

### Commit 7: `feat: add REST API routes and middleware`
- **Files:** `src/api/mod.rs`, `src/api/auth.rs`, `src/api/error.rs`, `src/api/extractors.rs`, `src/api/exchanges.rs`, `src/api/refresh.rs`, `src/api/stats.rs`, `src/api/symbols.rs`, `src/api/types.rs`, `src/api/utils.rs`
- **Purpose:** Adds the full REST API layer — authentication, exchange management, symbol listing, stats endpoints, data refresh, shared extractors, type definitions, and utility helpers.
- **Acceptance:** All API route modules compile. `cargo check` passes for the full backend.

### Commit 8: `refactor: update entry point and remove legacy CLI/TUI code`
- **Files (modified):** `src/main.rs`
- **Files (deleted):** `src/cli.rs`, `src/tui/` (16 files), `src/db/` (4 files), `src/exchange/` (4 files), `src/fetcher/` (2 files), `src/output/` (3 files), `src/screener/` (4 files), `src/models/` (6 files), `src/error.rs`, `src/utils.rs`, `src/logging.rs`, `src/test_utils.rs`, `docs/image/TUI.png`
- **Purpose:** Updates `main.rs` to launch the web server instead of the CLI/TUI. Removes all legacy code that has been superseded by the new `core/` and `api/` modules.
- **Acceptance:** `cargo build` succeeds with zero references to deleted modules. `src/main.rs` starts the web server. No dead code remains.

### Commit 9: `ci: add Docker config, scripts, and update CI pipeline`
- **Files:** `Dockerfile.backend`, `Dockerfile.frontend`, `docker-compose.yml`, `docker-compose.dev.yml`, `.dockerignore`, `.env.example`, `scripts/build-docker.sh`, `scripts/dev.sh`, `.gitignore`, `.github/workflows/ci.yml`
- **Purpose:** Adds containerization (separate backend/frontend Dockerfiles, compose configs for prod and dev), build/dev scripts, Docker ignore rules, environment template, updated gitignore, and CI workflow for the new web architecture.
- **Acceptance:** `docker-compose -f docker-compose.dev.yml up` starts both services. CI workflow references correct paths.

### Commit 10: `feat: add React frontend application`
- **Files:** `frontend/` (39 files — Next.js app with components, UI primitives, pages for screener/stats/symbols, config files)
- **Purpose:** Adds the complete React/Next.js frontend with routing, UI components (shadcn/ui), and pages for screener, stats, and symbols views.
- **Acceptance:** `cd frontend && npm install && npm run build` succeeds. Frontend can connect to backend API.

### Commit 11: `docs: update documentation for web application`
- **Files:** `docs/API.md`, `docs/CONFIGURATION.md`, `docs/DEPLOYMENT.md`, `docs/DEVELOPMENT.md`, `README.md`, `SESSION_LOG.md`
- **Purpose:** Replaces outdated CLI/TUI documentation with web app guides covering API usage, configuration options, deployment instructions, and development setup. Updates the root README.
- **Acceptance:** All docs reference the web app (no TUI mentions). README accurately describes the project structure and quick-start.

## Execution Checklist

Run these commands in order from the repo root:

```bash
# 0. Create branch and unstage existing staged files
git checkout -b feat/refactor-web-app
git reset HEAD src/api/screener.rs src/core/models/statistics.rs

# 1. Build: library/binary layout
git add src/lib.rs src/bin/ Cargo.toml Cargo.lock
git commit -m "build: restructure project into library/binary layout"

# 2. Core models + error
git add src/core/models/ src/core/error.rs
git commit -m "refactor: migrate core models and error handling"

# 3. Core utilities + logging + output
git add src/core/mod.rs src/core/utils/ src/core/output.rs src/core/logging.rs src/core/test_utils.rs
git commit -m "refactor: migrate core utilities, logging, and output"

# 4. Database layer
git add src/core/db/
git commit -m "refactor: migrate database layer"

# 5. Exchange integration
git add src/core/exchange/
git commit -m "refactor: migrate exchange integration"

# 6. Fetcher + screener engine
git add src/core/fetcher/ src/core/screener/ src/api/screener.rs
git commit -m "refactor: migrate data fetcher and screener engine"

# 7. REST API routes
git add src/api/mod.rs src/api/auth.rs src/api/error.rs src/api/extractors.rs \
        src/api/exchanges.rs src/api/refresh.rs src/api/stats.rs \
        src/api/symbols.rs src/api/types.rs src/api/utils.rs
git commit -m "feat: add REST API routes and middleware"

# 8. Entry point + remove legacy code
git add src/main.rs
git rm src/cli.rs src/tui/ src/db/ src/exchange/ src/fetcher/ src/output/ \
       src/screener/ src/models/ src/error.rs src/utils.rs src/logging.rs \
       src/test_utils.rs docs/image/TUI.png
git commit -m "refactor: update entry point and remove legacy CLI/TUI code"

# 9. Docker + CI + scripts
git add Dockerfile.backend Dockerfile.frontend docker-compose.yml \
        docker-compose.dev.yml .dockerignore .env.example scripts/ \
        .gitignore .github/workflows/ci.yml
git commit -m "ci: add Docker config, scripts, and update CI pipeline"

# 10. React frontend
git add frontend/
git commit -m "feat: add React frontend application"

# 11. Documentation
git add docs/API.md docs/CONFIGURATION.md docs/DEPLOYMENT.md \
        docs/DEVELOPMENT.md README.md SESSION_LOG.md
git commit -m "docs: update documentation for web application"
```

## Scope Notes

- **In scope:** All changed files listed above — restructuring old CLI/TUI code into a web app (Rust backend + React frontend) with Docker deployment
- **Out of scope:** New features beyond the refactoring (e.g., new screener filters, additional exchanges, auth providers, frontend feature additions)
- **Risk:** Commit 8 is the largest (~45 file deletions + 1 modification). If review is difficult, it can be split into two commits: one for `main.rs` changes and one for bulk deletions.

## Verification After All Commits

```bash
# Backend compiles
cargo check
cargo build --release

# Frontend builds
cd frontend && npm install && npm run build

# Docker compose works
docker-compose -f docker-compose.dev.yml up --build

# CI passes
# Push branch and verify GitHub Actions
```
