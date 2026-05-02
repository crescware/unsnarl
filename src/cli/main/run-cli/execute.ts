import type { Command } from "commander";
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
import { readSourceFile, readStdin } from "../../io.js";
import { deriveOutputBasename } from "../../output-name/output-name.js";
import { detectLanguage } from "../detect-language.js";
import { resolveGenerations } from "../resolve-generations.js";
import { CliUsageError } from "./cli-usage-error.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

export async function execute(
  program: Command,
  file: string | null,
  opts: NormalizedCliOptions,
): Promise<void> {
  const emitters = createDefaultEmitterRegistry();

  // Validate the output destination up-front: if -o/--out-dir cannot
  // produce a filename (e.g. --stdin without -r), bail out before we read
  // any input or do any analysis.
  let outputPath: string | null = null;
  if (opts.outDir !== null) {
    const derived = deriveOutputBasename({
      roots: opts.roots,
      descendants: opts.descendants,
      ancestors: opts.ancestors,
      context: opts.context,
      // --stdin overrides any positional file for content, so it should
      // override it for naming too: a stdin run has no usable filename.
      inputPath: opts.stdin ? null : file,
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

  let code: string;
  let sourcePath: string;
  if (opts.stdin) {
    code = await readStdin();
    sourcePath = `stdin.${opts.lang}`;
  } else if (file !== null) {
    code = readSourceFile(file);
    sourcePath = file;
  } else {
    throw new CliUsageError(
      "no input file (use --stdin or pass a path)",
      program.helpInformation(),
    );
  }

  const language = opts.stdin ? opts.lang : detectLanguage(file, opts.lang);

  const pipeline = createDefaultPipeline(emitters);
  const pruning: PruningRunOptions | null =
    opts.roots.length > 0
      ? {
          roots: opts.roots,
          ...resolveGenerations({
            descendants: opts.descendants,
            ancestors: opts.ancestors,
            context: opts.context,
          }),
        }
      : null;
  const runOpts: PipelineRunOptions = {
    format: opts.format,
    language,
    sourcePath,
    emit: { pretty: opts.pretty, prunedGraph: null },
    pruning,
  };

  const result = pipeline.runDetailed(code, runOpts);
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
  } else {
    process.stdout.write(result.text);
  }
}
