import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../../analyzer/scope-type.js";
import type { AstNode } from "../../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { ScopeImpl } from "../scope-impl.js";
import { declareImplicitArguments } from "./declare-implicit-arguments.js";

const node = (type: string): AstNode => ({ type }) as unknown as AstNode;

const makeFunctionScope = (): ScopeImpl =>
  new ScopeImpl({
    type: SCOPE_TYPE.Function,
    isStrict: true,
    upper: null,
    block: node(AST_TYPE.FunctionDeclaration),
  });

describe("declareImplicitArguments", () => {
  test("registers an 'arguments' Variable with empty identifiers and defs", () => {
    const scope = makeFunctionScope();
    declareImplicitArguments(scope);
    expect(scope.variables).toHaveLength(1);
    const v = scope.variables[0];
    expect(v?.name).toEqual("arguments");
    expect(v?.identifiers).toEqual([]);
    expect(v?.defs).toEqual([]);
    expect(v?.references).toEqual([]);
    expect(scope.set.get("arguments")).toEqual(v);
    expect(v?.scope).toEqual(scope);
  });

  test("does nothing when 'arguments' is already declared", () => {
    const scope = makeFunctionScope();
    declareImplicitArguments(scope);
    const first = scope.set.get("arguments");
    declareImplicitArguments(scope);
    expect(scope.variables).toHaveLength(1);
    expect(scope.set.get("arguments")).toEqual(first);
  });
});
