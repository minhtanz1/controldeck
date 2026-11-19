<script lang="ts">
  import { onDestroy, onMount, untrack } from "svelte";
  import type { MobileMode } from "./lib/types";
  import { wsClient } from "./lib/ws.svelte";
  import { settings } from "./lib/settings.svelte";
  import Header from "./components/Header.svelte";
  import ModeNav from "./components/ModeNav.svelte";
  import VirtualGamepad from "./components/VirtualGamepad.svelte";
  import VirtualTrackpad from "./components/VirtualTrackpad.svelte";
  import ActionDeck from "./components/ActionDeck.svelte";
  import MobileSettingsDrawer from "./components/MobileSettingsDrawer.svelte";
  import LayoutEditBar from "./components/LayoutEditBar.svelte";

  let currentMode = $state<MobileMode>("gamepad");
  let isSettingsOpen = $state(false);
  let isEditingLayout = $state(false);
  let theme = $state<"light" | "dark">("light");
  let controlZoom = $derived(
    settings.data.scale === "small" ? 0.88 : settings.data.scale === "large" ? 1.08 : 1,
  );
  let themeNotice = $state("");
  let themeNoticeTimer: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    const profileId = wsClient.activeProfileId;
    if (profileId) untrack(() => settings.activateProfile(profileId));
  });

  onMount(() => {
    try {
      const savedTheme = localStorage.getItem("controldeck-theme");
      if (savedTheme === "light" || savedTheme === "dark") {
        theme = savedTheme;
      } else if (savedTheme !== null) {
        console.error("Invalid saved theme preference:", savedTheme);
      }
    } catch (error) {
      console.error("Failed to load theme preference:", error);
      showThemeNotice("Could not load the saved theme; using light mode.");
    }
    document.documentElement.dataset.theme = theme;
    wsClient.connect();
    wsClient.setSensitivity(settings.data.mouseSensitivity);
  });

  onDestroy(() => {
    if (themeNoticeTimer) clearTimeout(themeNoticeTimer);
  });

  function showThemeNotice(message: string) {
    if (themeNoticeTimer) clearTimeout(themeNoticeTimer);
    themeNotice = message;
    themeNoticeTimer = setTimeout(() => {
      themeNotice = "";
      themeNoticeTimer = null;
    }, 4000);
  }

  function toggleTheme() {
    theme = theme === "light" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
    try {
      localStorage.setItem("controldeck-theme", theme);
    } catch (error) {
      console.error("Failed to save theme preference:", error);
      showThemeNotice("Theme changed for this session, but the preference could not be saved.");
    }
  }
</script>

<div class="mobile-app-root">
  <!-- Top Bar -->
  <Header onOpenSettings={() => (isSettingsOpen = true)} />

  <!-- Main Viewports -->
  <main class="mobile-stage-container" style:--control-zoom={controlZoom}>
    {#if isEditingLayout && currentMode === "gamepad"}
      <LayoutEditBar
        layouts={settings.layoutsForActiveProfile}
        activeLayoutId={settings.activeLayoutId}
        canSaveLayout={Boolean(settings.activeProfileId)}
        onSelectLayout={(layoutId) => settings.selectLayout(layoutId)}
        onSaveLayout={(name) => settings.saveLayout(name)}
        onDeleteLayout={() => settings.deleteActiveLayout()}
        onReset={() => settings.resetCustomPositions()}
        onSave={() => (isEditingLayout = false)}
      />
    {/if}

    {#if currentMode === "gamepad"}
      <VirtualGamepad {isEditingLayout} />
    {:else if currentMode === "mouse"}
      <VirtualTrackpad />
    {:else if currentMode === "deck"}
      <ActionDeck />
    {/if}
  </main>

  <!-- Bottom Mode Navigation -->
  <ModeNav {currentMode} onSelectMode={(mode) => (currentMode = mode)} />

  <!-- Layout & App Settings Drawer -->
  <MobileSettingsDrawer
    isOpen={isSettingsOpen}
    {theme}
    onToggleTheme={toggleTheme}
    onClose={() => (isSettingsOpen = false)}
    onEnterLayoutEdit={() => {
      currentMode = "gamepad";
      isEditingLayout = true;
    }}
  />

  {#if themeNotice}
    <div class="theme-notice" role="alert" aria-live="assertive">{themeNotice}</div>
  {/if}
</div>

<style>
  :global(:root) {
    color-scheme: light;
    --bg: #f1f3f1;
    --surface: #ffffff;
    --surface-elevated: #e6eae8;
    --surface-hover: #edf0ef;
    --border: rgba(23, 35, 42, 0.16);
    --accent: #c6282d;
    --accent-strong: #a91f25;
    --accent-muted: #f9e9e8;
    --text: #17232a;
    --text-muted: #59676d;
    --danger: #b4232a;
    --danger-muted: #f9e9e8;
    --success: #24694e;
    --xbox-a: #107c10;
    --xbox-b: #c50f1f;
    --xbox-x: #0063b1;
    --xbox-y: #f2c811;
    --xbox-label-light: #ffffff;
    --xbox-label-dark: #17232a;
  }

  :global(:root[data-theme="dark"]) {
    color-scheme: dark;
    --bg: #151a1c;
    --surface: #1e2527;
    --surface-elevated: #293133;
    --surface-hover: #303a3c;
    --border: rgba(220, 232, 226, 0.16);
    --accent: #b3262c;
    --accent-strong: #f47b80;
    --accent-muted: #482629;
    --text: #edf2ef;
    --text-muted: #a3afac;
    --danger: #ff777d;
    --danger-muted: #462326;
    --success: #82d2a4;
    --xbox-a: #107c10;
    --xbox-b: #c50f1f;
    --xbox-x: #0063b1;
    --xbox-y: #f2c811;
    --xbox-label-light: #ffffff;
    --xbox-label-dark: #17232a;
  }

  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    -webkit-tap-highlight-color: transparent;
  }

  :global(html, body) {
    width: 100%;
    height: 100%;
    height: 100dvh;
    overflow: hidden;
    background: var(--bg);
    color: var(--text);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  }

  .mobile-app-root {
    width: 100%;
    height: 100vh;
    height: 100dvh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    background: var(--bg);
    padding-top: env(safe-area-inset-top);
    padding-bottom: env(safe-area-inset-bottom);
    color: var(--text);
  }

  .mobile-stage-container {
    flex: 1;
    min-width: 0;
    min-height: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }

  .theme-notice {
    position: absolute;
    z-index: 120;
    right: 12px;
    bottom: calc(58px + env(safe-area-inset-bottom));
    max-width: calc(100% - 24px);
    padding: 10px 12px;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    box-shadow: 0 4px 14px rgba(23, 35, 42, 0.18);
    font-size: 12px;
    line-height: 1.4;
  }
</style>
