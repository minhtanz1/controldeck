# AI Agent Guidelines & Token Optimization Kit

This repository is optimized for autonomous AI coding agents (Antigravity, Claude Code, Cursor, Codex, Gemini CLI).
Follow these guidelines to maintain system stability and reduce token consumption by up to 90%.

---

## 1. Token Optimization Hierarchy (Strict Priority)

Do NOT read full files or run brute-force `grep`/`find` when exploring the codebase. Always follow this 4-tier navigation order:

```
┌────────────────────────────────────────────────────────┐
│ Tier 1: codebase-memory-mcp (Fastest & Zero Token Read) │
│ -> search_graph, trace_path, get_code_snippet          │
└────────────────────────────────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ Tier 2: graphify (Architecture & Call Flow Insights)  │
│ -> graphify explain, graphify path, graphify section   │
└────────────────────────────────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ Tier 3: justless (Surgical Code Extraction & Context)  │
│ -> justless lookup, justless pack, justless exec       │
└────────────────────────────────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ Tier 4: RTK (Rust Token Killer CLI Proxy)              │
│ -> Automatically trims verbose command output by 90%   │
└────────────────────────────────────────────────────────┘
```

### Tier 1: `codebase-memory-mcp`
When `codebase-memory-mcp` is connected, use project `D-waze-controldeck-prod`:
- Search symbols/routes: `search_graph(project="D-waze-controldeck-prod", name_pattern=".*Profile.*")`
- Trace callers/callees: `trace_path(project="D-waze-controldeck-prod", function_name="inject_input", direction="inbound")`
- Read exact snippet: `get_code_snippet(project="D-waze-controldeck-prod", qualified_name="...")`

### Tier 2: `graphify`
Consult the precomputed knowledge graph in `graphify-out/`:
- Explain node & neighbors: `graphify explain "handle_websocket" --graph graphify-out/graph.json`
- Find path between components: `graphify path "VirtualGamepad" "sendinput" --graph graphify-out/graph.json`
- Visual call flow: open `graphify-out/callflow.html` or `graphify-out/graph.html` in browser.

### Tier 3: `justless`
- Search code and docs without loading files: `justless lookup "WebSocket port"`
- Pack context for a specific task: `justless pack "Fix pocketpad stick sensitivity"`
- Targeted slice read: `justless code src-tauri/src/server/mod.rs 40 80`
- Replay command output: `justless recall <ID>`

### Tier 4: `RTK` (Rust Token Killer)
- All shell executions (git, cargo, npm) should pass through RTK to filter redundant stdout/stderr logs.
- Use `rtk git status`, `rtk cargo check`, `rtk npm test`.

---

## 2. Project Architecture Overview

ControlDeck consists of 3 integrated layers:

1. **Desktop Client (Tauri v2 + Svelte 5 + SvelteKit)**:
   - UI for mapping physical controllers (Xbox / DirectInput) to keyboard and mouse actions.
   - Profile management with auto-switching based on active foreground window.
   - QR code and connection manager for PocketPad companion.

2. **PocketPad Mobile Web Companion (Svelte 5 Singlefile Bundle)**:
   - Embedded HTML/JS bundle served locally via Rust HTTP/WebSocket server on port 1421.
   - Provides Virtual Gamepad, Trackpad, and customizable Action Deck for smartphones.
   - Low-latency binary/JSON WebSocket protocol for instant input transmission.

3. **Core Engine (Rust Backend)**:
   - Controller polling via Windows XInput API.
   - Active window detector via Win32 API (`GetForegroundWindow`, `GetWindowTextW`).
   - Virtual input injection via Win32 `SendInput` (mouse movements, clicks, keystrokes).
   - Local Axum WebSocket server for mobile phone connection.

---

## 3. Standard Verification Commands

Always run non-destructive checks after editing code:

```bash
# Frontend validation
npm run check
npm run build

# Rust backend validation
cargo check --manifest-path src-tauri/Cargo.toml
```

Do not commit build artifacts (`build/`, `target/`, `dist/`).
