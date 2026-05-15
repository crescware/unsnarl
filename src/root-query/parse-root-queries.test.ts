import { describe, expect, test } from "vitest";

import { parseRootQueries } from "./parse-root-queries.js";
import { ROOT_QUERY_KIND } from "./root-query-kind.js";

describe("parseRootQueries", () => {
  test("parses a single token", () => {
    const r = parseRootQueries("10:foo");
    expect(r.ok).toEqual(true);
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
    expect(r.ok).toEqual(true);
    if (r.ok) {
      expect(r.queries).toHaveLength(3);
      expect(r.queries.map((v) => v.kind)).toEqual([
        "line-name",
        "line",
        "range-name",
      ]);
    }
  });

  test("parses L-prefixed forms in a comma list", () => {
    const r = parseRootQueries("L10,L5-10,L20");
    expect(r.ok).toEqual(true);
    if (r.ok) {
      expect(r.queries).toHaveLength(3);
      expect(r.queries[0]).toMatchObject({
        kind: ROOT_QUERY_KIND.LineOrName,
        line: 10,
        name: "L10",
      });
      expect(r.queries[1]).toMatchObject({
        kind: ROOT_QUERY_KIND.Range,
        start: 5,
        end: 10,
        raw: "L5-10",
      });
      expect(r.queries[2]).toMatchObject({
        kind: ROOT_QUERY_KIND.LineOrName,
        line: 20,
        name: "L20",
      });
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

  test("rejects an empty token in the middle of a comma list", () => {
    expect(parseRootQueries("10,,42")).toMatchObject({ ok: false });
  });

  test("propagates the offending token in the error", () => {
    const r = parseRootQueries("10,foo-bar");
    expect(r.ok).toEqual(false);
    if (!r.ok) {
      expect(r.error).toContain("foo-bar");
    }
  });
});
