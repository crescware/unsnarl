import { DEFAULT_GENERATIONS } from "./cli-args.js";

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
  -A, --descendants <N>        Keep N generations of descendants from each root.
                               Default: ${DEFAULT_GENERATIONS} if no radius flag is given;
                               --context value if -C is given;
                               otherwise 0 (asymmetric, like grep -A/-B).
  -B, --ancestors <N>          Keep N generations of ancestors from each root.
                               Default: ${DEFAULT_GENERATIONS} if no radius flag is given;
                               --context value if -C is given;
                               otherwise 0 (asymmetric, like grep -A/-B).
  -C, --context <N>            Shorthand for -A N -B N (overridable by -A/-B).
  -o, --out-dir <dir>          Write output to <dir>/<auto-name>.<ext>
                               instead of stdout. The filename is derived
                               from -r queries and -A/-B/-C, e.g.
                                 -r value -A 1   -> value-a1.<ext>
                                 -r 10-12 -C 2   -> l10-12-c2.<ext>
                               When -r is omitted, the input filename
                               (without extension) is used. The directory
                               is created if missing.
  --list-formats               List registered emitters
  -h, --help                   Show this help
  -v, --version                Show version
`;
}
