import { CommanderError } from "commander";

import { ParseError } from "../../../parser/oxc.js";
import {
  buildCommand,
  type ParsedCliOptions,
} from "../../args/build-command.js";
import { calcSource } from "./calc-source.js";
import { CliUsageError } from "./cli-usage-error.js";
import { execute } from "./execute.js";
import { handleCliUsageError } from "./handle-cli-usage-error.js";
import { handleCommanderError } from "./handle-commander-error.js";
import { handleError } from "./handle-error.js";
import { handleParseError } from "./handle-parse-error.js";
import { normalizeCliOptions } from "./normalized-cli-options.js";

export async function runCli(argv: readonly string[]): Promise<number> {
  const command = buildCommand();

  command.action(async (file: string | undefined, opts: ParsedCliOptions) => {
    const normalized = normalizeCliOptions(opts);
    const src = await calcSource(command, file, normalized);
    await execute(src, normalized);
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
