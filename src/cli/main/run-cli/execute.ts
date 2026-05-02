import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "../../../pipeline/default.js";
import type {
  PipelineRunOptions,
  PruningRunOptions,
} from "../../../pipeline/types.js";
import { readSourceFile } from "../../io.js";
import { deriveOutputBasename } from "../../output-name/output-name.js";
import { detectLanguage } from "../detect-language.js";
import { resolveGenerations } from "../resolve-generations.js";
import { CliUsageError } from "./cli-usage-error.js";
import type { ExecuteSource } from "./execute-source.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

export async function execute(
  src: ExecuteSource,
  opts: NormalizedCliOptions,
): Promise<void> {
  const emitters = createDefaultEmitterRegistry();

  // Validate the output destination up-front: if -o/--out-dir cannot
  // produce a filename (e.g. --stdin without -r), bail out before we
  // spend cycles on analysis.
  let outputPath: string | null = null;
  if (opts.outDir !== null) {
    const derived = deriveOutputBasename({
      roots: opts.roots,
      descendants: opts.descendants,
      ancestors: opts.ancestors,
      context: opts.context,
      // --stdin overrides any positional file for content, so it should
      // override it for naming too: a stdin run has no usable filename.
      inputPath: src.stdin ? null : src.path,
    });
    if (!derived.ok) {
      throw new CliUsageError(derived.error, null);
    }
    const emitter = emitters.get(opts.format);
    if (emitter === undefined) {
      const available = emitters.list().join(", ");
      throw new Error(
        `Unknown emitter format: ${opts.format}. Available: ${available}`,
      );
    }
    outputPath = join(opts.outDir, `${derived.basename}.${emitter.extension}`);
  }

  const text = src.stdin ? src.text : readSourceFile(src.path);
  const sourcePath = src.stdin ? `stdin.${src.lang}` : src.path;
  const language = src.stdin ? src.lang : detectLanguage(src.path);

  const pipeline = createDefaultPipeline(emitters);

  const pruning =
    0 < opts.roots.length
      ? ({
          roots: opts.roots,
          ...resolveGenerations({
            descendants: opts.descendants,
            ancestors: opts.ancestors,
            context: opts.context,
          }),
        } satisfies PruningRunOptions)
      : null;

  const runOpts = {
    format: opts.format,
    language,
    sourcePath,
    emit: { prettyJson: opts.prettyJson, prunedGraph: null },
    pruning,
  } satisfies PipelineRunOptions;

  const result = pipeline.runDetailed(text, runOpts);

  if (result.pruning !== null) {
    for (const r of result.pruning) {
      if (r.matched === 0) {
        process.stderr.write(
          `unsnarl: warning: query '${r.query}' matched 0 roots\n`,
        );
      }
    }
  }

  if (outputPath !== null && opts.outDir !== null) {
    mkdirSync(opts.outDir, { recursive: true });
    writeFileSync(outputPath, result.text);
    return;
  }

  process.stdout.write(result.text);
}
