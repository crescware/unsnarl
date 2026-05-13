import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import { isSyntheticNode } from "./is-synthetic-node.js";
import {
  baseImportBindingDefault,
  baseNode,
  baseSimpleNode,
  baseWriteOpNode,
} from "./testing/make-node.js";

function nodeOfKind(kind: VisualNode["kind"]): VisualNode {
  switch (kind) {
    case NODE_KIND.LegacyVariable:
      return baseNode();
    case NODE_KIND.LegacyWriteOp:
      return baseWriteOpNode();
    case NODE_KIND.LegacyImportBinding:
      return baseImportBindingDefault();
    default:
      return baseSimpleNode(kind);
  }
}

describe("isSyntheticNode", () => {
  test.each<{ kind: VisualNode["kind"]; expected: boolean }>([
    { kind: NODE_KIND.LegacyModuleSink, expected: true },
    { kind: NODE_KIND.LegacyModuleSource, expected: true },
    { kind: NODE_KIND.LegacyImportIntermediate, expected: true },
    { kind: NODE_KIND.LegacyExpressionStatement, expected: true },
    { kind: NODE_KIND.LegacyVariable, expected: false },
    { kind: NODE_KIND.LegacyFunctionName, expected: false },
    { kind: NODE_KIND.LegacyClassName, expected: false },
    { kind: NODE_KIND.LegacyParameter, expected: false },
    { kind: NODE_KIND.LegacyCatchClause, expected: false },
    { kind: NODE_KIND.LegacyImportBinding, expected: false },
    { kind: NODE_KIND.LegacyImplicitGlobalVariable, expected: false },
    { kind: NODE_KIND.LegacyWriteOp, expected: false },
    { kind: NODE_KIND.LegacyReturnUse, expected: false },
  ])("kind=$kind -> $expected", ({ kind, expected }) => {
    expect(isSyntheticNode(nodeOfKind(kind))).toEqual(expected);
  });
});
