import { describe, expect, test } from "vitest";

import { ROOT_QUERY_KIND } from "../../constants.js";
import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import { rootQueryToken } from "./root-query-token.js";

describe("rootQueryToken", () => {
  test("name → identifier verbatim", () => {
    const q = {
      kind: ROOT_QUERY_KIND.Name,
      name: "value",
      raw: "value",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("value");
  });

  test("line → l<n>", () => {
    const q = {
      kind: ROOT_QUERY_KIND.Line,
      line: 42,
      raw: "42",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l42");
  });

  test("line-name → l<n>-<id>", () => {
    const q = {
      kind: ROOT_QUERY_KIND.LineName,
      line: 42,
      name: "render",
      raw: "42:render",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l42-render");
  });

  test("range → l<start>-<end>", () => {
    const q = {
      kind: ROOT_QUERY_KIND.Range,
      start: 10,
      end: 12,
      raw: "10-12",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l10-12");
  });

  test("range-name → l<start>-<end>-<id>", () => {
    const q = {
      kind: ROOT_QUERY_KIND.RangeName,
      start: 10,
      end: 12,
      name: "render",
      raw: "10-12:render",
    } as const satisfies ParsedRootQuery;
    expect(rootQueryToken(q)).toBe("l10-12-render");
  });
});
