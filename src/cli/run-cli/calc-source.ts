import type { Command } from "commander";

import { readStdin } from "../io.js";
import { CliUsageError } from "./cli-usage-error.js";
import type { ExecuteSource } from "./execute-source.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

export async function calcSource(
  command: Command,
  file: string | null,
  opts: NormalizedCliOptions,
): Promise<ExecuteSource> {
  if (opts.stdin) {
    return {
      stdin: true,
      text: await readStdin(),
      stdinLang: opts.stdinLang,
    };
  }

  if (file === null) {
    throw new CliUsageError(
      "no input file (use --stdin or pass a path)",
      command.helpInformation(),
    );
  }

  return { stdin: false, path: file };
}
