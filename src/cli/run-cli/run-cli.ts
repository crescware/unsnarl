import { CommanderError } from "commander";

import { ParseError } from "../../parser/parse-error.js";
import { createDefaultEmitterRegistry } from "../../pipeline/create-default-emitter-registry.js";
import { createDefaultPipeline } from "../../pipeline/create-default-pipeline.js";
import { buildCommand } from "../args/build-command.js";
import type { ParsedCliOptions } from "../args/parsed-cli-options.js";
import { buildRunOpts } from "./build-run-opts.js";
import { calcSource } from "./calc-source.js";
import { CliUsageError } from "./cli-usage-error.js";
import { emitPruningWarnings } from "./emit-pruning-warnings.js";
import { emitResolutionNotices } from "./emit-resolution-notices.js";
import { handleCliUsageError } from "./handle-cli-usage-error.js";
import { handleCommanderError } from "./handle-commander-error.js";
import { handleError } from "./handle-error.js";
import { handleParseError } from "./handle-parse-error.js";
import { normalizeCliOptions } from "./normalize-cli-options.js";
import { resolveOutputPath } from "./resolve-output-path/resolve-output-path.js";
import { writeOutput } from "./write-output.js";

export async function runCli(argv: readonly string[]): Promise<number> {
  const command = buildCommand();

  command.action(async (fileArg: unknown, opts: ParsedCliOptions) => {
    const file = typeof fileArg === "string" ? fileArg : null;
    const normalized = normalizeCliOptions(opts);
    const src = await calcSource(command, file, normalized);

    const emitters = createDefaultEmitterRegistry();
    const outputPath = resolveOutputPath(src, normalized, emitters);
    const { text, runOpts } = buildRunOpts(src, normalized);

    const pipeline = createDefaultPipeline(emitters);
    const result = pipeline.runDetailed(text, runOpts);

    emitResolutionNotices(result.resolutions);
    emitPruningWarnings(result.pruning);
    writeOutput(outputPath, result.text);
  });

  try {
    await command.parseAsync([...argv], { from: "user" });
  } catch (e) {
    if (e instanceof CommanderError) {
      return handleCommanderError(e);
    }
    if (e instanceof ParseError) {
      return handleParseError(e);
    }
    if (e instanceof CliUsageError) {
      return handleCliUsageError(e);
    }
    if (e instanceof Error) {
      return handleError(e);
    }
    throw e;
  }

  return 0;
}
