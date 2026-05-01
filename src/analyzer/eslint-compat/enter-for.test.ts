import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
import { enterFor } from "./enter-for.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";

describe("enterFor", () => {
  test("pushes a for scope and declares loop bindings", () => {
    const code = "for (let i = 0; i < 10; i++) {}";
    const program = parse(code);
    const forNode = findFirst(program, "ForStatement");
    const parent: NodeLike = { type: "Program", start: 0 };
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFor(forNode, parent, "body", manager, code, diagnostics);

    const scope = manager.current();
    expect(scope.type).toBe("for");
    expect(scope.variables.map((v) => v.name)).toEqual(["i"]);
    expect(scope.unsnarlBlockContext).toEqual({
      parentType: "Program",
      key: "body",
      parentSpanOffset: 0,
    });
  });

  test("works for ForOfStatement", () => {
    const code = "for (const x of items) {}";
    const program = parse(code);
    const forNode = findFirst(program, "ForOfStatement");
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFor(forNode, null, null, manager, code, diagnostics);

    expect(manager.current().variables.map((v) => v.name)).toEqual(["x"]);
  });
});
