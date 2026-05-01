import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import { DEFINITION_TYPE } from "../../definition-type.js";
import type {
  AstIdentifier,
  AstNode,
  Definition,
  Variable,
} from "../../ir/model.js";
import { pickVariableOffset } from "./pick-variable-offset.js";

const ident = (name: string, start?: number): AstIdentifier =>
  ({ type: AST_TYPE.Identifier, name, start }) as unknown as AstIdentifier;

const def = (nameStart?: number): Definition => ({
  type: DEFINITION_TYPE.Variable,
  name: ident("x", nameStart),
  node: { type: AST_TYPE.VariableDeclarator } as unknown as AstNode,
  parent: null,
});

const variable = (
  identifiers: readonly AstIdentifier[],
  defs: readonly Definition[] = [],
): Variable => ({ identifiers, defs }) as unknown as Variable;

describe("pickVariableOffset", () => {
  test("uses first identifier's start when present", () => {
    expect(pickVariableOffset(variable([ident("x", 7), ident("x", 9)]))).toBe(
      7,
    );
  });

  test("falls back to defs[0].name.start when identifiers is empty", () => {
    expect(pickVariableOffset(variable([], [def(15)]))).toBe(15);
  });

  test("returns 0 when both sources are missing or have no start", () => {
    expect(pickVariableOffset(variable([]))).toBe(0);
    expect(pickVariableOffset(variable([], [def(undefined)]))).toBe(0);
    expect(pickVariableOffset(variable([ident("x", undefined)]))).toBe(0);
  });
});
