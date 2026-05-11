import { InvalidArgumentError, Option } from "commander";

import { CLI_COLOR_THEME, type CliColorTheme } from "../../cli-color-theme.js";
import type { CliMermaidRenderer } from "../../cli-mermaid-renderer.js";
import { createDefaultEmitterRegistry } from "../../pipeline/create-default-emitter-registry.js";
import { COLOR_THEMES } from "./cli-color-theme.js";
import { MERMAID_RENDERERS } from "./cli-mermaid-renderer.js";

function coerceMermaidRenderer(value: string): CliMermaidRenderer {
  if (!MERMAID_RENDERERS.has(value)) {
    throw new InvalidArgumentError(`Invalid mermaid renderer: ${value}`);
  }
  return value as CliMermaidRenderer;
}

function coerceColorTheme(value: string): CliColorTheme {
  if (!COLOR_THEMES.has(value)) {
    throw new InvalidArgumentError(`Invalid color theme: ${value}`);
  }
  return value as CliColorTheme;
}

export function formatOptions(): readonly Option[] {
  const availableFormats = createDefaultEmitterRegistry().list();
  return [
    new Option(
      "-f, --format <id>",
      `Emitter format (${availableFormats.join(", ")})`,
    ).default("mermaid"),
    new Option("--no-pretty-json", "Disable pretty-printed JSON output"),
    new Option(
      "--mermaid-renderer <renderer>",
      "Layout engine for Mermaid output",
    )
      .argParser(coerceMermaidRenderer)
      .default(null, "auto"),
    new Option(
      "--color-theme <theme>",
      "Color theme for Mermaid output (dark, light)",
    )
      .argParser(coerceColorTheme)
      .default(CLI_COLOR_THEME.Dark),
  ];
}
