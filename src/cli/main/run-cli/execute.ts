import { mkdirSync, writeFileSync } from "node:fs";

import {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "../../../pipeline/default.js";
import type {
  PipelineRunOptions,
  PruningRunOptions,
} from "../../../pipeline/types.js";
import { readSourceFile } from "../../io.js";
import { detectLanguage } from "../detect-language.js";
import { resolveGenerations } from "../resolve-generations.js";
import type { ExecuteSource } from "./execute-source.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";
import { resolveOutputPath } from "./resolve-output-path/resolve-output-path.js";

export async function execute(
  src: ExecuteSource,
  opts: NormalizedCliOptions,
): Promise<void> {
  const emitters = createDefaultEmitterRegistry();

  const outputPath = resolveOutputPath(src, opts, emitters);

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
