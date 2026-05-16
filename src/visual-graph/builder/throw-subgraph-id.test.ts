import { describe, expect, test } from "vitest";

import { throwSubgraphId } from "./throw-subgraph-id.js";

describe("throwSubgraphId", () => {
  test.each([
    { varId: "v1", containerKey: "10-20", expected: "s_throw_v1_10_20" },
    {
      varId: "v.1",
      containerKey: "implicit",
      expected: "s_throw_v_1_implicit",
    },
    { varId: "", containerKey: "", expected: "s_throw__" },
    { varId: "owner-x", containerKey: "5-9", expected: "s_throw_owner_x_5_9" },
  ])(
    "throwSubgraphId($varId, $containerKey) = $expected",
    ({ varId, containerKey, expected }) => {
      expect(throwSubgraphId(varId, containerKey)).toEqual(expected);
    },
  );
});
