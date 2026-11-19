# Graph Report - controldeck-prod  (2026-11-19)

## Corpus Check
- 50 files · ~36,029 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 4 file(s) not represented in the graph (top: (none) 2, .icns 1, .ico 1)

## Summary
- 641 nodes · 1051 edges · 41 communities (33 shown, 8 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 20 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- server/mod.rs
- VirtualGamepad.svelte
- schema.rs
- with
- commands.rs
- +page.svelte
- SettingsStore
- window.rs
- MappingInspector.svelte
- WebSocketClient
- WsClientMessage
- tauri.conf.json
- steps
- AppRulesModal.svelte
- haptic
- settings.svelte.ts
- package.json
- compilerOptions
- ProfileList.svelte
- src/lib/types.ts
- App.svelte
- ws.svelte.ts
- scripts
- ControllerView.svelte
- mobile/components/Header.svelte
- VirtualTrackpad.svelte
- devDependencies
- MobileConnectModal.svelte
- LayoutEditBar.svelte
- vite
- @sveltejs/vite-plugin-svelte
- default.json
- svelte.config.js
- $lib
- svelte
- AGENTS.md
- CLAUDE.md
- +layout.ts
- controldeck

## God Nodes (most connected - your core abstractions)
1. `haptic()` - 41 edges
2. `SettingsStore` - 24 edges
3. `AppState` - 21 edges
4. `WebSocketClient` - 21 edges
5. `WsClientMessage` - 20 edges
6. `ProfileStore` - 19 edges
7. `handle_socket()` - 19 edges
8. `execute_action()` - 17 edges
9. `Action` - 12 edges
10. `with` - 11 edges

## Surprising Connections (you probably didn't know these)
- `simulate_controller_button()` --calls--> `execute_action()`  [INFERRED]
  src-tauri/src/commands.rs → src-tauri/src/injection/sendinput.rs
- `simulate_controller_button()` --calls--> `release_action()`  [INFERRED]
  src-tauri/src/commands.rs → src-tauri/src/injection/sendinput.rs
- `test_action()` --calls--> `execute_action()`  [INFERRED]
  src-tauri/src/commands.rs → src-tauri/src/injection/sendinput.rs
- `triggerAction()` --calls--> `haptic()`  [EXTRACTED]
  src/mobile/components/ActionDeck.svelte → src/mobile/lib/haptics.ts
- `toggleFullscreen()` --calls--> `haptic()`  [EXTRACTED]
  src/mobile/components/Header.svelte → src/mobile/lib/haptics.ts

## Import Cycles
- 1-file cycle: `src-tauri/src/server/mod.rs -> src-tauri/src/server/mod.rs`

## Communities (41 total, 8 thin omitted)

### Community 0 - "server/mod.rs"
Cohesion: 0.05
Nodes (33): execute_action(), execute_macro_step(), release_action(), send_key_down(), send_key_up(), send_mouse_click(), send_mouse_down(), send_mouse_move() (+25 more)

### Community 1 - "VirtualGamepad.svelte"
Cohesion: 0.06
Nodes (45): btnDown(), btnUp(), capturePointer(), compactLandscape, controlScalePreview, draggingCluster, dragStartOffset, endDragCluster() (+37 more)

### Community 2 - "schema.rs"
Cohesion: 0.08
Nodes (15): apply_deadzone(), ControllerState, Action, ComboKey, Key, Macro, Mouse, Toggle (+7 more)

### Community 3 - "with"
Cohesion: 0.06
Nodes (33): permissions, runs-on, steps, strategy, GITHUB_TOKEN, args, platform, jobs (+25 more)

### Community 4 - "commands.rs"
Cohesion: 0.14
Nodes (18): AppState, delete_profile(), export_profile(), get_active_profile(), get_active_window(), get_controller_state(), get_mobile_server_info(), get_open_windows() (+10 more)

### Community 5 - "+page.svelte"
Cohesion: 0.09
Nodes (18): activeProfile, activeWindow, controllerState, createProfile(), enabled, exportProfile(), importProfile(), mobileInfo (+10 more)

### Community 6 - "SettingsStore"
Cohesion: 0.20
Nodes (3): SettingsStore, ControlLayoutSnapshot, MobileSettings

### Community 7 - "window.rs"
Cohesion: 0.12
Nodes (7): ActiveWindowInfo, enum_windows_callback(), ENUMERATED_WINDOWS, extract_clean_title(), get_active_window(), get_open_windows(), OpenWindowInfo

### Community 8 - "MappingInspector.svelte"
Cohesion: 0.10
Nodes (20): actionType, buildCurrentAction(), commitSave(), currentKey, handleKeyDown(), isRecording, isStick, isTrigger (+12 more)

### Community 10 - "WsClientMessage"
Cohesion: 0.10
Nodes (19): WsClientMessage, Axis, BtnDown, BtnUp, DeckAction, GetProfile, KeyCombo, KeyTap (+11 more)

### Community 11 - "tauri.conf.json"
Cohesion: 0.11
Nodes (17): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+9 more)

### Community 12 - "steps"
Cohesion: 0.12
Nodes (16): runs-on, steps, jobs, check-project, name, on, pull_request, push (+8 more)

### Community 13 - "AppRulesModal.svelte"
Cohesion: 0.12
Nodes (11): activeTab, addWindowRule(), browserTabs, filteredWindows, isLoadingWindows, isRuleAdded(), nativeApps, openWindows (+3 more)

### Community 14 - "haptic"
Cohesion: 0.24
Nodes (14): applyPreset(), closeDrawer(), handleBackdropClick(), {
    isOpen,
    theme,
    onClose,
    onToggleTheme,
    onEnterLayoutEdit,
  }, resetAll(), setDelay(), setHapticLevel(), setRamp() (+6 more)

### Community 15 - "settings.svelte.ts"
Cohesion: 0.22
Nodes (10): DEFAULT_CLUSTER_SCALES, DEFAULT_STICK_SIZES, defaultSettings, ClusterPosition, ControlClusterScales, HapticLevel, ResizableControlCluster, SavedControlLayout (+2 more)

### Community 16 - "package.json"
Cohesion: 0.19
Nodes (12): dependencies, qrcode@^1.5.4, @tauri-apps/api@^2, @tauri-apps/plugin-opener@^2, description, license, name, type (+4 more)

### Community 17 - "compilerOptions"
Cohesion: 0.15
Nodes (12): ./.svelte-kit/tsconfig.json, compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, moduleResolution, resolveJsonModule (+4 more)

### Community 18 - "ProfileList.svelte"
Cohesion: 0.18
Nodes (9): cancelRename(), editingProfileId, handleRenameKeydown(), isSavingRename, {
    profiles = [],
    activeProfile = null,
    onSelectProfile,
    onCreateProfile,
    onOpenRules,
    onExportProfile,
    onImportProfile,
    onDeleteProfile,
    onRenameProfile,
  }, renameDraft, renameError, renameInput (+1 more)

### Community 19 - "src/lib/types.ts"
Cohesion: 0.15
Nodes (12): Action, ActiveWindowInfo, AppRule, ComboAction, ControllerState, DeckItem, Mappings, MobileServerInfo (+4 more)

### Community 20 - "App.svelte"
Cohesion: 0.18
Nodes (11): controlZoom, currentMode, isEditingLayout, isSettingsOpen, showThemeNotice(), theme, themeNotice, toggleTheme() (+3 more)

### Community 21 - "ws.svelte.ts"
Cohesion: 0.21
Nodes (9): groupedCategories, handleKeydown(), sendText(), textInputValue, triggerAction(), DeckItem, ProfileSummary, ProfileSyncMessage (+1 more)

### Community 22 - "scripts"
Cohesion: 0.18
Nodes (11): scripts, build, build:desktop, build:mobile, check, check:watch, dev, dev:mobile (+3 more)

### Community 23 - "ControllerView.svelte"
Cohesion: 0.18
Nodes (8): controlGroups, leftStickOffsetX, leftStickOffsetY, ltPercent, {
    profile = null,
    controllerState = null,
    selectedButton = "A",
    onSelectButton,
    onTestButton,
  }, rightStickOffsetX, rightStickOffsetY, rtPercent

### Community 24 - "mobile/components/Header.svelte"
Cohesion: 0.18
Nodes (8): isFullscreen, isMenuOpen, menuButton, menuElement, { onOpenSettings }, openSettings(), toggleFullscreen(), toggleOrientation()

### Community 25 - "VirtualTrackpad.svelte"
Cohesion: 0.29
Nodes (8): clearHoldTimer(), isDragging, onTouchEnd(), onTouchMove(), onTouchStart(), trackpadElem, HAPTIC_MULTIPLIERS, settings

### Community 26 - "devDependencies"
Cohesion: 0.22
Nodes (9): devDependencies, svelte-check@^4.6.0, @tauri-apps/cli@^2, @types/qrcode@^1.5.6, typescript@~6.0.3, svelte-check, @tauri-apps/cli, @types/qrcode (+1 more)

### Community 27 - "MobileConnectModal.svelte"
Cohesion: 0.22
Nodes (5): activeTab, canvasEl, copied, {
    mobileInfo = null,
    onClose,
  }, modalEl

### Community 28 - "LayoutEditBar.svelte"
Cohesion: 0.25
Nodes (6): deleteLayout(), feedback, isLibraryOpen, layoutName, {
    layouts,
    activeLayoutId,
    canSaveLayout,
    onSelectLayout,
    onSaveLayout,
    onDeleteLayout,
    onReset,
    onSave,
  }, saveLayout()

### Community 29 - "vite"
Cohesion: 0.33
Nodes (4): @sveltejs/kit@^2.65.1, vite@^8.0.16, @sveltejs/kit, vite

### Community 30 - "@sveltejs/vite-plugin-svelte"
Cohesion: 0.33
Nodes (4): @sveltejs/vite-plugin-svelte@^7.1.2, vite-plugin-singlefile@^2.3.4, @sveltejs/vite-plugin-svelte, vite-plugin-singlefile

### Community 31 - "default.json"
Cohesion: 0.33
Nodes (5): description, identifier, permissions, $schema, windows

### Community 32 - "svelte.config.js"
Cohesion: 0.40
Nodes (3): @sveltejs/adapter-static@^3.0.10, @sveltejs/adapter-static, config

### Community 34 - "svelte"
Cohesion: 0.50
Nodes (3): svelte@^5.56.3, svelte, target

## Knowledge Gaps
- **25 isolated node(s):** `controldeck`, `Key`, `ComboKey`, `Toggle`, `Mouse` (+20 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 304 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `svelte` connect `svelte` to `VirtualGamepad.svelte`, `+page.svelte`, `AppRulesModal.svelte`, `package.json`, `ProfileList.svelte`, `App.svelte`, `MobileConnectModal.svelte`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **What connects `controldeck`, `Key`, `ComboKey` to the rest of the system?**
  _25 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `server/mod.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05432098765432099 - nodes in this community are weakly interconnected._
- **Why does `WebSocketClient` connect `WebSocketClient` to `ws.svelte.ts`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **Should `VirtualGamepad.svelte` be split into smaller, more focused modules?**
  _Cohesion score 0.0596078431372549 - nodes in this community are weakly interconnected._
- **Why does `haptic()` connect `haptic` to `VirtualGamepad.svelte`, `WebSocketClient`, `App.svelte`, `ws.svelte.ts`, `mobile/components/Header.svelte`, `VirtualTrackpad.svelte`, `LayoutEditBar.svelte`?**
  _High betweenness centrality (0.026) - this node is a cross-community bridge._
- **Should `schema.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08403361344537816 - nodes in this community are weakly interconnected._