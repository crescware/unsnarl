import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeSyntax } from "./node-syntax.js";
import { baseNode } from "./testing/make-node.js";

describe("nodeSyntax", () => {
  test('WriteOp uses stadium brackets (["..."])', () => {
    const got = nodeSyntax({
      ...baseNode(),
      kind: NODE_KIND.WriteOp,
      name: "x",
      line: 3,
    });
    expect(got.startsWith('(["')).toBe(true);
    expect(got.endsWith('"])')).toBe(true);
  });

  test("ModuleSink uses double-circle brackets ((...))", () => {
    const got = nodeSyntax({
      ...baseNode(),
      kind: NODE_KIND.ModuleSink,
      name: "module",
    });
    expect(got).toBe("((module))");
  });

  test('IfTest uses diamond brackets {"..."}', () => {
    const got = nodeSyntax({
      ...baseNode(),
      kind: NODE_KIND.IfTest,
      name: "if-test",
      line: 5,
    });
    expect(got).toBe('{"if L5"}');
  });

  test('default kind uses square brackets ["..."]', () => {
    const got = nodeSyntax({
      ...baseNode(),
      kind: NODE_KIND.Variable,
      name: "x",
      line: 4,
    });
    expect(got.startsWith('["')).toBe(true);
    expect(got.endsWith('"]')).toBe(true);
  });
});
