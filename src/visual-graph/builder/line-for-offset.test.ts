import { describe, expect, test } from "vitest";

import { lineForOffset } from "./line-for-offset.js";

describe("lineForOffset", () => {
  test.each([
    { name: "offset 0 returns 1", raw: "foo\nbar", offset: 0, expected: 1 },
    {
      name: "before first newline returns 1",
      raw: "foo\nbar",
      offset: 3,
      expected: 1,
    },
    {
      name: "right after first newline returns 2",
      raw: "foo\nbar",
      offset: 4,
      expected: 2,
    },
    {
      name: "counts each newline once",
      raw: "a\nb\nc\nd",
      offset: 6,
      expected: 4,
    },
    { name: "empty raw always returns 1", raw: "", offset: 100, expected: 1 },
    {
      name: "consecutive newlines each advance the line",
      raw: "\n\n\n",
      offset: 3,
      expected: 4,
    },
    {
      name: "carriage return alone does not advance",
      raw: "a\rb\nc",
      offset: 4,
      expected: 2,
    },
  ])("$name", ({ raw, offset, expected }) => {
    expect(lineForOffset(raw, offset)).toBe(expected);
  });

  test("clamps offset > raw.length to raw.length", () => {
    const raw = "a\nb\nc";
    expect(lineForOffset(raw, 999)).toBe(lineForOffset(raw, raw.length));
  });
});
