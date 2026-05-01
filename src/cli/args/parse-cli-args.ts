import { parseRootQueries } from "../root-query/parse-root-queries.js";
import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import { LANGUAGES, type CliLanguage } from "./cli-language.js";
import {
  MERMAID_RENDERERS,
  type CliMermaidRenderer,
} from "./cli-mermaid-renderer.js";
import type { CliParseResult } from "./cli-parse-result.js";
import { parseGenerationCount } from "./parse-generation-count.js";

export function parseCliArgs(argv: readonly string[]): CliParseResult {
  let format = "ir";
  let stdin = false;
  let language: CliLanguage = "ts";
  let pretty = true;
  let listFormats = false;
  let help = false;
  let version = false;
  let inputFile: string | null = null;
  let mermaidRenderer: CliMermaidRenderer | null = null;
  const roots: /* mutable */ ParsedRootQuery[] = [];
  let descendants: number | null = null;
  let ancestors: number | null = null;
  let context: number | null = null;
  let outDir: string | null = null;

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
    if (arg === "-o" || arg === "--out-dir") {
      const next = argv[i + 1];
      if (next === undefined) {
        return { ok: false, error: `Missing value for ${arg}` };
      }
      outDir = next;
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
      outDir,
    },
  };
}
