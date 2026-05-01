import { describe, expect, test } from "vitest";

import { ifContainerSubgraphId } from "./if-container-subgraph-id.js";

describe("ifContainerSubgraphId", () => {
  test.each([
    { parentScopeId: "scope1", offset: 42, expected: "cont_if_scope1_42" },
    { parentScopeId: "a.b", offset: 100, expected: "cont_if_a_b_100" },
    { parentScopeId: "s", offset: 0, expected: "cont_if_s_0" },
    { parentScopeId: "", offset: 7, expected: "cont_if__7" },
  ])(
    "ifContainerSubgraphId($parentScopeId, $offset) = $expected",
    ({ parentScopeId, offset, expected }) => {
      expect(ifContainerSubgraphId(parentScopeId, offset)).toBe(expected);
    },
  );
});
