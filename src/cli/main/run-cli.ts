import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { ParseError } from "../../parser/oxc.js";
import {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "../../pipeline/default.js";
import type { PruningRunOptions } from "../../pipeline/types.js";
import { parseCliArgs } from "../args/parse-cli-args.js";
import { usage } from "../args/usage.js";
import { readSourceFile, readStdin } from "../io.js";
import { deriveOutputBasename } from "../output-name/output-name.js";
import { detectLanguage } from "./detect-language.js";
import { resolveGenerations } from "./resolve-generations.js";
import { VERSION } from "./version.js";

export async function runCli(argv: ReadonlyArray<string>): Promise<number> {
  const parsed = parseCliArgs(argv);
  if (!parsed.ok) {
    process.stderr.write(`error: ${parsed.error}\n`);
    process.stderr.write(usage());
    return 2;
  }
  const args = parsed.args;

  if (args.help) {
    process.stdout.write(usage());
    return 0;
  }
  if (args.version) {
    process.stdout.write(`${VERSION}\n`);
    return 0;
  }

  // CLI lets the user omit --mermaid-renderer (= null). The internal API
  // demands an explicit choice, so the default is resolved here at the
  // boundary instead of being smuggled into the emitter as undefined.
  const emitters = createDefaultEmitterRegistry({
    mermaidRenderer: args.mermaidRenderer ?? "elk",
  });
  if (args.listFormats) {
    for (const f of emitters.list()) {
      process.stdout.write(`${f}\n`);
    }
    return 0;
  }

  // Validate the output destination up-front: if -o/--out-dir cannot
  // produce a filename (e.g. --stdin without -r), bail out before we read
  // any input or do any analysis.
  let outputPath: string | null = null;
  if (args.outDir !== null) {
    const derived = deriveOutputBasename({
      roots: args.roots,
      descendants: args.descendants,
      ancestors: args.ancestors,
      context: args.context,
      // --stdin overrides any positional file for content, so it should
      // override it for naming too: a stdin run has no usable filename.
      inputPath: args.stdin ? null : args.inputFile,
    });
    if (!derived.ok) {
      process.stderr.write(`error: ${derived.error}\n`);
      return 2;
    }
    const emitter = emitters.get(args.format);
    if (emitter === undefined) {
      const available = emitters.list().join(", ");
      process.stderr.write(
        `error: Unknown emitter format: ${args.format}. Available: ${available}\n`,
      );
      return 1;
    }
    outputPath = join(args.outDir, `${derived.basename}.${emitter.extension}`);
  }

  let code: string;
  let sourcePath: string;
  if (args.stdin) {
    code = await readStdin();
    sourcePath = `stdin.${args.language}`;
  } else if (args.inputFile !== null) {
    code = readSourceFile(args.inputFile);
    sourcePath = args.inputFile;
  } else {
    process.stderr.write("error: no input file (use --stdin or pass a path)\n");
    process.stderr.write(usage());
    return 2;
  }

  const language = args.stdin
    ? args.language
    : detectLanguage(args.inputFile, args.language);

  const pipeline = createDefaultPipeline(emitters);
  const baseRunOpts = {
    format: args.format,
    language,
    sourcePath,
    emit: { pretty: args.pretty },
  };
  const pruning: PruningRunOptions | null =
    args.roots.length > 0
      ? {
          roots: args.roots,
          ...resolveGenerations(args),
        }
      : null;

  try {
    const result = pipeline.runDetailed(
      code,
      pruning === null ? baseRunOpts : { ...baseRunOpts, pruning },
    );
    if (result.pruning !== null) {
      for (const r of result.pruning) {
        if (r.matched === 0) {
          process.stderr.write(
            `unsnarl: warning: query '${r.query}' matched 0 roots\n`,
          );
        }
      }
    }
    if (outputPath !== null && args.outDir !== null) {
      mkdirSync(args.outDir, { recursive: true });
      writeFileSync(outputPath, result.text);
    } else {
      process.stdout.write(result.text);
    }
    return 0;
  } catch (e) {
    if (e instanceof ParseError) {
      process.stderr.write(`parse error: ${e.message}\n`);
      return 1;
    }
    process.stderr.write(
      `error: ${e instanceof Error ? e.message : String(e)}\n`,
    );
    return 1;
  }
}
