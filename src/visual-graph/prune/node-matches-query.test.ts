import { describe, expect, test } from "vitest";

import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import type { NodeKind, VisualNode } from "../model.js";
import { nodeMatchesQuery } from "./node-matches-query.js";

const node = (overrides: Partial<VisualNode> = {}): VisualNode => ({
  type: "node",
  id: "n1",
  kind: "Variable",
  name: "x",
  line: 5,
  isJsxElement: false,
  ...overrides,
});

describe("nodeMatchesQuery", () => {
  test("kind=line matches when line falls within [start,end]", () => {
    const q = {
      kind: "line",
      line: 5,
      raw: "5",
    } as const satisfies ParsedRootQuery;
    expect(nodeMatchesQuery(node({ line: 5 }), q)).toBe(true);
    expect(nodeMatchesQuery(node({ line: 4 }), q)).toBe(false);
    expect(
      nodeMatchesQuery(node({ line: 5, endLine: 7 }), { ...q, line: 6 }),
    ).toBe(true);
    expect(
      nodeMatchesQuery(node({ line: 5, endLine: 7 }), { ...q, line: 8 }),
    ).toBe(false);
  });

  test("kind=line-name additionally requires exact name match", () => {
    const q = {
      kind: "line-name",
      line: 5,
      name: "x",
      raw: "5:x",
    } as const satisfies ParsedRootQuery;
    expect(nodeMatchesQuery(node({ line: 5, name: "x" }), q)).toBe(true);
    expect(nodeMatchesQuery(node({ line: 5, name: "y" }), q)).toBe(false);
  });

  test("kind=range overlaps node line range", () => {
    const q = {
      kind: "range",
      start: 4,
      end: 6,
      raw: "4-6",
    } as const satisfies ParsedRootQuery;
    expect(nodeMatchesQuery(node({ line: 5 }), q)).toBe(true);
    expect(nodeMatchesQuery(node({ line: 7 }), q)).toBe(false);
    expect(nodeMatchesQuery(node({ line: 1, endLine: 4 }), q)).toBe(true);
  });

  test("kind=range-name additionally requires exact name match", () => {
    const q = {
      kind: "range-name",
      start: 4,
      end: 6,
      name: "x",
      raw: "4-6:x",
    } as const satisfies ParsedRootQuery;
    expect(nodeMatchesQuery(node({ line: 5, name: "x" }), q)).toBe(true);
    expect(nodeMatchesQuery(node({ line: 5, name: "y" }), q)).toBe(false);
  });

  test("kind=name matches by name except for excluded use-site kinds", () => {
    const q = {
      kind: "name",
      name: "x",
      raw: "x",
    } as const satisfies ParsedRootQuery;
    expect(nodeMatchesQuery(node({ name: "x" }), q)).toBe(true);
    for (const kind of ["WriteOp", "ReturnUse"] satisfies NodeKind[]) {
      expect(nodeMatchesQuery(node({ name: "x", kind }), q)).toBe(false);
    }
  });
});
