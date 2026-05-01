import { describe, expect, test } from "vitest";

import { isFunctionSubgraph } from "./is-function-subgraph.js";
import { makeScope } from "./testing/make-scope.js";

describe("isFunctionSubgraph", () => {
  const owners = new Map<string, string>([["scope1", "var1"]]);

  test.each([
    { name: "id present in owner map -> true", id: "scope1", expected: true },
    {
      name: "id absent from owner map -> false",
      id: "scope2",
      expected: false,
    },
  ])("$name", ({ id, expected }) => {
    expect(isFunctionSubgraph(makeScope({ id }), owners)).toBe(expected);
  });

  test("empty owner map -> false", () => {
    expect(isFunctionSubgraph(makeScope({ id: "anything" }), new Map())).toBe(
      false,
    );
  });
});
