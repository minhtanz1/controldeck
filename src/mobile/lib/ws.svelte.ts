import type { ProfileSyncMessage, ProfileSummary, DeckItem } from "./types";
import { haptic } from "./haptics";

class WebSocketClient {
  private socket: WebSocket | null = null;
  private pingTimer: number | null = null;
  private lastPingTime = 0;

  connected = $state(false);
  pingMs = $state<number | null>(null);
  statusText = $state("Connecting...");

  activeProfileId = $state("");
  activeProfileName = $state("");
  profiles = $state<ProfileSummary[]>([]);
  deck = $state<DeckItem[]>([]);

  connect() {
    if (typeof window === "undefined") return;
    const loc = window.location;
    const proto = loc.protocol === "https:" ? "wss:" : "ws:";
    const url = `${proto}//${loc.host}/ws`;

    try {
      this.socket = new WebSocket(url);

      this.socket.onopen = () => {
        this.connected = true;
        this.statusText = "Online";
        this.startPing();
        this.requestProfileSync();
      };

      this.socket.onclose = () => {
        this.connected = false;
        this.pingMs = null;
        this.statusText = "Reconnecting...";
        this.stopPing();
        setTimeout(() => this.connect(), 1500);
      };

      this.socket.onerror = () => {
        if (this.socket) {
          this.socket.close();
        }
      };

      this.socket.onmessage = (e) => {
        try {
          const msg = JSON.parse(e.data);
          if (msg.type === "pong") {
            this.pingMs = Math.round(performance.now() - this.lastPingTime);
            this.statusText = `${this.pingMs} ms`;
          } else if (msg.type === "profile_sync") {
            const sync = msg as ProfileSyncMessage;
            this.activeProfileId = sync.active_profile_id;
            this.activeProfileName = sync.active_profile_name || sync.active_profile_id;
            this.profiles = sync.profiles || [];
            this.deck = sync.deck || [];
          }
        } catch (_) {}
      };
    } catch (_) {
      setTimeout(() => this.connect(), 2000);
    }
  }

  private startPing() {
    this.stopPing();
    this.pingTimer = window.setInterval(() => {
      this.lastPingTime = performance.now();
      this.send({ type: "ping" });
    }, 2000);
  }

  private stopPing() {
    if (this.pingTimer) {
      clearInterval(this.pingTimer);
      this.pingTimer = null;
    }
  }

  send(obj: Record<string, unknown>) {
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(obj));
    }
  }

  sendButtonDown(btn: string) {
    this.send({ type: "btn_down", btn });
  }

  sendButtonUp(btn: string) {
    this.send({ type: "btn_up", btn });
  }

  sendTriggerMove(trigger: "lt" | "rt", value: number) {
    this.send({ type: "trigger_move", trigger, value });
  }

  sendStickMove(stick: "left" | "right", x: number, y: number) {
    this.send({ type: "stick_move", stick, x, y });
  }

  sendMouseMove(dx: number, dy: number) {
    this.send({ type: "mouse_move", dx, dy });
  }

  sendMouseDown(btn: "left" | "middle" | "right") {
    this.send({ type: "mouse_down", btn });
  }

  sendMouseUp(btn: "left" | "middle" | "right") {
    this.send({ type: "mouse_up", btn });
  }

  sendMouseClick(btn: "left" | "middle" | "right") {
    this.send({ type: "mouse_click", btn });
  }

  sendMouseWheel(delta_y: number) {
    this.send({ type: "mouse_wheel", delta_y });
  }

  sendTextInput(text: string) {
    this.send({ type: "text_input", text });
  }

  sendKeyTap(key: string) {
    this.send({ type: "key_tap", key });
  }

  sendKeyCombo(key: string, mods: string[]) {
    this.send({ type: "key_combo", key, mods });
  }

  sendDeckAction(action: Record<string, unknown>) {
    this.send({ type: "deck_action", action });
  }

  switchProfile(profileId: string) {
    if (!profileId || profileId === this.activeProfileId) return;
    haptic();
    this.send({ type: "switch_profile", profile_id: profileId });
  }

  requestProfileSync() {
    this.send({ type: "get_profile" });
  }

  setSensitivity(sensitivity: number) {
    this.send({ type: "set_sensitivity", sensitivity });
  }
}

export const wsClient = new WebSocketClient();
