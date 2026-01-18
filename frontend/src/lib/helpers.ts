import type { Settings } from "$lib/types";

export function getSettingData(settings: Settings): Record<string, unknown> {
  const { id, data } = settings;
  return data;
}
