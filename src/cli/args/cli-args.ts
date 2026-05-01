import type { CliMermaidRenderer } from "../cli-mermaid-renderer.js";
import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import type { CliLanguage } from "./cli-language.js";

// Default generations used when the user gives -r/--roots but no -A/-B/-C.
export const DEFAULT_GENERATIONS = 10;

export type CliArgs = Readonly<{
  format: string;
  stdin: boolean;
  language: CliLanguage;
  pretty: boolean;
  listFormats: boolean;
  help: boolean;
  version: boolean;
  inputFile: string | null;
  mermaidRenderer: CliMermaidRenderer | null;
  roots: readonly ParsedRootQuery[];
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
  outDir: string | null;
}>;
