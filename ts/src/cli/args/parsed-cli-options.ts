import type { CliColorTheme } from "../../cli-color-theme.js";
import type { CliMermaidRenderer } from "../../cli-mermaid-renderer.js";
import type { CliLanguage } from "../../language.js";
import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";
import type { RawHighlight } from "./highlight-options.js";

export type ParsedCliOptions = Readonly<{
  format: string;
  stdin: boolean;
  stdinLang: CliLanguage;
  prettyJson: boolean;
  mermaidRenderer: CliMermaidRenderer | null;
  colorTheme: CliColorTheme;
  roots: readonly ParsedRootQuery[];
  highlight: RawHighlight;
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
  depth: number | null;
  depthFunction: number | null;
  depthBlock: number | null;
  outDir: string | null;
  outFile: string | null;
  debug: boolean;
  plugins: readonly string[];
}>;

// What commander hands to the action callback. The CLI flag is named
// `--plugin` (singular), so commander derives the attribute as `plugin`.
// We rename it to `plugins` at the action boundary before propagating.
export type RawCliOptions = Omit<ParsedCliOptions, "plugins"> &
  Readonly<{ plugin: readonly string[] }>;
