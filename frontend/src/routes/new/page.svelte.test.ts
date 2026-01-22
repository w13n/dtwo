import { page, userEvent } from "vitest/browser";
import { describe, expect, it } from "vitest";
import { render } from "vitest-browser-svelte";
import Page from "./+page.svelte";

describe("new page tests", () => {
  it("invalid json disables the button", async () => {
    render(Page);

    await expect(
      page.getByRole("button", { name: "Create Settings" }),
    ).not.toBeDisabled();

    const input = page.getByText("{");
    await input.click();
    await userEvent.keyboard("a");
    await userEvent.keyboard("b");

    await expect(
      page.getByRole("button", { name: "Create Settings" }),
    ).toBeDisabled();
  });
});
