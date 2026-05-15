import { describe, expect, test } from "vitest";

import { parseRootQueryAst } from "./parse-root-query-ast.js";
import type { RootQueryScope } from "./root-query-scope.js";
import type { RootQuery } from "./root-query.js";
import { validateRootQuery } from "./validate-root-query.js";

const SCOPE_FULL: RootQueryScope = {
  point: true,
  path: true,
  direction: true,
  directionLevel: true,
};

function parseFull(text: string): RootQuery {
  const r = parseRootQueryAst(text, SCOPE_FULL);
  if (!r.ok) {
    throw new Error(
      `parse failed for '${text}': ${r.errors[0]?.message ?? "(no message)"}`,
    );
  }
  return r.value;
}

describe("validateRootQuery", () => {
  test.each(["0", "0..foo", "foo..0", "0..+a"])(
    "rejects %s with 'line must be >= 1'",
    (input) => {
      const r = validateRootQuery(parseFull(input));
      if (r.ok) {
        throw new Error(`expected validation failure for '${input}'`);
      }
      expect(r.errors[0]?.message ?? "").toContain("line must be >= 1");
    },
  );

  test.each([
    "foo",
    "10",
    "foo..bar",
    "1..10",
    "foo..+a",
    "foo..+a0",
    "foo..+a3",
  ])("accepts %s", (input) => {
    expect(validateRootQuery(parseFull(input))).toMatchObject({ ok: true });
  });
});
