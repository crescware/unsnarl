import { describe, expect, test } from "vitest";

import { ifTestNodeId } from "./if-test-node-id.js";

describe("ifTestNodeId", () => {
  test.each([
    { parentScopeId: "scope1", offset: 42, expected: "if_test_scope1_42" },
    { parentScopeId: "a.b", offset: 100, expected: "if_test_a_b_100" },
    { parentScopeId: "s", offset: 0, expected: "if_test_s_0" },
    { parentScopeId: "", offset: 7, expected: "if_test__7" },
  ])(
    "ifTestNodeId($parentScopeId, $offset) = $expected",
    ({ parentScopeId, offset, expected }) => {
      expect(ifTestNodeId(parentScopeId, offset)).toBe(expected);
    },
  );
});
