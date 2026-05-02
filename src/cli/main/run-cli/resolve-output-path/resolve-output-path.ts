import { join } from "node:path";

import type { EmitterRegistry } from "../../../../pipeline/types.js";
import { CliUsageError } from "../cli-usage-error.js";
import type { ExecuteSource } from "../execute-source.js";
import type { NormalizedCliOptions } from "../normalized-cli-options.js";
import { deriveOutputBasename } from "./derive-output-basename.js";

/**
 * Validate the output destination up-front: if -o/--out-dir cannot
 * produce a filename (e.g. --stdin without -r), bail out before we
 * spend cycles on analysis.
 */
export function resolveOutputPath(
  src: ExecuteSource,
  opts: NormalizedCliOptions,
  emitters: EmitterRegistry,
): string | null {
  if (opts.outDir === null) {
    return null;
  }

  // --stdin overrides any positional file for content, so it should
  // override it for naming too: a stdin run has no usable filename and
  // therefore must rely on -r/--roots to produce a basename.
  if (src.stdin && opts.roots.length === 0) {
    throw new CliUsageError(
      "--out-dir requires either -r/--roots or an input file path",
      null,
    );
  }

  const baseName = deriveOutputBasename({
    roots: opts.roots,
    descendants: opts.descendants,
    ancestors: opts.ancestors,
    context: opts.context,
    // The guard above guarantees roots are non-empty when src is stdin,
    // so deriveOutputBasename never reads inputPath in that branch.
    inputPath: src.stdin ? "" : src.path,
  });

  const emitter = emitters.get(opts.format) ?? null;
  if (emitter === null) {
    const available = emitters.list().join(", ");
    throw new Error(
      `Unknown emitter format: ${opts.format}. Available: ${available}`,
    );
  }

  return join(opts.outDir, `${baseName}.${emitter.extension}`);
}
