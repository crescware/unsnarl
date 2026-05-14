import { describe, expect, test } from "vitest";

import { asFilledString } from "../../util/filled-string.js";
import { lineForOffset } from "./line-for-offset.js";

describe("lineForOffset", () => {
  test.each([
    {
      name: asFilledString("offset 0 returns 1"),
      raw: "foo\nbar",
      offset: 0,
      expected: 1,
    },
    {
      name: asFilledString("before first newline returns 1"),
      raw: "foo\nbar",
      offset: 3,
      expected: 1,
    },
    {
      name: asFilledString("right after first newline returns 2"),
      raw: "foo\nbar",
      offset: 4,
      expected: 2,
    },
    {
      name: asFilledString("counts each newline once"),
      raw: "a\nb\nc\nd",
      offset: 6,
      expected: 4,
    },
    {
      name: asFilledString("empty raw always returns 1"),
      raw: "",
      offset: 100,
      expected: 1,
    },
    {
      name: asFilledString("consecutive newlines each advance the line"),
      raw: "\n\n\n",
      offset: 3,
      expected: 4,
    },
    {
      name: asFilledString("carriage return alone does not advance"),
      raw: "a\rb\nc",
      offset: 4,
      expected: 2,
    },
  ])("$name", ({ raw, offset, expected }) => {
    expect(lineForOffset(raw, offset)).toEqual(expected);
  });

  test("clamps offset > raw.length to raw.length", () => {
    const raw = "a\nb\nc";
    expect(lineForOffset(raw, 999)).toEqual(lineForOffset(raw, raw.length));
  });
});
