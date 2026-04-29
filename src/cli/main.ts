#!/usr/bin/env node
import { ParseError } from "../parser/oxc.js";
import {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "../pipeline/default.js";
import { parseCliArgs, usage } from "./args.js";
import { readSourceFile, readStdin } from "./io.js";

const VERSION = "0.0.0";

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
  try {
    const out = pipeline.run(code, {
      format: args.format,
      language,
      sourcePath,
      emit: { pretty: args.pretty },
    });
    process.stdout.write(out);
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

function detectLanguage(
  path: string | null,
  fallback: "ts" | "tsx" | "js" | "jsx",
): "ts" | "tsx" | "js" | "jsx" {
  if (path === null) {
    return fallback;
  }
  if (path.endsWith(".tsx")) {
    return "tsx";
  }
  if (path.endsWith(".jsx")) {
    return "jsx";
  }
  if (path.endsWith(".js")) {
    return "js";
  }
  return "ts";
}

const isMain = import.meta.url === `file://${process.argv[1] ?? ""}`;
if (isMain) {
  runCli(process.argv.slice(2))
    .then((code) => {
      process.exit(code);
    })
    .catch((e: unknown) => {
      process.stderr.write(
        `fatal: ${e instanceof Error ? e.message : String(e)}\n`,
      );
      process.exit(1);
    });
}
