<script lang="ts">
  import type { MobileMode } from "../lib/types";
  import { haptic } from "../lib/haptics";

  let { currentMode, onSelectMode }: { currentMode: MobileMode; onSelectMode: (mode: MobileMode) => void } = $props();

  function select(mode: MobileMode) {
    if (mode === currentMode) return;
    haptic();
    onSelectMode(mode);
  }
</script>

<nav class="mode-nav">
  <button class="mode-btn" class:active={currentMode === "gamepad"} onclick={() => select("gamepad")}>
    <svg viewBox="0 0 24 24"><path d="M6 12h4m-2-2v4m7-2h.01m3 0h.01M3 8a4 4 0 0 1 4-4h10a4 4 0 0 1 4 4v8a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4V8z"/></svg>
    Gamepad
  </button>
  <button class="mode-btn" class:active={currentMode === "mouse"} onclick={() => select("mouse")}>
    <svg viewBox="0 0 24 24"><rect x="5" y="2" width="14" height="20" rx="7"/><path d="M12 6v4"/></svg>
    Trackpad
  </button>
  <button class="mode-btn" class:active={currentMode === "deck"} onclick={() => select("deck")}>
    <svg viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M7 8h.01M12 8h.01M17 8h.01M7 12h.01M12 12h.01M17 12h.01M7 16h10"/></svg>
    Action Deck
  </button>
</nav>

<style>
  .mode-nav {
    display: flex;
    background: var(--surface);
    border-top: 1px solid var(--border);
    height: 44px;
    flex-shrink: 0;
    user-select: none;
    -webkit-user-select: none;
  }
  .mode-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 10px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s ease;
    padding: 0;
    white-space: nowrap;
  }
  .mode-btn svg {
    width: 14px;
    height: 14px;
    flex: 0 0 14px;
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .mode-btn.active {
    color: var(--accent-strong);
    background: var(--accent-muted);
    border-top: 2px solid var(--accent);
  }

  .mode-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -3px;
  }
</style>
