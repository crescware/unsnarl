import { describe, expect, test } from "vitest";

import { CLI_COLOR_THEME } from "../../../cli-color-theme.js";
import { COLOR_THEMES } from "./color-themes.js";
import { darkTheme } from "./dark-theme.js";
import { lightTheme } from "./light-theme.js";

describe("COLOR_THEMES", () => {
  test("resolves 'dark' to darkTheme", () => {
    expect(COLOR_THEMES[CLI_COLOR_THEME.Dark]).toEqual(darkTheme);
  });

  test("resolves 'light' to lightTheme", () => {
    expect(COLOR_THEMES[CLI_COLOR_THEME.Light]).toEqual(lightTheme);
  });
});
