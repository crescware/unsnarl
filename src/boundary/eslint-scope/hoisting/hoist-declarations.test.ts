import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import { DIAGNOSTIC_KIND } from "../../../analyzer/diagnostic-kind.js";
import { SCOPE_TYPE } from "../../../analyzer/scope-type.js";
import type { AstNode } from "../../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { DiagnosticCollector } from "../../../util/diagnostic.js";
import { ScopeImpl } from "../scope-impl.js";
import { hoistDeclarations } from "./hoist-declarations.js";

const programBody = (code: string): readonly unknown[] => {
  const program = parseSync("input.ts", code, { lang: "ts" })
    .program as unknown as {
    body: readonly unknown[];
  };
  return program.body;
};

const newScope = (): ScopeImpl =>
  new ScopeImpl({
    type: SCOPE_TYPE.Module,
    isStrict: true,
    upper: null,
    block: { type: AST_TYPE.Program } as unknown as AstNode,
  });

describe("hoistDeclarations", () => {
  test("hoists each declaration in source order", () => {
    const scope = newScope();
    hoistDeclarations(
      programBody("const x = 1; function f() {} class C {}"),
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["x", "f", "C"]);
  });

  test("non-NodeLike entries in body are skipped", () => {
    const scope = newScope();
    hoistDeclarations(
      [null, undefined, "noise", { type: 1 }, ...programBody("const a = 1;")],
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["a"]);
  });

  test("var hoists alongside the rest and emits a var-detected diagnostic", () => {
    const scope = newScope();
    const diags = new DiagnosticCollector();
    const raw = "var x = 1; const y = 2;";
    hoistDeclarations(programBody(raw), scope, raw, diags);
    expect(scope.variables.map((v) => v.name)).toEqual(["x", "y"]);
    expect(diags.list().map((d) => d.kind)).toEqual([
      DIAGNOSTIC_KIND.VarDetected,
    ]);
  });

  test("empty body declares nothing", () => {
    const scope = newScope();
    hoistDeclarations([], scope, "", new DiagnosticCollector());
    expect(scope.variables).toEqual([]);
  });
});
