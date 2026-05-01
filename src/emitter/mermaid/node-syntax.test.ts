import { describe, expect, test } from "vitest";

import { nodeSyntax } from "./node-syntax.js";
import { makeNode } from "./testing/make-node.js";

describe("nodeSyntax", () => {
  test('WriteOp uses stadium brackets (["..."])', () => {
    const got = nodeSyntax(makeNode({ kind: "WriteOp", name: "x", line: 3 }));
    expect(got.startsWith('(["')).toBe(true);
    expect(got.endsWith('"])')).toBe(true);
  });

  test("ModuleSink uses double-circle brackets ((...))", () => {
    const got = nodeSyntax(makeNode({ kind: "ModuleSink", name: "module" }));
    expect(got).toBe("((module))");
  });

  test('default kind uses square brackets ["..."]', () => {
    const got = nodeSyntax(makeNode({ kind: "Variable", name: "x", line: 4 }));
    expect(got.startsWith('["')).toBe(true);
    expect(got.endsWith('"]')).toBe(true);
  });
});
