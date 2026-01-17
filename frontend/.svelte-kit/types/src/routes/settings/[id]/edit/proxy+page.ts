// @ts-nocheck
import { error } from "@sveltejs/kit";
import { getSettingsById, ApiClientError } from "$lib/api";
import type { PageLoad } from "./$types";

export const load = async ({ params, fetch }: Parameters<PageLoad>[0]) => {
  try {
    const settings = await getSettingsById(params.id, fetch);
    return { settings };
  } catch (e) {
    if (e instanceof ApiClientError && e.status === 404) {
      throw error(404, "Settings not found");
    }
    throw error(500, "Failed to load settings");
  }
};
