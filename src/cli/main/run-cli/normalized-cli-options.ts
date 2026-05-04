import type { ParsedCliOptions } from "../../args/build-command.js";
import type { CliMermaidRenderer } from "../../cli-mermaid-renderer.js";
import type { CliLanguage } from "../../language.js";
import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";

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
  outDir: string | null;
}>;

export function normalizeCliOptions(
  opts: ParsedCliOptions,
): NormalizedCliOptions {
  return {
    format: opts.format,
    stdin: opts.stdin,
    stdinLang: opts.stdinLang,
    prettyJson: opts.prettyJson,
    mermaidRenderer: opts.mermaidRenderer,
    roots: opts.roots,
    descendants: opts.descendants,
    ancestors: opts.ancestors,
    context: opts.context,
    outDir: opts.outDir,
  };
}
