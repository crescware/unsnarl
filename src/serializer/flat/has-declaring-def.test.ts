import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import { DEFINITION_TYPE } from "../../definition-type.js";
import type {
  AstIdentifier,
  AstNode,
  Definition,
  Variable,
} from "../../ir/model.js";
import { hasDeclaringDef } from "./has-declaring-def.js";

const ident = (name: string): AstIdentifier =>
  ({ type: AST_TYPE.Identifier, name }) as unknown as AstIdentifier;
const node = (type: string): AstNode => ({ type }) as unknown as AstNode;

const variableWith = (defs: readonly Definition[]): Variable =>
  ({ defs }) as unknown as Variable;

describe("hasDeclaringDef", () => {
  test("true when at least one def has a non-implicit type", () => {
    const v = variableWith([
      {
        type: DEFINITION_TYPE.Variable,
        name: ident("x"),
        node: node(AST_TYPE.VariableDeclarator),
        parent: null,
      },
    ]);
    expect(hasDeclaringDef(v)).toBe(true);
  });

  test("false when every def is ImplicitGlobalVariable", () => {
    const v = variableWith([
      {
        type: DEFINITION_TYPE.ImplicitGlobalVariable,
        name: ident("x"),
        node: node(AST_TYPE.Identifier),
        parent: null,
      },
    ]);
    expect(hasDeclaringDef(v)).toBe(false);
  });

  test("true when mixed (any single non-implicit def is enough)", () => {
    const v = variableWith([
      {
        type: DEFINITION_TYPE.ImplicitGlobalVariable,
        name: ident("x"),
        node: node(AST_TYPE.Identifier),
        parent: null,
      },
      {
        type: DEFINITION_TYPE.FunctionName,
        name: ident("x"),
        node: node(AST_TYPE.FunctionDeclaration),
        parent: null,
      },
    ]);
    expect(hasDeclaringDef(v)).toBe(true);
  });

  test("false when defs is empty", () => {
    expect(hasDeclaringDef(variableWith([]))).toBe(false);
  });
});
