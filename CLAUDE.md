# CLAUDE.md - ControlDeck Assistant Rules

## Codebase Orientation
- Framework: Tauri v2 + SvelteKit (Desktop) & Svelte 5 Singlefile (PocketPad Mobile Web) + Rust (Backend).
- OS Platform: Windows (DirectInput/XInput, SendInput, Win32 Window Watcher).

## Token Savings Protocols
- Prefer codebase-memory-mcp (`search_graph`, `get_code_snippet`) over reading entire files.
- Consult `graphify-out/graph.json` and `graphify explain` before modifying multi-layer connections.
- Use `justless lookup` and `justless pack` to gather focused context for instructions.
- Execute terminal tasks using `rtk` (Rust Token Killer) to avoid log spam.

## Build & Test Commands
```bash
npm run check              # Run svelte-check
npm run build              # Build both desktop and mobile bundles
cargo check --manifest-path src-tauri/Cargo.toml  # Verify Rust backend
```
