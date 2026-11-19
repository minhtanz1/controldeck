export type MobileMode = "gamepad" | "mouse" | "deck";
export type HapticLevel = "off" | "light" | "medium" | "strong";

export interface DeckItem {
  id: string;
  label: string;
  category: string;
  shortcut: string;
  color?: string | null;
  action: Record<string, unknown>;
}

export interface ProfileSummary {
  id: string;
  name: string;
}

export interface ProfileSyncMessage {
  type: "profile_sync";
  active_profile_id: string;
  active_profile_name: string;
  profiles: ProfileSummary[];
  deck: DeckItem[];
}

export interface ClusterPosition {
  x: number;
  y: number;
}

export interface StickSizes {
  left: number;
  right: number;
}

export type ResizableControlCluster = "shoulders" | "menu" | "dpad" | "face";

export type ControlClusterScales = Record<ResizableControlCluster, number>;

export interface ControlLayoutSnapshot {
  customPositions: Record<string, ClusterPosition> | null;
  joystickSizes: StickSizes;
  clusterScales: ControlClusterScales;
}

export interface SavedControlLayout extends ControlLayoutSnapshot {
  id: string;
  name: string;
}

export type SaveLayoutResult =
  | "saved"
  | "profile-required"
  | "name-required"
  | "limit-reached";

export interface MobileSettings {
  preset: "wme" | "standard" | "split" | "minimal";
  scale: "small" | "medium" | "large";
  lefty: boolean;
  stickDragMode: "none" | "left" | "right";
  triggerRampMs: number;
  hapticLevel: HapticLevel;
  mouseSensitivity: number;
  holdToDragDelay: number;
  customPositions: Record<string, ClusterPosition> | null;
  joystickSizes: StickSizes;
  clusterScales: ControlClusterScales;
  layoutsByProfile: Record<string, SavedControlLayout[]>;
  activeLayoutByProfile: Record<string, string | null>;
  layoutDraftsByProfile: Record<string, ControlLayoutSnapshot>;
  draftLayoutIdsByProfile: Record<string, string | null>;
}
