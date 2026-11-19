<script lang="ts">
  import type { ControllerState, Profile } from "$lib/types";

  let {
    profile = null,
    controllerState = null,
    selectedButton = "A",
    onSelectButton,
    onTestButton,
  }: {
    profile: Profile | null;
    controllerState: ControllerState | null;
    selectedButton: string;
    onSelectButton: (btn: string) => void;
    onTestButton?: (btn: string) => void;
  } = $props();

  function isPressed(btn: string): boolean {
    return !!controllerState?.buttons?.[btn];
  }

  function getBindingLabel(btn: string): string {
    if (!profile?.mappings) return "";
    const b = profile.mappings.buttons?.[btn];
    if (b) {
      if (b.type === "key") {
        const mods = b.modifiers?.length ? `${b.modifiers.join("+")}+` : "";
        return `${mods}${b.key}`;
      }
      if (b.type === "combo_key") {
        return `${b.modifiers.join("+")}+${b.key}`;
      }
      if (b.type === "toggle") {
        return `[T] ${b.key}`;
      }
      if (b.type === "macro") {
        return `Macro (${b.sequence.length})`;
      }
      if (b.type === "mouse") {
        return `${b.button.toUpperCase()} mouse · ${b.on || "click"}`;
      }
    }
    if (btn === "LeftTrigger" && profile.mappings.triggers?.LeftTrigger) {
      const act = profile.mappings.triggers.LeftTrigger.action;
      return act.type === "key" ? `LT: ${act.key}` : act.type === "mouse" ? `LT: ${act.button} mouse` : "LT Active";
    }
    if (btn === "RightTrigger" && profile.mappings.triggers?.RightTrigger) {
      const act = profile.mappings.triggers.RightTrigger.action;
      return act.type === "key" ? `RT: ${act.key}` : act.type === "mouse" ? `RT: ${act.button} mouse` : "RT Active";
    }
    if (btn === "LeftThumb" || btn === "RightThumb") {
      const stickKey = btn === "LeftThumb" ? "LeftStick" : "RightStick";
      const stick = profile.mappings.sticks?.[stickKey];
      return stick ? `${stick.mode.toUpperCase()} · ${(stick.sensitivity ?? 1).toFixed(1)}x` : "";
    }
    return "";
  }

  const controlGroups = [
    { title: "Face", buttons: ["Y", "X", "B", "A"] },
    { title: "D-pad", buttons: ["DPadUp", "DPadLeft", "DPadRight", "DPadDown"] },
    { title: "Shoulders", buttons: ["LB", "RB"] },
    { title: "System", buttons: ["Back", "Start"] },
    { title: "Sticks", buttons: ["LeftThumb", "RightThumb"] },
    { title: "Triggers", buttons: ["LeftTrigger", "RightTrigger"] },
  ];

  // Thumbstick deflections (pixel offset from center)
  let leftStickOffsetX = $derived((controllerState?.left_stick_x ?? 0) * 14);
  let leftStickOffsetY = $derived(-(controllerState?.left_stick_y ?? 0) * 14);
  let rightStickOffsetX = $derived((controllerState?.right_stick_x ?? 0) * 14);
  let rightStickOffsetY = $derived(-(controllerState?.right_stick_y ?? 0) * 14);

  let ltPercent = $derived(Math.round((controllerState?.left_trigger ?? 0) * 100));
  let rtPercent = $derived(Math.round((controllerState?.right_trigger ?? 0) * 100));
</script>

<div class="controller-stage">
  <!-- Trigger Telemetry Gauges -->
  <div class="trigger-gauges">
    <button
      type="button"
      class="trigger-gauge {selectedButton === 'LeftTrigger' ? 'selected' : ''}"
      onclick={() => onSelectButton("LeftTrigger")}
    >
      <div class="gauge-header">
        <span class="gauge-name">LT / Left Trigger</span>
        <span class="gauge-val">{ltPercent}%</span>
      </div>
      <div class="gauge-track">
        <div class="gauge-fill" style="width: {ltPercent}%"></div>
      </div>
      {#if getBindingLabel("LeftTrigger")}
        <span class="gauge-binding">{getBindingLabel("LeftTrigger")}</span>
      {/if}
    </button>

    <button
      type="button"
      class="trigger-gauge {selectedButton === 'RightTrigger' ? 'selected' : ''}"
      onclick={() => onSelectButton("RightTrigger")}
    >
      <div class="gauge-header">
        <span class="gauge-name">RT / Right Trigger</span>
        <span class="gauge-val">{rtPercent}%</span>
      </div>
      <div class="gauge-track">
        <div class="gauge-fill" style="width: {rtPercent}%"></div>
      </div>
      {#if getBindingLabel("RightTrigger")}
        <span class="gauge-binding">{getBindingLabel("RightTrigger")}</span>
      {/if}
    </button>
  </div>

  <!-- SVG Controller -->
  <div class="controller-content">
    <div class="svg-container">
      <svg viewBox="0 0 700 480" class="controller-svg" aria-label="Interactive Xbox Controller Diagram">
      <defs>
        <!-- Gradients for realistic matte finish -->
        <linearGradient id="body-grad" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="var(--controller-shell-top)" />
          <stop offset="60%" stop-color="var(--controller-shell-mid)" />
          <stop offset="100%" stop-color="var(--controller-shell-bottom)" />
        </linearGradient>

        <linearGradient id="grip-grad" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="var(--controller-shell-mid)" />
          <stop offset="100%" stop-color="var(--controller-grip-bottom)" />
        </linearGradient>

        <radialGradient id="thumb-grad" cx="50%" cy="50%" r="50%">
          <stop offset="0%" stop-color="var(--controller-thumb-center)" />
          <stop offset="85%" stop-color="var(--controller-neutral)" />
          <stop offset="100%" stop-color="var(--controller-thumb-edge)" />
        </radialGradient>

      </defs>

      <!-- Main Shell Body -->
      <path
        d="M 230,70
           C 280,68 420,68 470,70
           C 540,70 600,100 625,160
           C 655,230 670,340 640,430
           C 625,470 560,460 520,380
           C 490,320 460,300 350,300
           C 240,300 210,320 180,380
           C 140,460 75,470 60,430
           C 30,340 45,230 75,160
           C 100,100 160,70 230,70 Z"
        fill="url(#body-grad)"
        stroke="var(--controller-outline)"
        stroke-width="2"
      />

      <!-- Grip Insets -->
      <path
        d="M 68,260 C 50,320 60,400 90,430 C 120,460 145,430 165,370 C 130,340 100,300 68,260 Z"
        fill="url(#grip-grad)"
        opacity="0.6"
      />
      <path
        d="M 632,260 C 650,320 640,400 610,430 C 580,460 555,430 535,370 C 570,340 600,300 632,260 Z"
        fill="url(#grip-grad)"
        opacity="0.6"
      />

      <!-- Bumpers: LB & RB -->
      <!-- Left Bumper (LB) -->
      <g
        class="ctrl-button {selectedButton === 'LB' ? 'selected' : ''} {isPressed('LB') ? 'pressed' : ''}"
        onclick={() => onSelectButton("LB")}
        onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') onSelectButton("LB"); }}
        role="button"
        tabindex="0"
        aria-label="Left Bumper"
      >
        <path
          d="M 180,72 C 210,65 270,62 300,64 L 295,95 C 265,92 210,95 185,102 Z"
          fill={isPressed("LB") || selectedButton === "LB" ? "var(--accent)" : "var(--controller-button)"}
          stroke={selectedButton === "LB" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
          stroke-width="2"
        />
        <text x="240" y="85" text-anchor="middle" font-size="11" font-weight="700" fill="var(--controller-label)">LB</text>
      </g>

      <!-- Right Bumper (RB) -->
      <g
        class="ctrl-button {selectedButton === 'RB' ? 'selected' : ''} {isPressed('RB') ? 'pressed' : ''}"
        onclick={() => onSelectButton("RB")}
        role="button"
        tabindex="0"
      >
        <path
          d="M 520,72 C 490,65 430,62 400,64 L 405,95 C 435,92 490,95 515,102 Z"
          fill={isPressed("RB") || selectedButton === "RB" ? "var(--accent)" : "var(--controller-button)"}
          stroke={selectedButton === "RB" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
          stroke-width="2"
        />
        <text x="460" y="85" text-anchor="middle" font-size="11" font-weight="700" fill="var(--controller-label)">RB</text>
      </g>

      <!-- Center Xbox Nexus Logo -->
      <circle cx="350" cy="120" r="26" fill="var(--controller-surface)" stroke="var(--controller-outline-secondary)" stroke-width="2" />
      <path d="M 338,110 Q 350,118 362,110 Q 355,124 364,132 Q 350,126 336,132 Q 345,124 338,110 Z" fill="var(--controller-label)" />

      <!-- View & Menu Buttons -->
      <!-- View Button (Back) -->
      <g
        class="ctrl-button {selectedButton === 'Back' ? 'selected' : ''} {isPressed('Back') ? 'pressed' : ''}"
        onclick={() => onSelectButton("Back")}
        role="button"
        tabindex="0"
      >
        <circle
          cx="295"
          cy="155"
          r="12"
          fill={isPressed("Back") || selectedButton === "Back" ? "var(--accent)" : "var(--controller-button-soft)"}
          stroke={selectedButton === "Back" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
          stroke-width="1.5"
        />
        <!-- Two overlapping squares icon -->
        <rect x="290" y="151" width="7" height="7" fill="none" stroke="var(--controller-label)" stroke-width="1.2" />
        <rect x="293" y="153" width="7" height="7" fill="none" stroke="var(--controller-label)" stroke-width="1.2" />
      </g>

      <!-- Menu Button (Start) -->
      <g
        class="ctrl-button {selectedButton === 'Start' ? 'selected' : ''} {isPressed('Start') ? 'pressed' : ''}"
        onclick={() => onSelectButton("Start")}
        role="button"
        tabindex="0"
      >
        <circle
          cx="405"
          cy="155"
          r="12"
          fill={isPressed("Start") || selectedButton === "Start" ? "var(--accent)" : "var(--controller-button-soft)"}
          stroke={selectedButton === "Start" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
          stroke-width="1.5"
        />
        <!-- Hamburger 3 lines icon -->
        <line x1="399" y1="151" x2="411" y2="151" stroke="var(--controller-label)" stroke-width="1.2" />
        <line x1="399" y1="155" x2="411" y2="155" stroke="var(--controller-label)" stroke-width="1.2" />
        <line x1="399" y1="159" x2="411" y2="159" stroke="var(--controller-label)" stroke-width="1.2" />
      </g>

      <!-- LEFT THUMBSTICK (Analog) -->
      <g
        class="ctrl-button {selectedButton === 'LeftThumb' ? 'selected' : ''}"
        onclick={() => onSelectButton("LeftThumb")}
        role="button"
        tabindex="0"
      >
        <!-- Well base ring -->
        <circle cx="210" cy="180" r="44" fill="var(--controller-neutral)" stroke="var(--controller-outline)" stroke-width="2" />
        <!-- Movable thumbstick cap -->
        <g transform="translate({leftStickOffsetX}, {leftStickOffsetY})">
          <circle
            cx="210"
            cy="180"
            r="32"
            fill="url(#thumb-grad)"
            stroke={selectedButton === "LeftThumb" || isPressed("LeftThumb") ? "var(--accent)" : "var(--controller-outline-secondary)"}
            stroke-width={selectedButton === "LeftThumb" ? "2.5" : "1.5"}
          />
          <!-- Thumb grip ring -->
          <circle cx="210" cy="180" r="22" fill="none" stroke="var(--controller-outline)" stroke-width="1.5" stroke-dasharray="3,3" />
        </g>
      </g>

      <!-- D-PAD (Directional Pad) -->
      <g class="dpad-group">
        <!-- Well background -->
        <circle cx="275" cy="275" r="46" fill="var(--controller-neutral)" stroke="var(--controller-outline)" stroke-width="2" />
        
        <!-- DPad Up -->
        <g
          class="ctrl-button {selectedButton === 'DPadUp' ? 'selected' : ''} {isPressed('DPadUp') ? 'pressed' : ''}"
          onclick={() => onSelectButton("DPadUp")}
          role="button"
          tabindex="0"
        >
          <path
            d="M 264,264 L 264,240 C 264,237 286,237 286,240 L 286,264 Z"
            fill={isPressed("DPadUp") || selectedButton === "DPadUp" ? "var(--accent)" : "var(--controller-button-soft)"}
            stroke={selectedButton === "DPadUp" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
            stroke-width="1.5"
          />
          <polygon points="275,244 271,250 279,250" fill="var(--controller-icon)" />
        </g>

        <!-- DPad Down -->
        <g
          class="ctrl-button {selectedButton === 'DPadDown' ? 'selected' : ''} {isPressed('DPadDown') ? 'pressed' : ''}"
          onclick={() => onSelectButton("DPadDown")}
          role="button"
          tabindex="0"
        >
          <path
            d="M 264,286 L 264,310 C 264,313 286,313 286,310 L 286,286 Z"
            fill={isPressed("DPadDown") || selectedButton === "DPadDown" ? "var(--accent)" : "var(--controller-button-soft)"}
            stroke={selectedButton === "DPadDown" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
            stroke-width="1.5"
          />
          <polygon points="275,306 271,300 279,300" fill="var(--controller-icon)" />
        </g>

        <!-- DPad Left -->
        <g
          class="ctrl-button {selectedButton === 'DPadLeft' ? 'selected' : ''} {isPressed('DPadLeft') ? 'pressed' : ''}"
          onclick={() => onSelectButton("DPadLeft")}
          role="button"
          tabindex="0"
        >
          <path
            d="M 264,264 L 240,264 C 237,264 237,286 240,286 L 264,286 Z"
            fill={isPressed("DPadLeft") || selectedButton === "DPadLeft" ? "var(--accent)" : "var(--controller-button-soft)"}
            stroke={selectedButton === "DPadLeft" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
            stroke-width="1.5"
          />
          <polygon points="244,275 250,271 250,279" fill="var(--controller-icon)" />
        </g>

        <!-- DPad Right -->
        <g
          class="ctrl-button {selectedButton === 'DPadRight' ? 'selected' : ''} {isPressed('DPadRight') ? 'pressed' : ''}"
          onclick={() => onSelectButton("DPadRight")}
          role="button"
          tabindex="0"
        >
          <path
            d="M 286,264 L 310,264 C 313,264 313,286 310,286 L 286,286 Z"
            fill={isPressed("DPadRight") || selectedButton === "DPadRight" ? "var(--accent)" : "var(--controller-button-soft)"}
            stroke={selectedButton === "DPadRight" ? "var(--accent-strong)" : "var(--controller-outline-secondary)"}
            stroke-width="1.5"
          />
          <polygon points="306,275 300,271 300,279" fill="var(--controller-icon)" />
        </g>

        <!-- DPad Center Hub -->
        <rect x="264" y="264" width="22" height="22" fill="var(--controller-button-soft)" />
      </g>

      <!-- RIGHT THUMBSTICK (Analog) -->
      <g
        class="ctrl-button {selectedButton === 'RightThumb' ? 'selected' : ''}"
        onclick={() => onSelectButton("RightThumb")}
        role="button"
        tabindex="0"
      >
        <circle cx="425" cy="275" r="44" fill="var(--controller-neutral)" stroke="var(--controller-outline)" stroke-width="2" />
        <g transform="translate({rightStickOffsetX}, {rightStickOffsetY})">
          <circle
            cx="425"
            cy="275"
            r="32"
            fill="url(#thumb-grad)"
            stroke={selectedButton === "RightThumb" || isPressed("RightThumb") ? "var(--accent)" : "var(--controller-outline-secondary)"}
            stroke-width={selectedButton === "RightThumb" ? "2.5" : "1.5"}
          />
          <circle cx="425" cy="275" r="22" fill="none" stroke="var(--controller-outline)" stroke-width="1.5" stroke-dasharray="3,3" />
        </g>
      </g>

      <!-- FACE BUTTONS: X, Y, A, B -->
      <!-- Button Y (Top - Yellow) -->
      <g
        class="ctrl-button {selectedButton === 'Y' ? 'selected' : ''} {isPressed('Y') ? 'pressed' : ''}"
        onclick={() => onSelectButton("Y")}
        role="button"
        tabindex="0"
      >
        <circle
          cx="510"
          cy="145"
          r="16"
          fill="var(--xbox-y)"
          stroke={selectedButton === "Y" ? "var(--accent-strong)" : "var(--xbox-y)"}
          stroke-width={selectedButton === "Y" ? "2.5" : "1.5"}
        />
        <text x="510" y="151" text-anchor="middle" font-size="14" font-weight="800" fill="var(--xbox-label-dark)">Y</text>
      </g>

      <!-- Button X (Left - Blue) -->
      <g
        class="ctrl-button {selectedButton === 'X' ? 'selected' : ''} {isPressed('X') ? 'pressed' : ''}"
        onclick={() => onSelectButton("X")}
        role="button"
        tabindex="0"
      >
        <circle
          cx="475"
          cy="180"
          r="16"
          fill="var(--xbox-x)"
          stroke={selectedButton === "X" ? "var(--accent-strong)" : "var(--xbox-x)"}
          stroke-width={selectedButton === "X" ? "2.5" : "1.5"}
        />
        <text x="475" y="186" text-anchor="middle" font-size="14" font-weight="800" fill="var(--xbox-label-light)">X</text>
      </g>

      <!-- Button B (Right - Red) -->
      <g
        class="ctrl-button {selectedButton === 'B' ? 'selected' : ''} {isPressed('B') ? 'pressed' : ''}"
        onclick={() => onSelectButton("B")}
        role="button"
        tabindex="0"
      >
        <circle
          cx="545"
          cy="180"
          r="16"
          fill="var(--xbox-b)"
          stroke={selectedButton === "B" ? "var(--accent-strong)" : "var(--xbox-b)"}
          stroke-width={selectedButton === "B" ? "2.5" : "1.5"}
        />
        <text x="545" y="186" text-anchor="middle" font-size="14" font-weight="800" fill="var(--xbox-label-light)">B</text>
      </g>

      <!-- Button A (Bottom - Green) -->
      <g
        class="ctrl-button {selectedButton === 'A' ? 'selected' : ''} {isPressed('A') ? 'pressed' : ''}"
        onclick={() => onSelectButton("A")}
        role="button"
        tabindex="0"
      >
        <circle
          cx="510"
          cy="215"
          r="16"
          fill="var(--xbox-a)"
          stroke={selectedButton === "A" ? "var(--accent-strong)" : "var(--xbox-a)"}
          stroke-width={selectedButton === "A" ? "2.5" : "1.5"}
        />
        <text x="510" y="221" text-anchor="middle" font-size="14" font-weight="800" fill="var(--xbox-label-light)">A</text>
      </g>
      </svg>
    </div>

    <section class="mapping-index" aria-label="Controller mapping index">
      <div class="mapping-index-header">
        <span>Control index</span>
        <span>INPUT / ACTION</span>
      </div>
      <div class="mapping-groups">
        {#each controlGroups as group}
          <div class="mapping-group">
            <h3 class="mapping-group-title">{group.title}</h3>
            {#each group.buttons as button}
              {@const binding = getBindingLabel(button)}
              <button
                type="button"
                class="mapping-row {selectedButton === button ? 'selected' : ''} {isPressed(button) ? 'pressed' : ''}"
                onclick={() => onSelectButton(button)}
                aria-pressed={selectedButton === button}
              >
                <span class="mapping-input">{button.replace("DPad", "D-")}</span>
                <span class="mapping-binding">{binding || "Not mapped"}</span>
              </button>
            {/each}
          </div>
        {/each}
      </div>
    </section>
  </div>

  <!-- Instruction Footer Bar -->
  <div class="stage-footer">
    <span class="hint-text">
      Click any button or trigger on the controller to customize its key mapping.
    </span>
    {#if onTestButton}
      <button
        type="button"
        class="test-btn"
        onclick={() => onTestButton(selectedButton)}
        title="Simulate pressing this button"
      >
        Test Press [{selectedButton}]
      </button>
    {/if}
  </div>
</div>

<style>
  .controller-stage {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    height: 100%;
    min-width: 0;
    min-height: 0;
    padding: 10px;
    background: var(--bg-base);
    position: relative;
    user-select: none;
  }

  .trigger-gauges {
    display: flex;
    gap: 8px;
    width: 100%;
    margin-bottom: 7px;
  }

  .trigger-gauge {
    flex: 1;
    min-width: 0;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 2px;
    padding: 6px 8px;
    cursor: pointer;
    text-align: left;
    transition: background-color 120ms ease, border-color 120ms ease;
  }

  .trigger-gauge:hover {
    border-color: var(--accent-strong);
    background: var(--accent-muted);
  }

  .trigger-gauge.selected {
    border-color: var(--accent-strong);
    background: var(--accent-muted);
  }

  .gauge-header {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 5px;
  }

  .gauge-val {
    color: var(--accent-strong);
    font-variant-numeric: tabular-nums;
  }

  .gauge-track {
    height: 4px;
    background: var(--bg-muted);
    border-radius: 1px;
    overflow: hidden;
  }

  .gauge-fill {
    height: 100%;
    background: var(--accent);
    transition: width 50ms linear;
  }

  .gauge-binding {
    display: block;
    margin-top: 6px;
    font-size: 10px;
    color: var(--text-primary);
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .controller-content {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(260px, 1fr);
    gap: 8px;
    width: 100%;
    flex: 1;
    min-width: 0;
    min-height: 0;
  }

  .svg-container {
    width: 100%;
    min-width: 0;
    min-height: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-surface);
    border: 1px solid var(--border);
  }

  .controller-svg {
    width: 100%;
    height: auto;
    max-height: 100%;
    filter: none;
  }

  .ctrl-button {
    cursor: pointer;
    transition: transform 0.08s ease, filter 0.15s ease;
  }

  .ctrl-button:hover {
    filter: brightness(0.94);
  }

  .ctrl-button:focus-visible {
    outline: none;
  }

  .ctrl-button.selected {
    opacity: 0.96;
  }

  .ctrl-button.pressed {
    transform: scale(0.97);
  }

  .stage-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    gap: 8px;
    padding-top: 7px;
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  .hint-text {
    min-width: 0;
    font-size: 10px;
    color: var(--text-muted);
    line-height: 1.4;
  }

  .test-btn {
    background: var(--accent);
    color: #fff;
    border: 1px solid var(--accent-strong);
    border-radius: 2px;
    padding: 6px 9px;
    font-size: 10px;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
    transition: background-color 120ms ease;
  }

  .test-btn:hover {
    background: var(--accent-strong);
  }

  .mapping-index {
    min-width: 0;
    min-height: 0;
    overflow: auto;
    background: var(--bg-surface);
    border: 1px solid var(--border);
  }

  .mapping-index-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 7px 8px;
    background: var(--bg-muted);
    border-bottom: 1px solid var(--border);
    color: var(--text-primary);
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }

  .mapping-index-header span:last-child {
    color: var(--text-muted);
    font-size: 8px;
    letter-spacing: 0.06em;
  }

  .mapping-groups {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    column-gap: 8px;
    padding: 0 7px;
  }

  .mapping-group {
    min-width: 0;
    padding: 5px 0 3px;
    border-bottom: 1px solid var(--border);
  }

  .mapping-group-title {
    margin: 0;
    padding: 0 2px 3px;
    color: var(--text-muted);
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .mapping-row {
    display: grid;
    grid-template-columns: minmax(42px, 0.65fr) minmax(0, 1fr);
    align-items: center;
    gap: 5px;
    width: 100%;
    min-height: 25px;
    padding: 3px 4px;
    border: 0;
    border-top: 1px solid var(--border);
    border-radius: 0;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    text-align: left;
  }

  .mapping-row:hover {
    background: var(--bg-inset);
  }

  .mapping-row.selected {
    background: var(--accent-muted);
    color: var(--accent-strong);
  }

  .mapping-row.pressed .mapping-input::before {
    content: "";
    display: inline-block;
    width: 5px;
    height: 5px;
    margin-right: 4px;
    background: var(--success);
    vertical-align: 1px;
  }

  .mapping-row:focus-visible,
  .trigger-gauge:focus-visible,
  .test-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
  }

  .mapping-input,
  .mapping-binding {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .mapping-input {
    color: var(--text-primary);
    font-size: 9px;
    font-weight: 700;
  }

  .mapping-binding {
    color: var(--text-muted);
    font-size: 9px;
    font-variant-numeric: tabular-nums;
  }

  .mapping-row.selected .mapping-input,
  .mapping-row.selected .mapping-binding {
    color: var(--accent-strong);
  }

  @media (max-width: 1190px) {
    .controller-content {
      grid-template-columns: minmax(0, 1fr);
      grid-template-rows: minmax(170px, 1fr) minmax(180px, 0.9fr);
    }

    .mapping-index {
      max-height: 230px;
    }
  }

  @media (max-width: 600px) {
    .controller-stage {
      height: auto;
      min-height: 0;
      padding: 8px;
    }

    .controller-content {
      display: flex;
      flex-direction: column;
      flex: 0 0 auto;
    }

    .svg-container {
      min-height: 180px;
      max-height: 260px;
    }

    .controller-svg {
      width: 100%;
      max-height: 250px;
    }

    .mapping-index {
      max-height: 350px;
    }

    .mapping-groups {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .stage-footer {
      align-items: flex-start;
    }

    .hint-text {
      max-width: 65%;
    }
  }

  @media (max-width: 360px) {
    .mapping-groups {
      grid-template-columns: minmax(0, 1fr);
    }
  }
</style>
