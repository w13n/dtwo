import { defineConfig } from "vitest/config";
import { playwright } from "@vitest/browser-playwright";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig(({ mode }) => ({
  plugins: [sveltekit()],
  test: {
    setupFiles: ["./vitest-setup.ts"],
    browser: {
      enabled: true,
      provider: playwright(),
      instances: [{ browser: "chromium", headless: true }],
    },
    include: ["src/**/*.svelte.{test,spec}.{js,ts}"],
  },
  resolve: {
    conditions: mode === "test" ? ["browser"] : [],
  },
}));
