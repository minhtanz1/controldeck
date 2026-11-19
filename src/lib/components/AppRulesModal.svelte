<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { ActiveWindowInfo, AppRule, OpenWindowInfo, Profile } from "$lib/types";

  let {
    profile,
    activeWindow = null,
    onClose,
    onSave,
  }: {
    profile: Profile;
    activeWindow: ActiveWindowInfo | null;
    onClose: () => void;
    onSave: (rules: AppRule[]) => void;
  } = $props();

  let rules = $state<AppRule[]>(JSON.parse(JSON.stringify(profile.app_rules || [])));
  let openWindows = $state<OpenWindowInfo[]>([]);
  let searchQuery = $state("");
  let activeTab = $state<"picker" | "rules">("picker");
  let isLoadingWindows = $state(false);

  onMount(async () => {
    await refreshOpenWindows();
  });

  async function refreshOpenWindows() {
    isLoadingWindows = true;
    try {
      openWindows = await invoke<OpenWindowInfo[]>("get_open_windows");
    } catch (e) {
      console.error("Failed to get open windows:", e);
    } finally {
      isLoadingWindows = false;
    }
  }

  let filteredWindows = $derived(
    openWindows.filter((w) => {
      if (!searchQuery.trim()) return true;
      const q = searchQuery.toLowerCase();
      return (
        w.clean_title.toLowerCase().includes(q) ||
        w.title.toLowerCase().includes(q) ||
        w.process_name.toLowerCase().includes(q)
      );
    })
  );

  let browserTabs = $derived(filteredWindows.filter((w) => w.is_browser));
  let nativeApps = $derived(filteredWindows.filter((w) => !w.is_browser));

  function isRuleAdded(pattern: string): boolean {
    const pLower = pattern.toLowerCase();
    return rules.some((r) => r.pattern.toLowerCase() === pLower);
  }

  function addWindowRule(pattern: string, matchType: "window_title" | "process_name") {
    if (isRuleAdded(pattern)) return;
    rules.push({
      match_type: matchType,
      pattern,
      case_sensitive: false,
    });
  }

  function addEmptyRule() {
    rules.push({
      match_type: "window_title",
      pattern: "",
      case_sensitive: false,
    });
    activeTab = "rules";
  }

  function removeRule(index: number) {
    rules.splice(index, 1);
  }

  function saveAndClose() {
    onSave(rules.filter((r) => r.pattern.trim().length > 0));
    onClose();
  }
</script>

<div class="modal-backdrop" onclick={onClose} role="presentation">
  <div class="modal-window" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1">
    <div class="modal-header">
      <div class="header-text">
        <h2 class="modal-title">Auto-Switch App Rules</h2>
        <span class="modal-subtitle">Profile: <strong>{profile.name}</strong></span>
      </div>
      <button type="button" class="close-btn" onclick={onClose} aria-label="Close">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18 6 6 18M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Tab navigation -->
    <div class="modal-tabs">
      <button
        type="button"
        class="tab-btn {activeTab === 'picker' ? 'active' : ''}"
        onclick={() => (activeTab = "picker")}
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" />
          <path d="m10 15 5-3-5-3v6Z" />
        </svg>
        <span>Open Apps & Browser Tabs ({openWindows.length})</span>
      </button>
      <button
        type="button"
        class="tab-btn {activeTab === 'rules' ? 'active' : ''}"
        onclick={() => (activeTab = "rules")}
      >
        <span>Active Rules ({rules.length})</span>
      </button>
    </div>

    <div class="modal-body">
      {#if activeTab === "picker"}
        <!-- Search & Refresh bar -->
        <div class="search-bar">
          <div class="search-input-wrapper">
            <svg class="search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="8" />
              <path d="m21 21-4.3-4.3" />
            </svg>
            <input
              type="text"
              class="search-input"
              placeholder="Search open tabs, web pages or apps..."
              bind:value={searchQuery}
            />
          </div>
          <button type="button" class="refresh-btn" onclick={refreshOpenWindows} title="Refresh open windows list">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class={isLoadingWindows ? 'spin' : ''}>
              <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
              <path d="M21 3v5h-5" />
              <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" />
              <path d="M3 21v-5h5" />
            </svg>
          </button>
        </div>

        <div class="windows-container">
          <!-- Browser Tabs Section -->
          {#if browserTabs.length > 0}
            <div class="window-category">
              <span class="category-title">Browser Web Tabs</span>
              <div class="window-cards-list">
                {#each browserTabs as w}
                  {@const added = isRuleAdded(w.clean_title)}
                  <div class="window-card {added ? 'already-added' : ''}">
                    <div class="window-info">
                      <div class="window-title-row">
                        <span class="clean-page-title">{w.clean_title}</span>
                        <span class="browser-badge">{w.process_name}</span>
                      </div>
                      <span class="full-title" title={w.title}>{w.title}</span>
                    </div>
                    <button
                      type="button"
                      class="add-win-btn {added ? 'added' : ''}"
                      onclick={() => addWindowRule(w.clean_title, "window_title")}
                      disabled={added}
                    >
                      {added ? "Linked" : "+ Link Tab"}
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          {/if}

          <!-- Native Applications Section -->
          {#if nativeApps.length > 0}
            <div class="window-category">
              <span class="category-title">Desktop Applications</span>
              <div class="window-cards-list">
                {#each nativeApps as w}
                  {@const added = isRuleAdded(w.title)}
                  <div class="window-card {added ? 'already-added' : ''}">
                    <div class="window-info">
                      <div class="window-title-row">
                        <span class="clean-page-title">{w.clean_title || w.title}</span>
                        <span class="app-badge">{w.process_name}</span>
                      </div>
                      <span class="full-title">{w.title}</span>
                    </div>
                    <button
                      type="button"
                      class="add-win-btn {added ? 'added' : ''}"
                      onclick={() => addWindowRule(w.title, "window_title")}
                      disabled={added}
                    >
                      {added ? "Linked" : "+ Link App"}
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          {/if}

          {#if filteredWindows.length === 0}
            <div class="empty-state">
              No matching open windows or web tabs found.
            </div>
          {/if}
        </div>
      {:else}
        <!-- Rules Manager Tab -->
        <p class="guide-text">
          ControlDeck auto-activates <strong>{profile.name}</strong> whenever focus switches to any window matching these rules:
        </p>

        <div class="rules-list">
          {#each rules as r, i}
            <div class="rule-row">
              <select bind:value={r.match_type} class="type-select">
                <option value="window_title">Window Title Contains</option>
                <option value="process_name">Process Name Equals</option>
              </select>
              <input
                type="text"
                class="pattern-input"
                placeholder={r.match_type === 'window_title' ? 'e.g. Waze Map Editor' : 'e.g. chrome.exe'}
                bind:value={r.pattern}
              />
              <button type="button" class="del-rule-btn" onclick={() => removeRule(i)} title="Remove rule">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                </svg>
              </button>
            </div>
          {/each}

          {#if rules.length === 0}
            <div class="empty-state">
              No auto-switch rules yet. Pick tabs from the "Open Apps & Browser Tabs" tab or click "+ Add Manual Rule" below.
            </div>
          {/if}
        </div>

        <button type="button" class="add-rule-btn" onclick={addEmptyRule}>
          + Add Manual Rule
        </button>
      {/if}
    </div>

    <div class="modal-footer">
      <span class="active-rule-summary">{rules.length} rule{rules.length === 1 ? '' : 's'} assigned</span>
      <div class="footer-actions">
        <button type="button" class="cancel-btn" onclick={onClose}>Cancel</button>
        <button type="button" class="save-btn" onclick={saveAndClose}>Apply Rules</button>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    padding: 8px;
    background: rgba(23, 35, 42, 0.36);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-window {
    background: var(--bg-surface);
    border: 1px solid var(--border-strong);
    border-radius: 2px;
    width: min(620px, calc(100vw - 16px));
    max-width: 620px;
    box-shadow: 0 16px 40px rgba(23, 35, 42, 0.2);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 0;
    flex: 1;
    max-height: calc(100dvh - 16px);
    min-height: 0;
  }

  .modal-header {
    padding: 16px 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--border);
  }

  .modal-title {
    margin: 0;
    font-size: 16px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .modal-subtitle {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .modal-subtitle strong {
    color: var(--accent-strong);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .modal-tabs {
    display: flex;
    background: var(--bg-muted);
    border-bottom: 1px solid var(--border);
    padding: 0 16px;
    gap: 8px;
    min-width: 0;
  }

  .tab-btn {
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 600;
    padding: 10px 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    color: var(--text-primary);
  }

  .tab-btn.active {
    color: var(--accent-strong);
    border-bottom-color: var(--accent-strong);
  }

  .modal-body {
    min-height: 0;
    padding: 16px 20px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    overflow-y: auto;
    flex: 1;
  }

  .search-bar {
    display: flex;
    gap: 8px;
  }

  .search-input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 0 10px;
    gap: 8px;
  }

  .search-icon {
    color: var(--text-muted);
  }

  .search-input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 13px;
    padding: 8px 0;
    outline: none;
  }

  .refresh-btn {
    background: var(--bg-inset);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    border-radius: 6px;
    padding: 0 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
  }

  .refresh-btn:hover {
    color: var(--accent-strong);
    border-color: var(--accent-strong);
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .windows-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .window-category {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .category-title {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .window-cards-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .window-card {
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 2px;
    padding: 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    transition: all 0.12s ease;
  }

  .window-card:hover {
    background: var(--bg-muted);
    border-color: var(--border-strong);
  }

  .window-card.already-added {
    opacity: 0.75;
    border-color: var(--success-border);
  }

  .window-info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    overflow: hidden;
  }

  .window-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .clean-page-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .browser-badge {
    background: var(--accent-muted);
    color: var(--accent-strong);
    font-size: 10px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 4px;
  }

  .app-badge {
    background: var(--border);
    color: var(--text-secondary);
    font-size: 10px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 4px;
  }

  .full-title {
    font-size: 11px;
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .add-win-btn {
    background: var(--accent-muted);
    color: var(--accent-strong);
    border: 1px solid var(--accent);
    border-radius: 6px;
    padding: 6px 12px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.12s ease;
  }

  .add-win-btn:hover:not(:disabled) {
    background: var(--accent-muted);
  }

  .add-win-btn.added {
    background: var(--success-muted);
    color: var(--success);
    border-color: var(--success-border);
    cursor: default;
  }

  .guide-text {
    margin: 0;
    font-size: 12px;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .guide-text strong {
    color: var(--accent-strong);
  }

  .rules-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .rule-row {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .type-select {
    min-width: 0;
    max-width: 44%;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    border-radius: 6px;
    padding: 7px 10px;
    font-size: 12px;
    font-weight: 500;
  }

  .pattern-input {
    min-width: 0;
    flex: 1;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    color: var(--text-primary);
    border-radius: 6px;
    padding: 7px 12px;
    font-size: 12px;
  }

  .pattern-input:focus {
    outline: none;
    border-color: var(--accent-strong);
  }

  .del-rule-btn {
    background: transparent;
    border: 1px solid var(--danger-muted);
    color: var(--danger);
    border-radius: 6px;
    padding: 7px;
    cursor: pointer;
  }

  .del-rule-btn:hover {
    background: var(--danger-muted);
    border-color: var(--danger);
  }

  .empty-state {
    text-align: center;
    padding: 24px;
    background: var(--bg-muted);
    border: 1px dashed var(--border);
    border-radius: 8px;
    font-size: 12px;
    color: var(--text-muted);
  }

  .add-rule-btn {
    background: var(--bg-muted);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    border-radius: 6px;
    padding: 8px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .add-rule-btn:hover {
    color: var(--accent-strong);
    border-color: var(--accent-strong);
  }

  .modal-footer {
    padding: 14px 20px;
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .active-rule-summary {
    font-size: 12px;
    color: var(--text-muted);
  }

  .footer-actions {
    display: flex;
    gap: 10px;
  }

  .cancel-btn {
    background: var(--bg-surface);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    border-radius: 6px;
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
  }

  .save-btn {
    background: var(--accent);
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 6px 16px;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
  }

  .save-btn:hover {
    background: var(--accent-strong);
  }

  .modal-window button:focus-visible,
  .modal-window input:focus-visible,
  .modal-window select:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  @media (max-width: 520px) {
    .modal-header {
      padding: 12px;
      gap: 8px;
    }

    .modal-tabs {
      flex-direction: column;
      gap: 0;
      padding: 0 8px;
    }

    .tab-btn {
      min-width: 0;
      padding: 8px;
      border-bottom: 0;
      border-left: 2px solid transparent;
      text-align: left;
    }

    .tab-btn.active {
      border-left-color: var(--accent-strong);
    }

    .tab-btn span {
      min-width: 0;
      overflow-wrap: anywhere;
    }

    .modal-body {
      padding: 10px;
      gap: 10px;
    }

    .window-card {
      align-items: flex-start;
      flex-wrap: wrap;
    }

    .add-win-btn {
      margin-left: auto;
    }

    .rule-row {
      flex-wrap: wrap;
    }

    .type-select {
      flex: 1 1 100%;
      max-width: none;
    }

    .pattern-input {
      flex: 1 1 calc(100% - 44px);
    }

    .modal-footer {
      flex-wrap: wrap;
      gap: 8px;
      padding: 10px 12px;
    }

    .active-rule-summary {
      flex: 1 1 100%;
    }

    .footer-actions {
      margin-left: auto;
    }
  }
</style>
