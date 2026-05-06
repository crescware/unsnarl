import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DEFINITION_TYPE } from "../definition-type.js";
import { ScopeManager } from "../manager.js";
import { declareFunctionParams } from "./declare-function-params.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";

function setup(code: string) {
  const program = parse(code);
  const fn = findFirst(program, AST_TYPE.FunctionDeclaration);
  const manager = new ScopeManager("module", program as unknown as AstNode);
  const fnScope = manager.push("function", fn as unknown as AstNode, null);
  return { fn, fnScope };
}

describe("declareFunctionParams", () => {
  test("declares simple identifier parameters as Parameter definitions", () => {
    const { fn, fnScope } = setup("function f(a, b) {}");
    declareFunctionParams(fn, fnScope);
    expect(
      fnScope.variables.map((v) => ({
        name: v.name,
        defType: v.defs[0]?.type,
        defNode: v.defs[0]?.node,
        defParent: v.defs[0]?.parent,
      })),
    ).toEqual([
      {
        name: "a",
        defType: DEFINITION_TYPE.Parameter,
        defNode: fn,
        defParent: null,
      },
      {
        name: "b",
        defType: DEFINITION_TYPE.Parameter,
        defNode: fn,
        defParent: null,
      },
    ]);
  });

  test("declares destructured parameters at the binding identifiers", () => {
    const { fn, fnScope } = setup("function f({ x, y }) {}");
    declareFunctionParams(fn, fnScope);
    expect(
      [...fnScope.variables]
        .sort((a, b) => a.name.localeCompare(b.name))
        .map((v) => ({
          name: v.name,
          defType: v.defs[0]?.type,
          defNode: v.defs[0]?.node,
          defParent: v.defs[0]?.parent,
        })),
    ).toEqual([
      {
        name: "x",
        defType: DEFINITION_TYPE.Parameter,
        defNode: fn,
        defParent: null,
      },
      {
        name: "y",
        defType: DEFINITION_TYPE.Parameter,
        defNode: fn,
        defParent: null,
      },
    ]);
  });

  test("declares the underlying identifier when the parameter is a RestElement", () => {
    const { fn, fnScope } = setup("function f(...rest) {}");
    declareFunctionParams(fn, fnScope);
    expect(
      fnScope.variables.map((v) => ({
        name: v.name,
        defType: v.defs[0]?.type,
        defNode: v.defs[0]?.node,
        defParent: v.defs[0]?.parent,
      })),
    ).toEqual([
      {
        name: "rest",
        defType: DEFINITION_TYPE.Parameter,
        defNode: fn,
        defParent: null,
      },
    ]);
  });

  test("does nothing when params is missing or not an array", () => {
    const program = parse("");
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const fnScope = manager.push(
      "function",
      program as unknown as AstNode,
      null,
    );
    declareFunctionParams({ type: AST_TYPE.FunctionDeclaration }, fnScope);
    expect(fnScope.variables).toHaveLength(0);
  });
});
