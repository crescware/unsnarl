import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeSyntax } from "./node-syntax.js";
import { baseNode } from "./testing/make-node.js";

describe("nodeSyntax", () => {
  test('WriteOp uses stadium brackets (["..."])', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.WriteOp,
        name: "x",
        line: 3,
      },
      false,
    );
    expect(got.startsWith('(["')).toBe(true);
    expect(got.endsWith('"])')).toBe(true);
  });

  test("ModuleSink uses double-circle brackets ((...))", () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.ModuleSink,
        name: "module",
      },
      false,
    );
    expect(got).toBe("((module))");
  });

  test('IfTest uses diamond brackets {"..."}', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.IfTest,
        name: "if-test",
        line: 5,
      },
      false,
    );
    expect(got).toBe('{"if ()<br/>L5"}');
  });

  test('SwitchDiscriminant uses diamond brackets {"..."}', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.SwitchDiscriminant,
        name: "switch-discriminant",
        line: 7,
      },
      false,
    );
    expect(got).toBe('{"switch ()<br/>L7"}');
  });

  test('default kind uses square brackets ["..."]', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.Variable,
        name: "x",
        line: 4,
      },
      false,
    );
    expect(got.startsWith('["')).toBe(true);
    expect(got.endsWith('"]')).toBe(true);
  });

  test("debug=true threads NODE_KIND into the inner label", () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.IfTest,
        name: "if-test",
        line: 5,
      },
      true,
    );
    expect(got).toBe('{"if ()<br/>L5<br/>IfTest"}');
  });
});
