import { describe, expect, test } from "vitest";

import { DEFAULT_GENERATIONS } from "../args/default-generations.js";
import { resolveGenerations } from "./resolve-generations.js";

describe("resolveGenerations", () => {
  test.each<{
    name: string;
    input: {
      descendants: number | null;
      ancestors: number | null;
      context: number | null;
    };
    expected: { descendants: number; ancestors: number };
  }>([
    {
      name: "no flag → symmetric DEFAULT_GENERATIONS on both sides",
      input: { descendants: null, ancestors: null, context: null },
      expected: {
        descendants: DEFAULT_GENERATIONS,
        ancestors: DEFAULT_GENERATIONS,
      },
    },
    {
      name: "only -A specified → other side falls to 0 (asymmetric grep semantics)",
      input: { descendants: 3, ancestors: null, context: null },
      expected: { descendants: 3, ancestors: 0 },
    },
    {
      name: "only -B specified → other side falls to 0",
      input: { descendants: null, ancestors: 4, context: null },
      expected: { descendants: 0, ancestors: 4 },
    },
    {
      name: "only -C specified → both sides take the context value",
      input: { descendants: null, ancestors: null, context: 5 },
      expected: { descendants: 5, ancestors: 5 },
    },
    {
      name: "-C plus -A only → -A overrides; -B inherits context",
      input: { descendants: 1, ancestors: null, context: 5 },
      expected: { descendants: 1, ancestors: 5 },
    },
    {
      name: "-A and -B both explicit → -C is irrelevant",
      input: { descendants: 1, ancestors: 2, context: 99 },
      expected: { descendants: 1, ancestors: 2 },
    },
    {
      name: "zero is preserved (treated as explicit, not falsy)",
      input: { descendants: 0, ancestors: null, context: null },
      expected: { descendants: 0, ancestors: 0 },
    },
  ])("$name", ({ input, expected }) => {
    expect(resolveGenerations(input)).toEqual(expected);
  });
});
