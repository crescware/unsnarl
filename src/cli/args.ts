import { type ParsedRootQuery, parseRootQueries } from "./root-query.js";

export type CliLanguage = "ts" | "tsx" | "js" | "jsx";
export type CliMermaidRenderer = "dagre" | "elk";

export interface CliArgs {
  format: string;
  stdin: boolean;
  language: CliLanguage;
  pretty: boolean;
  listFormats: boolean;
  help: boolean;
  version: boolean;
  inputFile: string | null;
  mermaidRenderer: CliMermaidRenderer | null;
  roots: readonly ParsedRootQuery[];
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
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

const MERMAID_RENDERERS: ReadonlySet<string> = new Set([
  "dagre",
  "elk",
] satisfies CliMermaidRenderer[]);

export function parseCliArgs(argv: ReadonlyArray<string>): CliParseResult {
  let format = "ir";
  let stdin = false;
  let language: CliLanguage = "ts";
  let pretty = true;
  let listFormats = false;
  let help = false;
  let version = false;
  let inputFile: string | null = null;
  let mermaidRenderer: CliMermaidRenderer | null = null;
  const roots: ParsedRootQuery[] = [];
  let descendants: number | null = null;
  let ancestors: number | null = null;
  let context: number | null = null;

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
    if (arg === "--mermaid-renderer") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for --mermaid-renderer` };
      }
      if (!MERMAID_RENDERERS.has(next)) {
        return { ok: false, error: `Invalid mermaid renderer: ${next}` };
      }
      mermaidRenderer = next as CliMermaidRenderer;
      i += 2;
      continue;
    }
    if (arg === "-r" || arg === "--roots") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for ${arg}` };
      }
      const parsed = parseRootQueries(next);
      if (!parsed.ok) {
        return { ok: false, error: parsed.error };
      }
      roots.push(...parsed.queries);
      i += 2;
      continue;
    }
    if (arg === "-A" || arg === "--descendants") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for ${arg}` };
      }
      const n = parseGenerationCount(next);
      if (n === null) {
        return {
          ok: false,
          error: `Invalid value for ${arg}: ${next} (expected non-negative integer)`,
        };
      }
      descendants = n;
      i += 2;
      continue;
    }
    if (arg === "-B" || arg === "--ancestors") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for ${arg}` };
      }
      const n = parseGenerationCount(next);
      if (n === null) {
        return {
          ok: false,
          error: `Invalid value for ${arg}: ${next} (expected non-negative integer)`,
        };
      }
      ancestors = n;
      i += 2;
      continue;
    }
    if (arg === "-C" || arg === "--context") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for ${arg}` };
      }
      const n = parseGenerationCount(next);
      if (n === null) {
        return {
          ok: false,
          error: `Invalid value for ${arg}: ${next} (expected non-negative integer)`,
        };
      }
      context = n;
      i += 2;
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
      mermaidRenderer,
      roots,
      descendants,
      ancestors,
      context,
    },
  };
}

function parseGenerationCount(value: string): number | null {
  if (!/^\d+$/.test(value)) {
    return null;
  }
  return Number.parseInt(value, 10);
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
  --mermaid-renderer <dagre|elk>
                               Layout engine for Mermaid output (default: elk).
                               Use 'dagre' when the consumer cannot register
                               the elk loader (e.g. GitHub markdown preview).
  -r, --roots <queries>        Comma-separated root queries to prune the graph.
                               Each query is one of:
                                 n           line n
                                 n:id        line n with identifier id
                                 n-m         line range [n,m]
                                 n-m:id      line range [n,m] with identifier id
                                 id          identifier across all scopes
                               Repeat -r/--roots to add more queries.
  -A, --descendants <N>        Keep N generations of descendants from each root
                               (default: --context value, else 10).
  -B, --ancestors <N>          Keep N generations of ancestors from each root
                               (default: --context value, else 10).
  -C, --context <N>            Shorthand for -A N -B N.
  --list-formats               List registered emitters
  -h, --help                   Show this help
  -v, --version                Show version
`;
}
