import { mkdirSync, writeFileSync } from "node:fs";

import {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "../../../pipeline/default.js";
import { buildRunOpts } from "./build-run-opts.js";
import type { ExecuteSource } from "./execute-source.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";
import { resolveOutputPath } from "./resolve-output-path/resolve-output-path.js";

export async function execute(
  src: ExecuteSource,
  opts: NormalizedCliOptions,
): Promise<void> {
  const emitters = createDefaultEmitterRegistry();
  const outputPath = resolveOutputPath(src, opts, emitters);
  const { text, runOpts } = buildRunOpts(src, opts);

  const pipeline = createDefaultPipeline(emitters);
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
