import type { UnsnarlPlugin } from "../../pipeline/plugin/unsnarl-plugin.js";
import { CliUsageError } from "./cli-usage-error.js";

export function validatePluginModule(
  mod: unknown,
  name: string,
): UnsnarlPlugin {
  const def = (mod as { default?: unknown }).default;
  if (def === undefined || def === null) {
    throw new CliUsageError(
      `Plugin 'unsnarl-plugin-${name}' has no default export.`,
      null,
    );
  }
  return def as UnsnarlPlugin;
}
