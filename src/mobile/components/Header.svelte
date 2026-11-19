<script lang="ts">
  import { wsClient } from "../lib/ws.svelte";
  import { haptic } from "../lib/haptics";

  let { onOpenSettings }: { onOpenSettings: () => void } = $props();

  let isMenuOpen = $state(false);
  let isFullscreen = $state(false);
  let menuElement: HTMLDivElement | null = $state(null);
  let menuButton: HTMLButtonElement | null = $state(null);

  function closeMenuOnOutside(event: PointerEvent) {
    if (isMenuOpen && !menuElement?.contains(event.target as Node)) {
      isMenuOpen = false;
    }
  }

  function handleWindowKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && isMenuOpen) {
      isMenuOpen = false;
      menuButton?.focus();
    }
  }

  function toggleFullscreen() {
    haptic();
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(() => {});
      isFullscreen = true;
    } else {
      document.exitFullscreen().catch(() => {});
      isFullscreen = false;
    }
  }

  function openSettings() {
    haptic();
    isMenuOpen = false;
    onOpenSettings();
  }

  function toggleOrientation() {
    haptic();
    // Hint / screen orientation lock API if available
    try {
      const orientation = screen.orientation;
      if (orientation && "lock" in orientation) {
        if (orientation.type.startsWith("portrait")) {
          (orientation as any).lock("landscape").catch(() => {});
        } else {
          (orientation as any).lock("portrait").catch(() => {});
        }
      }
    } catch (_) {}
  }
</script>

<svelte:window onpointerdown={closeMenuOnOutside} onkeydown={handleWindowKeydown} />

<header class="top-bar">
  <div class="status-indicator">
    <span class="status-dot" class:connected={wsClient.connected}></span>
    <span class="ping-label">{wsClient.statusText}</span>
  </div>

  <div class="header-title">ControlDeck</div>

  <div class="utility-menu-wrap" bind:this={menuElement}>
    <button
      class="icon-btn"
      class:active={isMenuOpen}
      bind:this={menuButton}
      onclick={() => { haptic(); isMenuOpen = !isMenuOpen; }}
      title="Tùy chọn"
      aria-label="Tùy chọn"
      aria-expanded={isMenuOpen}
      aria-haspopup="menu"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
      </svg>
    </button>

    {#if isMenuOpen}
      <div class="utility-menu" role="menu" aria-label="Tùy chọn PocketPad">
        <button class="utility-menu-item" role="menuitem" onclick={toggleOrientation}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2" />
          </svg>
          <span>Xoay màn hình</span>
        </button>
        <button class="utility-menu-item" role="menuitem" onclick={toggleFullscreen}>
          {#if isFullscreen}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3" />
            </svg>
            <span>Thoát toàn màn hình</span>
          {:else}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" />
            </svg>
            <span>Toàn màn hình</span>
          {/if}
        </button>
        <button class="utility-menu-item" role="menuitem" onclick={openSettings}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 6h16M4 12h16M4 18h16" />
            <circle cx="8" cy="6" r="2" fill="var(--surface)" />
            <circle cx="16" cy="12" r="2" fill="var(--surface)" />
            <circle cx="10" cy="18" r="2" fill="var(--surface)" />
          </svg>
          <span>Cài đặt PocketPad</span>
        </button>
      </div>
    {/if}
  </div>
</header>

<style>
  .top-bar {
    height: 40px;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    flex-shrink: 0;
    position: relative;
    z-index: 60;
    user-select: none;
    -webkit-user-select: none;
  }
  .status-indicator {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 10px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
  }
  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--danger);
    transition: background 0.3s, box-shadow 0.3s;
  }
  .status-dot.connected {
    background: var(--success);
    box-shadow: 0 2px 6px rgba(36, 105, 78, 0.2);
  }
  .header-title {
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.1em;
    color: var(--accent-strong);
    text-transform: uppercase;
  }
  .utility-menu-wrap {
    position: relative;
    display: flex;
  }
  .icon-btn {
    width: 38px;
    height: 38px;
    border-radius: 2px;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    padding: 0;
  }
  .icon-btn:active {
    color: var(--accent-strong);
    border-color: var(--accent-strong);
  }
  .icon-btn.active {
    color: var(--accent-strong);
    border-color: var(--accent-strong);
    background: var(--accent-muted);
  }

  .icon-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
  }
  .icon-btn svg {
    width: 16px;
    height: 16px;
  }
  .utility-menu {
    position: absolute;
    top: calc(100% + 5px);
    right: 0;
    width: min(220px, calc(100vw - 16px));
    padding: 4px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: 0 4px 14px rgba(23, 35, 42, 0.18);
  }
  .utility-menu-item {
    width: 100%;
    min-height: 44px;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 10px;
    background: transparent;
    border: 0;
    color: var(--text);
    font: inherit;
    font-size: 12px;
    font-weight: 600;
    text-align: left;
    cursor: pointer;
  }
  .utility-menu-item + .utility-menu-item {
    border-top: 1px solid var(--border);
  }
  .utility-menu-item:active {
    background: var(--surface-elevated);
    color: var(--accent-strong);
  }
  .utility-menu-item svg {
    width: 16px;
    height: 16px;
    flex: 0 0 16px;
    color: var(--text-muted);
  }
  .utility-menu-item:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
</style>
