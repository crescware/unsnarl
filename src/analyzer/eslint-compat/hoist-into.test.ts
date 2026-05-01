import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { DIAGNOSTIC_KIND } from "../diagnostic-kind.js";
import { ScopeManager } from "../manager.js";
import { hoistInto } from "./hoist-into.js";
import type { NodeLike } from "./node-like.js";
import { parse } from "./testing/parse.js";

function freshManager(program: object): ScopeManager {
  return new ScopeManager("module", program as unknown as AstNode);
}

describe("hoistInto", () => {
  test("hoists let declarations from program body into the given scope", () => {
    const code = "let a = 1; let b = 2;";
    const program = parse(code);
    const manager = freshManager(program);
    const diagnostics = new DiagnosticCollector();

    hoistInto(program, manager.globalScope, code, diagnostics);

    expect(manager.globalScope.variables.map((v) => v.name)).toEqual([
      "a",
      "b",
    ]);
  });

  test("hoists const declarations", () => {
    const code = "const x = 1;";
    const program = parse(code);
    const manager = freshManager(program);
    const diagnostics = new DiagnosticCollector();

    hoistInto(program, manager.globalScope, code, diagnostics);

    expect(manager.globalScope.variables.map((v) => v.name)).toEqual(["x"]);
  });

  test("emits a diagnostic for var declarations", () => {
    const code = "var legacy = 1;";
    const program = parse(code, "js");
    const manager = freshManager(program);
    const diagnostics = new DiagnosticCollector();

    hoistInto(program, manager.globalScope, code, diagnostics);

    expect(
      diagnostics.list().some((d) => d.kind === DIAGNOSTIC_KIND.VarDetected),
    ).toBe(true);
  });

  test("does nothing when program has no body array", () => {
    const program = { type: AST_TYPE.Program } satisfies NodeLike;
    const manager = freshManager(program);
    const diagnostics = new DiagnosticCollector();

    hoistInto(program, manager.globalScope, "", diagnostics);

    expect(manager.globalScope.variables).toHaveLength(0);
    expect(diagnostics.list()).toHaveLength(0);
  });
});
