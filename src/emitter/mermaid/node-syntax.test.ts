import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeSyntax } from "./node-syntax.js";
import { baseNode } from "./testing/make-node.js";

describe("nodeSyntax", () => {
  test('WriteOp uses stadium brackets (["..."])', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.LegacyWriteOp,
        name: "x",
        line: 3,
      },
      false,
    );
    expect(got.startsWith('(["')).toEqual(true);
    expect(got.endsWith('"])')).toEqual(true);
  });

  test("ModuleSink uses double-circle brackets ((...))", () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.SyntheticModuleSink,
        name: "module",
      },
      false,
    );
    expect(got).toEqual("((module))");
  });

  test('IfTest uses diamond brackets {"..."}', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.SyntheticIfStatementTest,
        name: "if-test",
        line: 5,
      },
      false,
    );
    expect(got).toEqual('{"if ()<br/>L5"}');
  });

  test('SwitchDiscriminant uses diamond brackets {"..."}', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.LegacySwitchDiscriminant,
        name: "switch-discriminant",
        line: 7,
      },
      false,
    );
    expect(got).toEqual('{"switch ()<br/>L7"}');
  });

  test('default kind uses square brackets ["..."]', () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.LegacyVariable,
        name: "x",
        line: 4,
      },
      false,
    );
    expect(got.startsWith('["')).toEqual(true);
    expect(got.endsWith('"]')).toEqual(true);
  });

  test("debug=true threads NODE_KIND into the inner label", () => {
    const got = nodeSyntax(
      {
        ...baseNode(),
        kind: NODE_KIND.SyntheticIfStatementTest,
        name: "if-test",
        line: 5,
      },
      true,
    );
    expect(got).toEqual('{"if ()<br/>L5<br/>SyntheticIfStatementTest"}');
  });
});
