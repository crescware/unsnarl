import { describe, expect, test } from "vitest";

import {
  AST_TYPE,
  DIAGNOSTIC_KIND,
  LANGUAGE,
  type Language,
} from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
import { declareForLeft } from "./declare-for-left.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";

function setup(
  code: string,
  forType: string,
  language: Language = LANGUAGE.Ts,
) {
  const program = parse(code, language);
  const forNode = findFirst(program, forType);
  const manager = new ScopeManager("module", program as unknown as AstNode);
  const forScope = manager.push("for", forNode as unknown as AstNode);
  return {
    forNode,
    forScope,
    code,
    diagnostics: new DiagnosticCollector(),
  };
}

describe("declareForLeft", () => {
  test("declares a let binding in a C-style for loop", () => {
    const { forNode, forScope, code, diagnostics } = setup(
      "for (let i = 0; i < 10; i++) {}",
      AST_TYPE.ForStatement,
    );
    declareForLeft(forNode, forScope, code, diagnostics);
    expect(forScope.variables.map((v) => v.name)).toEqual(["i"]);
    expect(diagnostics.list()).toHaveLength(0);
  });

  test("declares const bindings in a for-of loop", () => {
    const { forNode, forScope, code, diagnostics } = setup(
      "for (const x of items) {}",
      "ForOfStatement",
    );
    declareForLeft(forNode, forScope, code, diagnostics);
    expect(forScope.variables.map((v) => v.name)).toEqual(["x"]);
    expect(diagnostics.list()).toHaveLength(0);
  });

  test("declares destructured bindings in a for-in loop", () => {
    const { forNode, forScope, code, diagnostics } = setup(
      "for (const { a, b } in obj) {}",
      "ForInStatement",
    );
    declareForLeft(forNode, forScope, code, diagnostics);
    expect(forScope.variables.map((v) => v.name).sort()).toEqual(["a", "b"]);
  });

  test("emits a var-detected diagnostic and skips the binding when var is used", () => {
    const { forNode, forScope, code, diagnostics } = setup(
      "for (var i = 0; i < 1; i++) {}",
      AST_TYPE.ForStatement,
      "js",
    );
    declareForLeft(forNode, forScope, code, diagnostics);
    expect(forScope.variables).toHaveLength(0);
    const events = diagnostics.list();
    expect(events.some((d) => d.kind === DIAGNOSTIC_KIND.VarDetected)).toBe(
      true,
    );
  });

  test("ignores assignment-only init (no VariableDeclaration)", () => {
    const { forNode, forScope, code, diagnostics } = setup(
      "for (i = 0; i < 1; i++) {}",
      AST_TYPE.ForStatement,
      "js",
    );
    declareForLeft(forNode, forScope, code, diagnostics);
    expect(forScope.variables).toHaveLength(0);
    expect(diagnostics.list()).toHaveLength(0);
  });
});
