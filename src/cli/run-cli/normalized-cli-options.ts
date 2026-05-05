import type { CliMermaidRenderer } from "../../cli-mermaid-renderer.js";
import type { CliLanguage } from "../../language.js";
import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";
import type { OutTarget } from "./out-target.js";

export type NormalizedCliOptions = Readonly<{
  format: string;
  stdin: boolean;
  stdinLang: CliLanguage;
  prettyJson: boolean;
  mermaidRenderer: CliMermaidRenderer | null;
  roots: readonly ParsedRootQuery[];
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
  out: OutTarget | null;
  debug: boolean;
}>;
