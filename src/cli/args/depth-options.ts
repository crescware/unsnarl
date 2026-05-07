import { InvalidArgumentError, Option } from "commander";

import type { CategoryDepths } from "../../ir/annotations/scope-annotation.js";
import { makeDepths } from "../../serializer/category.js";
import { parseGenerationCount } from "./parse-generation-count.js";

export const DEFAULT_DEPTH = 10;

export function defaultDepths(): CategoryDepths {
  return makeDepths(DEFAULT_DEPTH);
}

function coerceDepth(flag: string): (value: string) => number {
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

export function depthOptions(): readonly Option[] {
  return [
    new Option(
      "--depth <N>",
      "Sugar: set both --depth-function and --depth-block to <N>",
    )
      .argParser(coerceDepth("--depth"))
      .default(null, `${DEFAULT_DEPTH}`),

    new Option(
      "--depth-function <N>",
      "Max function-scope nesting depth before scopes collapse to a single node",
    )
      .argParser(coerceDepth("--depth-function"))
      .default(null, `${DEFAULT_DEPTH}`),

    new Option(
      "--depth-block <N>",
      "Max block-scope nesting depth (applies to if/for/while/switch/try-catch-finally/block) before scopes collapse to a single node",
    )
      .argParser(coerceDepth("--depth-block"))
      .default(null, `${DEFAULT_DEPTH}`),
  ];
}
