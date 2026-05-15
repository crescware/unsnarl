import { describe, expect, test } from "vitest";

import { parseRootQueryAst } from "./parse-root-query-ast.js";
import { ROOT_QUERY_KIND } from "./root-query-kind.js";
import {
  ROOT_QUERY_SCOPE_POINT_ONLY,
  type RootQueryScope,
} from "./root-query-scope.js";

const SCOPE_FULL: RootQueryScope = {
  point: true,
  path: true,
  direction: true,
  directionLevel: true,
};

describe("parseRootQueryAst (point wrapping under POINT_ONLY)", () => {
  test("wraps a bare line as a single RootQuery", () => {
    expect(parseRootQueryAst("10", ROOT_QUERY_SCOPE_POINT_ONLY)).toEqual({
      ok: true,
      value: {
        kind: "single",
        query: { kind: ROOT_QUERY_KIND.Line, line: 10, raw: "10" },
        raw: "10",
      },
    });
  });

  test("wraps an identifier", () => {
    expect(parseRootQueryAst("foo", ROOT_QUERY_SCOPE_POINT_ONLY)).toMatchObject(
      {
        ok: true,
        value: {
          kind: "single",
          query: { kind: ROOT_QUERY_KIND.Name, name: "foo" },
        },
      },
    );
  });

  test("rejects empty token", () => {
    expect(parseRootQueryAst("", ROOT_QUERY_SCOPE_POINT_ONLY)).toMatchObject({
      ok: false,
      errors: [{ message: "empty root query" }],
    });
  });

  test("syntactically accepts line 0 (numeric validation is not part of parsing)", () => {
    expect(parseRootQueryAst("0", ROOT_QUERY_SCOPE_POINT_ONLY)).toMatchObject({
      ok: true,
      value: { kind: "single", query: { line: 0 } },
    });
  });

  test("syntactically accepts descending range", () => {
    expect(parseRootQueryAst("5-1", ROOT_QUERY_SCOPE_POINT_ONLY)).toMatchObject(
      {
        ok: true,
        value: { kind: "single", query: { start: 5, end: 1 } },
      },
    );
  });
});

describe("parseRootQueryAst path / direction under SCOPE_FULL", () => {
  test("parses a path query", () => {
    expect(parseRootQueryAst("foo..bar", SCOPE_FULL)).toEqual({
      ok: true,
      value: {
        kind: "path",
        lhs: { kind: ROOT_QUERY_KIND.Name, name: "foo", raw: "foo" },
        rhs: { kind: ROOT_QUERY_KIND.Name, name: "bar", raw: "bar" },
        raw: "foo..bar",
      },
    });
  });

  test("parses a path with mixed endpoint kinds", () => {
    expect(parseRootQueryAst("10..L20", SCOPE_FULL)).toMatchObject({
      ok: true,
      value: {
        kind: "path",
        lhs: { kind: ROOT_QUERY_KIND.Line, line: 10 },
        rhs: { kind: ROOT_QUERY_KIND.LineOrName, line: 20, name: "L20" },
      },
    });
  });

  test.each([
    ["+a", "a"],
    ["+b", "b"],
    ["+c", "c"],
  ] as const)("parses direction foo..%s", (suffix, dir) => {
    expect(parseRootQueryAst(`foo..${suffix}`, SCOPE_FULL)).toEqual({
      ok: true,
      value: {
        kind: "direction",
        lhs: { kind: ROOT_QUERY_KIND.Name, name: "foo", raw: "foo" },
        dir,
        level: null,
        raw: `foo..${suffix}`,
      },
    });
  });

  test("parses direction +a<N> with level = N", () => {
    expect(parseRootQueryAst("foo..+a3", SCOPE_FULL)).toMatchObject({
      ok: true,
      value: { kind: "direction", dir: "a", level: 3 },
    });
    expect(parseRootQueryAst("foo..+a0", SCOPE_FULL)).toMatchObject({
      ok: true,
      value: { kind: "direction", dir: "a", level: 0 },
    });
  });

  test("rejects empty left-hand side of '..'", () => {
    expect(parseRootQueryAst("..foo", SCOPE_FULL)).toMatchObject({
      ok: false,
      errors: [
        {
          message: expect.stringContaining(
            "unexpected empty left-hand side of '..'",
          ),
        },
      ],
    });
  });

  test("rejects empty right-hand side of '..'", () => {
    expect(parseRootQueryAst("foo..", SCOPE_FULL)).toMatchObject({
      ok: false,
      errors: [
        {
          message: expect.stringContaining(
            "unexpected empty right-hand side of '..'",
          ),
        },
      ],
    });
  });

  test("rejects duplicate '..'", () => {
    expect(parseRootQueryAst("foo..bar..baz", SCOPE_FULL)).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected duplicate '..'") },
      ],
    });
  });

  test("rejects invalid direction tokens", () => {
    expect(parseRootQueryAst("foo..+x", SCOPE_FULL)).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });
});

describe("parseRootQueryAst scope enforcement", () => {
  test("POINT_ONLY rejects path with a syntax error and no 'unsupported' wording", () => {
    const r = parseRootQueryAst("foo..bar", ROOT_QUERY_SCOPE_POINT_ONLY);
    if (r.ok) {
      throw new Error("expected parse to fail under POINT_ONLY");
    }
    const msg = r.errors[0]?.message ?? "";
    expect(msg).toContain("unexpected '..'");
    expect(msg).not.toMatch(
      /未実装|未サポート|reserved|not yet|unsupported|coming soon/i,
    );
  });

  test("POINT_ONLY rejects direction with a syntax error", () => {
    expect(
      parseRootQueryAst("foo..+a", ROOT_QUERY_SCOPE_POINT_ONLY),
    ).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });

  test("scope with path only rejects direction tokens", () => {
    const scope: RootQueryScope = {
      point: true,
      path: true,
      direction: false,
      directionLevel: false,
    };
    expect(parseRootQueryAst("foo..+a", scope)).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });

  test("scope with direction only rejects paths", () => {
    const scope: RootQueryScope = {
      point: true,
      path: false,
      direction: true,
      directionLevel: false,
    };
    expect(parseRootQueryAst("foo..bar", scope)).toMatchObject({
      ok: false,
      errors: [{ message: expect.stringContaining("unexpected '..'") }],
    });
  });

  test("scope with direction but not directionLevel rejects +a<N>", () => {
    const scope: RootQueryScope = {
      point: true,
      path: false,
      direction: true,
      directionLevel: false,
    };
    expect(parseRootQueryAst("foo..+a3", scope)).toMatchObject({
      ok: false,
      errors: [
        {
          message: expect.stringContaining(
            "unexpected level in direction token",
          ),
        },
      ],
    });
  });
});
