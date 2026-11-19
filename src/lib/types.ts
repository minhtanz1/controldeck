export interface AppRule {
  match_type: "window_title" | "process_name";
  pattern: string;
  case_sensitive?: boolean;
}

export type Action =
  | {
      type: "key";
      key: string;
      modifiers?: string[];
      on?: "press" | "hold" | "release";
    }
  | {
      type: "combo_key";
      key: string;
      modifiers: string[];
    }
  | {
      type: "toggle";
      key: string;
      modifiers?: string[];
      state?: boolean;
    }
  | {
      type: "mouse";
      button: "left" | "right" | "middle";
      on?: "click" | "hold" | "toggle";
    }
  | {
      type: "macro";
      sequence: Array<{
        key: string;
        modifiers?: string[];
        delay_ms: number;
      }>;
      repeat?: boolean;
    };

export interface TriggerAction {
  threshold: number;
  action: Action;
}

export interface StickAction {
  deadzone: number;
  mode: "dpad" | "scroll" | "mouse";
  sensitivity?: number;
  drag_button?: "none" | "left" | "right";
  up?: Action;
  down?: Action;
  left?: Action;
  right?: Action;
}

export interface ComboAction {
  buttons: string[];
  action: Action;
  window_ms: number;
}

export interface Mappings {
  buttons: Record<string, Action>;
  triggers?: Record<string, TriggerAction>;
  sticks?: Record<string, StickAction>;
  combos?: ComboAction[];
}

export interface DeckItem {
  id: string;
  label: string;
  category?: string;
  shortcut?: string;
  color?: string;
  action: Action;
}

export interface Profile {
  id: string;
  name: string;
  description: string;
  version: number;
  app_rules: AppRule[];
  mappings: Mappings;
  deck?: DeckItem[];
}

export interface ControllerState {
  connected: boolean;
  player_index: number;
  buttons: Record<string, boolean>;
  left_trigger: number;
  right_trigger: number;
  left_stick_x: number;
  left_stick_y: number;
  right_stick_x: number;
  right_stick_y: number;
}

export interface ActiveWindowInfo {
  title: string;
  process_name: string;
  hwnd: number;
}

export interface OpenWindowInfo {
  hwnd: number;
  title: string;
  process_name: string;
  is_browser: boolean;
  clean_title: string;
}

export interface MobileServerInfo {
  port: number;
  local_ip: string;
  url: string;
  connected_clients: number;
}
