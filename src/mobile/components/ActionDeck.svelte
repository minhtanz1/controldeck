<script lang="ts">
  import { wsClient } from "../lib/ws.svelte";
  import { haptic } from "../lib/haptics";
  import type { DeckItem } from "../lib/types";

  let textInputValue = $state("");

  function onSwitchProfile(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    if (val) {
      wsClient.switchProfile(val);
    }
  }

  function sendText() {
    const trimmed = textInputValue.trim();
    if (trimmed) {
      haptic();
      wsClient.sendTextInput(trimmed);
      textInputValue = "";
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      sendText();
    }
  }

  function triggerAction(item: DeckItem) {
    haptic(20);
    wsClient.sendDeckAction(item.action);
  }

  // Derive grouped categories from deck items
  let groupedCategories = $derived.by(() => {
    const groups: Record<string, DeckItem[]> = {};
    for (const item of wsClient.deck) {
      const cat = item.category || "General";
      if (!groups[cat]) groups[cat] = [];
      groups[cat].push(item);
    }
    return groups;
  });
</script>

<div class="deck-stage">
  <!-- Active Profile Selector Bar -->
  <div class="deck-profile-bar">
    <div class="deck-profile-meta">
      <span class="deck-profile-tag">ACTIVE PROFILE</span>
      <select class="profile-select" value={wsClient.activeProfileId} onchange={onSwitchProfile}>
        {#if wsClient.profiles.length === 0}
          <option value={wsClient.activeProfileId}>
            {wsClient.activeProfileName || wsClient.activeProfileId || "Đang tải profile..."}
          </option>
        {:else}
          {#each wsClient.profiles as p (p.id)}
            <option value={p.id}>{p.name}</option>
          {/each}
        {/if}
      </select>
    </div>
    <button class="deck-refresh-btn" onclick={() => { haptic(); wsClient.requestProfileSync(); }} title="Đồng bộ Profile">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2" />
      </svg>
    </button>
  </div>

  <!-- Quick Text Input Row -->
  <div class="text-input-row">
    <input
      type="text"
      class="typing-input"
      placeholder="Gõ văn bản gửi trực tiếp tới máy tính..."
      bind:value={textInputValue}
      onkeydown={handleKeydown}
    />
    <button class="send-text-btn" onclick={sendText}>Gửi</button>
  </div>

  <!-- Dynamic Deck Container -->
  <div class="dynamic-deck-container">
    {#if wsClient.deck.length === 0}
      <div class="deck-empty-state">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="3" y="4" width="18" height="16" rx="2" />
          <path d="M7 8h.01M12 8h.01M17 8h.01M7 12h.01M12 12h.01M17 12h.01M7 16h10" />
        </svg>
        <div class="empty-title">Chưa có Action Deck cho Profile này</div>
        <div class="empty-desc">
          Profile hiện tại chưa thiết lập phím tắt riêng. Hãy gán phím hoặc tạo macro trên máy tính để tự động hiện ở đây.
        </div>
      </div>
    {:else}
      {#each Object.entries(groupedCategories) as [categoryName, items] (categoryName)}
        <div class="deck-category-block">
          <div class="deck-section-title">{categoryName}</div>
          <div class="macro-grid">
            {#each items as item (item.id)}
              <button
                class="macro-btn"
                style:border-color={item.color || undefined}
                onclick={() => triggerAction(item)}
              >
                <span class="macro-name">{item.label}</span>
                {#if item.shortcut}
                  <span class="macro-key" style:color={item.color || undefined}>{item.shortcut}</span>
                {/if}
              </button>
            {/each}
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .deck-stage {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 12px;
    gap: 12px;
    zoom: var(--control-zoom, 1);
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    touch-action: pan-y !important;
  }
  .deck-profile-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 8px 12px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    flex-shrink: 0;
  }
  .deck-profile-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
  }
  .deck-profile-tag {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--accent-strong);
    text-transform: uppercase;
  }
  .profile-select {
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    color: var(--text);
    font-size: 13px;
    font-weight: 700;
    padding: 6px 10px;
    border-radius: 6px;
    outline: none;
    width: 100%;
    cursor: pointer;
  }
  .profile-select:focus {
    border-color: var(--accent-strong);
  }
  .deck-refresh-btn {
    width: 36px;
    height: 36px;
    border-radius: 6px;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    flex-shrink: 0;
  }
  .deck-refresh-btn:active {
    color: var(--accent-strong);
    border-color: var(--accent-strong);
  }
  .deck-refresh-btn svg {
    width: 16px;
    height: 16px;
  }
  .text-input-row {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }
  .typing-input {
    flex: 1;
    height: 42px;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 0 12px;
    border-radius: 6px;
    font-size: 14px;
    outline: none;
  }
  .typing-input:focus {
    border-color: var(--accent-strong);
  }
  .send-text-btn {
    padding: 0 16px;
    background: var(--accent);
    color: #fff;
    font-weight: 700;
    border: none;
    border-radius: 6px;
    font-size: 13px;
    cursor: pointer;
  }
  .dynamic-deck-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding-bottom: 24px;
  }
  .deck-category-block {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .deck-section-title {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    margin-top: 4px;
    padding-left: 2px;
  }
  .macro-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }
  .macro-btn {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 12px;
    text-align: left;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    gap: 6px;
    transition: transform 0.08s ease, border-color 0.15s ease, background 0.15s ease;
    touch-action: manipulation;
  }
  .macro-btn:active {
    transform: scale(0.97);
    border-color: var(--accent-strong);
    background: var(--surface-elevated);
  }

  .macro-btn:focus-visible,
  .deck-refresh-btn:focus-visible,
  .send-text-btn:focus-visible,
  .profile-select:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
  .macro-name {
    font-size: 13px;
    font-weight: 700;
    color: var(--text);
    line-height: 1.25;
    word-break: break-word;
  }
  .macro-key {
    font-size: 11px;
    color: var(--accent-strong);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-weight: 600;
    line-height: 1.2;
  }
  .deck-empty-state {
    padding: 36px 16px;
    text-align: center;
    background: var(--surface);
    border: 1px dashed var(--border);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    margin-top: 10px;
  }
  .deck-empty-state svg {
    width: 36px;
    height: 36px;
    stroke: var(--text-muted);
  }
  .deck-empty-state .empty-title {
    font-size: 14px;
    font-weight: 700;
    color: var(--text);
  }
  .deck-empty-state .empty-desc {
    font-size: 12px;
    color: var(--text-muted);
    max-width: 320px;
    line-height: 1.4;
  }
</style>
