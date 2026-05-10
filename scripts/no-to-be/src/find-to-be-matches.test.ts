import { describe, expect, test } from "vitest";

import { findToBeMatches } from "./find-to-be-matches";

describe("findToBeMatches()", () => {
  describe("toBe として一致する場合", () => {
    test.each([
      {
        description: "toBe",
        content: "expect(value).toBe(true);",
        expected: [".toBe("],
      },
      {
        description: "toBeDefined",
        content: "expect(value).toBeDefined();",
        expected: [".toBeDefined("],
      },
      {
        description: "toBeNull",
        content: "expect(value).toBeNull();",
        expected: [".toBeNull("],
      },
      {
        description: "toBeInstanceOf",
        content: "expect(value).toBeInstanceOf(AbortSignal);",
        expected: [".toBeInstanceOf("],
      },
      {
        description: "toBeTruthy",
        content: "expect(value).toBeTruthy();",
        expected: [".toBeTruthy("],
      },
      {
        description: "toBeFalsy",
        content: "expect(value).toBeFalsy();",
        expected: [".toBeFalsy("],
      },
      {
        description: "expect.soft 付き",
        content: "expect.soft(value).toBeDefined();",
        expected: [".toBeDefined("],
      },
      {
        description: "複数マッチ",
        content: "expect(a).toBeDefined();\nexpect(b).toBeNull();",
        expected: [".toBeDefined(", ".toBeNull("],
      },
    ] as const satisfies ReadonlyArray<{
      description: string;
      content: string;
      expected: readonly string[];
    }>)("$description", ({ content, expected }) => {
      const actual = findToBeMatches(content);
      expect(actual).toEqual(expected);
    });
  });

  describe("一致しない場合", () => {
    test.each([
      {
        description: "toEqual",
        content: "expect(value).toEqual(null);",
      },
      {
        description: "toHaveBeenCalled",
        content: "expect(fn).toHaveBeenCalled();",
      },
      {
        description: "no-to-be-disable-next-line コメントが直上にある",
        content: "// no-to-be-disable-next-line\nexpect(value).toBe(true);",
      },
      {
        description:
          "no-to-be-disable-next-line コメントが直上にある（インデントあり）",
        content: "\t// no-to-be-disable-next-line\n\texpect(value).toBe(true);",
      },
      {
        description:
          "no-to-be-disable-next-line コメントが直上にある（toBeDefined）",
        content: "// no-to-be-disable-next-line\nexpect(value).toBeDefined();",
      },
      {
        description: "no-to-be-disable-next-line コメントに理由が付いている",
        content:
          "// no-to-be-disable-next-line 参照を比較したいため\nexpect(value).toBe(expected);",
      },
    ] as const satisfies ReadonlyArray<{
      description: string;
      content: string;
    }>)("$description", ({ content }) => {
      const actual = findToBeMatches(content);
      expect(actual).toEqual(null);
    });
  });

  describe("no-to-be-disable-next-line が直上でない場合は一致する", () => {
    test.each([
      {
        description: "コメントと toBe の間に空行がある",
        content: "// no-to-be-disable-next-line\n\nexpect(value).toBe(true);",
        expected: [".toBe("],
      },
      {
        description: "コメントが2行上にある",
        content:
          "// no-to-be-disable-next-line\nconst x = 1;\nexpect(value).toBe(true);",
        expected: [".toBe("],
      },
      {
        description: "コメントテキストが異なる",
        content: "// no-to-be-disable\nexpect(value).toBe(true);",
        expected: [".toBe("],
      },
    ] as const satisfies ReadonlyArray<{
      description: string;
      content: string;
      expected: readonly string[];
    }>)("$description", ({ content, expected }) => {
      const actual = findToBeMatches(content);
      expect(actual).toEqual(expected);
    });
  });
});
