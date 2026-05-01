import { describe, expect, test } from "vitest";

import { ROOT_QUERY_KIND } from "../../constants.js";
import { parseRootQueries } from "./parse-root-queries.js";

describe("parseRootQueries", () => {
  test("parses a single token", () => {
    const r = parseRootQueries("10:foo");
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.queries).toHaveLength(1);
      expect(r.queries[0]).toMatchObject({
        kind: ROOT_QUERY_KIND.LineName,
        line: 10,
      });
    }
  });

  test("parses comma-separated tokens", () => {
    const r = parseRootQueries("10:foo,42,9-13:bar");
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.queries).toHaveLength(3);
      expect(r.queries.map((q) => q.kind)).toEqual([
        "line-name",
        "line",
        "range-name",
      ]);
    }
  });

  test("rejects empty value", () => {
    expect(parseRootQueries("")).toEqual({
      ok: false,
      error: "empty --roots value",
    });
  });

  test("rejects values with whitespace around tokens", () => {
    expect(parseRootQueries("10, 42")).toMatchObject({ ok: false });
    expect(parseRootQueries(" 10")).toMatchObject({ ok: false });
  });

  test("rejects trailing comma", () => {
    expect(parseRootQueries("10,")).toMatchObject({ ok: false });
    expect(parseRootQueries(",10")).toMatchObject({ ok: false });
  });

  test("propagates the offending token in the error", () => {
    const r = parseRootQueries("10,foo-bar");
    expect(r.ok).toBe(false);
    if (!r.ok) {
      expect(r.error).toContain("foo-bar");
    }
  });
});
