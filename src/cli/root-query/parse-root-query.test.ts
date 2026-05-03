import { describe, expect, test } from "vitest";

import { parseRootQuery } from "./parse-root-query.js";
import { ROOT_QUERY_KIND } from "./root-query-kind.js";

describe("parseRootQuery", () => {
  test("parses a bare line number", () => {
    expect(parseRootQuery("10")).toEqual({
      kind: ROOT_QUERY_KIND.Line,
      line: 10,
      raw: "10",
    });
  });

  test("parses line:name", () => {
    expect(parseRootQuery("10:counter")).toEqual({
      kind: ROOT_QUERY_KIND.LineName,
      line: 10,
      name: "counter",
      raw: "10:counter",
    });
  });

  test("parses a range n-m", () => {
    expect(parseRootQuery("9-13")).toEqual({
      kind: ROOT_QUERY_KIND.Range,
      start: 9,
      end: 13,
      raw: "9-13",
    });
  });

  test("parses range:name", () => {
    expect(parseRootQuery("9-13:value")).toEqual({
      kind: ROOT_QUERY_KIND.RangeName,
      start: 9,
      end: 13,
      name: "value",
      raw: "9-13:value",
    });
  });

  test("parses a bare identifier", () => {
    expect(parseRootQuery("foo")).toEqual({
      kind: ROOT_QUERY_KIND.Name,
      name: "foo",
      raw: "foo",
    });
  });

  test("accepts identifiers starting with $ or _", () => {
    expect(parseRootQuery("$ok")).toMatchObject({
      kind: ROOT_QUERY_KIND.Name,
      name: "$ok",
    });
    expect(parseRootQuery("_ok")).toMatchObject({
      kind: ROOT_QUERY_KIND.Name,
      name: "_ok",
    });
  });

  test("treats n-n as a single-line range", () => {
    expect(parseRootQuery("5-5")).toMatchObject({
      kind: ROOT_QUERY_KIND.Range,
      start: 5,
      end: 5,
    });
  });

  test("rejects empty string", () => {
    expect(parseRootQuery("")).toEqual({ error: "empty root query" });
  });

  test("rejects identifier starting with a digit", () => {
    const r = parseRootQuery("10:1foo");
    expect(r).toMatchObject({ error: expect.stringContaining("invalid") });
  });

  test("rejects empty identifier after colon", () => {
    expect(parseRootQuery("10:")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
  });

  test("rejects descending range", () => {
    expect(parseRootQuery("5-1")).toMatchObject({
      error: expect.stringContaining("range start must be <= end"),
    });
  });

  test("rejects malformed ranges", () => {
    expect(parseRootQuery("1-")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
    expect(parseRootQuery("-5")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
    expect(parseRootQuery("1-2-3")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
  });

  test("rejects line 0", () => {
    expect(parseRootQuery("0")).toMatchObject({
      error: expect.stringContaining("line must be >= 1"),
    });
    expect(parseRootQuery("0-3")).toMatchObject({
      error: expect.stringContaining("line must be >= 1"),
    });
  });

  test("rejects identifiers with disallowed characters", () => {
    expect(parseRootQuery("foo-bar")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
    expect(parseRootQuery("foo.bar")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
  });

  test("parses L<n> as line-or-name (uppercase)", () => {
    expect(parseRootQuery("L12")).toEqual({
      kind: ROOT_QUERY_KIND.LineOrName,
      line: 12,
      name: "L12",
      raw: "L12",
    });
  });

  test("parses l<n> as line-or-name (lowercase)", () => {
    expect(parseRootQuery("l1")).toEqual({
      kind: ROOT_QUERY_KIND.LineOrName,
      line: 1,
      name: "l1",
      raw: "l1",
    });
  });

  test("parses L<n>-<m> as range, preserving raw", () => {
    expect(parseRootQuery("L12-34")).toEqual({
      kind: ROOT_QUERY_KIND.Range,
      start: 12,
      end: 34,
      raw: "L12-34",
    });
    expect(parseRootQuery("l9-13")).toEqual({
      kind: ROOT_QUERY_KIND.Range,
      start: 9,
      end: 13,
      raw: "l9-13",
    });
  });

  test("rejects L0 / l0", () => {
    expect(parseRootQuery("L0")).toMatchObject({
      error: expect.stringContaining("line must be >= 1"),
    });
    expect(parseRootQuery("l0")).toMatchObject({
      error: expect.stringContaining("line must be >= 1"),
    });
  });

  test("rejects descending L-prefixed range", () => {
    expect(parseRootQuery("L5-1")).toMatchObject({
      error: expect.stringContaining("range start must be <= end"),
    });
  });

  test("treats LL12 as a plain identifier (Name)", () => {
    // The L-prefix tolerance only applies to a single L/l followed by
    // digits, so LL12 falls through to the ID_RE branch as a Name.
    expect(parseRootQuery("LL12")).toEqual({
      kind: ROOT_QUERY_KIND.Name,
      name: "LL12",
      raw: "LL12",
    });
  });

  test("rejects 1L2 (digit-leading, not an identifier)", () => {
    expect(parseRootQuery("1L2")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
  });

  test("does not extend tolerance to L<n>:<id>", () => {
    expect(parseRootQuery("L12:foo")).toMatchObject({
      error: expect.stringContaining("invalid"),
    });
  });
});
