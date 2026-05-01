import { describe, expect, test } from "vitest";

import { nodeId } from "./node-id.js";

describe("nodeId", () => {
  test.each([
    { input: "foo123", expected: "n_foo123" },
    { input: "a.b-c", expected: "n_a_b_c" },
    { input: "", expected: "n_" },
  ])("nodeId($input) = $expected", ({ input, expected }) => {
    expect(nodeId(input)).toBe(expected);
  });
});
