import { describe, expect, test } from "vitest";

import type {
  AstIdentifier,
  AstNode,
  Definition,
  Variable,
} from "../../ir/model.js";
import { hasDeclaringDef } from "./has-declaring-def.js";

const ident = (name: string): AstIdentifier =>
  ({ type: "Identifier", name }) as unknown as AstIdentifier;
const node = (type: string): AstNode => ({ type }) as unknown as AstNode;

const variableWith = (defs: Definition[]): Variable =>
  ({ defs }) as unknown as Variable;

describe("hasDeclaringDef", () => {
  test("true when at least one def has a non-implicit type", () => {
    const v = variableWith([
      {
        type: "Variable",
        name: ident("x"),
        node: node("VariableDeclarator"),
        parent: null,
      },
    ]);
    expect(hasDeclaringDef(v)).toBe(true);
  });

  test("false when every def is ImplicitGlobalVariable", () => {
    const v = variableWith([
      {
        type: "ImplicitGlobalVariable",
        name: ident("x"),
        node: node("Identifier"),
        parent: null,
      },
    ]);
    expect(hasDeclaringDef(v)).toBe(false);
  });

  test("true when mixed (any single non-implicit def is enough)", () => {
    const v = variableWith([
      {
        type: "ImplicitGlobalVariable",
        name: ident("x"),
        node: node("Identifier"),
        parent: null,
      },
      {
        type: "FunctionName",
        name: ident("x"),
        node: node("FunctionDeclaration"),
        parent: null,
      },
    ]);
    expect(hasDeclaringDef(v)).toBe(true);
  });

  test("false when defs is empty", () => {
    expect(hasDeclaringDef(variableWith([]))).toBe(false);
  });
});
