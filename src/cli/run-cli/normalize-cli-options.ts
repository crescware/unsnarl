import type { ParsedCliOptions } from "../args/parsed-cli-options.js";
import { CliUsageError } from "./cli-usage-error.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";
import type { OutTarget } from "./out-target.js";

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
    out: resolveOutTarget(opts),
    debug: opts.debug,
  };
}

function resolveOutTarget(opts: ParsedCliOptions): OutTarget | null {
  if (opts.outDir !== null && opts.outFile !== null) {
    throw new CliUsageError(
      "-o/--out-dir and --out-file are mutually exclusive; pick one",
      null,
    );
  }
  if (opts.outFile !== null) {
    return { mode: "file", path: opts.outFile };
  }
  if (opts.outDir !== null) {
    return { mode: "dir", path: opts.outDir };
  }
  return null;
}
