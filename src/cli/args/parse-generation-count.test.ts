import { describe, expect, test } from "vitest";

import { parseGenerationCount } from "./parse-generation-count.js";

describe("parseGenerationCount", () => {
  test.each<{ name: string; input: string; expected: number | null }>([
    { name: "non-negative integer 0 → 0", input: "0", expected: 0 },
    { name: "non-negative integer 1 → 1", input: "1", expected: 1 },
    { name: "non-negative integer 42 → 42", input: "42", expected: 42 },
    { name: "negative -1 → null", input: "-1", expected: null },
    { name: "decimal 1.5 → null", input: "1.5", expected: null },
    { name: "non-numeric abc → null", input: "abc", expected: null },
    { name: "empty string → null", input: "", expected: null },
    { name: "whitespace ' 1 ' → null", input: " 1 ", expected: null },
    { name: "leading + '+1' → null", input: "+1", expected: null },
    { name: "hex '0x10' → null", input: "0x10", expected: null },
    { name: "scientific '1e3' → null", input: "1e3", expected: null },
  ])("$name", ({ input, expected }) => {
    expect(parseGenerationCount(input)).toBe(expected);
  });
});
