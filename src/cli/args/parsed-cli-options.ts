import type { CliMermaidRenderer } from "../../cli-mermaid-renderer.js";
import type { CliLanguage } from "../../language.js";
import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";

export type ParsedCliOptions = Readonly<{
  format: string;
  stdin: boolean;
  stdinLang: CliLanguage;
  prettyJson: boolean;
  mermaidRenderer: CliMermaidRenderer | null;
  roots: readonly ParsedRootQuery[];
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
  depth: number | null;
  depthFunction: number | null;
  depthBlock: number | null;
  outDir: string | null;
  outFile: string | null;
  debug: boolean;
}>;
