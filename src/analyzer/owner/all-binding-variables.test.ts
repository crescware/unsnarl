import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { declareVariable } from "../declare/declare-variable.js";
import { DEFINITION_TYPE } from "../definition-type.js";
import { ScopeImpl } from "../scope-impl.js";
import { SCOPE_TYPE } from "../scope-type.js";
import { allBindingVariables } from "./all-binding-variables.js";

const ident = (name: string): AstIdentifier =>
  ({ type: AST_TYPE.Identifier, name }) as unknown as AstIdentifier;

const declId = (code: string): AstNode => {
  const program = parseSync("input.ts", code, { lang: "ts" })
    .program as unknown as {
    body: readonly { declarations: readonly { id: AstNode }[] }[];
  };
  const decl = program.body[0]?.declarations[0]?.id;
  if (decl === undefined) {
    throw new Error("test fixture missing declarator id");
  }
  return decl;
};

const scopeWith = (...names: readonly string[]): ScopeImpl => {
  const scope = new ScopeImpl({
    type: SCOPE_TYPE.Module,
    isStrict: true,
    upper: null,
    block: { type: AST_TYPE.Program } as unknown as AstNode,
    blockContext: null,
  });
  for (const n of names) {
    declareVariable(
      scope,
      ident(n),
      DEFINITION_TYPE.Variable,
      { type: AST_TYPE.VariableDeclarator } as unknown as AstNode,
      null,
    );
  }
  return scope;
};

describe("allBindingVariables", () => {
  test("Identifier pattern returns the resolved variable", () => {
    const scope = scopeWith("x");
    const out = allBindingVariables(declId("const x = 1;"), scope);
    expect(out.map((v) => v.name)).toEqual(["x"]);
  });

  test("ObjectPattern returns each declared property in source order", () => {
    const scope = scopeWith("a", "b");
    const out = allBindingVariables(declId("const { a, b } = obj;"), scope);
    expect(out.map((v) => v.name)).toEqual(["a", "b"]);
  });

  test("unresolved identifiers are dropped", () => {
    const scope = scopeWith("a"); // b is missing in scope
    const out = allBindingVariables(declId("const { a, b } = obj;"), scope);
    expect(out.map((v) => v.name)).toEqual(["a"]);
  });

  test("walks up the scope chain", () => {
    const upper = scopeWith("outer");
    const inner = new ScopeImpl({
      type: SCOPE_TYPE.Block,
      isStrict: true,
      upper,
      block: { type: AST_TYPE.BlockStatement } as unknown as AstNode,
      blockContext: null,
    });
    const out = allBindingVariables(declId("const outer = 1;"), inner);
    expect(out.map((v) => v.name)).toEqual(["outer"]);
  });

  test("duplicate identifiers in pattern resolve to the same variable once", () => {
    const scope = scopeWith("a");
    const pattern = {
      type: AST_TYPE.ArrayPattern,
      elements: [ident("a"), ident("a")],
    } as unknown as AstNode;
    const out = allBindingVariables(pattern, scope);
    expect(out).toHaveLength(1);
  });

  test("empty pattern → empty output", () => {
    const scope = scopeWith();
    const empty = {
      type: AST_TYPE.ObjectPattern,
      properties: [],
    } as unknown as AstNode;
    expect(allBindingVariables(empty, scope)).toEqual([]);
  });
});
