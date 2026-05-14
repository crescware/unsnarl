import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeLabel } from "./node-label.js";
import { baseNode, baseSimpleNode } from "./testing/make-node.js";

describe("nodeLabel", () => {
  test("IfTest emits 'if ()<br/>L<line>' without the head/range/unused decorations", () => {
    expect(
      nodeLabel(
        {
          ...baseNode(),
          kind: NODE_KIND.SyntheticIfStatementTest,
          name: "ignored",
          line: 3,
        },
        false,
      ),
    ).toEqual("if ()<br/>L3");
  });

  test("SwitchDiscriminant emits 'switch ()<br/>L<line>'", () => {
    expect(
      nodeLabel(
        {
          ...baseNode(),
          kind: NODE_KIND.SyntheticSwitchStatementDiscriminant,
          name: "ignored",
          line: 6,
        },
        false,
      ),
    ).toEqual("switch ()<br/>L6");
  });

  test("WhileTest emits 'while ()<br/>L<line>'", () => {
    expect(
      nodeLabel(
        {
          ...baseNode(),
          kind: NODE_KIND.SyntheticWhileStatementTest,
          name: "ignored",
          line: 5,
        },
        false,
      ),
    ).toEqual("while ()<br/>L5");
  });

  test("DoWhileTest emits 'do while ()<br/>L<line>'", () => {
    expect(
      nodeLabel(
        {
          ...baseNode(),
          kind: NODE_KIND.SyntheticDoWhileStatementTest,
          name: "ignored",
          line: 7,
        },
        false,
      ),
    ).toEqual("do while ()<br/>L7");
  });

  test("ForTest emits 'for ()<br/>L<line>'", () => {
    expect(
      nodeLabel(
        {
          ...baseNode(),
          kind: NODE_KIND.LegacyForTest,
          name: "ignored",
          line: 4,
        },
        false,
      ),
    ).toEqual("for ()<br/>L4");
  });

  test("ModuleSink shortcuts to the literal 'module'", () => {
    expect(
      nodeLabel(
        { ...baseNode(), kind: NODE_KIND.SyntheticModuleSink, name: "ignored" },
        false,
      ),
    ).toEqual("module");
  });

  test("ImplicitGlobalVariable renders without a line suffix because it has no source location", () => {
    expect(
      nodeLabel(
        {
          ...baseSimpleNode(NODE_KIND.SyntheticImplicitGlobal),
          name: "Math",
          line: 0,
        },
        false,
      ),
    ).toEqual("global Math");
  });

  test("ExpressionStatement renders the head followed by the statement line", () => {
    expect(
      nodeLabel(
        {
          ...baseSimpleNode(NODE_KIND.SyntheticExpressionStatement),
          name: "console.log()",
          line: 7,
        },
        false,
      ),
    ).toEqual("console.log()<br/>L7");
  });

  test("appends the line range as a single line", () => {
    expect(nodeLabel({ ...baseNode(), name: "x", line: 7 }, false)).toEqual(
      "x<br/>L7",
    );
  });

  test("appends the line range when endLine differs from line", () => {
    expect(
      nodeLabel({ ...baseNode(), name: "x", line: 7, endLine: 9 }, false),
    ).toEqual("x<br/>L7-9");
  });

  test("collapses to a single line when endLine equals line", () => {
    expect(
      nodeLabel({ ...baseNode(), name: "x", line: 4, endLine: 4 }, false),
    ).toEqual("x<br/>L4");
  });

  test("prefixes with 'unused' when node.unused is true", () => {
    expect(
      nodeLabel({ ...baseNode(), name: "x", line: 2, unused: true }, false),
    ).toEqual("unused x<br/>L2");
  });

  test("'unused' prefix is suppressed when unused is missing or false", () => {
    expect(
      nodeLabel({ ...baseNode(), name: "x", unused: false }, false),
    ).toEqual("x<br/>L1");
    expect(nodeLabel({ ...baseNode(), name: "x" }, false)).toEqual("x<br/>L1");
  });

  describe("debug=true", () => {
    test("appends NODE_KIND to the standard label", () => {
      expect(nodeLabel({ ...baseNode(), name: "x", line: 7 }, true)).toEqual(
        "x<br/>L7<br/>ConstBinding",
      );
    });

    test("appends NODE_KIND to the IfTest anchor label", () => {
      expect(
        nodeLabel(
          {
            ...baseNode(),
            kind: NODE_KIND.SyntheticIfStatementTest,
            name: "ignored",
            line: 3,
          },
          true,
        ),
      ).toEqual("if ()<br/>L3<br/>SyntheticIfStatementTest");
    });

    test("appends NODE_KIND to ModuleSink even when the base label has no line", () => {
      expect(
        nodeLabel(
          {
            ...baseNode(),
            kind: NODE_KIND.SyntheticModuleSink,
            name: "ignored",
          },
          true,
        ),
      ).toEqual("module<br/>SyntheticModuleSink");
    });

    test("appends NODE_KIND to ImplicitGlobalVariable", () => {
      expect(
        nodeLabel(
          {
            ...baseSimpleNode(NODE_KIND.SyntheticImplicitGlobal),
            name: "Math",
            line: 0,
          },
          true,
        ),
      ).toEqual("global Math<br/>SyntheticImplicitGlobal");
    });
  });
});
