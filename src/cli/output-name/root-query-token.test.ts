import { describe, expect, test } from "vitest";

import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import { rootQueryToken } from "./root-query-token.js";

describe("rootQueryToken", () => {
  test("name → identifier verbatim", () => {
    const q = {
      kind: "name",
      name: "value",
      raw: "value",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("value");
  });

  test("line → l<n>", () => {
    const q = {
      kind: "line",
      line: 42,
      raw: "42",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l42");
  });

  test("line-name → l<n>-<id>", () => {
    const q = {
      kind: "line-name",
      line: 42,
      name: "render",
      raw: "42:render",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l42-render");
  });

  test("range → l<start>-<end>", () => {
    const q = {
      kind: "range",
      start: 10,
      end: 12,
      raw: "10-12",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l10-12");
  });

  test("range-name → l<start>-<end>-<id>", () => {
    const q = {
      kind: "range-name",
      start: 10,
      end: 12,
      name: "render",
      raw: "10-12:render",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l10-12-render");
  });
});
