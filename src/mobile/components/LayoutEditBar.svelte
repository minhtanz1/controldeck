<script lang="ts">
  import { haptic } from "../lib/haptics";
  import type { SavedControlLayout, SaveLayoutResult } from "../lib/types";

  let {
    layouts,
    activeLayoutId,
    canSaveLayout,
    onSelectLayout,
    onSaveLayout,
    onDeleteLayout,
    onReset,
    onSave,
  }: {
    layouts: SavedControlLayout[];
    activeLayoutId: string | null;
    canSaveLayout: boolean;
    onSelectLayout: (layoutId: string) => void;
    onSaveLayout: (name: string) => SaveLayoutResult;
    onDeleteLayout: () => void;
    onReset: () => void;
    onSave: () => void;
  } = $props();

  let isLibraryOpen = $state(false);
  let layoutName = $state("");
  let feedback = $state("");

  function selectLayout(event: Event) {
    const layoutId = (event.currentTarget as HTMLSelectElement).value;
    onSelectLayout(layoutId);
    layoutName = layouts.find((layout) => layout.id === layoutId)?.name ?? "";
    feedback = "";
  }

  function saveLayout() {
    const result: SaveLayoutResult = onSaveLayout(layoutName);
    if (result === "saved") {
      feedback = `Đã lưu "${layoutName.trim()}".`;
      haptic();
    } else if (result === "profile-required") {
      feedback = "Kết nối với PC để lưu layout theo profile.";
    } else if (result === "name-required") {
      feedback = "Nhập tên cho layout.";
    } else {
      feedback = "Đã đủ 5 layout. Hãy chọn một layout để cập nhật hoặc xóa.";
    }
  }

  function deleteLayout() {
    const layout = layouts.find((saved) => saved.id === activeLayoutId);
    if (!layout || !window.confirm(`Xóa layout "${layout.name}"?`)) return;
    onDeleteLayout();
    layoutName = "";
    feedback = "Đã xóa layout.";
    haptic();
  }
</script>

<div class="layout-edit-bar" class:library-open={isLibraryOpen}>
  <div class="layout-edit-header">
    <span class="layout-edit-title">SẮP XẾP ĐIỀU KHIỂN</span>
    <div class="layout-edit-actions">
      <button
        class="layout-bar-btn library-toggle"
        aria-expanded={isLibraryOpen}
        onclick={() => (isLibraryOpen = !isLibraryOpen)}
      >Bố cục {layouts.length}/5</button>
      <button class="layout-bar-btn reset" onclick={() => { haptic(); onReset(); }}>Đặt lại</button>
      <button class="layout-bar-btn done" onclick={() => { haptic(); onSave(); }}>Xong</button>
    </div>
  </div>

  {#if isLibraryOpen}
    <div class="layout-library">
      <div class="layout-library-row">
        <select
          aria-label="Chọn layout đã lưu"
          value={activeLayoutId ?? ""}
          onchange={selectLayout}
        >
          <option value="">Bản nháp hiện tại</option>
          {#each layouts as layout (layout.id)}
            <option value={layout.id}>{layout.name}</option>
          {/each}
        </select>
        {#if activeLayoutId}
          <button class="layout-library-btn delete" onclick={deleteLayout}>Xóa</button>
        {/if}
      </div>
      <div class="layout-library-row">
        <input
          type="text"
          bind:value={layoutName}
          maxlength="24"
          aria-label="Tên layout"
          placeholder={activeLayoutId ? "Tên layout" : "Tên layout mới"}
        />
        <button
          class="layout-library-btn save-layout"
          disabled={!canSaveLayout || (!activeLayoutId && layouts.length >= 5)}
          onclick={saveLayout}
        >{activeLayoutId ? "Cập nhật" : "Lưu mới"}</button>
      </div>
      <p class="layout-library-status" role="status" aria-live="polite">
        {feedback || (canSaveLayout
          ? `${layouts.length}/5 layout đã lưu cho profile này.`
          : "Kết nối với PC để lưu layout theo profile.")}
      </p>
    </div>
  {/if}
</div>

<style>
  .layout-edit-bar {
    position: relative;
    width: calc(100% - 16px);
    max-width: 520px;
    margin-inline: auto;
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    background: var(--surface);
    border: 1px solid var(--accent);
    box-shadow: 0 4px 14px rgba(23, 35, 42, 0.18);
    border-radius: 2px;
    z-index: 50;
    user-select: none;
    -webkit-user-select: none;
  }
  .layout-edit-bar.library-open {
    position: absolute;
    top: 8px;
    left: 8px;
    right: 8px;
    width: auto;
  }
  .layout-edit-header,
  .layout-edit-actions,
  .layout-library-row {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .layout-edit-header {
    justify-content: space-between;
  }
  .layout-edit-title {
    min-width: 0;
    color: var(--accent-strong);
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.04em;
    white-space: nowrap;
  }
  .layout-bar-btn,
  .layout-library-btn {
    min-height: 34px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: 2px;
    font-size: 10px;
    font-weight: 700;
    white-space: nowrap;
    cursor: pointer;
  }
  .layout-bar-btn {
    background: var(--surface-elevated);
    color: var(--text-muted);
  }
  .layout-bar-btn.done,
  .layout-library-btn.save-layout {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }
  .layout-bar-btn.library-toggle[aria-expanded="true"] {
    border-color: var(--accent);
    color: var(--accent-strong);
  }
  .layout-library {
    display: flex;
    flex-direction: column;
    gap: 5px;
    padding-top: 6px;
    border-top: 1px solid var(--border);
  }
  .layout-library-row select,
  .layout-library-row input {
    min-width: 0;
    min-height: 36px;
    flex: 1;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: 2px;
    background: var(--surface);
    color: var(--text);
    font: inherit;
    font-size: 11px;
  }
  .layout-library-row input:focus-visible,
  .layout-library-row select:focus-visible,
  .layout-bar-btn:focus-visible,
  .layout-library-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
  }
  .layout-library-btn.delete {
    background: var(--surface-elevated);
    color: var(--danger);
  }
  .layout-library-btn:disabled {
    cursor: not-allowed;
    opacity: 0.55;
  }
  .layout-library-status {
    min-height: 14px;
    color: var(--text-muted);
    font-size: 10px;
    line-height: 1.4;
  }
</style>
