export const CLI_COLOR_THEME = {
  Dark: "dark",
  Light: "light",
} as const;
export type CliColorTheme =
  (typeof CLI_COLOR_THEME)[keyof typeof CLI_COLOR_THEME];
