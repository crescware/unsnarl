import { parse, ValiError } from "valibot";
import { describe, expect, test } from "vitest";

import { AST_TYPE, asAstType, astType$, UNKNOWN_AST_TYPE } from "./ast-type.js";

describe("asAstType", () => {
  test.each<{ raw: string; expected: string }>([
    { raw: "LogicalExpression", expected: AST_TYPE.LogicalExpression },
    { raw: "Identifier", expected: AST_TYPE.Identifier },
    { raw: "TSAsExpression", expected: AST_TYPE.TSAsExpression },
  ])("known type $raw passes through as $expected", ({ raw, expected }) => {
    expect(asAstType(raw)).toEqual(expected);
  });

  test.each<{ raw: string }>([
    { raw: "NotAnAstType" },
    { raw: "" },
    { raw: "variable_declaration" },
    { raw: "SomeFutureOxcType" },
    // The sentinel string itself is not an oxc emit value, so it
    // round-trips through the "unknown" branch rather than passing
    // through as a known type.
    { raw: UNKNOWN_AST_TYPE },
  ])("unrecognized $raw collapses to UNKNOWN_AST_TYPE", ({ raw }) => {
    expect(asAstType(raw)).toEqual(UNKNOWN_AST_TYPE);
  });
});

describe("astType$ schema", () => {
  test.each(Object.values(AST_TYPE).map((value) => ({ value })))(
    "accepts AST_TYPE.$value",
    ({ value }) => {
      expect(parse(astType$, value)).toEqual(value);
    },
  );

  test("accepts UNKNOWN_AST_TYPE sentinel", () => {
    expect(parse(astType$, UNKNOWN_AST_TYPE)).toEqual(UNKNOWN_AST_TYPE);
  });

  test.each<{ raw: string }>([{ raw: "NotAnAstType" }, { raw: "" }])(
    "rejects $raw which is outside the picklist",
    ({ raw }) => {
      expect(() => parse(astType$, raw)).toThrow(ValiError);
    },
  );

  test.each<{ raw: string }>([
    { raw: "LogicalExpression" },
    { raw: "ConditionalExpression" },
    { raw: "NotAnAstType" },
    { raw: "" },
    { raw: "SomeFutureOxcType" },
  ])("asAstType($raw) output satisfies the schema", ({ raw }) => {
    expect(() => parse(astType$, asAstType(raw))).not.toThrow();
  });
});
