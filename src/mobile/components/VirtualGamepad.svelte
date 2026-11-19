<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { wsClient } from "../lib/ws.svelte";
  import { settings } from "../lib/settings.svelte";
  import { haptic } from "../lib/haptics";
  import type {
    ClusterPosition,
    ResizableControlCluster,
    StickSizes,
  } from "../lib/types";

  let { isEditingLayout }: { isEditingLayout: boolean } = $props();

  // Drag lock toggle state
  let isDragLockActive = $state(false);

  // Progressive Triggers state
  let ltPct = $state(0);
  let rtPct = $state(0);
  let ltAnimFrame: number | null = null;
  let rtAnimFrame: number | null = null;

  // Joysticks puck positions (relative to base center)
  let leftPuckPos = $state({ x: 0, y: 0 });
  let rightPuckPos = $state({ x: 0, y: 0 });
  let leftStickPointerId: number | null = null;
  let rightStickPointerId: number | null = null;
  let leftBaseRect: DOMRect | null = null;
  let rightBaseRect: DOMRect | null = null;
  let leftStickDragTriggered = false;
  let compactLandscape = $state(false);
  let resizePreview = $state<Partial<StickSizes>>({});
  let resizingStick = $state<{
    side: keyof StickSizes;
    pointerId: number;
    startX: number;
    startY: number;
    startSize: number;
  } | null>(null);
  let controlScalePreview = $state<Partial<Record<ResizableControlCluster, number>>>({});
  let resizingControl = $state<{
    cluster: ResizableControlCluster;
    pointerId: number;
    startX: number;
    startY: number;
    startScale: number;
  } | null>(null);

  // Custom layout dragging state
  let draggingCluster = $state<string | null>(null);
  let dragStartOffset = { x: 0, y: 0 };

  const compactLandscapeQuery = "(max-height: 500px) and (orientation: landscape)";

  onMount(() => {
    const mediaQuery = window.matchMedia(compactLandscapeQuery);
    const updateCompactLandscape = () => (compactLandscape = mediaQuery.matches);
    updateCompactLandscape();
    mediaQuery.addEventListener("change", updateCompactLandscape);
    return () => mediaQuery.removeEventListener("change", updateCompactLandscape);
  });

  onDestroy(() => {
    if (resizingStick) {
      settings.setJoystickSize(
        resizingStick.side,
        resizePreview[resizingStick.side] ?? resizingStick.startSize,
      );
    }
    if (resizingControl) {
      settings.setClusterScale(
        resizingControl.cluster,
        controlScalePreview[resizingControl.cluster] ?? resizingControl.startScale,
      );
    }
  });

  // ================= 1. Progressive Triggers =================
  function startTrigger(trigger: "lt" | "rt") {
    haptic();
    const startTime = performance.now();
    const duration = settings.data.triggerRampMs;

    function ramp() {
      const elapsed = performance.now() - startTime;
      const progress = Math.min(1.0, elapsed / duration);
      const pct = Math.round(progress * 100);

      if (trigger === "lt") {
        ltPct = pct;
        wsClient.sendTriggerMove("lt", progress);
        if (progress < 1.0) ltAnimFrame = requestAnimationFrame(ramp);
      } else {
        rtPct = pct;
        wsClient.sendTriggerMove("rt", progress);
        if (progress < 1.0) rtAnimFrame = requestAnimationFrame(ramp);
      }
    }

    if (trigger === "lt") {
      if (ltAnimFrame) cancelAnimationFrame(ltAnimFrame);
      ltAnimFrame = requestAnimationFrame(ramp);
    } else {
      if (rtAnimFrame) cancelAnimationFrame(rtAnimFrame);
      rtAnimFrame = requestAnimationFrame(ramp);
    }
  }

  function stopTrigger(trigger: "lt" | "rt") {
    if (trigger === "lt") {
      if (ltAnimFrame) cancelAnimationFrame(ltAnimFrame);
      ltAnimFrame = null;
      ltPct = 0;
      wsClient.sendTriggerMove("lt", 0);
    } else {
      if (rtAnimFrame) cancelAnimationFrame(rtAnimFrame);
      rtAnimFrame = null;
      rtPct = 0;
      wsClient.sendTriggerMove("rt", 0);
    }
  }

  // ================= 2. Standard Buttons =================
  function btnDown(btn: string) {
    haptic();
    wsClient.sendButtonDown(btn);
  }
  function btnUp(btn: string) {
    wsClient.sendButtonUp(btn);
  }

  function toggleDragLock() {
    isDragLockActive = !isDragLockActive;
    haptic(25);
    if (isDragLockActive) {
      wsClient.sendMouseDown("left");
    } else {
      wsClient.sendMouseUp("left");
    }
  }

  // ================= 3. Virtual Joysticks =================
  const STICK_MAX_RADIUS = 36;
  const DEFAULT_STICK_SIZE = 112;
  const STICK_PUCK_SIZE = 52;

  const pressedButtons = new Map<number, string>();
  const pressedTriggers = new Map<number, "lt" | "rt">();

  function capturePointer(e: PointerEvent) {
    if (e.isTrusted) {
      (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    }
  }

  function onButtonPointerDown(e: PointerEvent, btn: string) {
    if (isEditingLayout || e.button !== 0 || pressedButtons.has(e.pointerId)) return;
    e.preventDefault();
    capturePointer(e);
    pressedButtons.set(e.pointerId, btn);
    btnDown(btn);
  }

  function onButtonPointerEnd(e: PointerEvent) {
    const btn = pressedButtons.get(e.pointerId);
    if (!btn) return;
    pressedButtons.delete(e.pointerId);
    btnUp(btn);
  }

  function onTriggerPointerDown(e: PointerEvent, trigger: "lt" | "rt") {
    if (isEditingLayout || e.button !== 0 || pressedTriggers.has(e.pointerId)) return;
    e.preventDefault();
    capturePointer(e);
    pressedTriggers.set(e.pointerId, trigger);
    startTrigger(trigger);
  }

  function onTriggerPointerEnd(e: PointerEvent) {
    const trigger = pressedTriggers.get(e.pointerId);
    if (!trigger) return;
    pressedTriggers.delete(e.pointerId);
    stopTrigger(trigger);
  }

  function onClusterPointerDown(e: PointerEvent, clusterId: string) {
    if (!isEditingLayout || e.button !== 0) return;
    e.preventDefault();
    capturePointer(e);
    startDragCluster(clusterId, e.clientX, e.clientY);
  }

  function onStagePointerMove(e: PointerEvent) {
    if (isEditingLayout && draggingCluster) {
      moveDragCluster(e.clientX, e.clientY);
    }
  }

  function onStagePointerEnd() {
    if (isEditingLayout && draggingCluster) endDragCluster();
  }

  function onLeftStickStart(e: PointerEvent) {
    if (e.button !== 0 || leftStickPointerId !== null) return;
    e.preventDefault();
    if (isEditingLayout) return;
    leftStickPointerId = e.pointerId;
    haptic();
    capturePointer(e);
    const base = e.currentTarget as HTMLElement;
    leftBaseRect = base.getBoundingClientRect();
    updateStick(e.clientX, e.clientY, "left");
  }

  function onLeftStickMove(e: PointerEvent) {
    if (leftStickPointerId !== e.pointerId) return;
    e.preventDefault();
    if (isEditingLayout) return;
    updateStick(e.clientX, e.clientY, "left");
  }

  function onLeftStickEnd(e: PointerEvent) {
    e.preventDefault();
    if (leftStickPointerId !== e.pointerId) return;
    leftStickPointerId = null;
    leftPuckPos = { x: 0, y: 0 };
    wsClient.sendStickMove("left", 0, 0);

    if (leftStickDragTriggered) {
      leftStickDragTriggered = false;
      wsClient.sendMouseUp("left");
    }
  }

  function onRightStickStart(e: PointerEvent) {
    if (e.button !== 0 || rightStickPointerId !== null) return;
    e.preventDefault();
    if (isEditingLayout) return;
    rightStickPointerId = e.pointerId;
    haptic();
    capturePointer(e);
    const base = e.currentTarget as HTMLElement;
    rightBaseRect = base.getBoundingClientRect();
    updateStick(e.clientX, e.clientY, "right");
  }

  function onRightStickMove(e: PointerEvent) {
    if (rightStickPointerId !== e.pointerId) return;
    e.preventDefault();
    if (isEditingLayout) return;
    updateStick(e.clientX, e.clientY, "right");
  }

  function onRightStickEnd(e: PointerEvent) {
    e.preventDefault();
    if (rightStickPointerId !== e.pointerId) return;
    rightStickPointerId = null;
    rightPuckPos = { x: 0, y: 0 };
    wsClient.sendStickMove("right", 0, 0);
  }

  function getStickSize(side: keyof StickSizes): number {
    return resizePreview[side] ?? settings.data.joystickSizes[side];
  }

  function getRenderedStickSize(side: keyof StickSizes): number {
    const size = getStickSize(side);
    return compactLandscape ? Math.min(size, 88) : size;
  }

  function getStickStyle(side: keyof StickSizes): string {
    const size = getRenderedStickSize(side);
    const puckSize = Math.round((size * STICK_PUCK_SIZE) / DEFAULT_STICK_SIZE);
    const maxRadius = Math.round((size * STICK_MAX_RADIUS) / DEFAULT_STICK_SIZE);
    const clusterId = side === "left" ? "leftStick" : "rightStick";
    return `${getClusterStyle(clusterId)} --stick-size: ${size}px; --stick-puck-size: ${puckSize}px; --stick-max-radius: ${maxRadius}px;`;
  }

  function onStickResizeStart(e: PointerEvent, side: keyof StickSizes) {
    if (!isEditingLayout || e.button !== 0 || resizingStick) return;
    e.preventDefault();
    e.stopPropagation();
    capturePointer(e);
    const size = getRenderedStickSize(side);
    resizePreview = { ...resizePreview, [side]: size };
    resizingStick = {
      side,
      pointerId: e.pointerId,
      startX: e.clientX,
      startY: e.clientY,
      startSize: size,
    };
    haptic();
  }

  function onStickResizeMove(e: PointerEvent) {
    if (!resizingStick || resizingStick.pointerId !== e.pointerId) return;
    e.preventDefault();
    e.stopPropagation();
    const delta = (e.clientX - resizingStick.startX + e.clientY - resizingStick.startY) / 2;
    const size = Math.max(72, Math.min(144, resizingStick.startSize + delta));
    resizePreview = {
      ...resizePreview,
      [resizingStick.side]: Math.round(size / 4) * 4,
    };
  }

  function onStickResizeEnd(e: PointerEvent) {
    if (!resizingStick || resizingStick.pointerId !== e.pointerId) return;
    e.preventDefault();
    e.stopPropagation();
    const { side, startSize } = resizingStick;
    const size = resizePreview[side] ?? startSize;
    resizingStick = null;
    resizePreview = {};
    if (size !== startSize) {
      settings.setJoystickSize(side, size);
      haptic();
    }
  }

  function updateStick(clientX: number, clientY: number, stick: "left" | "right") {
    const rect = stick === "left" ? leftBaseRect : rightBaseRect;
    if (!rect) return;

    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;
    const dx = clientX - centerX;
    const dy = clientY - centerY;
    const dist = Math.hypot(dx, dy);

    const maxRadius = Math.round((getRenderedStickSize(stick) * STICK_MAX_RADIUS) / DEFAULT_STICK_SIZE);
    let clampedDist = dist;
    let normX = 0;
    let normY = 0;

    if (dist > 0) {
      clampedDist = Math.min(dist, maxRadius);
      normX = (dx / dist) * (clampedDist / maxRadius);
      normY = (dy / dist) * (clampedDist / maxRadius);
    }

    const puckX = normX * maxRadius;
    const puckY = normY * maxRadius;

    if (stick === "left") {
      leftPuckPos = { x: puckX, y: puckY };
      wsClient.sendStickMove("left", normX, -normY);

      // Handle Left Stick Drag option
      if (settings.data.stickDragMode === "left") {
        if (clampedDist > 6 && !leftStickDragTriggered) {
          leftStickDragTriggered = true;
          wsClient.sendMouseDown("left");
        } else if (clampedDist <= 6 && leftStickDragTriggered) {
          leftStickDragTriggered = false;
          wsClient.sendMouseUp("left");
        }
      }
    } else {
      rightPuckPos = { x: puckX, y: puckY };
      wsClient.sendStickMove("right", normX, -normY);
    }
  }

  // ================= 4. Custom Layout Drag & Drop =================
  function startDragCluster(clusterId: string, clientX: number, clientY: number) {
    if (!isEditingLayout) return;
    draggingCluster = clusterId;
    const currentPos = settings.data.customPositions?.[clusterId] || { x: 0, y: 0 };
    dragStartOffset = {
      x: clientX - currentPos.x,
      y: clientY - currentPos.y,
    };
    haptic(20);
  }

  function moveDragCluster(clientX: number, clientY: number) {
    if (!isEditingLayout || !draggingCluster) return;
    const newX = clientX - dragStartOffset.x;
    const newY = clientY - dragStartOffset.y;
    settings.setCustomPosition(draggingCluster, { x: newX, y: newY });
  }

  function endDragCluster() {
    if (draggingCluster) {
      draggingCluster = null;
      haptic(15);
    }
  }

  function getClusterStyle(clusterId: string): string {
    const pos = settings.data.customPositions?.[clusterId];
    if (!pos) return "";
    return `transform: translate3d(${pos.x}px, ${pos.y}px, 0);`;
  }

  function getControlScale(cluster: ResizableControlCluster): number {
    return controlScalePreview[cluster] ?? settings.data.clusterScales[cluster];
  }

  function getControlClusterStyle(cluster: ResizableControlCluster): string {
    return `${getClusterStyle(cluster)} --cluster-zoom: ${getControlScale(cluster)};`;
  }

  function onControlResizeStart(e: PointerEvent, cluster: ResizableControlCluster) {
    if (!isEditingLayout || e.button !== 0 || resizingControl) return;
    e.preventDefault();
    e.stopPropagation();
    capturePointer(e);
    resizingControl = {
      cluster,
      pointerId: e.pointerId,
      startX: e.clientX,
      startY: e.clientY,
      startScale: getControlScale(cluster),
    };
    controlScalePreview = {
      ...controlScalePreview,
      [cluster]: getControlScale(cluster),
    };
    haptic();
  }

  function onControlResizeMove(e: PointerEvent) {
    if (!resizingControl || resizingControl.pointerId !== e.pointerId) return;
    e.preventDefault();
    e.stopPropagation();
    const delta = (e.clientX - resizingControl.startX + e.clientY - resizingControl.startY) / 2;
    const scale = Math.max(0.7, Math.min(1.3, resizingControl.startScale + delta / 120));
    controlScalePreview = {
      ...controlScalePreview,
      [resizingControl.cluster]: Math.round(scale / 0.05) * 0.05,
    };
  }

  function onControlResizeEnd(e: PointerEvent) {
    if (!resizingControl || resizingControl.pointerId !== e.pointerId) return;
    e.preventDefault();
    e.stopPropagation();
    const { cluster, startScale } = resizingControl;
    const scale = controlScalePreview[cluster] ?? startScale;
    resizingControl = null;
    controlScalePreview = {};
    if (scale !== startScale) {
      settings.setClusterScale(cluster, scale);
      haptic();
    }
  }
</script>

<div
  class="gamepad-stage scale-{settings.data.scale}"
  class:editing-mode={isEditingLayout}
  class:lefty-mode={settings.data.lefty}
  role="region"
  aria-label="Virtual Gamepad"
  onpointermove={onStagePointerMove}
  onpointerup={onStagePointerEnd}
  onpointercancel={onStagePointerEnd}
>
  <!-- Shoulder & Progressive Trigger Row -->
  <div
    class="bumpers-row cluster-draggable"
    class:dragging={draggingCluster === "shoulders"}
    style={getControlClusterStyle("shoulders")}
    role="region"
    aria-label="Shoulders and triggers"
    onpointerdown={(e) => onClusterPointerDown(e, "shoulders")}
  >
    <div class="shoulder-group">
      <button
        class="shoulder-btn trigger-btn"
        onpointerdown={(e) => onTriggerPointerDown(e, "lt")}
        onpointerup={onTriggerPointerEnd}
        onpointercancel={onTriggerPointerEnd}
        onlostpointercapture={onTriggerPointerEnd}
      >
        <div class="trigger-gauge" style="height: {ltPct}%;"></div>
        <span class="trigger-text">LT <span class="trigger-pct">{ltPct}%</span></span>
      </button>
      <button
        class="shoulder-btn"
        onpointerdown={(e) => onButtonPointerDown(e, "LB")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >LB</button>
    </div>

    <div class="shoulder-group">
      <button
        class="shoulder-btn"
        onpointerdown={(e) => onButtonPointerDown(e, "RB")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >RB</button>
      <button
        class="shoulder-btn trigger-btn"
        onpointerdown={(e) => onTriggerPointerDown(e, "rt")}
        onpointerup={onTriggerPointerEnd}
        onpointercancel={onTriggerPointerEnd}
        onlostpointercapture={onTriggerPointerEnd}
      >
        <div class="trigger-gauge" style="height: {rtPct}%;"></div>
        <span class="trigger-text">RT <span class="trigger-pct">{rtPct}%</span></span>
      </button>
    </div>
    {#if isEditingLayout}
      <button
        class="control-resize-handle"
        aria-label={`Đổi kích thước vai và trigger, ${Math.round(getControlScale("shoulders") * 100)}%`}
        title={`Vai và trigger, ${Math.round(getControlScale("shoulders") * 100)}%`}
        onpointerdown={(e) => onControlResizeStart(e, "shoulders")}
        onpointermove={onControlResizeMove}
        onpointerup={onControlResizeEnd}
        onpointercancel={onControlResizeEnd}
        onlostpointercapture={onControlResizeEnd}
      >
        <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M5 11 11 5M6 5h5v5" /></svg>
      </button>
    {/if}
  </div>

  <!-- Center Menu Row (Back, Drag Lock, Start) -->
  <div
    class="center-menu-row cluster-draggable"
    class:dragging={draggingCluster === "menu"}
    style={getControlClusterStyle("menu")}
    role="region"
    aria-label="Center menu buttons"
    onpointerdown={(e) => onClusterPointerDown(e, "menu")}
  >
    <button
      class="menu-btn"
      onpointerdown={(e) => onButtonPointerDown(e, "Back")}
      onpointerup={onButtonPointerEnd}
      onpointercancel={onButtonPointerEnd}
      onlostpointercapture={onButtonPointerEnd}
    >BACK</button>

    <!-- Node Drag Lock Toggle -->
    <button
      class="drag-lock-btn"
      class:active={isDragLockActive}
      onclick={toggleDragLock}
      title="Giữ chuột trái để kéo thả (Drag Lock)"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M5 9l4-4 4 4" /><path d="M9 5v14" /><path d="M19 15l-4 4-4-4" /><path d="M15 19V5" />
      </svg>
      <span>{isDragLockActive ? "DRAGGING" : "DRAG OFF"}</span>
    </button>

    <button
      class="menu-btn"
      onpointerdown={(e) => onButtonPointerDown(e, "Start")}
      onpointerup={onButtonPointerEnd}
      onpointercancel={onButtonPointerEnd}
      onlostpointercapture={onButtonPointerEnd}
    >START</button>
    {#if isEditingLayout}
      <button
        class="control-resize-handle"
        aria-label={`Đổi kích thước cụm menu, ${Math.round(getControlScale("menu") * 100)}%`}
        title={`Cụm menu, ${Math.round(getControlScale("menu") * 100)}%`}
        onpointerdown={(e) => onControlResizeStart(e, "menu")}
        onpointermove={onControlResizeMove}
        onpointerup={onControlResizeEnd}
        onpointercancel={onControlResizeEnd}
        onlostpointercapture={onControlResizeEnd}
      >
        <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M5 11 11 5M6 5h5v5" /></svg>
      </button>
    {/if}
  </div>

  <!-- Controls Main Row (D-Pad & Face Buttons) -->
  <div class="controls-main-row" class:reverse-controls={settings.data.lefty}>
    <!-- D-PAD -->
    <div
      class="dpad-container cluster-draggable"
      class:dragging={draggingCluster === "dpad"}
      style={getControlClusterStyle("dpad")}
      role="region"
      aria-label="Directional pad"
      onpointerdown={(e) => onClusterPointerDown(e, "dpad")}
    >
      <button
        class="dpad-btn dpad-up"
        onpointerdown={(e) => onButtonPointerDown(e, "DPadUp")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >▲</button>
      <button
        class="dpad-btn dpad-left"
        onpointerdown={(e) => onButtonPointerDown(e, "DPadLeft")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >◀</button>
      <div class="dpad-center"></div>
      <button
        class="dpad-btn dpad-right"
        onpointerdown={(e) => onButtonPointerDown(e, "DPadRight")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >▶</button>
      <button
        class="dpad-btn dpad-down"
        onpointerdown={(e) => onButtonPointerDown(e, "DPadDown")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >▼</button>
      {#if isEditingLayout}
        <button
          class="control-resize-handle"
          aria-label={`Đổi kích thước D-pad, ${Math.round(getControlScale("dpad") * 100)}%`}
          title={`D-pad, ${Math.round(getControlScale("dpad") * 100)}%`}
          onpointerdown={(e) => onControlResizeStart(e, "dpad")}
          onpointermove={onControlResizeMove}
          onpointerup={onControlResizeEnd}
          onpointercancel={onControlResizeEnd}
          onlostpointercapture={onControlResizeEnd}
        >
          <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M5 11 11 5M6 5h5v5" /></svg>
        </button>
      {/if}
    </div>

    <!-- FACE BUTTONS (ABXY) -->
    <div
      class="face-container cluster-draggable"
      class:dragging={draggingCluster === "face"}
      style={getControlClusterStyle("face")}
      role="region"
      aria-label="Face action buttons"
      onpointerdown={(e) => onClusterPointerDown(e, "face")}
    >
      <button
        class="face-btn face-y"
        onpointerdown={(e) => onButtonPointerDown(e, "Y")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >Y</button>
      <button
        class="face-btn face-x"
        onpointerdown={(e) => onButtonPointerDown(e, "X")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >X</button>
      <button
        class="face-btn face-b"
        onpointerdown={(e) => onButtonPointerDown(e, "B")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >B</button>
      <button
        class="face-btn face-a"
        onpointerdown={(e) => onButtonPointerDown(e, "A")}
        onpointerup={onButtonPointerEnd}
        onpointercancel={onButtonPointerEnd}
        onlostpointercapture={onButtonPointerEnd}
      >A</button>
      {#if isEditingLayout}
        <button
          class="control-resize-handle"
          aria-label={`Đổi kích thước ABXY, ${Math.round(getControlScale("face") * 100)}%`}
          title={`ABXY, ${Math.round(getControlScale("face") * 100)}%`}
          onpointerdown={(e) => onControlResizeStart(e, "face")}
          onpointermove={onControlResizeMove}
          onpointerup={onControlResizeEnd}
          onpointercancel={onControlResizeEnd}
          onlostpointercapture={onControlResizeEnd}
        >
          <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M5 11 11 5M6 5h5v5" /></svg>
        </button>
      {/if}
    </div>
  </div>

  <!-- Dual Virtual Joysticks Row -->
  <div class="sticks-row">
    <!-- Left Stick -->
    <div
      class="stick-base cluster-draggable"
      class:dragging={draggingCluster === "leftStick"}
      style={getStickStyle("left")}
      role="group"
      aria-label="Left analog stick"
      onpointerdown={(e) => {
        if (isEditingLayout) onClusterPointerDown(e, "leftStick");
        else onLeftStickStart(e);
      }}
      onpointermove={onLeftStickMove}
      onpointerup={onLeftStickEnd}
      onpointercancel={onLeftStickEnd}
      onlostpointercapture={onLeftStickEnd}
    >
      <div
        class="stick-puck"
        style="transform: translate3d({leftPuckPos.x}px, {leftPuckPos.y}px, 0);"
      >
        LS
      </div>
      {#if isEditingLayout}
        <button
          class="stick-resize-handle"
          aria-label={`Đổi kích thước joystick trái, ${getRenderedStickSize("left")} px`}
          title={`Joystick trái, ${getRenderedStickSize("left")} px`}
          onpointerdown={(e) => onStickResizeStart(e, "left")}
          onpointermove={onStickResizeMove}
          onpointerup={onStickResizeEnd}
          onpointercancel={onStickResizeEnd}
          onlostpointercapture={onStickResizeEnd}
        >
          <svg viewBox="0 0 16 16" aria-hidden="true">
            <path d="M5 11 11 5M6 5h5v5" />
          </svg>
        </button>
      {/if}
    </div>

    <!-- Right Stick -->
    <div
      class="stick-base cluster-draggable"
      class:dragging={draggingCluster === "rightStick"}
      style={getStickStyle("right")}
      role="group"
      aria-label="Right analog stick"
      onpointerdown={(e) => {
        if (isEditingLayout) onClusterPointerDown(e, "rightStick");
        else onRightStickStart(e);
      }}
      onpointermove={onRightStickMove}
      onpointerup={onRightStickEnd}
      onpointercancel={onRightStickEnd}
      onlostpointercapture={onRightStickEnd}
    >
      <div
        class="stick-puck"
        style="transform: translate3d({rightPuckPos.x}px, {rightPuckPos.y}px, 0);"
      >
        RS
      </div>
      {#if isEditingLayout}
        <button
          class="stick-resize-handle"
          aria-label={`Đổi kích thước joystick phải, ${getRenderedStickSize("right")} px`}
          title={`Joystick phải, ${getRenderedStickSize("right")} px`}
          onpointerdown={(e) => onStickResizeStart(e, "right")}
          onpointermove={onStickResizeMove}
          onpointerup={onStickResizeEnd}
          onpointercancel={onStickResizeEnd}
          onlostpointercapture={onStickResizeEnd}
        >
          <svg viewBox="0 0 16 16" aria-hidden="true">
            <path d="M5 11 11 5M6 5h5v5" />
          </svg>
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .gamepad-stage {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 8px;
    zoom: var(--control-zoom, 1);
    user-select: none;
    -webkit-user-select: none;
    position: relative;
    touch-action: none !important;
  }

  /* Drag & Drop Layout customization */
  .editing-mode .cluster-draggable {
    border: 2px dashed var(--accent) !important;
    border-radius: 2px;
    background: var(--accent-muted);
    cursor: move;
  }
  .editing-mode .cluster-draggable.dragging {
    opacity: 0.8;
    box-shadow: 0 3px 12px rgba(23, 35, 42, 0.18);
  }

  /* Bumpers & Triggers Row */
  .bumpers-row {
    display: flex;
    justify-content: space-between;
    width: 100%;
    padding: 0 4px;
    flex-shrink: 0;
    position: relative;
  }
  .shoulder-group {
    display: flex;
    gap: 8px;
    zoom: var(--cluster-zoom, 1);
  }
  .shoulder-btn {
    width: 68px;
    height: 42px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    color: var(--text);
    font-size: 13px;
    font-weight: 800;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    position: relative;
    overflow: hidden;
  }
  .shoulder-btn:active {
    background: var(--surface-hover);
    color: var(--accent-strong);
  }
  .trigger-btn {
    width: 78px;
    background: var(--surface-elevated);
    border-color: var(--border);
  }
  .trigger-gauge {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: var(--accent);
    opacity: 0.45;
    pointer-events: none;
    transition: height 0.04s linear;
  }
  .trigger-text {
    position: relative;
    z-index: 2;
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .trigger-pct {
    font-size: 10px;
    color: var(--accent-strong);
    font-variant-numeric: tabular-nums;
  }

  /* Center Menu Row */
  .center-menu-row {
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    gap: 12px;
    padding: 4px 0;
    flex-shrink: 0;
  }
  .center-menu-row > :not(.control-resize-handle) {
    zoom: var(--cluster-zoom, 1);
  }
  .menu-btn {
    padding: 6px 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 16px;
    color: var(--text-muted);
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.05em;
    cursor: pointer;
  }
  .menu-btn:active {
    background: var(--accent);
    color: #fff;
  }
  .drag-lock-btn {
    padding: 6px 12px;
    border-radius: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 700;
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .drag-lock-btn.active {
    background: var(--accent-muted);
    border-color: var(--accent-strong);
    color: var(--accent-strong);
    box-shadow: 0 2px 8px rgba(23, 35, 42, 0.12);
  }
  .drag-lock-btn svg {
    width: 14px;
    height: 14px;
  }

  /* Controls Main Row */
  .controls-main-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex: 1;
    width: 100%;
    padding: 0 6px;
  }
  .controls-main-row.reverse-controls {
    flex-direction: row-reverse;
  }

  /* D-PAD */
  .dpad-container {
    width: 140px;
    height: 140px;
    position: relative;
    flex-shrink: 0;
    zoom: var(--cluster-zoom, 1);
  }
  .dpad-btn {
    position: absolute;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 15px;
    font-weight: 700;
    cursor: pointer;
  }
  .dpad-btn:active {
    background: var(--accent);
    color: #fff;
  }
  .dpad-up { top: 0; left: 46px; width: 48px; height: 48px; border-radius: 7px 7px 0 0; }
  .dpad-down { bottom: 0; left: 46px; width: 48px; height: 48px; border-radius: 0 0 7px 7px; }
  .dpad-left { top: 46px; left: 0; width: 48px; height: 48px; border-radius: 7px 0 0 7px; }
  .dpad-right { top: 46px; right: 0; width: 48px; height: 48px; border-radius: 0 7px 7px 0; }
  .dpad-center { top: 46px; left: 46px; width: 48px; height: 48px; background: var(--surface-elevated); position: absolute; }

  /* FACE BUTTONS */
  .face-container {
    width: 140px;
    height: 140px;
    position: relative;
    flex-shrink: 0;
    zoom: var(--cluster-zoom, 1);
  }
  .face-btn {
    position: absolute;
    width: 46px;
    height: 46px;
    border-radius: 50%;
    background: var(--surface);
    border: 2px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    font-weight: 800;
    cursor: pointer;
  }
  .face-btn:active {
    transform: scale(0.92);
    filter: brightness(0.86);
  }
  .face-y { top: 0; left: 47px; }
  .face-a { bottom: 0; left: 47px; }
  .face-x { top: 47px; left: 0; }
  .face-b { top: 47px; right: 0; }

  .face-a {
    background: var(--xbox-a);
    border-color: var(--xbox-a);
    color: var(--xbox-label-light);
  }
  .face-b {
    background: var(--xbox-b);
    border-color: var(--xbox-b);
    color: var(--xbox-label-light);
  }
  .face-x {
    background: var(--xbox-x);
    border-color: var(--xbox-x);
    color: var(--xbox-label-light);
  }
  .face-y {
    background: var(--xbox-y);
    border-color: var(--xbox-y);
    color: var(--xbox-label-dark);
  }

  /* VIRTUAL JOYSTICKS */
  .sticks-row {
    display: flex;
    justify-content: space-around;
    align-items: center;
    width: 100%;
    padding-bottom: 6px;
    flex-shrink: 0;
  }
  .stick-base {
    width: var(--stick-size, 112px);
    height: var(--stick-size, 112px);
    border-radius: 50%;
    background: radial-gradient(circle, var(--surface) 40%, var(--surface-elevated) 100%);
    border: 2px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    touch-action: none;
    box-shadow: inset 0 2px 6px rgba(23, 35, 42, 0.12);
  }
  .stick-puck {
    width: var(--stick-puck-size, 52px);
    height: var(--stick-puck-size, 52px);
    border-radius: 50%;
    background: radial-gradient(circle, var(--surface) 0%, var(--surface-elevated) 100%);
    border: 2px solid var(--accent);
    color: var(--accent-strong);
    font-size: 11px;
    font-weight: 800;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 3px 8px rgba(23, 35, 42, 0.2);
    pointer-events: none;
    transition: transform 0.05s ease-out;
    will-change: transform;
  }
  .stick-resize-handle,
  .control-resize-handle {
    position: absolute;
    right: -9px;
    bottom: -9px;
    width: 34px;
    height: 34px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--accent);
    border-radius: 50%;
    background: var(--surface);
    color: var(--accent-strong);
    box-shadow: 0 2px 6px rgba(23, 35, 42, 0.2);
    touch-action: none;
    cursor: nwse-resize;
    z-index: 3;
  }
  .control-resize-handle {
    width: 38px;
    height: 38px;
  }
  .bumpers-row .control-resize-handle,
  .center-menu-row .control-resize-handle {
    right: calc(50% - 19px);
    bottom: -12px;
  }
  .stick-resize-handle svg,
  .control-resize-handle svg {
    width: 16px;
    height: 16px;
    fill: none;
    stroke: currentColor;
    stroke-width: 1.6;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .stick-resize-handle:focus-visible,
  .control-resize-handle:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  @media (max-height: 500px) and (orientation: landscape) {
    .gamepad-stage.editing-mode {
      zoom: 0.82;
    }

    .gamepad-stage {
      padding: 4px;
      gap: 2px;
    }

    .shoulder-group {
      gap: 5px;
    }

    .shoulder-btn {
      width: 56px;
      height: 34px;
    }

    .trigger-btn {
      width: 64px;
    }

    .center-menu-row {
      gap: 7px;
      padding: 1px 0;
    }

    .menu-btn,
    .drag-lock-btn {
      padding: 4px 9px;
    }

    .dpad-container,
    .face-container {
      width: 112px;
      height: 112px;
    }

    .dpad-btn {
      font-size: 13px;
    }

    .dpad-up { left: 35px; width: 42px; height: 42px; }
    .dpad-down { left: 35px; width: 42px; height: 42px; }
    .dpad-left { top: 35px; width: 42px; height: 42px; }
    .dpad-right { top: 35px; width: 42px; height: 42px; }
    .dpad-center { top: 35px; left: 35px; width: 42px; height: 42px; }

    .face-btn {
      width: 42px;
      height: 42px;
      font-size: 14px;
    }

    .face-y { left: 35px; }
    .face-a { left: 35px; }
    .face-x { top: 35px; }
    .face-b { top: 35px; }

    .sticks-row {
      padding-bottom: 2px;
    }

    .stick-base {
      width: var(--stick-size, 88px);
      height: var(--stick-size, 88px);
    }

    .stick-puck {
      width: var(--stick-puck-size, 42px);
      height: var(--stick-puck-size, 42px);
    }
  }
</style>
