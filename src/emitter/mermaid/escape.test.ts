import { describe, expect, test } from "vitest";

import { escape } from "./escape.js";

describe("escape", () => {
  test.each([
    { name: "ampersand", input: "a&b", expected: "a&amp;b" },
    { name: "double quote", input: 'a"b', expected: "a&quot;b" },
    { name: "less-than", input: "a<b", expected: "a&lt;b" },
    { name: "greater-than", input: "a>b", expected: "a&gt;b" },
    {
      name: "all four together",
      input: '&"<>',
      expected: "&amp;&quot;&lt;&gt;",
    },
    {
      name: "ampersand-first ordering preserves nested entities",
      input: "&lt;",
      expected: "&amp;lt;",
    },
    { name: "no-op for plain ASCII", input: "abc 123", expected: "abc 123" },
    { name: "empty string", input: "", expected: "" },
  ])("$name", ({ input, expected }) => {
    expect(escape(input)).toBe(expected);
  });

  test("does NOT escape single quotes", () => {
    expect(escape("a'b")).toBe("a'b");
  });
});
