import { describe, expect, test } from "vitest";

import { returnSubgraphId } from "./return-subgraph-id.js";

describe("returnSubgraphId", () => {
  test.each([
    { varId: "v1", containerKey: "10-20", expected: "s_return_v1_10_20" },
    {
      varId: "v.1",
      containerKey: "implicit",
      expected: "s_return_v_1_implicit",
    },
    { varId: "", containerKey: "", expected: "s_return__" },
    { varId: "owner-x", containerKey: "5-9", expected: "s_return_owner_x_5_9" },
  ])(
    "returnSubgraphId($varId, $containerKey) = $expected",
    ({ varId, containerKey, expected }) => {
      expect(returnSubgraphId(varId, containerKey)).toBe(expected);
    },
  );
});
