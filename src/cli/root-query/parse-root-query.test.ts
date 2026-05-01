import { describe, expect, test } from "vitest";

import { parseRootQuery } from "./parse-root-query.js";

describe("parseRootQuery", () => {
  test("parses a bare line number", () => {
    expect(parseRootQuery("10")).toEqual({ kind: "line", line: 10, raw: "10" });
  });

  test("parses line:name", () => {
    expect(parseRootQuery("10:counter")).toEqual({
      kind: "line-name",
      line: 10,
      name: "counter",
      raw: "10:counter",
    });
  });

  test("parses a range n-m", () => {
    expect(parseRootQuery("9-13")).toEqual({
      kind: "range",
      start: 9,
      end: 13,
      raw: "9-13",
    });
  });

  test("parses range:name", () => {
    expect(parseRootQuery("9-13:value")).toEqual({
      kind: "range-name",
      start: 9,
      end: 13,
      name: "value",
      raw: "9-13:value",
    });
  });

  test("parses a bare identifier", () => {
    expect(parseRootQuery("foo")).toEqual({
      kind: "name",
      name: "foo",
      raw: "foo",
    });
  });

  test("accepts identifiers starting with $ or _", () => {
    expect(parseRootQuery("$ok")).toMatchObject({ kind: "name", name: "$ok" });
    expect(parseRootQuery("_ok")).toMatchObject({ kind: "name", name: "_ok" });
  });

  test("treats n-n as a single-line range", () => {
    expect(parseRootQuery("5-5")).toMatchObject({
      kind: "range",
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
});
