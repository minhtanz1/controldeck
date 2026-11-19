<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import type { Action, ActiveWindowInfo, AppRule, ControllerState, MobileServerInfo, Profile } from "$lib/types";
  import Header from "$lib/components/Header.svelte";
  import ProfileList from "$lib/components/ProfileList.svelte";
  import ControllerView from "$lib/components/ControllerView.svelte";
  import MappingInspector from "$lib/components/MappingInspector.svelte";
  import AppRulesModal from "$lib/components/AppRulesModal.svelte";
  import MobileConnectModal from "$lib/components/MobileConnectModal.svelte";

  let profiles = $state<Profile[]>([]);
  let activeProfile = $state<Profile | null>(null);
  let controllerState = $state<ControllerState | null>(null);
  let activeWindow = $state<ActiveWindowInfo | null>(null);
  let mobileInfo = $state<MobileServerInfo | null>(null);
  let selectedButton = $state<string>("A");
  let enabled = $state<boolean>(true);
  let ruleModalProfile = $state<Profile | null>(null);
  let showMobileModal = $state<boolean>(false);
  let theme = $state<"light" | "dark">("light");
  let toast = $state<{ type: "success" | "error"; message: string } | null>(null);
  let toastTimeout: ReturnType<typeof setTimeout> | null = null;

  function showToast(type: "success" | "error", message: string) {
    if (toastTimeout) clearTimeout(toastTimeout);
    toast = { type, message };
    toastTimeout = setTimeout(() => {
      toast = null;
      toastTimeout = null;
    }, 3500);
  }

  function toggleTheme() {
    theme = theme === "light" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
    try {
      localStorage.setItem("controldeck-theme", theme);
    } catch (e) {
      console.error("Failed to save theme preference:", e);
      showToast("error", "Theme changed for this session, but the preference could not be saved.");
    }
  }

  onMount(() => {
    try {
      const savedTheme = localStorage.getItem("controldeck-theme");
      if (savedTheme === "light" || savedTheme === "dark") {
        theme = savedTheme;
      } else if (savedTheme !== null) {
        console.error("Invalid saved theme preference:", savedTheme);
      }
    } catch (e) {
      console.error("Failed to load theme preference:", e);
      showToast("error", "Could not load the saved theme; using light mode.");
    }
    document.documentElement.dataset.theme = theme;

    if (!isTauri()) {
      console.info("ControlDeck native features require the desktop app; browser preview is UI-only.");
      return;
    }

    loadProfiles();
    pollMobileInfo();

    // 1. Controller Polling Loop (50ms ~ 20Hz for UI telemetry)
    const ctrlInterval = setInterval(async () => {
      try {
        controllerState = await invoke<ControllerState>("get_controller_state");
      } catch (err) {
        console.error("Failed to poll controller:", err);
      }
    }, 50);

    // 2. Active Window Detection Loop (500ms)
    const winInterval = setInterval(async () => {
      try {
        const info = await invoke<ActiveWindowInfo>("get_active_window");
        activeWindow = info;
        // Also sync active profile in case Rust background engine auto-switched it!
        const curr = await invoke<Profile | null>("get_active_profile");
        if (curr && curr.id !== activeProfile?.id) {
          activeProfile = curr;
        }
      } catch (err) {
        console.error("Failed to get active window:", err);
      }
    }, 500);

    // 3. Mobile Server Telemetry Loop (2000ms)
    const mobInterval = setInterval(pollMobileInfo, 2000);

    return () => {
      clearInterval(ctrlInterval);
      clearInterval(winInterval);
      clearInterval(mobInterval);
    };
  });

  onDestroy(() => {
    if (toastTimeout) clearTimeout(toastTimeout);
  });

  async function pollMobileInfo() {
    try {
      mobileInfo = await invoke<MobileServerInfo>("get_mobile_server_info");
    } catch (e) {
      console.error("Failed to get mobile server info:", e);
    }
  }

  async function loadProfiles() {
    try {
      profiles = await invoke<Profile[]>("get_profiles");
      const act = await invoke<Profile | null>("get_active_profile");
      activeProfile = act || profiles[0] || null;
      enabled = await invoke<boolean>("is_mapping_enabled");
    } catch (e) {
      console.error("Error loading profiles:", e);
    }
  }

  async function selectProfile(id: string) {
    try {
      await invoke("set_active_profile", { id });
      const found = profiles.find((p) => p.id === id);
      if (found) activeProfile = found;
    } catch (e) {
      console.error("Error setting active profile:", e);
    }
  }

  async function renameProfile(id: string, name: string): Promise<boolean> {
    const profile = profiles.find((p) => p.id === id);
    if (!profile) {
      console.error(`Cannot rename missing profile: ${id}`);
      showToast("error", "Could not find that profile. Refresh and try again.");
      return false;
    }

    const updated = { ...profile, name };
    try {
      await invoke("save_profile", { profile: updated });
      profiles = profiles.map((p) => (p.id === id ? updated : p));
      if (activeProfile?.id === id) activeProfile = updated;
      showToast("success", `Renamed profile to ${name}`);
      return true;
    } catch (e) {
      console.error("Failed to rename profile:", e);
      showToast("error", "Could not rename this profile. Please try again.");
      return false;
    }
  }

  async function handleToggleEnabled() {
    const next = !enabled;
    enabled = next;
    try {
      await invoke("set_mapping_enabled", { enabled: next });
    } catch (e) {
      console.error("Failed to set mapping enabled:", e);
    }
  }

  async function handleSaveMapping(button: string, action: Action | null) {
    if (!activeProfile) return;
    const updated = JSON.parse(JSON.stringify(activeProfile)) as Profile;
    if (!updated.mappings) updated.mappings = { buttons: {} };
    if (!updated.mappings.buttons) updated.mappings.buttons = {};

    if (action) {
      updated.mappings.buttons[button] = action;
    } else {
      delete updated.mappings.buttons[button];
    }

    try {
      await invoke("save_profile", { profile: updated });
      activeProfile = updated;
      const idx = profiles.findIndex((p) => p.id === updated.id);
      if (idx !== -1) profiles[idx] = updated;
    } catch (e) {
      console.error("Failed to save mapping:", e);
    }
  }

  async function createProfile() {
    const id = `profile-${Date.now().toString(36)}`;
    const newP: Profile = {
      id,
      name: "New Profile",
      description: "Custom mapping profile",
      version: 1,
      app_rules: [],
      mappings: {
        buttons: {},
      },
    };

    try {
      await invoke("save_profile", { profile: newP });
      profiles = [...profiles, newP];
      selectProfile(id);
    } catch (e) {
      console.error("Failed to create profile:", e);
    }
  }

  async function deleteProfile(id: string) {
    try {
      await invoke("delete_profile", { id });
      profiles = profiles.filter((p) => p.id !== id);
      if (activeProfile?.id === id) {
        activeProfile = profiles[0] || null;
      }
    } catch (e) {
      console.error("Failed to delete profile:", e);
    }
  }

  async function exportProfile(id: string) {
    const filename = `controldeck-${id}.json`;
    try {
      const json = await invoke<string>("export_profile", { id });
      const blob = new Blob([json], { type: "application/json" });
      const objectUrl = URL.createObjectURL(blob);
      let a: HTMLAnchorElement | null = null;
      try {
        a = document.createElement("a");
        a.href = objectUrl;
        a.download = filename;
        a.hidden = true;
        document.body.append(a);
        a.click();
      } finally {
        a?.remove();
        setTimeout(() => URL.revokeObjectURL(objectUrl), 1000);
      }
      showToast("success", `Exported ${filename}`);
    } catch (e) {
      console.error("Failed to export profile:", e);
      showToast("error", "Could not export this profile. Please try again.");
    }
  }

  async function importProfile(jsonContent: string) {
    try {
      const imported = await invoke<Profile>("import_profile", { jsonContent });
      const existingIdx = profiles.findIndex((p) => p.id === imported.id);
      if (existingIdx !== -1) {
        profiles[existingIdx] = imported;
      } else {
        profiles = [...profiles, imported];
      }
      selectProfile(imported.id);
    } catch (e) {
      alert(`Import failed: ${e}`);
    }
  }

  async function saveRules(rules: AppRule[]) {
    if (!ruleModalProfile) return;
    const updated = JSON.parse(JSON.stringify(ruleModalProfile)) as Profile;
    updated.app_rules = rules;

    try {
      await invoke("save_profile", { profile: updated });
      if (activeProfile?.id === updated.id) activeProfile = updated;
      const idx = profiles.findIndex((p) => p.id === updated.id);
      if (idx !== -1) profiles[idx] = updated;
    } catch (e) {
      console.error("Failed to save rules:", e);
    }
  }

  async function testButton(btn: string) {
    try {
      // Simulate button down then up + inject mapped key!
      await invoke("simulate_controller_button", { button: btn, pressed: true });
      setTimeout(async () => {
        await invoke("simulate_controller_button", { button: btn, pressed: false });
      }, 150);
    } catch (e) {
      console.error("Failed to simulate button:", e);
    }
  }

  async function testAction(action: Action) {
    try {
      await invoke("test_action", { action });
    } catch (e) {
      console.error("Failed to test action:", e);
    }
  }
</script>

<div class="app-layout">
  <Header
    {profiles}
    {activeProfile}
    {theme}
    {controllerState}
    {activeWindow}
    {mobileInfo}
    {enabled}
    onSelectProfile={selectProfile}
    onToggleTheme={toggleTheme}
    onToggleEnabled={handleToggleEnabled}
    onOpenMobileModal={() => (showMobileModal = true)}
  />

  <div class="workspace">
    <ProfileList
      {profiles}
      {activeProfile}
      onSelectProfile={selectProfile}
      onCreateProfile={createProfile}
      onOpenRules={(p) => (ruleModalProfile = p)}
      onExportProfile={exportProfile}
      onImportProfile={importProfile}
      onDeleteProfile={deleteProfile}
      onRenameProfile={renameProfile}
    />

    <main class="main-stage">
      <ControllerView
        profile={activeProfile}
        {controllerState}
        {selectedButton}
        onSelectButton={(b) => (selectedButton = b)}
        onTestButton={testButton}
      />
    </main>

    <MappingInspector
      {selectedButton}
      profile={activeProfile}
      onSaveMapping={handleSaveMapping}
      onTestAction={testAction}
    />
  </div>

  {#if ruleModalProfile}
    <AppRulesModal
      profile={ruleModalProfile}
      {activeWindow}
      onClose={() => (ruleModalProfile = null)}
      onSave={saveRules}
    />
  {/if}

  {#if showMobileModal}
    <MobileConnectModal
      {mobileInfo}
      onClose={() => (showMobileModal = false)}
    />
  {/if}

  {#if toast}
    <div class="toast {toast.type}" role={toast.type === "error" ? "alert" : "status"} aria-atomic="true">
      <span class="toast-mark" aria-hidden="true"></span>
      <span>{toast.message}</span>
    </div>
  {/if}
</div>

<style>
  :global(:root) {
    --bg-base: #f1f3f1;
    --bg-surface: #ffffff;
    --bg-muted: #e6eae8;
    --bg-inset: #edf0ef;
    --surface-hover: #e6ecea;
    --text-primary: #17232a;
    --text-secondary: #39474d;
    --text-muted: #59676d;
    --accent: #c6282d;
    --accent-strong: #a91f25;
    --accent-muted: #f9e9e8;
    --success: #24694e;
    --success-muted: #e8f2ec;
    --success-border: #b7d2c2;
    --warning: #855a12;
    --warning-muted: #f6f0df;
    --warning-border: #dfcea2;
    --danger: #b4232a;
    --danger-muted: #f9e9e8;
    --accent-border: #e4a5a3;
    --accent-hover: #f4d8d6;
    --border: rgba(23, 35, 42, 0.16);
    --border-strong: rgba(23, 35, 42, 0.3);
    --scrollbar-thumb: #aab4b1;
    --selection-bg: #f3ccca;
    --selection-fg: #17232a;
    --controller-shell-top: #f8f9f8;
    --controller-shell-mid: #e8ece9;
    --controller-shell-bottom: #d5dbd7;
    --controller-grip-bottom: #d1d8d4;
    --controller-thumb-center: #f7f8f7;
    --controller-neutral: #dfe4e1;
    --controller-thumb-edge: #c5ceca;
    --controller-outline: #a8b2ad;
    --controller-outline-secondary: #87938d;
    --controller-label: #39474d;
    --controller-icon: #59676d;
    --controller-button: #d7ddda;
    --controller-button-soft: #eef1ef;
    --controller-surface: #e4e8e6;
    --xbox-a: #107c10;
    --xbox-b: #c50f1f;
    --xbox-x: #0063b1;
    --xbox-y: #f2c811;
    --xbox-label-light: #ffffff;
    --xbox-label-dark: #17232a;
    --desktop-font-size: clamp(12pt, 1vw, 14pt);

    font-family: "Segoe UI", "Yu Gothic UI", Meiryo, sans-serif;
    color: var(--text-primary);
    background-color: var(--bg-base);
    color-scheme: light;
  }

  :global(:root[data-theme="dark"]) {
    --bg-base: #151a1c;
    --bg-surface: #1e2527;
    --bg-muted: #293133;
    --bg-inset: #191f21;
    --surface-hover: #303a3c;
    --text-primary: #edf2ef;
    --text-secondary: #c5cfcc;
    --text-muted: #a3afac;
    --accent: #b3262c;
    --accent-strong: #f47b80;
    --accent-muted: #482629;
    --success: #82d2a4;
    --success-muted: #1e3528;
    --success-border: #355b45;
    --warning: #e7c779;
    --warning-muted: #342e1b;
    --warning-border: #5b4c29;
    --danger: #ff777d;
    --danger-muted: #462326;
    --accent-border: #6a3438;
    --accent-hover: #53272b;
    --border: rgba(220, 232, 226, 0.16);
    --border-strong: rgba(220, 232, 226, 0.3);
    --scrollbar-thumb: #596467;
    --selection-bg: #6a3438;
    --selection-fg: #ffffff;
    --controller-shell-top: #303a3c;
    --controller-shell-mid: #262f31;
    --controller-shell-bottom: #1c2325;
    --controller-grip-bottom: #1d2527;
    --controller-thumb-center: #3b4648;
    --controller-neutral: #252e30;
    --controller-thumb-edge: #1b2224;
    --controller-outline: #5e6a67;
    --controller-outline-secondary: #6f7c78;
    --controller-label: #d7e0dd;
    --controller-icon: #bcc7c3;
    --controller-button: #303a3c;
    --controller-button-soft: #293234;
    --controller-surface: #353f41;
    color-scheme: dark;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    height: 100dvh;
    width: 100%;
    background: var(--bg-base);
    color: var(--text-primary);
  }

  :global(*),
  :global(*::before),
  :global(*::after) {
    box-sizing: border-box;
  }

  :global(.app-layout :not(svg):not(path):not(circle):not(rect):not(line):not(polyline):not(polygon):not(g):not(text)) {
    font-size: var(--desktop-font-size) !important;
  }

  :global(.app-layout svg text) {
    font-size: var(--desktop-font-size) !important;
  }

  :global(.app-layout :is(h1, h2, h3, .app-name, .target-name)) {
    font-size: 14pt !important;
  }

  :global(::selection) {
    background: var(--selection-bg);
    color: var(--selection-fg);
  }

  :global(::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(::-webkit-scrollbar-track) {
    background: var(--bg-muted);
  }

  :global(::-webkit-scrollbar-thumb) {
    background: var(--scrollbar-thumb);
    border: 2px solid var(--bg-muted);
    border-radius: 8px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: var(--text-muted);
  }

  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100dvh;
    width: 100%;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
    background: var(--bg-base);
  }

  .workspace {
    flex: 1;
    min-width: 0;
    min-height: 0;
    display: flex;
    overflow: hidden;
  }

  .main-stage {
    flex: 1;
    height: 100%;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .toast {
    position: fixed;
    z-index: 200;
    right: 20px;
    bottom: 20px;
    display: flex;
    align-items: center;
    gap: 10px;
    width: max-content;
    max-width: min(420px, calc(100vw - 32px));
    padding: 11px 14px;
    border: 1px solid var(--border-strong);
    background: var(--bg-surface);
    color: var(--text-primary);
    box-shadow: 0 8px 24px rgba(23, 35, 42, 0.16);
    font-size: 13px;
    line-height: 1.4;
  }

  .toast-mark {
    width: 8px;
    height: 8px;
    flex: 0 0 8px;
    background: var(--success);
  }

  .toast.error .toast-mark {
    background: var(--danger);
  }

  @media (max-width: 980px) {
    :global(body) {
      height: auto;
      min-height: 100dvh;
      overflow-x: hidden;
      overflow-y: auto;
    }

    .app-layout {
      height: auto;
      min-height: 100dvh;
      overflow: visible;
    }

    .workspace {
      flex-direction: column;
      overflow: visible;
    }

    .main-stage {
      height: auto;
      min-height: 500px;
      overflow: visible;
    }
  }

  @media (max-width: 600px) {
    .main-stage {
      min-height: 440px;
    }

    .toast {
      right: 12px;
      bottom: max(12px, env(safe-area-inset-bottom));
      max-width: calc(100vw - 24px);
    }
  }
</style>
