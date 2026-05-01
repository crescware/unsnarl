import { describe, expect, test } from "vitest";

import { writeOpNodeId } from "./write-op-node-id.js";

describe("writeOpNodeId", () => {
  test.each([
    { input: "ref42", expected: "wr_ref42" },
    { input: "ref:42/x", expected: "wr_ref_42_x" },
    { input: "", expected: "wr_" },
  ])("writeOpNodeId($input) = $expected", ({ input, expected }) => {
    expect(writeOpNodeId(input)).toBe(expected);
  });
});
