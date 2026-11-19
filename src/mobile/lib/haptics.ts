import { settings } from "./settings.svelte";

const HAPTIC_MULTIPLIERS = {
  off: 0,
  light: 0.6,
  medium: 1,
  strong: 1.8,
} as const;

export function haptic(ms: number | number[] = 15): void {
  const multiplier = HAPTIC_MULTIPLIERS[settings.data.hapticLevel];
  if (!multiplier || typeof navigator === "undefined" || !navigator.vibrate) return;

  const pattern = Array.isArray(ms)
    ? ms.map((duration) => Math.round(duration * multiplier))
    : Math.round(ms * multiplier);
  try {
    navigator.vibrate(pattern);
  } catch (_) {
    // Ignored if vibration is not permitted
  }
}
