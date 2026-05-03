import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeLabel } from "./node-label.js";
import { baseNode } from "./testing/make-node.js";

describe("nodeLabel", () => {
  test("IfTest emits 'if L<line>' without the head/range/unused decorations", () => {
    expect(
      nodeLabel({
        ...baseNode(),
        kind: NODE_KIND.IfTest,
        name: "ignored",
        line: 3,
      }),
    ).toBe("if L3");
  });

  test("ModuleSink shortcuts to the literal 'module'", () => {
    expect(
      nodeLabel({ ...baseNode(), kind: NODE_KIND.ModuleSink, name: "ignored" }),
    ).toBe("module");
  });

  test("appends the line range as a single line", () => {
    expect(nodeLabel({ ...baseNode(), name: "x", line: 7 })).toBe("x<br/>L7");
  });

  test("appends the line range when endLine differs from line", () => {
    expect(nodeLabel({ ...baseNode(), name: "x", line: 7, endLine: 9 })).toBe(
      "x<br/>L7-9",
    );
  });

  test("collapses to a single line when endLine equals line", () => {
    expect(nodeLabel({ ...baseNode(), name: "x", line: 4, endLine: 4 })).toBe(
      "x<br/>L4",
    );
  });

  test("prefixes with 'unused' when node.unused is true", () => {
    expect(nodeLabel({ ...baseNode(), name: "x", line: 2, unused: true })).toBe(
      "unused x<br/>L2",
    );
  });

  test("'unused' prefix is suppressed when unused is missing or false", () => {
    expect(nodeLabel({ ...baseNode(), name: "x", unused: false })).toBe(
      "x<br/>L1",
    );
    expect(nodeLabel({ ...baseNode(), name: "x" })).toBe("x<br/>L1");
  });
});
