<script lang="ts">
  import { settings } from "../lib/settings.svelte";
  import { wsClient } from "../lib/ws.svelte";
  import { haptic } from "../lib/haptics";
  import type { HapticLevel } from "../lib/types";

  let {
    isOpen,
    theme,
    onClose,
    onToggleTheme,
    onEnterLayoutEdit,
  }: {
    isOpen: boolean;
    theme: "light" | "dark";
    onClose: () => void;
    onToggleTheme: () => void;
    onEnterLayoutEdit: () => void;
  } = $props();

  function closeDrawer() {
    haptic();
    onClose();
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      closeDrawer();
    }
  }

  function applyPreset(name: "wme" | "standard" | "split" | "minimal") {
    haptic();
    settings.setPreset(name);
  }

  function setSensitivity(sens: number) {
    haptic();
    settings.setMouseSensitivity(sens);
    wsClient.setSensitivity(sens);
  }

  function setDelay(ms: number) {
    haptic();
    settings.setHoldToDragDelay(ms);
  }

  function setStickDrag(mode: "none" | "left" | "right") {
    haptic();
    settings.setStickDragMode(mode);
  }

  function setRamp(ms: number) {
    haptic();
    settings.setTriggerRamp(ms);
  }

  function setHapticLevel(level: HapticLevel) {
    settings.setHapticLevel(level);
    haptic(25);
  }

  function setScale(scale: "small" | "medium" | "large") {
    haptic();
    settings.setScale(scale);
  }

  function toggleLefty() {
    haptic();
    settings.setLefty(!settings.data.lefty);
  }

  function resetAll() {
    haptic(30);
    settings.resetDefaults();
    wsClient.setSensitivity(settings.data.mouseSensitivity);
  }
</script>

{#if isOpen}
  <div class="modal-overlay" onclick={handleBackdropClick} role="presentation">
    <div
      class="drawer-panel"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => { if (e.key === "Escape") closeDrawer(); }}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <div class="drawer-header">
        <span class="drawer-title">Tùy chỉnh Giao diện Mobile</span>
        <button class="drawer-close-btn" onclick={closeDrawer} aria-label="Đóng">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6 6 18M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="drawer-content">
        <div class="setting-group">
          <div class="setting-label-row">
            <span class="setting-label">Appearance</span>
            <span class="setting-value">{theme === "dark" ? "Dark" : "Light"}</span>
          </div>
          <button
            type="button"
            class="theme-toggle"
            onclick={onToggleTheme}
            aria-pressed={theme === "dark"}
          >
            {#if theme === "dark"}
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                <circle cx="12" cy="12" r="4" />
                <path d="M12 2v2m0 16v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" />
              </svg>
              <span>Switch to light mode</span>
            {:else}
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M20.5 14.2A8.5 8.5 0 0 1 9.8 3.5 8.5 8.5 0 1 0 20.5 14.2Z" />
              </svg>
              <span>Switch to dark mode</span>
            {/if}
          </button>
        </div>

        <!-- Visual Drag & Drop Layout Button -->
        <button class="action-highlight-btn" onclick={() => { closeDrawer(); onEnterLayoutEdit(); }}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 9l4-4 4 4" /><path d="M9 5v14" /><path d="M19 15l-4 4-4-4" /><path d="M15 19V5" />
          </svg>
          Chỉnh Vị Trí Các Phím (Kéo Thả Trực Quan)
        </button>

        <div class="setting-group">
          <div class="setting-label-row">
            <span class="setting-label">Độ rung phản hồi</span>
            <span class="setting-value">
              {settings.data.hapticLevel === "off"
                ? "Tắt"
                : settings.data.hapticLevel === "light"
                  ? "Nhẹ"
                  : settings.data.hapticLevel === "strong"
                    ? "Mạnh"
                    : "Vừa"}
            </span>
          </div>
          <span class="setting-description">
            Cường độ mô phỏng bằng thời lượng rung; khả năng hỗ trợ tùy thiết bị và trình duyệt.
          </span>
          <div class="segmented-control">
            <button
              class="segmented-btn"
              class:active={settings.data.hapticLevel === "off"}
              aria-pressed={settings.data.hapticLevel === "off"}
              onclick={() => setHapticLevel("off")}
            >Tắt</button>
            <button
              class="segmented-btn"
              class:active={settings.data.hapticLevel === "light"}
              aria-pressed={settings.data.hapticLevel === "light"}
              onclick={() => setHapticLevel("light")}
            >Nhẹ</button>
            <button
              class="segmented-btn"
              class:active={settings.data.hapticLevel === "medium"}
              aria-pressed={settings.data.hapticLevel === "medium"}
              onclick={() => setHapticLevel("medium")}
            >Vừa</button>
            <button
              class="segmented-btn"
              class:active={settings.data.hapticLevel === "strong"}
              aria-pressed={settings.data.hapticLevel === "strong"}
              onclick={() => setHapticLevel("strong")}
            >Mạnh</button>
          </div>
        </div>

        <!-- Mouse Speed Sensitivity Slider -->
        <div class="setting-group">
          <div class="setting-label-row">
            <span class="setting-label">Độ nhạy tốc độ chuột (Sensitivity)</span>
            <span class="setting-value">{settings.data.mouseSensitivity}x</span>
          </div>
          <input
            type="range"
            class="slider-input"
            min="0.5"
            max="3.5"
            step="0.1"
            value={settings.data.mouseSensitivity}
            oninput={(e) => setSensitivity(parseFloat((e.target as HTMLInputElement).value))}
          />
          <div class="segmented-control" style="margin-top:4px;">
            <button class="segmented-btn" class:active={settings.data.mouseSensitivity === 0.9} onclick={() => setSensitivity(0.9)}>Chậm (0.9x)</button>
            <button class="segmented-btn" class:active={settings.data.mouseSensitivity === 1.7} onclick={() => setSensitivity(1.7)}>Chuẩn (1.7x)</button>
            <button class="segmented-btn" class:active={settings.data.mouseSensitivity === 2.6} onclick={() => setSensitivity(2.6)}>Nhanh (2.6x)</button>
          </div>
        </div>

        <!-- Trackpad Hold-to-Drag Delay -->
        <div class="setting-group">
          <span class="setting-label">Thời gian giữ để Kéo Thả trên Trackpad</span>
          <div class="segmented-control">
            <button class="segmented-btn" class:active={settings.data.holdToDragDelay === 1500} onclick={() => setDelay(1500)}>1.5 Giây</button>
            <button class="segmented-btn" class:active={settings.data.holdToDragDelay === 2000} onclick={() => setDelay(2000)}>2.0 Giây (Chuẩn)</button>
            <button class="segmented-btn" class:active={settings.data.holdToDragDelay === 2500} onclick={() => setDelay(2500)}>2.5 Giây</button>
          </div>
        </div>

        <!-- Layout Presets -->
        <div class="setting-group">
          <span class="setting-label">Preset Bố Cục</span>
          <div class="preset-selector">
            <button class="preset-chip" class:active={settings.data.preset === "wme"} onclick={() => applyPreset("wme")}>Editor Compact</button>
            <button class="preset-chip" class:active={settings.data.preset === "standard"} onclick={() => applyPreset("standard")}>Full Gamepad</button>
            <button class="preset-chip" class:active={settings.data.preset === "split"} onclick={() => applyPreset("split")}>Trackpad Combo</button>
            <button class="preset-chip" class:active={settings.data.preset === "minimal"} onclick={() => applyPreset("minimal")}>Minimal</button>
          </div>
        </div>

        <!-- Left Stick Drag Option -->
        <div class="setting-group">
          <span class="setting-label">Left Stick Drag (Kéo thả chuột)</span>
          <div class="segmented-control">
            <button class="segmented-btn" class:active={settings.data.stickDragMode === "none"} onclick={() => setStickDrag("none")}>Normal</button>
            <button class="segmented-btn" class:active={settings.data.stickDragMode === "left"} onclick={() => setStickDrag("left")}>Auto Drag Left</button>
            <button class="segmented-btn" class:active={settings.data.stickDragMode === "right"} onclick={() => setStickDrag("right")}>Auto Pan Right</button>
          </div>
        </div>

        <!-- Trigger Ramp Speed -->
        <div class="setting-group">
          <span class="setting-label">Tốc độ Ramp Trigger (LT / RT)</span>
          <div class="segmented-control">
            <button class="segmented-btn" class:active={settings.data.triggerRampMs === 350} onclick={() => setRamp(350)}>Nhanh (350ms)</button>
            <button class="segmented-btn" class:active={settings.data.triggerRampMs === 700} onclick={() => setRamp(700)}>Chuẩn (700ms)</button>
          </div>
        </div>

        <!-- Controller Scale -->
        <div class="setting-group">
          <span class="setting-label">Kích thước giao diện điều khiển</span>
          <span class="setting-description">Áp dụng cho Gamepad, Trackpad và Action Deck.</span>
          <div class="segmented-control">
            <button class="segmented-btn" class:active={settings.data.scale === "small"} onclick={() => setScale("small")}>Nhỏ</button>
            <button class="segmented-btn" class:active={settings.data.scale === "medium"} onclick={() => setScale("medium")}>Vừa</button>
            <button class="segmented-btn" class:active={settings.data.scale === "large"} onclick={() => setScale("large")}>Lớn</button>
          </div>
        </div>

        <!-- Lefty Mode Toggle -->
        <div class="toggle-row">
          <span>Chế độ tay trái (Đảo D-Pad & ABXY)</span>
          <label class="toggle-switch">
            <input type="checkbox" checked={settings.data.lefty} onchange={toggleLefty} />
            <span class="slider-track"></span>
          </label>
        </div>

        <!-- Reset Defaults Button -->
        <button class="reset-defaults-btn" onclick={resetAll}>
          Đặt Lại Cài Đặt Mặc Định
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(23, 35, 42, 0.36);
    z-index: 100;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    touch-action: pan-y !important;
  }
  .drawer-panel {
    background: var(--surface);
    border: 1px solid var(--border);
    border-bottom: none;
    border-radius: 2px 2px 0 0;
    width: 100%;
    max-width: 500px;
    max-height: 85dvh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 -4px 20px rgba(23, 35, 42, 0.2);
    overflow: hidden;
  }
  .drawer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
  }
  .drawer-title {
    font-size: 14px;
    font-weight: 800;
    color: var(--accent-strong);
  }
  .drawer-close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 8px;
    width: 36px;
    height: 36px;
  }

  .drawer-close-btn svg {
    display: block;
    width: 18px;
    height: 18px;
  }

  .theme-toggle {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 10px;
    min-height: 44px;
    padding: 8px 10px;
    border: 1px solid var(--border);
    border-radius: 2px;
    background: var(--surface);
    color: var(--text);
    font-size: 13px;
    font-weight: 600;
    text-align: left;
    cursor: pointer;
  }

  .theme-toggle svg {
    width: 18px;
    height: 18px;
    flex: 0 0 18px;
    color: var(--accent-strong);
  }

  .theme-toggle:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
  .drawer-content {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow-y: scroll !important;
    -webkit-overflow-scrolling: touch !important;
    touch-action: pan-y !important;
    padding-bottom: max(16px, env(safe-area-inset-bottom)) !important;
  }
  .action-highlight-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 11px;
    background: var(--accent-muted);
    border: 1px solid var(--accent);
    border-radius: 7px;
    color: var(--accent-strong);
    font-size: 13px;
    font-weight: 700;
    cursor: pointer;
  }
  .action-highlight-btn svg {
    width: 16px;
    height: 16px;
  }
  .setting-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .setting-label-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .setting-label {
    font-size: 12px;
    font-weight: 700;
    color: var(--text);
  }
  .setting-description {
    font-size: 11px;
    line-height: 1.4;
    color: var(--text-muted);
  }
  .setting-value {
    font-size: 12px;
    color: var(--accent-strong);
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }
  .slider-input {
    width: 100%;
    accent-color: var(--accent-strong);
    cursor: pointer;
    height: 6px;
    background: var(--surface-elevated);
    border-radius: 3px;
    outline: none;
  }
  .segmented-control {
    display: flex;
    background: var(--surface-elevated);
    border-radius: 6px;
    padding: 3px;
    gap: 3px;
  }
  .segmented-btn {
    flex: 1;
    padding: 7px 0;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 12px;
    font-weight: 600;
    border-radius: 4px;
    cursor: pointer;
    text-align: center;
  }
  .segmented-btn.active {
    background: var(--surface-hover);
    color: var(--text);
  }
  .preset-selector {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 6px;
  }
  .preset-chip {
    padding: 8px 10px;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text);
    font-size: 12px;
    font-weight: 600;
    text-align: center;
    cursor: pointer;
  }
  .preset-chip.active {
    border-color: var(--accent-strong);
    color: var(--accent-strong);
    background: var(--accent-muted);
  }
  .toggle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 0;
    font-size: 13px;
  }
  .toggle-switch {
    position: relative;
    width: 40px;
    height: 22px;
  }
  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }
  .slider-track {
    position: absolute;
    cursor: pointer;
    top: 0; left: 0; right: 0; bottom: 0;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 22px;
    transition: .2s;
  }
  .slider-track:before {
    position: absolute;
    content: "";
    height: 16px;
    width: 16px;
    left: 2px;
    bottom: 2px;
    background-color: var(--text-muted);
    border-radius: 50%;
    transition: .2s;
  }
  input:checked + .slider-track {
    background-color: var(--accent-strong);
  }
  input:checked + .slider-track:before {
    transform: translateX(18px);
    background-color: #fff;
  }

  button:focus-visible,
  input:focus-visible + .slider-track {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
  .reset-defaults-btn {
    margin-top: 8px;
    padding: 10px;
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--danger);
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }
</style>
