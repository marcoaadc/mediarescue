# MediaRescue

Professional data recovery tool for corrupted images and videos from HDDs and USB drives.

## Architecture

- **Methodology**: TLA+/TLC spec-driven development — formal specs verified before implementation
- **Core**: Rust (Cargo workspace with 3 crates)
- **API**: Axum REST + WebSocket
- **Frontend**: React 19 + TypeScript + Vite + Tailwind CSS v4 + shadcn/ui
- **State**: Zustand + React Query

## Project Structure

```
specs/          → TLA+ formal specifications (Phase 0, blocking)
engine/         → Rust Cargo workspace
  mediarescue-core/  → Pure library, no web deps
  mediarescue-api/   → Axum web server
  mediarescue-cli/   → Optional CLI
frontend/       → React Web UI
test-data/      → Corrupted test files
```

## Conventions

### Rust
- Edition 2021, MSRV 1.75+
- `cargo fmt` and `cargo clippy -- -D warnings` must pass
- Error handling: `thiserror` for library errors, `anyhow` forbidden in library code
- Async runtime: `tokio` (full features in bins, `sync` only in core lib)
- All I/O behind traits for testability (e.g., `DeviceReader` trait)
- No `unwrap()` or `expect()` in library code — use `?` operator
- Tests: `#[cfg(test)]` modules in each file + integration tests in `engine/tests/`

### Frontend
- TypeScript strict mode
- Components: functional only, no class components
- State: Zustand for client state, React Query for server state
- Styling: Tailwind CSS utility classes, shadcn/ui components
- Testing: Vitest + React Testing Library

### Git
- Conventional Commits: `spec(tla):`, `feat(scanner):`, `fix(carver):`, `test(pipeline):`, etc.
- Branch naming: `spec/*`, `feat/*`, `fix/*`, `test/*` from `develop`
- PRs require CI green + tech lead review

### TLA+ Specs
- Every spec needs a `.cfg` file with INVARIANTS and PROPERTIES
- Model check must pass with zero violations before implementing the corresponding Rust module
- Specs model safety (invariants) and liveness (temporal properties)

## Commands

```bash
# Rust
cd engine && cargo build --workspace
cd engine && cargo test --workspace
cd engine && cargo clippy --workspace -- -D warnings

# Frontend
cd frontend && npm run dev
cd frontend && npm run lint
cd frontend && npm run test

# TLA+ (requires tla2tools.jar)
java -jar tla2tools.jar -config specs/ScannerStateMachine.cfg specs/ScannerStateMachine.tla
```

## Key Design Decisions

1. **FormatHandler trait**: Central abstraction — all format modules implement it
2. **Event-driven progress**: `tokio::sync::broadcast` channel for recovery events
3. **Copy-on-write**: Original data is NEVER modified during repair operations
4. **Platform**: Primary target is Windows (device access via `\\.\PhysicalDriveN`)
