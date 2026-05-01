import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/model.js";
import { isSyntheticNode } from "./is-synthetic-node.js";
import { makeNode } from "./testing/make-node.js";

describe("isSyntheticNode", () => {
  test.each<{ kind: VisualNode["kind"]; expected: boolean }>([
    { kind: "ModuleSink", expected: true },
    { kind: "ModuleSource", expected: true },
    { kind: "ImportIntermediate", expected: true },
    { kind: "Variable", expected: false },
    { kind: "FunctionName", expected: false },
    { kind: "ClassName", expected: false },
    { kind: "Parameter", expected: false },
    { kind: "CatchClause", expected: false },
    { kind: "ImportBinding", expected: false },
    { kind: "ImplicitGlobalVariable", expected: false },
    { kind: "WriteOp", expected: false },
    { kind: "ReturnUse", expected: false },
  ])("kind=$kind -> $expected", ({ kind, expected }) => {
    expect(isSyntheticNode(makeNode({ kind }))).toBe(expected);
  });
});
