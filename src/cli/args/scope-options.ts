import { InvalidArgumentError, Option } from "commander";

import { parseRootQueries } from "../root-query/parse-root-queries.js";
import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import { DEFAULT_GENERATIONS } from "./default-generations.js";
import { parseGenerationCount } from "./parse-generation-count.js";

function coerceGenerations(flag: string): (value: string) => number {
  return (value: string): number => {
    const n = parseGenerationCount(value);
    if (n === null) {
      throw new InvalidArgumentError(
        `Invalid value for ${flag}: ${value} (expected non-negative integer)`,
      );
    }
    return n;
  };
}

function collectRoots(
  value: string,
  prev: readonly ParsedRootQuery[],
): readonly ParsedRootQuery[] {
  const r = parseRootQueries(value);
  if (!r.ok) {
    throw new InvalidArgumentError(r.error);
  }
  return [...prev, ...r.queries];
}

export function scopeOptions(): readonly Option[] {
  return [
    new Option(
      "-r, --roots <queries>",
      "Comma-separated root queries (repeatable)",
    )
      .argParser(collectRoots)
      .default([] as readonly ParsedRootQuery[]),

    new Option("-A, --descendants <N>", "Descendants generations")
      .argParser(coerceGenerations("-A"))
      .default(
        null,
        `${DEFAULT_GENERATIONS} if no scope flag given, otherwise 0`,
      ),

    new Option("-B, --ancestors <N>", "Ancestors generations")
      .argParser(coerceGenerations("-B"))
      .default(
        null,
        `${DEFAULT_GENERATIONS} if no scope flag given, otherwise 0`,
      ),

    new Option("-C, --context <N>", "Context generations (-A and -B shorthand)")
      .argParser(coerceGenerations("-C"))
      .default(null, `${DEFAULT_GENERATIONS}`),
  ];
}
