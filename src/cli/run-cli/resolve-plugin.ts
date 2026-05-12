import type { UnsnarlPlugin } from "../../pipeline/plugin/unsnarl-plugin.js";
import { CliUsageError } from "./cli-usage-error.js";
import { validatePluginModule } from "./validate-plugin-module.js";

export async function resolvePlugin(name: string): Promise<UnsnarlPlugin> {
  const url = new URL(
    `../../plugins/unsnarl-plugin-${name}/index.js`,
    import.meta.url,
  );
  let mod: unknown;
  try {
    mod = await import(url.href);
  } catch (e) {
    const detail = e instanceof Error ? e.message : String(e);
    throw new CliUsageError(
      `Plugin 'unsnarl-plugin-${name}' is not bundled with this unsnarl build. Only built-in plugins are currently supported. (${detail})`,
      null,
    );
  }
  return validatePluginModule(mod, name);
}
