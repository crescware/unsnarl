export type CliLanguage = "ts" | "tsx" | "js" | "jsx";

export interface CliArgs {
  format: string;
  stdin: boolean;
  language: CliLanguage;
  pretty: boolean;
  listFormats: boolean;
  help: boolean;
  version: boolean;
  inputFile: string | null;
}

export interface CliParseSuccess {
  ok: true;
  args: CliArgs;
}

export interface CliParseFailure {
  ok: false;
  error: string;
}

export type CliParseResult = CliParseSuccess | CliParseFailure;

const LANGUAGES: ReadonlySet<string> = new Set([
  "ts",
  "tsx",
  "js",
  "jsx",
] satisfies CliLanguage[]);

export function parseCliArgs(argv: ReadonlyArray<string>): CliParseResult {
  let format = "ir";
  let stdin = false;
  let language: CliLanguage = "ts";
  let pretty = true;
  let listFormats = false;
  let help = false;
  let version = false;
  let inputFile: string | null = null;

  let i = 0;
  while (i < argv.length) {
    const arg = argv[i];
    if (arg === undefined) {
      break;
    }
    if (arg === "-f" || arg === "--format") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for ${arg}` };
      }
      format = next;
      i += 2;
      continue;
    }
    if (arg === "--lang") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for --lang` };
      }
      if (!LANGUAGES.has(next)) {
        return { ok: false, error: `Invalid language: ${next}` };
      }
      language = next as CliLanguage;
      i += 2;
      continue;
    }
    if (arg === "--stdin") {
      stdin = true;
      i += 1;
      continue;
    }
    if (arg === "--pretty") {
      pretty = true;
      i += 1;
      continue;
    }
    if (arg === "--no-pretty") {
      pretty = false;
      i += 1;
      continue;
    }
    if (arg === "--list-formats") {
      listFormats = true;
      i += 1;
      continue;
    }
    if (arg === "-h" || arg === "--help") {
      help = true;
      i += 1;
      continue;
    }
    if (arg === "-v" || arg === "--version") {
      version = true;
      i += 1;
      continue;
    }
    if (arg.startsWith("-")) {
      return { ok: false, error: `Unknown option: ${arg}` };
    }
    if (inputFile !== null) {
      return { ok: false, error: `Multiple input files: ${arg}` };
    }
    inputFile = arg;
    i += 1;
  }

  return {
    ok: true,
    args: {
      format,
      stdin,
      language,
      pretty,
      listFormats,
      help,
      version,
      inputFile,
    },
  };
}

export function usage(): string {
  return `Usage:
  unsnarl <file>                      Print SerializedIR (JSON) to stdout
  unsnarl --format mermaid <file>     Print Mermaid flowchart to stdout
  cat foo.ts | unsnarl --stdin --lang ts

Options:
  -f, --format <id>            Emitter format (default: ir)
  --stdin                      Read from stdin
  --lang <ts|tsx|js|jsx>       Language for stdin input (default: ts)
  --pretty / --no-pretty       Pretty-print JSON (default: pretty)
  --list-formats               List registered emitters
  -h, --help                   Show this help
  -v, --version                Show version
`;
}
