import { parse, ValiError } from "valibot";
import { describe, expect, test } from "vitest";

import { AST_TYPE, asAstType, astType$ } from "./ast-type.js";

describe("asAstType", () => {
  test.each<{ raw: string; expected: string }>([
    { raw: "LogicalExpression", expected: AST_TYPE.LogicalExpression },
    { raw: "Identifier", expected: AST_TYPE.Identifier },
    { raw: "TSAsExpression", expected: AST_TYPE.TSAsExpression },
    {
      raw: AST_TYPE.UnknownAstType,
      expected: AST_TYPE.UnknownAstType,
    },
  ])("known type $raw passes through as $expected", ({ raw, expected }) => {
    expect(asAstType(raw)).toEqual(expected);
  });

  test.each<{ raw: string }>([
    { raw: "NotAnAstType" },
    { raw: "" },
    { raw: "variable_declaration" },
    { raw: "SomeFutureOxcType" },
  ])("unrecognized $raw collapses to UnknownAstType", ({ raw }) => {
    expect(asAstType(raw)).toEqual(AST_TYPE.UnknownAstType);
  });
});

describe("astType$ schema", () => {
  test.each(Object.values(AST_TYPE).map((value) => ({ value })))(
    "accepts AST_TYPE.$value",
    ({ value }) => {
      expect(parse(astType$, value)).toEqual(value);
    },
  );

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
