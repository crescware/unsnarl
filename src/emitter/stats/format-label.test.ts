import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/model.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import { formatLabel } from "./format-label.js";

const node = (overrides: Partial<VisualNode> = {}): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id: "n1",
  kind: NODE_KIND.Variable,
  name: "x",
  line: 5,
  endLine: null,
  isJsxElement: false,
  unused: false,
  declarationKind: null,
  initIsFunction: false,
  importKind: null,
  importedName: null,
  importSource: null,
  ...overrides,
});

describe("formatLabel", () => {
  test("path:line name when not unused", () => {
    expect(formatLabel("foo.ts", node({ name: "value", line: 10 }))).toBe(
      "foo.ts:10 value",
    );
  });

  test('"unused " prefix when node.unused is true', () => {
    expect(
      formatLabel("foo.ts", node({ name: "value", line: 10, unused: true })),
    ).toBe("foo.ts:10 unused value");
  });

  test("unused=false is treated as not-unused (no prefix)", () => {
    expect(
      formatLabel("foo.ts", node({ name: "value", line: 10, unused: false })),
    ).toBe("foo.ts:10 value");
  });

  test("undefined unused → no prefix", () => {
    expect(formatLabel("a/b.ts", node({ name: "y", line: 1 }))).toBe(
      "a/b.ts:1 y",
    );
  });
});
