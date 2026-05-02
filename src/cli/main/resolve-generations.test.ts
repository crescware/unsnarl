import { describe, expect, test } from "vitest";

import { DEFAULT_GENERATIONS } from "./default-generations.js";
import { resolveGenerations } from "./resolve-generations.js";

describe("resolveGenerations", () => {
  test("no flag → symmetric DEFAULT_GENERATIONS on both sides", () => {
    expect(
      resolveGenerations({ descendants: null, ancestors: null, context: null }),
    ).toEqual({
      descendants: DEFAULT_GENERATIONS,
      ancestors: DEFAULT_GENERATIONS,
    });
  });

  test("only -A specified → other side falls to 0 (asymmetric grep semantics)", () => {
    expect(
      resolveGenerations({ descendants: 3, ancestors: null, context: null }),
    ).toEqual({ descendants: 3, ancestors: 0 });
  });

  test("only -B specified → other side falls to 0", () => {
    expect(
      resolveGenerations({ descendants: null, ancestors: 4, context: null }),
    ).toEqual({ descendants: 0, ancestors: 4 });
  });

  test("only -C specified → both sides take the context value", () => {
    expect(
      resolveGenerations({ descendants: null, ancestors: null, context: 5 }),
    ).toEqual({ descendants: 5, ancestors: 5 });
  });

  test("-C plus -A only → -A overrides; -B inherits context", () => {
    expect(
      resolveGenerations({ descendants: 1, ancestors: null, context: 5 }),
    ).toEqual({ descendants: 1, ancestors: 5 });
  });

  test("-A and -B both explicit → -C is irrelevant", () => {
    expect(
      resolveGenerations({ descendants: 1, ancestors: 2, context: 99 }),
    ).toEqual({ descendants: 1, ancestors: 2 });
  });

  test("zero is preserved (treated as explicit, not falsy)", () => {
    expect(
      resolveGenerations({ descendants: 0, ancestors: null, context: null }),
    ).toEqual({ descendants: 0, ancestors: 0 });
  });
});
