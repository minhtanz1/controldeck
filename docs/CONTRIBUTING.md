# Contributing to ControlDeck

Thank you for contributing to ControlDeck! We welcome issues, bug reports, and pull requests.

---

## Development Setup

1. Fork and clone the repository.
2. Install frontend dependencies:
   ```bash
   npm install
   ```
3. Ensure the Rust toolchain is up-to-date:
   ```bash
   rustup update stable
   ```

---

## Coding Standards

- **Desktop Frontend**: Written in Svelte 5 and TypeScript (`src/`). Follow reactive primitives (`$state`, `$derived`, `$props`).
- **Mobile Frontend**: Written in Svelte 5 (`src/mobile/`). Maintain touch event ergonomics and low latency.
- **Backend**: Written in Rust (`src-tauri/src/`). Adhere to idiomatic Rust error handling using `Result<T, E>` and `serde` serialization.

---

## Pre-PR Verification

Before submitting a Pull Request, run the validation suite:

```bash
# Type check and verify frontend
npm run check
npm run build

# Verify Rust backend
cargo check --manifest-path src-tauri/Cargo.toml
```

Ensure all CI checks pass on GitHub Actions.
