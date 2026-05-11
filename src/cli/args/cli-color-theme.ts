import { CLI_COLOR_THEME } from "../../cli-color-theme.js";

export const COLOR_THEMES: ReadonlySet<string> = new Set(
  Object.values(CLI_COLOR_THEME),
);
