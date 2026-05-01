import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import { ReferenceFlags } from "../../ir/model.js";
import type { AstExpression } from "../../ir/model.js";
import { reference } from "./reference.js";

describe("reference factory", () => {
  test("returns a reference-kind result with the given fields", () => {
    expect(reference(ReferenceFlags.Read, false, null)).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Read,
      init: false,
      writeExpr: null,
    });
  });

  test("preserves init=true and a writeExpr node", () => {
    const expr = {
      type: AST_TYPE.Literal,
      value: 1,
    } as unknown as AstExpression;
    expect(reference(ReferenceFlags.Write, true, expr)).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Write,
      init: true,
      writeExpr: expr,
    });
  });
});
