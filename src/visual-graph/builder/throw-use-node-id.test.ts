import { describe, expect, test } from "vitest";

import { throwUseNodeId } from "./throw-use-node-id.js";

describe("throwUseNodeId", () => {
  test.each([
    { input: "r42", expected: "throw_use_r42" },
    { input: "r-1.2", expected: "throw_use_r_1_2" },
    { input: "", expected: "throw_use_" },
  ])("throwUseNodeId($input) = $expected", ({ input, expected }) => {
    expect(throwUseNodeId(input)).toEqual(expected);
  });
});
