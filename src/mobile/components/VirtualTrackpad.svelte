<script lang="ts">
  import { wsClient } from "../lib/ws.svelte";
  import { settings } from "../lib/settings.svelte";
  import { haptic } from "../lib/haptics";

  let trackpadElem: HTMLDivElement | null = $state(null);
  let isDragging = $state(false);

  // Gesture tracking state
  let touchStartTime = 0;
  let touchStartX = 0;
  let touchStartY = 0;
  let lastTouchX = 0;
  let lastTouchY = 0;
  let hasMoved = false;
  let isTwoFinger = false;
  let initialPinchDist = 0;
  let holdTimer: number | null = null;

  function clearHoldTimer() {
    if (holdTimer !== null) {
      clearTimeout(holdTimer);
      holdTimer = null;
    }
  }

  function onTouchStart(e: TouchEvent) {
    e.preventDefault();

    if (e.touches.length === 1) {
      isTwoFinger = false;
      const t = e.touches[0];
      touchStartTime = performance.now();
      touchStartX = t.clientX;
      touchStartY = t.clientY;
      lastTouchX = t.clientX;
      lastTouchY = t.clientY;
      hasMoved = false;

      // Start hold-to-drag timer
      clearHoldTimer();
      holdTimer = window.setTimeout(() => {
        if (!hasMoved) {
          isDragging = true;
          haptic([30, 50, 30]);
          wsClient.sendMouseDown("left");
        }
      }, settings.data.holdToDragDelay);
    } else if (e.touches.length === 2) {
      clearHoldTimer();
      isTwoFinger = true;
      const t1 = e.touches[0];
      const t2 = e.touches[1];
      initialPinchDist = Math.hypot(t2.clientX - t1.clientX, t2.clientY - t1.clientY);
      lastTouchY = (t1.clientY + t2.clientY) / 2;
    }
  }

  function onTouchMove(e: TouchEvent) {
    e.preventDefault();

    if (e.touches.length === 1) {
      const t = e.touches[0];
      const dx = (t.clientX - lastTouchX) * settings.data.mouseSensitivity;
      const dy = (t.clientY - lastTouchY) * settings.data.mouseSensitivity;

      if (Math.abs(t.clientX - touchStartX) > 6 || Math.abs(t.clientY - touchStartY) > 6) {
        hasMoved = true;
        clearHoldTimer();
      }

      if (Math.abs(dx) > 0.1 || Math.abs(dy) > 0.1) {
        wsClient.sendMouseMove(Math.round(dx * 1.8), Math.round(dy * 1.8));
      }

      lastTouchX = t.clientX;
      lastTouchY = t.clientY;
    } else if (e.touches.length === 2) {
      const t1 = e.touches[0];
      const t2 = e.touches[1];
      const midY = (t1.clientY + t2.clientY) / 2;
      const deltaY = midY - lastTouchY;

      if (Math.abs(deltaY) > 2) {
        wsClient.sendMouseWheel(Math.round(deltaY * 3.5));
        lastTouchY = midY;
      }
    }
  }

  function onTouchEnd(e: TouchEvent) {
    e.preventDefault();
    clearHoldTimer();

    if (isDragging) {
      isDragging = false;
      haptic(25);
      wsClient.sendMouseUp("left");
      return;
    }

    if (e.touches.length === 0) {
      const duration = performance.now() - touchStartTime;

      if (!hasMoved && duration < 320) {
        if (isTwoFinger) {
          // 2-finger tap -> Right Click
          haptic(20);
          wsClient.sendMouseClick("right");
        } else {
          // 1-finger tap -> Left Click
          haptic(15);
          wsClient.sendMouseClick("left");
        }
      }
    }
  }
</script>

<div class="mouse-stage">
  <!-- Trackpad touch surface -->
  <div
    class="trackpad-surface"
    class:dragging-active={isDragging}
    bind:this={trackpadElem}
    ontouchstart={onTouchStart}
    ontouchmove={onTouchMove}
    ontouchend={onTouchEnd}
    ontouchcancel={onTouchEnd}
    role="region"
    aria-label="Virtual Trackpad"
  >
    <!-- Hold to Drag Active Badge -->
    <div class="trackpad-drag-badge" class:active={isDragging}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M5 9l4-4 4 4" /><path d="M9 5v14" /><path d="M19 15l-4 4-4-4" /><path d="M15 19V5" />
      </svg>
      <span>ĐANG GIỮ CHUỘT (Rê để kéo thả, buông để rơi)</span>
    </div>

    <div class="trackpad-hint">
      1 Ngón Rê: Di chuyển trỏ chuột<br />
      1 Ngón Chạm: Click chuột trái<br />
      <strong>Nhấn giữ {settings.data.holdToDragDelay / 1000}s: Chuyển sang Giữ Chuột & Kéo Thả (Buông tay là rơi)</strong><br />
      2 Ngón Chạm: Chuột phải • 2 Ngón Rê: Cuộn / Phóng to bản đồ
    </div>
  </div>

  <!-- Bottom Hardware Mouse Buttons -->
  <div class="mouse-buttons-row">
    <button class="mouse-click-btn" onclick={() => { haptic(); wsClient.sendMouseClick("left"); }}>LEFT</button>
    <button class="mouse-click-btn mouse-middle-btn" onclick={() => { haptic(); wsClient.sendMouseClick("middle"); }}>MID</button>
    <button class="mouse-click-btn" onclick={() => { haptic(); wsClient.sendMouseClick("right"); }}>RIGHT</button>
  </div>
</div>

<style>
  .mouse-stage {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 10px;
    gap: 10px;
    zoom: var(--control-zoom, 1);
    user-select: none;
    -webkit-user-select: none;
    touch-action: none;
  }
  .trackpad-surface {
    flex: 1;
    background: var(--surface);
    border: 2px dashed var(--border);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    position: relative;
    touch-action: none;
    transition: border-color 0.2s, background 0.2s;
  }
  .trackpad-surface.dragging-active {
    border-color: var(--accent-strong);
    background: var(--accent-muted);
    box-shadow: inset 0 2px 8px rgba(23, 35, 42, 0.12);
  }
  .trackpad-drag-badge {
    position: absolute;
    top: 14px;
    display: none;
    align-items: center;
    gap: 6px;
    padding: 6px 14px;
    background: var(--accent);
    color: #fff;
    border-radius: 20px;
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.05em;
    animation: pulseBadge 1.2s infinite ease-in-out;
  }
  .trackpad-drag-badge.active {
    display: flex;
  }
  @keyframes pulseBadge {
    0%, 100% { transform: scale(1); opacity: 1; }
    50% { transform: scale(1.04); opacity: 0.9; }
  }
  .trackpad-hint {
    text-align: center;
    color: var(--text-muted);
    font-size: 12px;
    line-height: 1.8;
    pointer-events: none;
    padding: 0 16px;
  }
  .trackpad-hint strong {
    color: var(--accent-strong);
  }
  .mouse-buttons-row {
    display: flex;
    gap: 8px;
    height: 52px;
    flex-shrink: 0;
  }
  .mouse-click-btn {
    flex: 1;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
    border-radius: 8px;
    font-weight: 800;
    font-size: 13px;
    cursor: pointer;
    touch-action: manipulation;
  }
  .mouse-click-btn:active {
    background: var(--accent);
    color: #fff;
    transform: scale(0.98);
  }
  .mouse-middle-btn {
    flex: 0.7;
    background: var(--surface-elevated);
    color: var(--text-muted);
  }

  .mouse-click-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
