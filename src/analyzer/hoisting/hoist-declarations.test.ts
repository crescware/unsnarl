import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { DIAGNOSTIC_KIND } from "../diagnostic-kind.js";
import { SCOPE_TYPE } from "../scope-type.js";
import { ScopeImpl } from "../scope.js";
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

  test("var emits a diagnostic but does not abort the rest of the body", () => {
    const scope = newScope();
    const diags = new DiagnosticCollector();
    const raw = "var x = 1; const y = 2;";
    hoistDeclarations(programBody(raw), scope, raw, diags);
    expect(scope.variables.map((v) => v.name)).toEqual(["y"]);
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
