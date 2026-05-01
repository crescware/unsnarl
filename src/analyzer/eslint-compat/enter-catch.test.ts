import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
import { enterCatch } from "./enter-catch.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
const tryParent = {
  type: AST_TYPE.TryStatement,
  start: 0,
} as const satisfies NodeLike;

describe("enterCatch", () => {
  test("pushes a catch scope, declares the param, and hoists body declarations", () => {
    const code = "try {} catch (err) { let z = 1; }";
    const program = parse(code);
    const catchNode = findFirst(program, AST_TYPE.CatchClause);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterCatch(catchNode, tryParent, "handler", manager, code, diagnostics);

    const scope = manager.current();
    expect(scope.type).toBe("catch");
    expect(scope.variables.map((v) => v.name).sort()).toEqual(["err", "z"]);
    expect(scope.variables.find((v) => v.name === "err")?.defs[0]?.type).toBe(
      AST_TYPE.CatchClause,
    );
  });

  test("supports a destructured catch parameter", () => {
    const code = "try {} catch ({ message }) {}";
    const program = parse(code);
    const catchNode = findFirst(program, AST_TYPE.CatchClause);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterCatch(catchNode, tryParent, "handler", manager, code, diagnostics);

    expect(manager.current().variables.map((v) => v.name)).toEqual(["message"]);
  });

  test("handles a missing catch parameter (catch {})", () => {
    const code = "try {} catch { let z = 1; }";
    const program = parse(code);
    const catchNode = findFirst(program, AST_TYPE.CatchClause);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterCatch(catchNode, tryParent, "handler", manager, code, diagnostics);

    expect(manager.current().variables.map((v) => v.name)).toEqual(["z"]);
  });
});
