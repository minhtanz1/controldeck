<script lang="ts">
  import { tick } from "svelte";
  import type { Profile } from "$lib/types";

  let {
    profiles = [],
    activeProfile = null,
    onSelectProfile,
    onCreateProfile,
    onOpenRules,
    onExportProfile,
    onImportProfile,
    onDeleteProfile,
    onRenameProfile,
  }: {
    profiles: Profile[];
    activeProfile: Profile | null;
    onSelectProfile: (id: string) => void;
    onCreateProfile: () => void;
    onOpenRules: (profile: Profile) => void;
    onExportProfile?: (id: string) => void;
    onImportProfile?: (content: string) => void;
    onDeleteProfile?: (id: string) => void;
    onRenameProfile: (id: string, name: string) => Promise<boolean>;
  } = $props();

  let fileInput: HTMLInputElement;
  let renameInput = $state<HTMLInputElement | null>(null);
  let editingProfileId = $state<string | null>(null);
  let renameDraft = $state("");
  let renameError = $state("");
  let isSavingRename = $state(false);

  async function beginRename(event: MouseEvent, profile: Profile) {
    event.stopPropagation();
    editingProfileId = profile.id;
    renameDraft = profile.name;
    renameError = "";
    await tick();
    renameInput?.focus();
    renameInput?.select();
  }

  function cancelRename(event: MouseEvent | KeyboardEvent) {
    event.stopPropagation();
    if (isSavingRename) return;
    editingProfileId = null;
    renameDraft = "";
    renameError = "";
  }

  function handleRenameKeydown(event: KeyboardEvent, profile: Profile) {
    event.stopPropagation();
    if (event.key === "Enter") {
      event.preventDefault();
      void saveRename(profile);
    } else if (event.key === "Escape") {
      cancelRename(event);
    }
  }

  async function saveRename(profile: Profile, event?: MouseEvent) {
    event?.stopPropagation();
    const name = renameDraft.trim();
    if (!name) {
      renameError = "Profile name cannot be empty.";
      renameInput?.focus();
      return;
    }

    isSavingRename = true;
    renameError = "";
    try {
      if (await onRenameProfile(profile.id, name)) {
        editingProfileId = null;
        renameDraft = "";
      } else {
        renameError = "Rename failed. Try again.";
      }
    } finally {
      isSavingRename = false;
    }
  }

  function triggerImport() {
    fileInput.click();
  }

  function handleFileSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      const file = input.files[0];
      const reader = new FileReader();
      reader.onload = (event) => {
        const content = event.target?.result as string;
        if (content && onImportProfile) {
          onImportProfile(content);
        }
      };
      reader.readAsText(file);
    }
  }
</script>

<aside class="sidebar">
  <input
    type="file"
    accept=".json"
    style="display: none;"
    bind:this={fileInput}
    onchange={handleFileSelected}
  />

  <div class="sidebar-header">
    <div class="header-title-row">
      <span class="section-title">Profiles</span>
      <span class="profile-count">{profiles.length}</span>
    </div>
    <div class="header-actions">
      <button type="button" class="action-icon-btn" onclick={triggerImport} title="Import Profile JSON">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5-5 5 5M12 5v12" />
        </svg>
      </button>
      <button type="button" class="add-profile-btn" onclick={onCreateProfile} title="Create New Profile">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path d="M12 5v14M5 12h14" />
        </svg>
        <span>New</span>
      </button>
    </div>
  </div>

  <div class="profile-scroll-list">
    {#each profiles as p}
      {@const isActive = activeProfile?.id === p.id}
      {@const btnCount = Object.keys(p.mappings?.buttons || {}).length}
      {@const ruleCount = p.app_rules?.length || 0}
      {@const isEditing = editingProfileId === p.id}

      <div
        class="profile-item {isActive ? 'active' : ''}"
        onclick={() => onSelectProfile(p.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) => { if (e.key === 'Enter') onSelectProfile(p.id); }}
      >
        <div class="item-main">
          <div class="item-header">
            {#if isEditing}
              <input
                bind:this={renameInput}
                class="item-name-input"
                aria-label={`Rename ${p.name}`}
                aria-invalid={renameError.length > 0}
                value={renameDraft}
                oninput={(event) => (renameDraft = event.currentTarget.value)}
                onclick={(event) => event.stopPropagation()}
                onkeydown={(event) => handleRenameKeydown(event, p)}
                disabled={isSavingRename}
              />
            {:else}
              <span class="item-name">{p.name}</span>
            {/if}
            <div class="item-badges">
              {#if isActive}
                <span class="active-badge">Active</span>
              {/if}
              {#if isEditing}
                <button
                  type="button"
                  class="item-rename-submit"
                  onclick={(event) => saveRename(p, event)}
                  disabled={isSavingRename || !renameDraft.trim()}
                  aria-label="Save profile name"
                  title="Save profile name"
                >
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="m5 12 4 4L19 6" />
                  </svg>
                </button>
                <button
                  type="button"
                  class="item-rename-cancel"
                  onclick={cancelRename}
                  disabled={isSavingRename}
                  aria-label="Cancel renaming"
                  title="Cancel"
                >
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M18 6 6 18M6 6l12 12" />
                  </svg>
                </button>
              {:else}
                <button
                  type="button"
                  class="item-rename-btn"
                  onclick={(event) => beginRename(event, p)}
                  aria-label={`Rename ${p.name}`}
                  title="Rename profile"
                >
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M12 20h9" />
                    <path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L8 18l-4 1 1-4Z" />
                  </svg>
                </button>
              {/if}
              {#if !isEditing && onExportProfile}
                <button
                  type="button"
                  class="item-export-btn"
                  onclick={(e) => { e.stopPropagation(); onExportProfile(p.id); }}
                  title="Export profile JSON"
                >
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3" />
                  </svg>
                </button>
              {/if}
              {#if !isEditing && onDeleteProfile && p.id !== "default"}
                <button
                  type="button"
                  class="item-del-btn"
                  onclick={(e) => { e.stopPropagation(); onDeleteProfile(p.id); }}
                  title="Delete profile"
                >
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
                  </svg>
                </button>
              {/if}
            </div>
          </div>
          {#if isEditing && renameError}
            <span class="rename-error" role="alert">{renameError}</span>
          {/if}
          <p class="item-desc">{p.description || "No description"}</p>
        </div>

        <div class="item-meta">
          <span class="meta-tag">{btnCount} key{btnCount === 1 ? '' : 's'}</span>
          {#if ruleCount > 0}
            <button
              type="button"
              class="rules-tag-btn"
              onclick={(e) => { e.stopPropagation(); onOpenRules(p); }}
              title="Configure auto-switch app rules"
            >
              {ruleCount} auto {ruleCount === 1 ? "rule" : "rules"}
            </button>
          {:else}
            <button
              type="button"
              class="add-rules-btn"
              onclick={(e) => { e.stopPropagation(); onOpenRules(p); }}
              title="Add auto-switch rule"
            >
              + Link App
            </button>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</aside>

<style>
  .sidebar {
    width: 232px;
    min-width: 0;
    flex: 0 0 232px;
    height: 100%;
    background-color: var(--bg-surface);
    border-right: 1px solid var(--border-strong);
    display: flex;
    flex-direction: column;
    user-select: none;
  }

  .sidebar-header {
    padding: 9px 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    border-bottom: 1px solid var(--border);
  }

  .header-title-row {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
  }

  .section-title {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: 0;
    white-space: nowrap;
  }

  .profile-count {
    font-size: 10px;
    font-weight: 600;
    background: var(--bg-muted);
    color: var(--text-secondary);
    padding: 2px 5px;
    border-radius: 2px;
    font-variant-numeric: tabular-nums;
  }

  .header-actions {
    display: flex;
    gap: 5px;
    flex-shrink: 0;
  }

  .action-icon-btn {
    background: var(--bg-surface);
    color: var(--text-secondary);
    border: 1px solid var(--border-strong);
    border-radius: 2px;
    width: 28px;
    height: 28px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .action-icon-btn:hover,
  .action-icon-btn:focus-visible {
    color: var(--accent-strong);
    border-color: var(--accent-strong);
  }

  .add-profile-btn {
    background: var(--accent);
    color: #fff;
    border: 1px solid var(--accent-strong);
    border-radius: 2px;
    padding: 4px 7px;
    font-size: 10px;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 3px;
    transition: background-color 120ms ease;
  }

  .add-profile-btn:hover {
    background: var(--accent-strong);
  }

  .profile-scroll-list {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 3px 8px 8px;
    display: flex;
    flex-direction: column;
  }

  .profile-item {
    min-width: 0;
    background: transparent;
    border: 0;
    border-bottom: 1px solid var(--border);
    padding: 8px 5px;
    cursor: pointer;
    transition: all 0.12s ease;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .profile-item:hover {
    background: var(--bg-inset);
  }

  .profile-item.active {
    background: var(--accent-muted);
  }

  .item-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 4px;
    min-width: 0;
  }

  .item-name {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .item-name-input {
    flex: 1;
    min-width: 0;
    padding: 3px 5px;
    border: 1px solid var(--accent);
    border-radius: 2px;
    background: var(--bg-surface);
    color: var(--text-primary);
    font: inherit;
    font-size: 11px;
  }

  .rename-error {
    display: block;
    margin-top: 4px;
    color: var(--danger);
    font-size: 9px;
  }

  .item-badges {
    display: flex;
    align-items: center;
    gap: 3px;
    flex-shrink: 0;
  }

  .active-badge {
    font-size: 9px;
    font-weight: 700;
    color: var(--accent-strong);
    background: var(--bg-surface);
    padding: 2px 4px;
    border-radius: 2px;
  }

  .item-export-btn,
  .item-del-btn,
  .item-rename-btn,
  .item-rename-submit,
  .item-rename-cancel {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 2px;
  }

  .item-export-btn:hover {
    color: var(--accent-strong);
    background: var(--accent-muted);
  }

  .item-rename-btn:hover,
  .item-rename-submit:hover:not(:disabled) {
    color: var(--accent-strong);
    background: var(--accent-muted);
  }

  .item-rename-submit {
    color: var(--success);
  }

  .item-rename-cancel:hover:not(:disabled) {
    color: var(--danger);
    background: var(--danger-muted);
  }

  .item-del-btn:hover {
    color: var(--danger);
    background: var(--danger-muted);
  }

  .item-desc {
    margin: 4px 0 0 0;
    font-size: 10px;
    color: var(--text-muted);
    line-height: 1.3;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .item-meta {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
    padding-top: 5px;
    border-top: 1px solid var(--border);
  }

  .meta-tag {
    font-size: 10px;
    color: var(--text-secondary);
    font-weight: 500;
    white-space: nowrap;
  }

  .rules-tag-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 10px;
    font-weight: 500;
    cursor: pointer;
    padding: 0;
    text-decoration: underline;
    text-underline-offset: 2px;
    text-align: right;
  }

  .rules-tag-btn:hover {
    color: var(--accent-strong);
  }

  .add-rules-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 10px;
    cursor: pointer;
    padding: 0;
  }

  .add-rules-btn:hover {
    color: var(--accent-strong);
  }

  .profile-item:focus-visible,
  .item-name-input:focus-visible,
  .item-export-btn:focus-visible,
  .item-del-btn:focus-visible,
  .item-rename-btn:focus-visible,
  .item-rename-submit:focus-visible,
  .item-rename-cancel:focus-visible,
  .rules-tag-btn:focus-visible,
  .add-rules-btn:focus-visible,
  .add-profile-btn:focus-visible,
  .action-icon-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  @media (max-width: 980px) {
    .sidebar {
      width: 100%;
      flex: 0 0 auto;
      height: auto;
      border-right: 0;
      border-bottom: 1px solid var(--border-strong);
    }

    .profile-scroll-list {
      display: grid;
      grid-auto-flow: column;
      grid-auto-columns: minmax(210px, 280px);
      overflow-x: auto;
      overflow-y: hidden;
      padding: 0 8px 5px;
      flex: 0 0 auto;
    }

    .profile-item {
      padding: 7px 8px;
      border-bottom: 0;
      border-right: 1px solid var(--border);
    }
  }

  @media (max-width: 520px) {
    .sidebar-header {
      padding: 7px 8px;
    }

    .profile-scroll-list {
      grid-auto-columns: minmax(240px, 82vw);
    }
  }
</style>
