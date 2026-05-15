import { describe, expect, test } from "vitest";

import { parseEndpointQuery } from "./parse-endpoint-query.js";
import { ROOT_QUERY_KIND } from "./root-query-kind.js";

describe("parseEndpointQuery", () => {
  test("parses a bare line number", () => {
    expect(parseEndpointQuery("10")).toEqual({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Line, line: 10, raw: "10" },
    });
  });

  test("parses line:name", () => {
    expect(parseEndpointQuery("10:counter")).toEqual({
      ok: true,
      value: {
        kind: ROOT_QUERY_KIND.LineName,
        line: 10,
        name: "counter",
        raw: "10:counter",
      },
    });
  });

  test("parses line:name with a name starting with $ or _", () => {
    expect(parseEndpointQuery("10:$counter")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.LineName, name: "$counter" },
    });
    expect(parseEndpointQuery("10:_counter")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.LineName, name: "_counter" },
    });
  });

  test("parses a range n-m", () => {
    expect(parseEndpointQuery("9-13")).toEqual({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Range, start: 9, end: 13, raw: "9-13" },
    });
  });

  test("parses range:name", () => {
    expect(parseEndpointQuery("9-13:value")).toEqual({
      ok: true,
      value: {
        kind: ROOT_QUERY_KIND.RangeName,
        start: 9,
        end: 13,
        name: "value",
        raw: "9-13:value",
      },
    });
  });

  test("parses a bare identifier", () => {
    expect(parseEndpointQuery("foo")).toEqual({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "foo", raw: "foo" },
    });
  });

  test("accepts identifiers starting with $ or _", () => {
    expect(parseEndpointQuery("$ok")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "$ok" },
    });
    expect(parseEndpointQuery("_ok")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "_ok" },
    });
  });

  test("accepts identifiers with digits in the middle and end", () => {
    expect(parseEndpointQuery("foo1")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "foo1" },
    });
    expect(parseEndpointQuery("bar2baz")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "bar2baz" },
    });
  });

  test("accepts a single $ or _ as an identifier", () => {
    expect(parseEndpointQuery("$")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "$" },
    });
    expect(parseEndpointQuery("_")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "_" },
    });
  });

  test("parses L<n> as line-or-name (uppercase)", () => {
    expect(parseEndpointQuery("L12")).toEqual({
      ok: true,
      value: {
        kind: ROOT_QUERY_KIND.LineOrName,
        line: 12,
        name: "L12",
        raw: "L12",
      },
    });
  });

  test("parses l<n> as line-or-name (lowercase)", () => {
    expect(parseEndpointQuery("l1")).toEqual({
      ok: true,
      value: {
        kind: ROOT_QUERY_KIND.LineOrName,
        line: 1,
        name: "l1",
        raw: "l1",
      },
    });
  });

  test("parses L<n>-<m> as range, preserving raw", () => {
    expect(parseEndpointQuery("L12-34")).toEqual({
      ok: true,
      value: {
        kind: ROOT_QUERY_KIND.Range,
        start: 12,
        end: 34,
        raw: "L12-34",
      },
    });
    expect(parseEndpointQuery("l9-13")).toEqual({
      ok: true,
      value: {
        kind: ROOT_QUERY_KIND.Range,
        start: 9,
        end: 13,
        raw: "l9-13",
      },
    });
  });

  test("treats LL12 as a plain identifier (Name)", () => {
    expect(parseEndpointQuery("LL12")).toMatchObject({
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: "LL12" },
    });
  });

  test.each([
    ["0", { kind: ROOT_QUERY_KIND.Line, line: 0 }],
    ["0:foo", { kind: ROOT_QUERY_KIND.LineName, line: 0, name: "foo" }],
    ["0-3", { kind: ROOT_QUERY_KIND.Range, start: 0, end: 3 }],
    ["L0", { kind: ROOT_QUERY_KIND.LineOrName, line: 0 }],
  ] as const)(
    "syntactically accepts %s (numeric validation moves to runtime layer)",
    (input, expected) => {
      expect(parseEndpointQuery(input)).toMatchObject({
        ok: true,
        value: expected,
      });
    },
  );

  test.each(["5-1", "L5-1", "l5-1"] as const)(
    "syntactically accepts descending range %s (validation moves to runtime layer)",
    (input) => {
      expect(parseEndpointQuery(input)).toMatchObject({
        ok: true,
        value: { kind: ROOT_QUERY_KIND.Range, start: 5, end: 1 },
      });
    },
  );

  test.each(["10:", "9-13:", "L12:"] as const)(
    "reports unexpected empty identifier after ':' for %s",
    (input) => {
      expect(parseEndpointQuery(input)).toMatchObject({
        ok: false,
        errors: [
          {
            message: expect.stringContaining(
              "unexpected empty identifier after ':'",
            ),
          },
        ],
      });
    },
  );

  test("reports unexpected empty range end", () => {
    expect(parseEndpointQuery("1-")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected empty range end") },
      ],
    });
    expect(parseEndpointQuery("L1-")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected empty range end") },
      ],
    });
  });

  test("reports unexpected character in identifiers with disallowed characters", () => {
    expect(parseEndpointQuery("foo-bar")).toMatchObject({
      ok: false,
      errors: [
        {
          message: expect.stringContaining(
            "unexpected character in identifier",
          ),
        },
      ],
    });
    expect(parseEndpointQuery("foo.bar")).toMatchObject({
      ok: false,
      errors: [
        {
          message: expect.stringContaining(
            "unexpected character in identifier",
          ),
        },
      ],
    });
  });

  test("reports unexpected character for L<n>:<id> (tolerance does not extend)", () => {
    expect(parseEndpointQuery("L12:foo")).toMatchObject({
      ok: false,
      errors: [
        {
          message: expect.stringContaining(
            "unexpected character in identifier",
          ),
        },
      ],
    });
  });

  test.each(["1L2", "10:1foo", "-5", "1-2-3"] as const)(
    "reports unrecognized token for %s",
    (input) => {
      expect(parseEndpointQuery(input)).toMatchObject({
        ok: false,
        errors: [{ message: expect.stringContaining("unrecognized token") }],
      });
    },
  );
});
