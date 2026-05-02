import { Command, InvalidArgumentError, Option } from "commander";

import { NAME } from "../../name.js";
import { VERSION } from "../../version.js";
import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import { type CliLanguage, LANGUAGES } from "./cli-language.js";
import type { CliMermaidRenderer } from "./cli-mermaid-renderer.js";
import { mermaidRendererOptions } from "./mermaid-renderer-options.js";
import { scopeOptions } from "./scope-options.js";

function coerceLanguage(value: string): CliLanguage {
  if (!LANGUAGES.has(value)) {
    throw new InvalidArgumentError(`Invalid language: ${value}`);
  }
  return value as CliLanguage;
}

export type ParsedCliOptions = Readonly<{
  format: string;
  stdin?: true;
  lang: CliLanguage;
  pretty: boolean;
  mermaidRenderer?: CliMermaidRenderer;
  roots: readonly ParsedRootQuery[];
  descendants?: number;
  ancestors?: number;
  context?: number;
  outDir?: string;
}>;

export function buildCommand(availableFormats: readonly string[]): Command {
  const command = new Command();
  command
    .name(NAME)
    .description("Generate visual graphs from JS/TS source")
    .version(VERSION, "-v, --version", "Show version")
    .allowExcessArguments(false)
    .exitOverride()
    .argument("[file]", "Input file");

  const options: readonly Option[] = [
    new Option(
      "-f, --format <id>",
      `Emitter format (${availableFormats.join(", ")})`,
    ).default("ir"),
    new Option("--stdin", "Read from stdin"),
    new Option("--lang <lang>", "Language for stdin input")
      .argParser(coerceLanguage)
      .default("ts" as CliLanguage),
    new Option("--no-pretty", "Disable pretty-printed JSON output"),
    ...mermaidRendererOptions(),
    ...scopeOptions(),
    new Option(
      "-o, --out-dir <dir>",
      "Write output to <dir>/<auto-name>.<ext>",
    ),
  ];

  for (const opt of options) {
    command.addOption(opt);
  }

  return command;
}
