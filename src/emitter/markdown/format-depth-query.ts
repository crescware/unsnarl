import { DEFAULT_DEPTH } from "../../cli/args/depth-options.js";
import type { NestingDepths } from "../../ir/annotations/scope-annotation.js";
import {
  NESTING_KIND,
  type NestingKind,
} from "../../serializer/nesting-kind.js";

const NON_FUNCTION_KINDS: readonly NestingKind[] = [
  NESTING_KIND.If,
  NESTING_KIND.For,
  NESTING_KIND.While,
  NESTING_KIND.Switch,
  NESTING_KIND.TryCatchFinally,
  NESTING_KIND.Block,
];

// Render the CLI invocation that would have produced these per-kind
// thresholds. Returns null when every kind is at DEFAULT_DEPTH (i.e.
// the user did not narrow anything). The CLI surface is just --depth /
// --depth-function / --depth-block, so this picks the shortest
// equivalent of those three; when the non-function kinds disagree
// (programmatic API only, not reachable from the CLI) we fall back to a
// per-kind sh-comment listing so the snapshot still records exactly
// what was applied.
export function formatDepthQuery(
  depths: NestingDepths | undefined,
): string | null {
  if (!depths) {
    return null;
  }
  const fn = depths[NESTING_KIND.Function];
  const blockValues = NON_FUNCTION_KINDS.map((k) => depths[k]);
  const firstBlock = blockValues[0] ?? DEFAULT_DEPTH;
  const blockUniform = blockValues.every((v) => v === firstBlock);

  if (blockUniform) {
    const block = firstBlock;
    if (fn === DEFAULT_DEPTH && block === DEFAULT_DEPTH) {
      return null;
    }
    if (fn === block) {
      return `--depth ${fn}`;
    }
    if (fn === DEFAULT_DEPTH) {
      return `--depth-block ${block}`;
    }
    if (block === DEFAULT_DEPTH) {
      return `--depth-function ${fn}`;
    }
    return `--depth-function ${fn} --depth-block ${block}`;
  }

  // Non-uniform non-function kinds -- only reachable via the
  // programmatic API. Surface every kind so the snapshot is faithful
  // even though no exact CLI flag string can produce it.
  const parts: /* mutable */ string[] = [];
  if (fn !== DEFAULT_DEPTH) {
    parts.push(`function=${fn}`);
  }
  for (const k of NON_FUNCTION_KINDS) {
    if (depths[k] !== DEFAULT_DEPTH) {
      parts.push(`${k}=${depths[k]}`);
    }
  }
  if (parts.length === 0) {
    return null;
  }
  return `# nesting depth: ${parts.join(" ")}`;
}
