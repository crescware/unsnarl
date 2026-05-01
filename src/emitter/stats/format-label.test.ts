import { describe, expect, test } from "vitest";

import { VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { VisualNode } from "../../visual-graph/model.js";
import { formatLabel } from "./format-label.js";

const node = (overrides: Partial<VisualNode> = {}): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id: "n1",
  kind: "Variable",
  name: "x",
  line: 5,
  isJsxElement: false,
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
