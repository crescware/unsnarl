import { describe, expect, test } from "vitest";

import { parseGenerationCount } from "./parse-generation-count.js";

describe("parseGenerationCount", () => {
  test("parses non-negative integers", () => {
    expect(parseGenerationCount("0")).toBe(0);
    expect(parseGenerationCount("1")).toBe(1);
    expect(parseGenerationCount("42")).toBe(42);
  });

  test("rejects negative numbers", () => {
    expect(parseGenerationCount("-1")).toBeNull();
  });

  test("rejects decimals", () => {
    expect(parseGenerationCount("1.5")).toBeNull();
  });

  test("rejects non-numeric input", () => {
    expect(parseGenerationCount("abc")).toBeNull();
    expect(parseGenerationCount("")).toBeNull();
  });

  test("rejects whitespace and signs prefixed", () => {
    expect(parseGenerationCount(" 1 ")).toBeNull();
    expect(parseGenerationCount("+1")).toBeNull();
  });

  test("rejects hex/octal/scientific notation", () => {
    expect(parseGenerationCount("0x10")).toBeNull();
    expect(parseGenerationCount("1e3")).toBeNull();
  });
});
