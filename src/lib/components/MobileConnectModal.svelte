<script lang="ts">
  import { onMount } from "svelte";
  import QRCode from "qrcode";
  import type { MobileServerInfo } from "$lib/types";

  let {
    mobileInfo = null,
    onClose,
  }: {
    mobileInfo: MobileServerInfo | null;
    onClose: () => void;
  } = $props();

  let canvasEl = $state<HTMLCanvasElement | null>(null);
  let modalEl = $state<HTMLDivElement | null>(null);
  let copied = $state(false);
  let activeTab = $state<"wifi" | "usb">("wifi");

  onMount(() => {
    const previousFocus = document.activeElement;
    modalEl?.focus();
    return () => {
      if (previousFocus instanceof HTMLElement) previousFocus.focus();
    };
  });

  $effect(() => {
    if (canvasEl && mobileInfo?.url) {
      QRCode.toCanvas(canvasEl, mobileInfo.url, {
        width: 200,
        margin: 2,
        color: {
          dark: "#000000",
          light: "#ffffff",
        },
      });
    }
  });

  function copyUrl() {
    if (!mobileInfo?.url) return;
    navigator.clipboard.writeText(mobileInfo.url);
    copied = true;
    setTimeout(() => {
      copied = false;
    }, 2000);
  }
</script>

<div
  class="modal-backdrop"
  onclick={onClose}
  onkeydown={(event) => event.key === "Escape" && onClose()}
  role="presentation"
>
  <div
    class="modal-window"
    bind:this={modalEl}
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    aria-labelledby="connect-modal-title"
    aria-describedby="connect-modal-subtitle"
    tabindex="-1"
  >
    <div class="modal-header">
      <div class="header-text">
        <div class="title-row">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" aria-hidden="true">
            <rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
            <path d="M12 18h.01" />
          </svg>
          <h2 class="modal-title" id="connect-modal-title">Connect PocketPad</h2>
        </div>
        <span class="modal-subtitle" id="connect-modal-subtitle">Use your phone as a controller. No app install required.</span>
      </div>
      <button type="button" class="close-btn" onclick={onClose} aria-label="Close">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18 6 6 18M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Mode Tabs -->
    <div class="connect-tabs">
      <button
        type="button"
        class="tab-btn {activeTab === 'wifi' ? 'active' : ''}"
        aria-pressed={activeTab === "wifi"}
        onclick={() => (activeTab = "wifi")}
      >
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <path d="M5 12.55a11 11 0 0 1 14.08 0" />
          <path d="M1.42 9a16 16 0 0 1 21.16 0" />
          <path d="M8.53 16.11a6 6 0 0 1 6.95 0" />
          <line x1="12" y1="20" x2="12.01" y2="20" />
        </svg>
        <span>Wi-Fi</span>
      </button>
      <button
        type="button"
        class="tab-btn {activeTab === 'usb' ? 'active' : ''}"
        aria-pressed={activeTab === "usb"}
        onclick={() => (activeTab = "usb")}
      >
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <circle cx="12" cy="12" r="10" />
          <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83" />
        </svg>
        <span>USB / ADB</span>
      </button>
    </div>

    <div class="modal-body">
      {#if activeTab === "wifi"}
        <div class="qr-section">
          <div class="qr-card">
            {#if mobileInfo?.url}
              <canvas bind:this={canvasEl} class="qr-canvas" aria-label="PocketPad connection QR code"></canvas>
            {:else}
              <div class="qr-placeholder" role="status" aria-live="polite">
                <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" aria-hidden="true">
                  <path d="M5 12.55a11 11 0 0 1 14.08 0" />
                  <path d="M1.42 9a16 16 0 0 1 21.16 0" />
                  <path d="M8.53 16.11a6 6 0 0 1 6.95 0" />
                  <line x1="12" y1="20" x2="12.01" y2="20" />
                </svg>
                <span>Address not ready</span>
              </div>
            {/if}
            <span class="qr-subtext">{mobileInfo?.url ? "Scan with your phone camera" : "Waiting for local address"}</span>
          </div>

          <div class="instruction-list">
            <div class="step-card">
              <span class="step-num">1</span>
              <div class="step-info">
                <span class="step-title">Join the same Wi-Fi</span>
                <span class="step-desc">Connect your phone and PC to the same local network.</span>
              </div>
            </div>

            <div class="step-card">
              <span class="step-num">2</span>
              <div class="step-info">
                <span class="step-title">Scan or open the link</span>
                <span class="step-desc">Scan the code, or copy this address into your phone's browser.</span>
                <div class="url-copy-box">
                  <div class="url-details">
                    <span class="url-label">Controller address</span>
                    <span class="url-text">{mobileInfo?.url || "Detecting local address..."}</span>
                  </div>
                  <button
                    type="button"
                    class="copy-btn"
                    onclick={copyUrl}
                    disabled={!mobileInfo?.url}
                    aria-label={copied ? "Address copied" : "Copy controller address"}
                  >
                    {copied ? "Copied" : "Copy link"}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      {:else}
        <div class="usb-section">
          <p class="usb-desc">Use Android USB tethering when Wi-Fi is unavailable.</p>
          <div class="step-card">
            <span class="step-num">1</span>
            <div class="step-info">
              <span class="step-title">Run this command on your PC</span>
              <div class="code-box">
                <code>adb forward tcp:4820 tcp:4820</code>
              </div>
            </div>
          </div>
          <div class="step-card">
            <span class="step-num">2</span>
            <div class="step-info">
              <span class="step-title">Open this address on your phone</span>
              <div class="url-copy-box">
                <span class="url-text">http://localhost:4820</span>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <div class="modal-footer">
      <div class="status-summary" role="status" aria-live="polite">
        <span class="live-dot {mobileInfo && mobileInfo.connected_clients > 0 ? 'connected' : ''}" aria-hidden="true"></span>
        <span>
          {#if mobileInfo && mobileInfo.connected_clients > 0}
            <strong>{mobileInfo.connected_clients}</strong> phone{mobileInfo.connected_clients === 1 ? "" : "s"} connected
          {:else}
            Waiting for a phone to connect
          {/if}
        </span>
      </div>
      <button type="button" class="done-btn" onclick={onClose}>Done</button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    padding: 16px;
    background: rgba(23, 35, 42, 0.52);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-window {
    background: var(--bg-surface);
    border: 1px solid var(--border-strong);
    border-radius: 2px;
    width: min(720px, calc(100vw - 32px));
    max-width: 720px;
    box-shadow: 0 18px 44px rgba(23, 35, 42, 0.24);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    max-height: calc(100dvh - 16px);
    min-height: 0;
    min-width: 0;
  }

  .modal-header {
    padding: 18px 22px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--border);
    gap: 16px;
  }

  .header-text {
    display: flex;
    flex-direction: column;
    gap: 5px;
    min-width: 0;
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .modal-title {
    margin: 0;
    font-size: 16px;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .modal-subtitle {
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .close-btn {
    width: 40px;
    height: 40px;
    display: grid;
    place-items: center;
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0;
    border-radius: 2px;
    flex-shrink: 0;
  }

  .close-btn:hover {
    border-color: var(--border-strong);
    background: var(--bg-muted);
    color: var(--text-primary);
  }

  .connect-tabs {
    display: flex;
    background: var(--bg-muted);
    border-bottom: 1px solid var(--border);
    padding: 0 22px;
  }

  .tab-btn {
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-secondary);
    font-weight: 600;
    padding: 12px 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    min-height: 48px;
    transition: background-color 120ms ease, border-color 120ms ease, color 120ms ease;
  }

  .tab-btn:hover {
    color: var(--text-primary);
  }

  .tab-btn.active {
    background: var(--bg-surface);
    color: var(--accent-strong);
    border-bottom-color: var(--accent-strong);
  }

  .modal-body {
    flex: 1;
    min-height: 0;
    padding: 22px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    overflow-y: auto;
  }

  .qr-section {
    display: grid;
    grid-template-columns: minmax(190px, 220px) minmax(0, 1fr);
    min-width: 0;
    gap: 24px;
    align-items: center;
  }

  .qr-card {
    background: #ffffff;
    border: 1px solid var(--border-strong);
    border-radius: 2px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    flex-shrink: 0;
  }

  .qr-canvas {
    display: block;
    width: 192px !important;
    height: 192px !important;
  }

  .qr-placeholder {
    width: 192px;
    height: 192px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    background: #f1f3f1;
    border: 1px dashed #aab4b1;
    color: #59676d;
    text-align: center;
  }

  .qr-subtext {
    color: #39474d;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    text-align: center;
    line-height: 1.3;
  }

  .instruction-list {
    display: flex;
    flex-direction: column;
    gap: 18px;
    min-width: 0;
  }

  .step-card {
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }

  .step-num {
    width: 26px;
    height: 26px;
    border-radius: 2px;
    background: var(--accent-muted);
    color: var(--accent-strong);
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .step-info {
    display: flex;
    flex-direction: column;
    gap: 5px;
    flex: 1;
    min-width: 0;
  }

  .step-title {
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.3;
  }

  .step-desc {
    color: var(--text-secondary);
    line-height: 1.45;
  }

  .url-copy-box {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 2px;
    padding: 10px;
    margin-top: 4px;
    gap: 12px;
    min-width: 0;
  }

  .url-details {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .url-label {
    color: var(--text-muted);
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .url-text {
    color: var(--text-primary);
    font-family: ui-monospace, "Cascadia Code", Consolas, monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }

  .copy-btn {
    min-height: 38px;
    background: var(--accent);
    color: #ffffff;
    border: 1px solid var(--accent-strong);
    border-radius: 2px;
    padding: 6px 12px;
    font-weight: 600;
    cursor: pointer;
    flex-shrink: 0;
    transition: background-color 120ms ease, opacity 120ms ease;
  }

  .copy-btn:hover {
    background: var(--accent-strong);
  }

  .copy-btn:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

  .usb-section {
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  .usb-desc {
    margin: 0;
    color: var(--text-secondary);
    line-height: 1.5;
  }

  .code-box {
    background: var(--bg-muted);
    border: 1px solid var(--border);
    border-radius: 2px;
    padding: 12px;
    font-family: monospace;
    color: var(--success);
    overflow-x: auto;
  }

  .code-box code {
    white-space: nowrap;
  }

  .modal-footer {
    padding: 12px 22px;
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .status-summary {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--text-secondary);
  }

  .status-summary strong {
    color: var(--success);
  }

  .live-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--text-muted);
    flex-shrink: 0;
  }

  .live-dot.connected {
    background: var(--success);
    box-shadow: 0 2px 6px rgba(36, 105, 78, 0.2);
  }

  .done-btn {
    background: var(--accent);
    color: #fff;
    border: 1px solid var(--accent-strong);
    border-radius: 2px;
    min-height: 40px;
    padding: 7px 18px;
    font-weight: 700;
    cursor: pointer;
    flex-shrink: 0;
  }

  .done-btn:hover {
    background: var(--accent-strong);
  }

  .modal-window button:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 3px;
  }

  @media (max-width: 560px) {
    .modal-backdrop {
      padding: 8px;
    }

    .modal-window {
      width: calc(100vw - 16px);
      max-height: calc(100dvh - 16px);
    }

    .modal-header {
      padding: 14px;
      gap: 10px;
    }

    .connect-tabs {
      padding: 0 10px;
    }

    .tab-btn {
      flex: 1;
      min-width: 0;
      min-height: 44px;
      padding: 10px 8px;
    }

    .modal-body {
      padding: 16px 14px;
      gap: 18px;
    }

    .qr-section {
      grid-template-columns: minmax(0, 1fr);
      justify-items: center;
      gap: 18px;
    }

    .qr-card {
      padding: 10px;
      border-radius: 2px;
    }

    .qr-canvas {
      width: min(192px, 58vw) !important;
      height: min(192px, 58vw) !important;
    }

    .qr-placeholder {
      width: min(192px, 58vw);
      height: min(192px, 58vw);
    }

    .instruction-list {
      width: 100%;
      gap: 16px;
    }

    .modal-footer {
      flex-wrap: wrap;
      padding: 12px 14px;
      gap: 10px;
    }

    .status-summary {
      flex: 1 1 100%;
      min-width: 0;
    }
  }
</style>
