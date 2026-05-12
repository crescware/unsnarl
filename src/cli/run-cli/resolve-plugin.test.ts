import { describe, expect, test } from "vitest";

import { CliUsageError } from "./cli-usage-error.js";
import { resolvePlugin } from "./resolve-plugin.js";

describe("resolvePlugin", () => {
  test("resolves a bundled plugin and returns its default export", async () => {
    const plugin = await resolvePlugin("react");
    expect(plugin.meta.name).toEqual("unsnarl-plugin-react");
    expect(typeof plugin.transform).toEqual("function");
  });

  test("rejects with a CliUsageError carrying a human-readable message when the plugin is not bundled", async () => {
    expect.assertions(2);
    try {
      await resolvePlugin("nonexistent-xyz");
      throw new Error("expected resolvePlugin to reject");
    } catch (error) {
      if (!(error instanceof CliUsageError)) {
        throw error;
      }
      expect(error.message).toContain(
        "Plugin 'unsnarl-plugin-nonexistent-xyz' is not bundled with this unsnarl build.",
      );
      expect(error.message).toContain(
        "Only built-in plugins are currently supported.",
      );
    }
  });
});
