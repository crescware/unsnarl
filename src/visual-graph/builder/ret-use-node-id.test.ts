import { describe, expect, test } from "vitest";

import { retUseNodeId } from "./ret-use-node-id.js";

describe("retUseNodeId", () => {
  test.each([
    { input: "r42", expected: "ret_use_r42" },
    { input: "r-1.2", expected: "ret_use_r_1_2" },
    { input: "", expected: "ret_use_" },
  ])("retUseNodeId($input) = $expected", ({ input, expected }) => {
    expect(retUseNodeId(input)).toBe(expected);
  });
});
