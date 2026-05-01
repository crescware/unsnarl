import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstIdentifier, AstNode } from "../../ir/model.js";
import { declareVariable } from "../declare/declare-variable.js";
import { ScopeImpl } from "../scope.js";
import { allBindingVariables } from "./all-binding-variables.js";

const ident = (name: string): AstIdentifier =>
  ({ type: "Identifier", name }) as unknown as AstIdentifier;

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

const scopeWith = (...names: string[]): ScopeImpl => {
  const scope = new ScopeImpl({
    type: "module",
    isStrict: true,
    upper: null,
    block: { type: "Program" } as unknown as AstNode,
  });
  for (const n of names) {
    declareVariable(
      scope,
      ident(n),
      "Variable",
      { type: "VariableDeclarator" } as unknown as AstNode,
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
      type: "block",
      isStrict: true,
      upper,
      block: { type: "BlockStatement" } as unknown as AstNode,
    });
    const out = allBindingVariables(declId("const outer = 1;"), inner);
    expect(out.map((v) => v.name)).toEqual(["outer"]);
  });

  test("duplicate identifiers in pattern resolve to the same variable once", () => {
    const scope = scopeWith("a");
    const pattern = {
      type: "ArrayPattern",
      elements: [ident("a"), ident("a")],
    } as unknown as AstNode;
    const out = allBindingVariables(pattern, scope);
    expect(out).toHaveLength(1);
  });

  test("empty pattern → empty output", () => {
    const scope = scopeWith();
    const empty = {
      type: "ObjectPattern",
      properties: [],
    } as unknown as AstNode;
    expect(allBindingVariables(empty, scope)).toEqual([]);
  });
});
