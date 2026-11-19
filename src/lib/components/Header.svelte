<script lang="ts">
  import type { ActiveWindowInfo, ControllerState, MobileServerInfo, Profile } from "$lib/types";

  let {
    profiles = [],
    activeProfile = null,
    controllerState = null,
    activeWindow = null,
    mobileInfo = null,
    enabled = true,
    theme = "light",
    onSelectProfile,
    onToggleEnabled,
    onToggleTheme,
    onOpenMobileModal,
  }: {
    profiles: Profile[];
    activeProfile: Profile | null;
    controllerState: ControllerState | null;
    activeWindow: ActiveWindowInfo | null;
    mobileInfo?: MobileServerInfo | null;
    enabled: boolean;
    theme: "light" | "dark";
    onSelectProfile: (id: string) => void;
    onToggleEnabled: () => void;
    onToggleTheme: () => void;
    onOpenMobileModal?: () => void;
  } = $props();
</script>

<header class="header">
  <div class="brand">
    <div class="logo-mark">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="2" y="6" width="20" height="12" rx="4" />
        <circle cx="8" cy="12" r="1.5" />
        <path d="M15 10v4" />
        <path d="M13 12h4" />
      </svg>
    </div>
    <div class="brand-text">
      <span class="app-name">ControlDeck</span>
      <span class="app-version">v0.2.0</span>
    </div>
  </div>

  <div class="status-group">
    <!-- Controller Connection Status -->
    <div class="status-pill {controllerState?.connected ? 'connected' : 'disconnected'}">
      <span class="status-dot"></span>
      <span class="status-label">
        {#if controllerState?.connected}
          Xbox Controller (Active)
        {:else}
          No Controller (Simulated)
        {/if}
      </span>
    </div>

    <!-- Mobile PocketPad Pill / Trigger Button -->
    {#if onOpenMobileModal}
      <button
        type="button"
        class="mobile-connect-btn {mobileInfo && mobileInfo.connected_clients > 0 ? 'mobile-active' : ''}"
        onclick={onOpenMobileModal}
        title="Open PocketPad QR code to connect mobile controller"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
          <path d="M12 18h.01" />
        </svg>
        <span>PocketPad Mobile</span>
        {#if mobileInfo && mobileInfo.connected_clients > 0}
          <span class="mobile-count-pill">{mobileInfo.connected_clients}</span>
        {/if}
      </button>
    {/if}

    <!-- Active Window Focus Pill -->
    <div class="window-pill" title={activeWindow?.title ? `${activeWindow.title} (${activeWindow.process_name})` : "No active window"}>
      <svg class="window-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" />
        <path d="M3 9h18" />
      </svg>
      <span class="window-text">
        {#if activeWindow?.title}
          {activeWindow.process_name || "App"}: {activeWindow.title.slice(0, 28)}{activeWindow.title.length > 28 ? "..." : ""}
        {:else}
          Detecting focus...
        {/if}
      </span>
    </div>
  </div>

  <div class="controls-group">
    <button
      type="button"
      class="theme-toggle"
      onclick={onToggleTheme}
      aria-label={theme === "dark" ? "Switch to light mode" : "Switch to dark mode"}
      aria-pressed={theme === "dark"}
      title={theme === "dark" ? "Switch to light mode" : "Switch to dark mode"}
    >
      {#if theme === "dark"}
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
          <circle cx="12" cy="12" r="4" />
          <path d="M12 2v2m0 16v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" />
        </svg>
      {:else}
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20.5 14.2A8.5 8.5 0 0 1 9.8 3.5 8.5 8.5 0 1 0 20.5 14.2Z" />
        </svg>
      {/if}
    </button>

    <!-- Profile Quick Selector -->
    <div class="profile-select-wrapper">
      <label for="profile-select" class="visually-hidden">Profile</label>
      <select
        id="profile-select"
        class="profile-select"
        value={activeProfile?.id ?? ""}
        onchange={(e) => onSelectProfile((e.target as HTMLSelectElement).value)}
      >
        {#each profiles as p}
          <option value={p.id}>{p.name}</option>
        {/each}
      </select>
    </div>

    <!-- Master Switch -->
    <button
      type="button"
      class="master-switch {enabled ? 'active' : 'inactive'}"
      onclick={onToggleEnabled}
      title={enabled ? "ControlDeck is running" : "Mapping is paused"}
    >
      <span class="switch-indicator"></span>
      <span>{enabled ? "Active" : "Paused"}</span>
    </button>
  </div>
</header>

<style>
  .header {
    min-height: 52px;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-strong);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 12px;
    gap: 10px;
    user-select: none;
  }

  .theme-toggle {
    width: 30px;
    height: 30px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border-strong);
    border-radius: 2px;
    background: var(--bg-surface);
    color: var(--text-secondary);
    cursor: pointer;
    transition: background-color 120ms ease, border-color 120ms ease, color 120ms ease;
  }

  .theme-toggle svg {
    width: 16px;
    height: 16px;
  }

  .theme-toggle:hover {
    background: var(--accent-muted);
    border-color: var(--accent-strong);
    color: var(--accent-strong);
  }

  .theme-toggle:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .logo-mark {
    width: 28px;
    height: 28px;
    border-radius: 3px;
    background: var(--accent);
    border: 1px solid var(--accent-strong);
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .brand-text {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 1px;
  }

  .app-name {
    font-weight: 700;
    font-size: 13px;
    letter-spacing: 0;
    color: var(--text-primary);
  }

  .app-version {
    font-size: 9px;
    color: var(--text-muted);
    font-weight: 500;
    font-variant-numeric: tabular-nums;
  }

  .status-group {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    min-width: 0;
    max-width: none;
  }

  .status-pill {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 4px 7px;
    border-radius: 2px;
    font-size: 10px;
    font-weight: 500;
    white-space: nowrap;
    border: 1px solid var(--border);
  }

  .status-pill.connected {
    background-color: var(--success-muted);
    color: var(--success);
    border-color: var(--success-border);
  }

  .status-pill.disconnected {
    background-color: var(--warning-muted);
    color: var(--warning);
    border-color: var(--warning-border);
  }

  .status-dot {
    width: 7px;
    height: 7px;
    border-radius: 1px;
    background-color: currentColor;
  }

  .mobile-connect-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 8px;
    border-radius: 2px;
    background: var(--bg-inset);
    border: 1px solid var(--border-strong);
    color: var(--text-primary);
    font-size: 10px;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 120ms ease, border-color 120ms ease;
  }

  .mobile-connect-btn:hover {
    background: var(--accent-muted);
    border-color: var(--accent-strong);
  }

  .mobile-connect-btn.mobile-active {
    background: var(--success-muted);
    color: var(--success);
    border-color: var(--success-border);
  }

  .mobile-count-pill {
    background: var(--success);
    color: #fff;
    font-size: 10px;
    font-weight: 700;
    padding: 1px 5px;
    border-radius: 2px;
  }

  .window-pill {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 4px 7px;
    border-radius: 2px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    font-size: 10px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: min(230px, 30vw);
    min-width: 0;
  }

  .window-icon {
    flex-shrink: 0;
    opacity: 0.7;
  }

  .window-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .controls-group {
    display: flex;
    align-items: center;
    gap: 7px;
    flex-shrink: 0;
  }

  .profile-select-wrapper {
    position: relative;
  }

  .profile-select {
    appearance: none;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-strong);
    border-radius: 2px;
    color: var(--text-primary);
    font-size: 11px;
    font-weight: 500;
    padding: 5px 25px 5px 8px;
    max-width: 150px;
    min-width: 90px;
    cursor: pointer;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
  }

  .profile-select:hover {
    border-color: var(--accent-strong);
  }

  .profile-select:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .master-switch {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 8px;
    border-radius: 2px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid;
    transition: background-color 120ms ease, border-color 120ms ease;
  }

  .master-switch.active {
    background: var(--accent-muted);
    color: var(--accent-strong);
    border-color: var(--accent-border);
  }

  .master-switch.active:hover {
    background: var(--accent-hover);
  }

  .master-switch.inactive {
    background: var(--bg-muted);
    color: var(--text-secondary);
    border-color: var(--border-strong);
  }

  .switch-indicator {
    width: 7px;
    height: 7px;
    border-radius: 1px;
    background-color: currentColor;
  }

  .visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    border: 0;
  }

  @media (max-width: 1120px) {
    .window-pill {
      display: none;
    }

    .status-group {
      max-width: none;
    }
  }

  @media (max-width: 720px) {
    .header {
      justify-content: flex-start;
      gap: 8px;
      flex-wrap: wrap;
    }

    .controls-group {
      margin-left: auto;
    }

    .status-group {
      order: 3;
      flex: 0 0 100%;
      flex-wrap: wrap;
    }

    .status-pill .status-label {
      max-width: 155px;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }

  @media (max-width: 420px) {
    .header {
      padding: 6px 8px;
      column-gap: 6px;
    }

    .controls-group {
      gap: 4px;
    }

    .mobile-connect-btn span:not(.mobile-count-pill) {
      display: none;
    }

    .profile-select {
      max-width: 90px;
      min-width: 64px;
      padding-right: 20px;
    }

    .master-switch {
      padding: 5px 6px;
    }
  }
</style>
