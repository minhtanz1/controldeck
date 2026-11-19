<script lang="ts">
  import type { Action, Profile } from "$lib/types";

  let {
    selectedButton = "A",
    profile = null,
    onSaveMapping,
    onTestAction,
  }: {
    selectedButton: string;
    profile: Profile | null;
    onSaveMapping: (button: string, action: Action | null) => void;
    onTestAction?: (action: Action) => void;
  } = $props();

  let actionType = $state<"key" | "combo_key" | "toggle" | "mouse" | "macro">("key");
  let currentKey = $state("A");
  let keyMode = $state<"press" | "hold">("press");
  let mouseButton = $state<"left" | "right" | "middle">("left");
  let mouseAction = $state<"click" | "hold" | "toggle">("click");
  let stickSensitivity = $state(1.0);
  let modifierCtrl = $state(false);
  let modifierAlt = $state(false);
  let modifierShift = $state(false);
  let modifierWin = $state(false);
  let isRecording = $state(false);
  let triggerThreshold = $state(50); // 10% - 90%

  let isTrigger = $derived(selectedButton === "LeftTrigger" || selectedButton === "RightTrigger");
  let isStick = $derived(selectedButton === "LeftThumb" || selectedButton === "RightThumb");

  // Sync state whenever selected button or profile changes
  $effect(() => {
    if (!profile?.mappings) return;
    const act = profile.mappings.buttons?.[selectedButton];
    if (act) {
      actionType = act.type;
      if (act.type === "key") {
        currentKey = act.key;
        keyMode = act.on === "hold" ? "hold" : "press";
        modifierCtrl = act.modifiers?.includes("Ctrl") ?? false;
        modifierAlt = act.modifiers?.includes("Alt") ?? false;
        modifierShift = act.modifiers?.includes("Shift") ?? false;
        modifierWin = act.modifiers?.includes("Win") ?? false;
      } else if (act.type === "combo_key") {
        currentKey = act.key;
        modifierCtrl = act.modifiers.includes("Ctrl");
        modifierAlt = act.modifiers.includes("Alt");
        modifierShift = act.modifiers.includes("Shift");
        modifierWin = act.modifiers.includes("Win");
      } else if (act.type === "toggle") {
        currentKey = act.key;
      } else if (act.type === "mouse") {
        mouseButton = act.button as "left" | "right" | "middle";
        mouseAction = (act.on === "hold" ? "hold" : act.on === "toggle" ? "toggle" : "click");
      }
    } else if (isTrigger && profile.mappings.triggers?.[selectedButton]) {
      const trg = profile.mappings.triggers[selectedButton];
      triggerThreshold = Math.round(trg.threshold * 100);
      actionType = trg.action.type;
      if (trg.action.type === "key") {
        currentKey = trg.action.key;
        keyMode = trg.action.on === "hold" ? "hold" : "press";
      } else if (trg.action.type === "mouse") {
        mouseButton = trg.action.button as "left" | "right" | "middle";
        mouseAction = (trg.action.on === "hold" ? "hold" : trg.action.on === "toggle" ? "toggle" : "click");
      }
    }

    if (isStick && profile) {
      const stickKey = selectedButton === "RightThumb" ? "RightStick" : "LeftStick";
      if (profile.mappings.sticks?.[stickKey]?.sensitivity) {
        stickSensitivity = profile.mappings.sticks[stickKey].sensitivity;
      }
    } else {
      currentKey = selectedButton.length === 1 ? selectedButton : "Space";
      modifierCtrl = false;
      modifierAlt = false;
      modifierShift = false;
      modifierWin = false;
      actionType = "key";
      keyMode = "press";
    }
  });

  function startRecording() {
    isRecording = true;
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (!isRecording) return;
    e.preventDefault();
    e.stopPropagation();

    modifierCtrl = e.ctrlKey;
    modifierAlt = e.altKey;
    modifierShift = e.shiftKey;
    modifierWin = e.metaKey;

    let k = e.key;
    if (k === "Control" || k === "Alt" || k === "Shift" || k === "Meta") {
      return; // wait for letter/symbol
    }
    if (k === " ") k = "Space";
    if (k.length === 1) k = k.toUpperCase();

    currentKey = k;
    isRecording = false;
    commitSave();
  }

  function toggleModifier(mod: "Ctrl" | "Alt" | "Shift" | "Win") {
    if (mod === "Ctrl") modifierCtrl = !modifierCtrl;
    if (mod === "Alt") modifierAlt = !modifierAlt;
    if (mod === "Shift") modifierShift = !modifierShift;
    if (mod === "Win") modifierWin = !modifierWin;
    commitSave();
  }

  function buildCurrentAction(): Action {
    const mods: string[] = [];
    if (modifierCtrl) mods.push("Ctrl");
    if (modifierAlt) mods.push("Alt");
    if (modifierShift) mods.push("Shift");
    if (modifierWin) mods.push("Win");

    if (actionType === "mouse") {
      return {
        type: "mouse",
        button: mouseButton,
        on: mouseAction,
      };
    } else if (actionType === "combo_key" || mods.length > 0) {
      return {
        type: "combo_key",
        key: currentKey,
        modifiers: mods,
      };
    } else if (actionType === "toggle") {
      return {
        type: "toggle",
        key: currentKey,
        modifiers: mods,
        state: false,
      };
    } else {
      return {
        type: "key",
        key: currentKey,
        modifiers: mods,
        on: keyMode,
      };
    }
  }

  function commitSave() {
    const act = buildCurrentAction();
    onSaveMapping(selectedButton, act);
  }

  function clearMapping() {
    onSaveMapping(selectedButton, null);
  }

  function testCurrentKey() {
    if (onTestAction) {
      onTestAction(buildCurrentAction());
    }
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="inspector">
  <div class="inspector-header">
    <div class="target-badge">
      <span class="target-type">{isTrigger ? "Analog Trigger" : isStick ? "Thumbstick" : "Button"}</span>
      <h2 class="target-name">{selectedButton}</h2>
    </div>
    <div class="header-actions">
      <button type="button" class="test-inject-btn" onclick={testCurrentKey} title="Inject this key into Windows right now">
        Test Key
      </button>
      <button type="button" class="clear-btn" onclick={clearMapping} title="Remove binding">
        Clear
      </button>
    </div>
  </div>

  <div class="inspector-body">
    <!-- Trigger threshold slider if Trigger selected -->
    {#if isTrigger}
      <div class="control-row">
        <div class="label-with-val">
          <span class="field-label">Activation Threshold</span>
          <span class="threshold-val">{triggerThreshold}%</span>
        </div>
        <input
          type="range"
          min="10"
          max="90"
          step="5"
          class="range-slider"
          bind:value={triggerThreshold}
          onchange={commitSave}
        />
      </div>
    {/if}

    <!-- Action Type -->
    <div class="control-row">
      <span class="field-label">Action Mode</span>
      <div class="type-segmented">
        <button
          type="button"
          class="seg-btn {actionType === 'key' ? 'active' : ''}"
          onclick={() => { actionType = 'key'; commitSave(); }}
        >
          Single Key
        </button>
        <button
          type="button"
          class="seg-btn {actionType === 'combo_key' ? 'active' : ''}"
          onclick={() => { actionType = 'combo_key'; commitSave(); }}
        >
          Combo
        </button>
        <button
          type="button"
          class="seg-btn {actionType === 'toggle' ? 'active' : ''}"
          onclick={() => { actionType = 'toggle'; commitSave(); }}
        >
          Toggle
        </button>
        <button
          type="button"
          class="seg-btn {actionType === 'mouse' ? 'active' : ''}"
          onclick={() => { actionType = 'mouse'; commitSave(); }}
        >
          Mouse Action
        </button>
      </div>
    </div>

    {#if actionType === 'mouse'}
      <!-- Mouse Button Choice -->
      <div class="control-row">
        <span class="field-label">Mouse Button</span>
        <div class="type-segmented">
          <button
            type="button"
            class="seg-btn {mouseButton === 'left' ? 'active' : ''}"
            onclick={() => { mouseButton = 'left'; commitSave(); }}
          >
            Left Click (Select / Drag)
          </button>
          <button
            type="button"
            class="seg-btn {mouseButton === 'right' ? 'active' : ''}"
            onclick={() => { mouseButton = 'right'; commitSave(); }}
          >
            Right Click (Pan / Menu)
          </button>
          <button
            type="button"
            class="seg-btn {mouseButton === 'middle' ? 'active' : ''}"
            onclick={() => { mouseButton = 'middle'; commitSave(); }}
          >
            Middle
          </button>
        </div>
      </div>

      <!-- Mouse Action Mode -->
      <div class="control-row">
        <span class="field-label">Action Behavior</span>
        <div class="type-segmented">
          <button
            type="button"
            class="seg-btn {mouseAction === 'click' ? 'active' : ''}"
            onclick={() => { mouseAction = 'click'; commitSave(); }}
          >
            Click
          </button>
          <button
            type="button"
            class="seg-btn {mouseAction === 'hold' ? 'active' : ''}"
            onclick={() => { mouseAction = 'hold'; commitSave(); }}
          >
            Hold (Drag node/pan)
          </button>
          <button
            type="button"
            class="seg-btn {mouseAction === 'toggle' ? 'active' : ''}"
            onclick={() => { mouseAction = 'toggle'; commitSave(); }}
          >
            Toggle Drag
          </button>
        </div>
        <p style="font-size: 11px; color: var(--text-muted); margin-top: 6px;">
          Tip: Gán "Hold Left" cho LeftStick (L3) hoặc Left Trigger để đè chuột kéo thả node hình học trong WME.
        </p>
      </div>

      <!-- Mouse Sensitivity Slider -->
      <div class="control-row">
        <div class="label-with-val">
          <span class="field-label">Mouse Speed Sensitivity</span>
          <span class="threshold-val">{stickSensitivity.toFixed(1)}x</span>
        </div>
        <input
          type="range"
          min="0.5"
          max="3.5"
          step="0.1"
          class="range-slider"
          bind:value={stickSensitivity}
          onchange={() => {
            if (profile) {
              const stickKey = selectedButton === "RightThumb" ? "RightStick" : "LeftStick";
              if (!profile.mappings.sticks) profile.mappings.sticks = {};
              if (!profile.mappings.sticks[stickKey]) {
                profile.mappings.sticks[stickKey] = {
                  deadzone: 0.15,
                  mode: "mouse",
                  sensitivity: stickSensitivity,
                };
              } else {
                profile.mappings.sticks[stickKey].sensitivity = stickSensitivity;
              }
            }
            commitSave();
          }}
        />
      </div>
    {:else}
      <!-- Key Capture Box -->
    <div class="control-row">
      <span class="field-label">Assigned Key</span>
      <div class="key-recorder-box {isRecording ? 'recording' : ''}">
        <div class="key-chips">
          {#if modifierCtrl}<span class="mod-chip">Ctrl</span>{/if}
          {#if modifierAlt}<span class="mod-chip">Alt</span>{/if}
          {#if modifierShift}<span class="mod-chip">Shift</span>{/if}
          {#if modifierWin}<span class="mod-chip">Win</span>{/if}
          <span class="main-key-chip">{currentKey}</span>
        </div>
        <button
          type="button"
          class="record-btn {isRecording ? 'pulse' : ''}"
          onclick={startRecording}
        >
          {isRecording ? "Press Key..." : "Rebind"}
        </button>
      </div>
    </div>

    <!-- Modifier Buttons Toggle -->
    <div class="control-row">
      <span class="field-label">Modifier Keys</span>
      <div class="modifiers-bar">
        <button
          type="button"
          class="mod-toggle {modifierCtrl ? 'active' : ''}"
          onclick={() => toggleModifier('Ctrl')}
        >
          Ctrl
        </button>
        <button
          type="button"
          class="mod-toggle {modifierAlt ? 'active' : ''}"
          onclick={() => toggleModifier('Alt')}
        >
          Alt
        </button>
        <button
          type="button"
          class="mod-toggle {modifierShift ? 'active' : ''}"
          onclick={() => toggleModifier('Shift')}
        >
          Shift
        </button>
        <button
          type="button"
          class="mod-toggle {modifierWin ? 'active' : ''}"
          onclick={() => toggleModifier('Win')}
        >
          Win
        </button>
      </div>
    </div>

    <!-- Hold vs Tap Mode (for single keys) -->
    {#if actionType === "key"}
      <div class="control-row">
        <span class="field-label">Key Press Behavior</span>
        <div class="behavior-options">
          <label class="behavior-label {keyMode === 'press' ? 'selected' : ''}">
            <input type="radio" name="keymode" value="press" bind:group={keyMode} onchange={commitSave} />
            <span>Tap (Single Keypress)</span>
          </label>
          <label class="behavior-label {keyMode === 'hold' ? 'selected' : ''}">
            <input type="radio" name="keymode" value="hold" bind:group={keyMode} onchange={commitSave} />
            <span>Hold (Keeps key down while button held)</span>
          </label>
        </div>
      </div>
    {/if}

    <!-- Quick Shortcuts Preset Palette -->
    <div class="control-row">
      <span class="field-label">Quick Presets</span>
      <div class="quick-presets">
        <button
          type="button"
          class="preset-pill"
          onclick={() => { modifierCtrl = true; modifierAlt = true; modifierShift = false; currentKey = "L"; commitSave(); }}
        >
          Ctrl+Alt+L (Mở GGMaps)
        </button>
        <button
          type="button"
          class="preset-pill"
          onclick={() => { modifierCtrl = true; modifierAlt = true; modifierShift = false; currentKey = "F"; commitSave(); }}
        >
          Ctrl+Alt+F (Mở OSM)
        </button>
        <button
          type="button"
          class="preset-pill"
          onclick={() => { modifierCtrl = true; modifierAlt = false; modifierShift = false; currentKey = "Z"; commitSave(); }}
        >
          Ctrl+Z (Undo)
        </button>
        <button
          type="button"
          class="preset-pill"
          onclick={() => { modifierCtrl = true; modifierAlt = false; modifierShift = false; currentKey = "S"; commitSave(); }}
        >
          Ctrl+S (Save Changes)
        </button>
        <button
          type="button"
          class="preset-pill"
          onclick={() => { modifierCtrl = false; modifierAlt = false; modifierShift = false; currentKey = "Delete"; commitSave(); }}
        >
          Delete (Xoá Vertex)
        </button>
      </div>
    </div>
    {/if}
  </div>
</div>

<style>
  .inspector {
    width: 320px;
    flex: 0 0 320px;
    min-width: 0;
    height: 100%;
    background-color: var(--bg-surface);
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    user-select: none;
  }

  .inspector-header {
    padding: 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    border-bottom: 1px solid var(--border);
  }

  .target-badge {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .target-type {
    font-size: 9px;
    text-transform: uppercase;
    font-weight: 700;
    letter-spacing: 0.05em;
    color: var(--accent-strong);
  }

  .target-name {
    margin: 0;
    font-size: 16px;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .header-actions {
    display: flex;
    gap: 6px;
    flex-shrink: 0;
  }

  .test-inject-btn {
    background: var(--accent-muted);
    border: 1px solid var(--accent);
    color: var(--accent-strong);
    border-radius: 2px;
    padding: 5px 8px;
    font-size: 10px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .test-inject-btn:hover {
    background: var(--accent-muted);
  }

  .clear-btn {
    background: transparent;
    border: 1px solid var(--danger);
    color: var(--danger);
    border-radius: 2px;
    padding: 5px 8px;
    font-size: 10px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .clear-btn:hover {
    background: var(--danger-muted);
    border-color: var(--danger);
  }

  .inspector-body {
    min-height: 0;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    overflow-y: auto;
  }

  .control-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .label-with-val {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .threshold-val {
    font-size: 11px;
    color: var(--accent-strong);
    font-weight: 600;
  }

  .range-slider {
    width: 100%;
    accent-color: var(--accent-strong);
    cursor: pointer;
  }

  .field-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .type-segmented {
    display: flex;
    flex-wrap: wrap;
    background: var(--bg-muted);
    padding: 2px;
    border-radius: 2px;
    border: 1px solid var(--border);
  }

  .seg-btn {
    flex: 1 1 calc(50% - 3px);
    min-width: 0;
    background: transparent;
    border: none;
    border-radius: 1px;
    color: var(--text-secondary);
    font-size: 10px;
    font-weight: 500;
    padding: 6px 3px;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .seg-btn.active {
    background: var(--bg-surface);
    color: var(--accent-strong);
    font-weight: 600;
  }

  .key-recorder-box {
    background: var(--bg-surface);
    border: 1px solid var(--border);
    min-width: 0;
    border-radius: 2px;
    padding: 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    transition: all 0.15s ease;
  }

  .key-recorder-box.recording {
    border-color: var(--accent-strong);
    background: var(--accent-muted);
    box-shadow: 0 2px 8px rgba(23, 35, 42, 0.12);
  }

  .key-chips {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }

  .mod-chip {
    background: var(--bg-muted);
    color: var(--text-secondary);
    font-size: 11px;
    font-weight: 600;
    padding: 3px 6px;
    border-radius: 2px;
    border: 1px solid var(--border);
  }

  .main-key-chip {
    background: var(--accent);
    color: #ffffff;
    font-size: 13px;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 2px;
  }

  .record-btn {
    background: var(--bg-muted);
    border: 1px solid var(--border);
    color: var(--text-primary);
    border-radius: 2px;
    padding: 6px 8px;
    font-size: 10px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .record-btn:hover {
    border-color: var(--accent-strong);
    color: var(--accent-strong);
  }

  .record-btn.pulse {
    background: var(--accent);
    color: #fff;
    border-color: var(--accent-strong);
  }

  .modifiers-bar {
    display: flex;
    gap: 5px;
  }

  .mod-toggle {
    flex: 1;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 2px;
    color: var(--text-secondary);
    font-size: 10px;
    font-weight: 600;
    padding: 6px 0;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .mod-toggle:hover {
    border-color: var(--border-strong);
  }

  .mod-toggle.active {
    background: var(--accent-muted);
    border-color: var(--accent-strong);
    color: var(--accent-strong);
  }

  .behavior-options {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .behavior-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-secondary);
    padding: 6px 8px;
    border-radius: 2px;
    background: var(--bg-surface);
    cursor: pointer;
  }

  .behavior-label.selected {
    color: var(--text-primary);
    background: var(--bg-inset);
  }

  .quick-presets {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .preset-pill {
    background: var(--bg-surface);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    border-radius: 2px;
    padding: 6px 8px;
    font-size: 10px;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .preset-pill:hover {
    background: var(--bg-inset);
    border-color: var(--accent-strong);
    color: var(--accent-strong);
  }

  .inspector button:focus-visible,
  .inspector input:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  @media (max-width: 980px) {
    .inspector {
      width: 100%;
      flex: 0 0 auto;
      height: auto;
      max-height: 72dvh;
      border-left: 0;
      border-top: 1px solid var(--border);
    }

    .inspector-body {
      max-height: calc(72dvh - 52px);
    }
  }

  @media (max-width: 480px) {
    .inspector-header {
      padding: 8px;
    }

    .inspector-body {
      padding: 8px;
      gap: 10px;
    }

    .header-actions {
      gap: 4px;
    }
  }
</style>
