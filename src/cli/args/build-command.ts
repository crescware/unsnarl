import { Command, InvalidArgumentError, Option } from "commander";

import type { CliLanguage } from "../../language.js";
import { NAME } from "../../name.js";
import { VERSION } from "../../version.js";
import { LANGUAGES } from "./cli-language.js";
import { formatOptions } from "./format-options.js";
import { scopeOptions } from "./scope-options.js";

function coerceLanguage(value: string): CliLanguage {
  if (!LANGUAGES.has(value)) {
    throw new InvalidArgumentError(`Invalid language: ${value}`);
  }
  return value as CliLanguage;
}

export function buildCommand(): Command {
  const command = new Command();
  command
    .name(NAME)
    .description("Generate visual graphs from JS/TS source")
    .version(VERSION, "-v, --version", "Show version")
    .allowExcessArguments(false)
    .exitOverride()
    .argument("[file]", "Input file");

  const options: readonly Option[] = [
    ...formatOptions(),
    new Option("--stdin", "Read from stdin").default(false),
    new Option("--stdin-lang <lang>", "Language for stdin input")
      .argParser(coerceLanguage)
      .default("ts" as CliLanguage),
    ...scopeOptions(),
    new Option(
      "-o, --out-dir <dir>",
      "Write output to <dir>/<auto-name>.<ext>",
    ).default(null, "stdout"),
  ];

  for (const opt of options) {
    command.addOption(opt);
  }

  return command;
}
