import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import type { AstNode } from "../../ir/model.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
import type { PathEntry } from "../walk/walk.js";
import { walk } from "../walk/walk.js";
import { handleIdentifierReference } from "./handle-identifier-reference.js";
import { hoistInto } from "./hoist-into.js";
import type { NodeLike } from "./node-like.js";
import { parse } from "./testing/parse.js";

type CapturedIdentifier = {
  node: NodeLike;
  parent: NodeLike | null;
  key: string | null;
  path: readonly PathEntry[];
};

function captureNthIdentifier(
  program: NodeLike,
  n: number,
): CapturedIdentifier | null {
  let count = 0;
  let captured: CapturedIdentifier | null = null;
  walk(program as unknown as AstNode, {
    enter(node, parent, key, path) {
      if (node.type !== AST_TYPE.Identifier) {
        return;
      }
      count++;
      if (count === n) {
        captured = {
          node: node as unknown as NodeLike,
          parent: parent as unknown as NodeLike | null,
          key,
          path: [...path],
        };
      }
    },
  });
  return captured;
}

describe("handleIdentifierReference", () => {
  test("creates a reference for an identifier used in expression position", () => {
    const code = "let foo = 1; foo;";
    const program = parse(code);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    hoistInto(program, manager.globalScope, code, new DiagnosticCollector());

    // Identifier #1 is the binding `foo`, #2 is the reference use.
    const captured = captureNthIdentifier(program, 2);
    expect(captured).not.toBeNull();
    if (!captured) {
      return;
    }

    handleIdentifierReference(
      captured.node,
      captured.parent,
      captured.key,
      captured.path,
      manager,
    );

    const fooVar = manager.globalScope.variables.find((v) => v.name === "foo");
    expect(fooVar?.references).toHaveLength(1);
  });

  test("does nothing for identifiers classified as non-references (e.g. a member property)", () => {
    const code = "obj.prop;";
    const program = parse(code);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    hoistInto(program, manager.globalScope, code, new DiagnosticCollector());

    // Identifier #1 is `obj`; #2 is `prop` (member property -- not a reference).
    const captured = captureNthIdentifier(program, 2);
    expect(captured).not.toBeNull();
    if (!captured) {
      return;
    }

    const refsBefore = manager.globalScope.references.length;
    handleIdentifierReference(
      captured.node,
      captured.parent,
      captured.key,
      captured.path,
      manager,
    );
    expect(manager.globalScope.references.length).toBe(refsBefore);
  });
});
