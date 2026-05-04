import type { ParsedCliOptions } from "../args/parsed-cli-options.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

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
