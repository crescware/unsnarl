import {
  CLI_COLOR_THEME,
  type CliColorTheme,
} from "../../../cli-color-theme.js";
import type { ColorTheme } from "./color-theme.js";
import { darkTheme } from "./dark-theme.js";
import { lightTheme } from "./light-theme.js";

export const COLOR_THEMES = {
  [CLI_COLOR_THEME.Dark]: darkTheme,
  [CLI_COLOR_THEME.Light]: lightTheme,
} as const satisfies Record<CliColorTheme, ColorTheme>;
