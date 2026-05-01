import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import { DEFINITION_TYPE } from "../../definition-type.js";
import type { AstNode } from "../../ir/model.js";
import { ScopeManager } from "../manager.js";
import { declareFunctionParams } from "./declare-function-params.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";

function setup(code: string) {
  const program = parse(code);
  const fn = findFirst(program, AST_TYPE.FunctionDeclaration);
  const manager = new ScopeManager("module", program as unknown as AstNode);
  const fnScope = manager.push("function", fn as unknown as AstNode);
  return { fn, fnScope };
}

describe("declareFunctionParams", () => {
  test("declares simple identifier parameters as Parameter definitions", () => {
    const { fn, fnScope } = setup("function f(a, b) {}");
    declareFunctionParams(fn, fnScope);
    expect(fnScope.variables.map((v) => v.name)).toEqual(["a", "b"]);
    expect(
      fnScope.variables.every(
        (v) => v.defs[0]?.type === DEFINITION_TYPE.Parameter,
      ),
    ).toBe(true);
  });

  test("declares destructured parameters at the binding identifiers", () => {
    const { fn, fnScope } = setup("function f({ x, y }) {}");
    declareFunctionParams(fn, fnScope);
    expect(fnScope.variables.map((v) => v.name).sort()).toEqual(["x", "y"]);
  });

  test("declares the underlying identifier when the parameter is a RestElement", () => {
    const { fn, fnScope } = setup("function f(...rest) {}");
    declareFunctionParams(fn, fnScope);
    expect(fnScope.variables.map((v) => v.name)).toEqual(["rest"]);
  });

  test("does nothing when params is missing or not an array", () => {
    const program = parse("");
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const fnScope = manager.push("function", program as unknown as AstNode);
    declareFunctionParams({ type: AST_TYPE.FunctionDeclaration }, fnScope);
    expect(fnScope.variables).toHaveLength(0);
  });
});
