import type {
  ClusterPosition,
  ControlClusterScales,
  ControlLayoutSnapshot,
  HapticLevel,
  MobileSettings,
  ResizableControlCluster,
  SavedControlLayout,
  SaveLayoutResult,
  StickSizes,
} from "./types";

const STORAGE_KEY = "controldeck_mobile_settings";
const MAX_SAVED_LAYOUTS = 5;
const DEFAULT_STICK_SIZES: StickSizes = { left: 112, right: 112 };
const DEFAULT_CLUSTER_SCALES: ControlClusterScales = {
  shoulders: 1,
  menu: 1,
  dpad: 1,
  face: 1,
};

const defaultSettings: MobileSettings = {
  preset: "wme",
  scale: "medium",
  lefty: false,
  stickDragMode: "none",
  triggerRampMs: 700,
  hapticLevel: "medium",
  mouseSensitivity: 1.7,
  holdToDragDelay: 2000,
  customPositions: null,
  joystickSizes: { ...DEFAULT_STICK_SIZES },
  clusterScales: { ...DEFAULT_CLUSTER_SCALES },
  layoutsByProfile: {},
  activeLayoutByProfile: {},
  layoutDraftsByProfile: {},
  draftLayoutIdsByProfile: {},
};

function loadSettings(): MobileSettings {
  if (typeof localStorage === "undefined") return { ...defaultSettings };
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...defaultSettings };
    const parsed: Partial<MobileSettings> = JSON.parse(raw);
    return {
      ...defaultSettings,
      ...parsed,
      joystickSizes: { ...DEFAULT_STICK_SIZES, ...parsed.joystickSizes },
      clusterScales: { ...DEFAULT_CLUSTER_SCALES, ...parsed.clusterScales },
      layoutsByProfile: parsed.layoutsByProfile ?? {},
      activeLayoutByProfile: parsed.activeLayoutByProfile ?? {},
      layoutDraftsByProfile: parsed.layoutDraftsByProfile ?? {},
      draftLayoutIdsByProfile: parsed.draftLayoutIdsByProfile ?? {},
    };
  } catch (_) {
    return { ...defaultSettings };
  }
}

class SettingsStore {
  data = $state<MobileSettings>(loadSettings());
  activeProfileId = $state("");
  activeLayoutId = $state<string | null>(null);

  get layoutsForActiveProfile(): SavedControlLayout[] {
    return this.activeProfileId
      ? this.data.layoutsByProfile[this.activeProfileId] ?? []
      : [];
  }

  save() {
    if (typeof localStorage !== "undefined") {
      try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(this.data));
      } catch (_) {}
    }
  }

  activateProfile(profileId: string) {
    if (!profileId || profileId === this.activeProfileId) return;

    const isFirstProfile = !this.activeProfileId;
    this.persistActiveDraft();
    this.activeProfileId = profileId;

    const layouts = this.data.layoutsByProfile[profileId] ?? [];
    const requestedLayoutId = this.data.activeLayoutByProfile[profileId] ?? null;
    const savedLayout = layouts.find((layout) => layout.id === requestedLayoutId);
    const draft = this.data.layoutDraftsByProfile[profileId];
    const draftLayoutId = this.data.draftLayoutIdsByProfile[profileId] ?? null;

    this.activeLayoutId = savedLayout ? requestedLayoutId : null;
    const snapshot =
      draft && draftLayoutId === this.activeLayoutId
        ? draft
        : savedLayout ??
          draft ??
          (isFirstProfile
            ? {
                customPositions: this.data.customPositions,
                joystickSizes: this.data.joystickSizes,
                clusterScales: this.data.clusterScales,
              }
            : this.defaultSnapshot());
    this.applySnapshot(snapshot);
    this.persistActiveDraft();
    this.save();
  }

  selectLayout(layoutId: string) {
    const profileId = this.activeProfileId;
    if (!profileId) return;

    this.persistActiveDraft();
    const layout = layoutId
      ? this.layoutsForActiveProfile.find((saved) => saved.id === layoutId)
      : undefined;
    if (layoutId && !layout) return;

    this.activeLayoutId = layout?.id ?? null;
    this.applySnapshot(
      layout ?? this.data.layoutDraftsByProfile[profileId] ?? this.defaultSnapshot(),
    );
    this.persistActiveDraft();
    this.save();
  }

  saveLayout(name: string): SaveLayoutResult {
    const profileId = this.activeProfileId;
    if (!profileId) return "profile-required";

    const trimmedName = name.trim();
    if (!trimmedName) return "name-required";

    const layouts = [...this.layoutsForActiveProfile];
    const snapshot = this.currentSnapshot();
    const existingIndex = layouts.findIndex((layout) => layout.id === this.activeLayoutId);
    if (existingIndex >= 0) {
      layouts[existingIndex] = {
        ...layouts[existingIndex],
        name: trimmedName,
        ...snapshot,
      };
      this.data.layoutsByProfile = { ...this.data.layoutsByProfile, [profileId]: layouts };
    } else {
      if (layouts.length >= MAX_SAVED_LAYOUTS) return "limit-reached";
      const id = `layout-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
      layouts.push({ id, name: trimmedName, ...snapshot });
      this.data.layoutsByProfile = { ...this.data.layoutsByProfile, [profileId]: layouts };
      this.activeLayoutId = id;
    }

    this.persistActiveDraft();
    this.save();
    return "saved";
  }

  deleteActiveLayout() {
    const profileId = this.activeProfileId;
    const layoutId = this.activeLayoutId;
    if (!profileId || !layoutId) return;

    this.data.layoutsByProfile = {
      ...this.data.layoutsByProfile,
      [profileId]: this.layoutsForActiveProfile.filter((layout) => layout.id !== layoutId),
    };
    this.activeLayoutId = null;
    this.persistActiveDraft();
    this.save();
  }

  setPreset(preset: MobileSettings["preset"]) {
    this.data.preset = preset;
    this.data.customPositions = null;
    if (preset === "wme") {
      this.data.scale = "medium";
      this.data.stickDragMode = "none";
    } else if (preset === "standard") {
      this.data.scale = "medium";
      this.data.stickDragMode = "none";
    } else if (preset === "split") {
      this.data.scale = "small";
      this.data.stickDragMode = "left";
    } else if (preset === "minimal") {
      this.data.scale = "small";
      this.data.stickDragMode = "none";
    }
    this.persistActiveDraft();
    this.save();
  }

  setScale(scale: MobileSettings["scale"]) {
    this.data.scale = scale;
    this.save();
  }

  setLefty(lefty: boolean) {
    this.data.lefty = lefty;
    this.save();
  }

  setStickDragMode(mode: MobileSettings["stickDragMode"]) {
    this.data.stickDragMode = mode;
    this.save();
  }

  setTriggerRamp(ms: number) {
    this.data.triggerRampMs = ms;
    this.save();
  }

  setHapticLevel(level: HapticLevel) {
    this.data.hapticLevel = level;
    this.save();
  }

  setMouseSensitivity(sens: number) {
    this.data.mouseSensitivity = Math.round(sens * 10) / 10;
    this.save();
  }

  setHoldToDragDelay(ms: number) {
    this.data.holdToDragDelay = ms;
    this.save();
  }

  setCustomPosition(clusterId: string, pos: ClusterPosition) {
    if (!this.data.customPositions) {
      this.data.customPositions = {};
    }
    this.data.customPositions[clusterId] = pos;
    this.persistActiveDraft();
    this.save();
  }

  setJoystickSize(side: keyof StickSizes, size: number) {
    const snappedSize = Math.max(72, Math.min(144, Math.round(size / 4) * 4));
    this.data.joystickSizes = { ...this.data.joystickSizes, [side]: snappedSize };
    this.persistActiveDraft();
    this.save();
  }

  setClusterScale(cluster: ResizableControlCluster, scale: number) {
    const snappedScale = Math.max(0.7, Math.min(1.3, Math.round(scale / 0.05) * 0.05));
    this.data.clusterScales = { ...this.data.clusterScales, [cluster]: snappedScale };
    this.persistActiveDraft();
    this.save();
  }

  resetCustomPositions() {
    this.data.customPositions = null;
    this.data.joystickSizes = { ...DEFAULT_STICK_SIZES };
    this.data.clusterScales = { ...DEFAULT_CLUSTER_SCALES };
    this.persistActiveDraft();
    this.save();
  }

  resetDefaults() {
    this.data = {
      ...defaultSettings,
      joystickSizes: { ...DEFAULT_STICK_SIZES },
      clusterScales: { ...DEFAULT_CLUSTER_SCALES },
      layoutsByProfile: {},
      activeLayoutByProfile: {},
      layoutDraftsByProfile: {},
      draftLayoutIdsByProfile: {},
    };
    this.activeLayoutId = null;
    this.save();
  }

  private defaultSnapshot(): ControlLayoutSnapshot {
    return {
      customPositions: null,
      joystickSizes: { ...DEFAULT_STICK_SIZES },
      clusterScales: { ...DEFAULT_CLUSTER_SCALES },
    };
  }

  private currentSnapshot(): ControlLayoutSnapshot {
    return {
      customPositions: this.data.customPositions
        ? Object.fromEntries(
            Object.entries(this.data.customPositions).map(([id, position]) => [
              id,
              { ...position },
            ]),
          )
        : null,
      joystickSizes: { ...this.data.joystickSizes },
      clusterScales: { ...this.data.clusterScales },
    };
  }

  private applySnapshot(snapshot: ControlLayoutSnapshot) {
    this.data.customPositions = snapshot.customPositions
      ? Object.fromEntries(
          Object.entries(snapshot.customPositions).map(([id, position]) => [
            id,
            { ...position },
          ]),
        )
      : null;
    this.data.joystickSizes = { ...DEFAULT_STICK_SIZES, ...snapshot.joystickSizes };
    this.data.clusterScales = { ...DEFAULT_CLUSTER_SCALES, ...snapshot.clusterScales };
  }

  private persistActiveDraft() {
    const profileId = this.activeProfileId;
    if (!profileId) return;

    this.data.layoutDraftsByProfile = {
      ...this.data.layoutDraftsByProfile,
      [profileId]: this.currentSnapshot(),
    };
    this.data.draftLayoutIdsByProfile = {
      ...this.data.draftLayoutIdsByProfile,
      [profileId]: this.activeLayoutId,
    };
    this.data.activeLayoutByProfile = {
      ...this.data.activeLayoutByProfile,
      [profileId]: this.activeLayoutId,
    };
  }
}

export const settings = new SettingsStore();
