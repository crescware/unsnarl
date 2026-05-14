import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import { subgraphScopeId } from "./subgraph-scope-id.js";
import { baseScope } from "./testing/make-scope.js";

describe("subgraphScopeId", () => {
  test.each([
    { id: "scope1", expected: "s_scope1" },
    { id: "scope.1-x", expected: "s_scope_1_x" },
  ])("subgraphScopeId(scope id=$id) = $expected", ({ id, expected }) => {
    expect(subgraphScopeId({ ...baseScope(), id: asScopeId(id) })).toEqual(
      expected,
    );
  });
});
