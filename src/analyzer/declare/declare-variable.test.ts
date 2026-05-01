import { describe, expect, test } from "vitest";

import type { AstIdentifier, AstNode } from "../../ir/model.js";
import { ScopeImpl } from "../scope.js";
import { declareVariable } from "./declare-variable.js";

const ident = (name: string): AstIdentifier =>
  ({ type: "Identifier", name }) as unknown as AstIdentifier;

const node = (type: string): AstNode =>
  ({ type }) as unknown as AstNode;

const makeScope = (): ScopeImpl =>
  new ScopeImpl({
    type: "module",
    isStrict: true,
    upper: null,
    block: node("Program"),
  });

describe("declareVariable", () => {
  test("creates a new variable, registers it in scope.set and scope.variables", () => {
    const scope = makeScope();
    const v = declareVariable(scope, ident("x"), "Variable", node("VariableDeclarator"), null);
    expect(scope.variables).toHaveLength(1);
    expect(scope.set.get("x")).toBe(v);
    expect(v.name).toBe("x");
  });

  test("appends to identifiers and defs each call", () => {
    const scope = makeScope();
    const idA = ident("x");
    const idB = ident("x");
    const declA = node("VariableDeclarator");
    const declB = node("FunctionDeclaration");
    const v = declareVariable(scope, idA, "Variable", declA, null);
    declareVariable(scope, idB, "FunctionName", declB, null);
    expect(v.identifiers).toEqual([idA, idB]);
    expect(v.defs).toEqual([
      { type: "Variable", name: idA, node: declA, parent: null },
      { type: "FunctionName", name: idB, node: declB, parent: null },
    ]);
  });

  test("re-declaring the same name reuses the existing Variable instance", () => {
    const scope = makeScope();
    const first = declareVariable(scope, ident("x"), "Variable", node("VariableDeclarator"), null);
    const second = declareVariable(scope, ident("x"), "Variable", node("VariableDeclarator"), null);
    expect(second).toBe(first);
    expect(scope.variables).toHaveLength(1);
  });

  test("distinct names create distinct variables", () => {
    const scope = makeScope();
    const a = declareVariable(scope, ident("a"), "Variable", node("VariableDeclarator"), null);
    const b = declareVariable(scope, ident("b"), "Variable", node("VariableDeclarator"), null);
    expect(a).not.toBe(b);
    expect(scope.variables).toEqual([a, b]);
  });

  test("parent node is propagated into the def", () => {
    const scope = makeScope();
    const parent = node("VariableDeclaration");
    const v = declareVariable(scope, ident("x"), "Variable", node("VariableDeclarator"), parent);
    expect(v.defs[0]?.parent).toBe(parent);
  });
});
