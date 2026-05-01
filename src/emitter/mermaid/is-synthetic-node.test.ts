import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/model.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { isSyntheticNode } from "./is-synthetic-node.js";
import { makeNode } from "./testing/make-node.js";

describe("isSyntheticNode", () => {
  test.each<{ kind: VisualNode["kind"]; expected: boolean }>([
    { kind: NODE_KIND.ModuleSink, expected: true },
    { kind: NODE_KIND.ModuleSource, expected: true },
    { kind: NODE_KIND.ImportIntermediate, expected: true },
    { kind: NODE_KIND.Variable, expected: false },
    { kind: NODE_KIND.FunctionName, expected: false },
    { kind: NODE_KIND.ClassName, expected: false },
    { kind: NODE_KIND.Parameter, expected: false },
    { kind: NODE_KIND.CatchClause, expected: false },
    { kind: NODE_KIND.ImportBinding, expected: false },
    { kind: NODE_KIND.ImplicitGlobalVariable, expected: false },
    { kind: NODE_KIND.WriteOp, expected: false },
    { kind: NODE_KIND.ReturnUse, expected: false },
  ])("kind=$kind -> $expected", ({ kind, expected }) => {
    expect(isSyntheticNode(makeNode({ kind }))).toBe(expected);
  });
});
