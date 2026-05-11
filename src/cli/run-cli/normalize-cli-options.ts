import type { NestingDepths } from "../../ir/annotations/scope-annotation.js";
import { NESTING_KIND } from "../../serializer/nesting-kind.js";
import { DEFAULT_DEPTH } from "../args/depth-options.js";
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
    colorTheme: opts.colorTheme,
    roots: opts.roots,
    descendants: opts.descendants,
    ancestors: opts.ancestors,
    context: opts.context,
    depths: resolveDepths(opts),
    out: resolveOutTarget(opts),
    debug: opts.debug,
  };
}

function resolveDepths(opts: ParsedCliOptions): NestingDepths {
  const general = opts.depth ?? DEFAULT_DEPTH;
  const fn = opts.depthFunction ?? general;
  const block = opts.depthBlock ?? general;
  return {
    [NESTING_KIND.Function]: fn,
    [NESTING_KIND.If]: block,
    [NESTING_KIND.For]: block,
    [NESTING_KIND.While]: block,
    [NESTING_KIND.Switch]: block,
    [NESTING_KIND.TryCatchFinally]: block,
    [NESTING_KIND.Block]: block,
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
