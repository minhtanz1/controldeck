# ControlDeck Production Deployment Guide

This guide covers building, signing, packaging, and deploying ControlDeck for Windows production environments.

---

## 1. Prerequisites for Building Production Releases

Before producing a release build on your build machine or CI server:

- **Operating System**: Windows 10/11 (64-bit)
- **Node.js**: v20 LTS or higher (`node -v`)
- **Package Manager**: npm v10+ (`npm -v`)
- **Rust Toolchain**: Stable MSVC toolchain (`rustup default stable-x86_64-pc-windows-msvc`)
- **WiX Toolset (Optional for MSI)**: For generating Windows Installer packages (`.msi`). Tauri can automatically download it or you can install it via `winget install WiX`.
- **C++ Build Tools**: Microsoft C++ Build Tools (included with Visual Studio or Visual Studio Build Tools).

---

## 2. Local Production Packaging

### Step 1: Clean Install Dependencies
```powershell
npm ci
```

### Step 2: Build Production Frontend Bundles
```powershell
npm run build
```
This generates:
- Desktop distribution assets in `./build`
- Mobile singlefile companion web app in `./src-tauri/src/server/dist-mobile/index.html`

### Step 3: Compile and Package Tauri Executables
```powershell
npm run tauri build
```

Upon completion, your production artifacts will be located in:
```
src-tauri/target/release/bundle/
  ├── nsis/
  │   └── controldeck_0.1.0_x64-setup.exe   (NSIS Windows Installer)
  └── msi/
      └── controldeck_0.1.0_x64_en-US.msi   (Windows Installer MSI package)
```

---

## 3. Automated CI/CD Deployment with GitHub Actions

The repository includes `.github/workflows/release.yml` for automated release packaging.

### Setting Up GitHub Repository
1. Initialize the Git repository and push to GitHub:
   ```powershell
   git init
   git add .
   git commit -m "feat: initial production release"
   git branch -M main
   git remote add origin https://github.com/YOUR_USERNAME/controldeck.git
   git push -u origin main
   ```

2. Triggering a Production Release:
   To create a new official release, push a Git tag starting with `v`:
   ```powershell
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

3. GitHub Actions will:
   - Check out code and set up Node.js & Rust MSVC toolchain.
   - Cache cargo and npm dependencies.
   - Run `npm run build` and `tauri-action`.
   - Publish a new GitHub Release with the installer `.exe` and `.msi` attached.

---

## 4. Network & Firewall Configuration for PocketPad

PocketPad runs an embedded web server on port **1421**. When distributing to end users:

1. **Local Network Rule**: On first launch, Windows Defender Firewall will prompt to allow network access. Users must check **Private Networks** to allow mobile devices on the same Wi-Fi network to connect.
2. **IP Binding**: ControlDeck binds to local LAN interfaces (e.g. `192.168.x.x:1421`).
3. **QR Code Pairing**: The desktop application computes the active LAN IP and generates a QR code. Any mobile device scanning this code opens the low-latency companion in Safari, Chrome, or Firefox.

---

## 5. Binary Verification & SmartScreen

- Unsigned Windows executables may trigger Windows SmartScreen ("Windows protected your PC").
- To avoid this in production enterprise environments, configure a code signing certificate (`TAURI_SIGNING_PRIVATE_KEY` / Windows Authenticode) in GitHub Actions secrets or your local build environment.
