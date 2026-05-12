import { describe, expect, test } from "vitest";

import unsnarlPluginReact from "../../plugins/unsnarl-plugin-react/index.js";
import { CliUsageError } from "./cli-usage-error.js";
import { validatePluginModule } from "./validate-plugin-module.js";

describe("validatePluginModule", () => {
  test("returns the default export when present", async () => {
    const mod = await import("../../plugins/unsnarl-plugin-react/index.js");
    const result = validatePluginModule(mod, "react");
    expect(result).toEqual(unsnarlPluginReact);
  });

  test("rejects with a CliUsageError when the module has no default export", async () => {
    expect.assertions(1);
    const mod = await import("./testing/no-default-plugin.js");
    try {
      validatePluginModule(mod, "no-default");
      throw new Error("expected validatePluginModule to throw");
    } catch (error) {
      if (!(error instanceof CliUsageError)) {
        throw error;
      }
      expect(error.message).toEqual(
        "Plugin 'unsnarl-plugin-no-default' has no default export.",
      );
    }
  });
});
