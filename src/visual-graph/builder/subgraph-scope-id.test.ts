import { describe, expect, test } from "vitest";

import { subgraphScopeId } from "./subgraph-scope-id.js";
import { baseScope } from "./testing/make-scope.js";

describe("subgraphScopeId", () => {
  test.each([
    { id: "scope1", expected: "s_scope1" },
    { id: "scope.1-x", expected: "s_scope_1_x" },
    { id: "", expected: "s_" },
  ])("subgraphScopeId(scope id=$id) = $expected", ({ id, expected }) => {
    expect(subgraphScopeId({ ...baseScope(), id })).toBe(expected);
  });
});
