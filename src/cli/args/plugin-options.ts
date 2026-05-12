import { Option } from "commander";

import { collectPlugins } from "./collect-plugins.js";

export function pluginOptions(): readonly Option[] {
  return [
    new Option(
      "--plugin <names>",
      "Enable plugin(s). Repeat the flag or comma-delimit for multiple. The 'unsnarl-plugin-' prefix may be omitted.",
    )
      .argParser(collectPlugins)
      .default([] as readonly string[]),
  ];
}
