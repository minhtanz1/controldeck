# ControlDeck

> Modern Gamepad Mapper, Window-Aware Auto Profile Switcher & PocketPad Mobile Controller Companion.

[![CI Verification](https://github.com/USER_OR_ORG/controldeck/actions/workflows/ci.yml/badge.svg)](https://github.com/USER_OR_ORG/controldeck/actions/workflows/ci.yml)
[![Release](https://github.com/USER_OR_ORG/controldeck/actions/workflows/release.yml/badge.svg)](https://github.com/USER_OR_ORG/controldeck/actions/workflows/release.yml)
[![Platform](https://img.shields.io/badge/Platform-Windows-0078D6.svg)](https://github.com/USER_OR_ORG/controldeck)
[![Framework](https://img.shields.io/badge/Engine-Tauri_v2_+_Rust-FFC107.svg)](https://tauri.app/)
[![Frontend](https://img.shields.io/badge/Frontend-Svelte_5_+_Tailwind/CSS-FF3E00.svg)](https://svelte.dev/)

---

## Overview

**ControlDeck** is a lightweight, high-performance desktop utility built with **Tauri v2**, **Rust**, and **Svelte 5**. It bridges the gap between hardware game controllers, mobile devices, and PC gaming workflows:

1. **Hardware Gamepad Remapping**: Map Xbox and DirectInput buttons, analog sticks, and triggers directly to keyboard shortcuts, mouse movements, or macros.
2. **Context-Aware Profile Auto-Switching**: Automatically detects the active foreground window or game executable and switches controller profiles instantly with zero input lag.
3. **PocketPad Mobile Controller Companion**: Turn any smartphone or tablet on your local WiFi network into a low-latency touch controller with Virtual Gamepad, Trackpad, and customizable Action Deck modes—without installing any native app on your phone.
4. **AI-Ready & Token-Optimized**: Pre-configured with knowledge graph indexes (Codebase-Memory MCP, Graphify, Justless, and RTK) allowing AI coding assistants to navigate the codebase with up to 90% fewer tokens.

---

## Key Features

- **Sub-millisecond Polling & Injection**: Native Win32 `XInput` polling combined with `SendInput` hardware simulation for fluid gaming response.
- **Embedded Web Companion Server**: Built-in Axum WebSocket server running locally on the PC, serving the mobile companion app with zero external cloud dependencies.
- **Sleek Cyber/Sci-Fi Interface**: Crafted with modern typography, dark-mode styling, real-time input telemetry, and visual button inspector.
- **Zero-Install Mobile Pairing**: Scan a QR code from your phone's camera to connect directly over local WiFi.
- **Enterprise-Grade CI/CD**: Fully automated GitHub Actions workflow to build release `.msi` and `.exe` installers on every release tag.

---

## System Architecture

```
┌────────────────────────────────────────────────────────┐
│               PocketPad Mobile Browser                 │
│      (Virtual Gamepad / Trackpad / Action Deck)        │
└──────────────────────────┬─────────────────────────────┘
                           │ Low-latency WebSocket (Port 1421)
                           ▼
┌────────────────────────────────────────────────────────┐
│              ControlDeck Core (Rust / Tauri)           │
│  ├─ Axum WebSocket & Static Server                     │
│  ├─ XInput Controller Poller                           │
│  ├─ Foreground Window Detector (Win32 Hooks)           │
│  ├─ Mapping Engine & Profile State Machine             │
│  └─ Win32 SendInput Hardware Injector                  │
└──────────────────────────┬─────────────────────────────┘
                           │ Tauri IPC
                           ▼
┌────────────────────────────────────────────────────────┐
│             Desktop UI (Svelte 5 / SvelteKit)          │
│      (Profile Manager, Mapping Inspector, Telemetry)   │
└────────────────────────────────────────────────────────┘
```

For detailed component diagrams and state transitions, see [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

---

## Quick Start (Development)

### Prerequisites
- [Node.js](https://nodejs.org/) (v20+ recommended)
- [Rust & Cargo](https://rustup.rs/) (stable toolchain)
- Windows 10/11 (x64)

### Setup & Run
```bash
# 1. Install frontend dependencies
npm install

# 2. Run in development mode (launches Tauri window + Vite dev server)
npm run tauri dev

# 3. Or test the PocketPad mobile interface standalone
npm run dev:mobile
```

---

## Production Build & Packaging

### Local Production Build
To compile the release binaries locally:

```bash
# Build desktop & mobile frontend assets
npm run build

# Package Windows release installer (.exe / .msi)
npm run tauri build
```
Compiled installers will be placed in `src-tauri/target/release/bundle/`.

### Automated Deployment via GitHub Actions
This repository includes a production-ready GitHub Actions release workflow:

1. Push your repository to GitHub.
2. Create and push a version tag:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```
3. GitHub Actions will automatically compile the Windows installer and create a GitHub Release with download links.

For full deployment instructions, see [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md).

---

## AI Agent & Token Optimization Architecture

This project is built from the ground up for high-efficiency AI development. When collaborating with AI agents:

- **Codebase-Memory MCP**: Run `search_graph` and `trace_path` to navigate code without consuming tokens on full-file reads.
- **Graphify**: Visual architecture maps and callflows are pre-rendered in `graphify-out/callflow.html` and `graphify-out/graph.html`.
- **Justless**: Indexing in `.justless/` enables surgical symbol resolution (`justless lookup <query>`).
- **RTK Proxy**: Cuts CLI output noise by 80-90%.

Learn more in [docs/TOKEN_OPTIMIZATION.md](docs/TOKEN_OPTIMIZATION.md).

---

## Documentation Index

- [Architecture & Data Flows](docs/ARCHITECTURE.md)
- [Deployment & Release Manual](docs/DEPLOYMENT.md)
- [Token Optimization Guide (MCP + Graphify + Justless)](docs/TOKEN_OPTIMIZATION.md)
- [Contributing Guidelines](docs/CONTRIBUTING.md)

---

## License

This project is distributed under the MIT License.
