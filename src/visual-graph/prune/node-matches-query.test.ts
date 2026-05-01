import { describe, expect, test } from "vitest";

import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import { ROOT_QUERY_KIND } from "../../cli/root-query/root-query-kind.js";
import type { VisualNode } from "../model.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { nodeMatchesQuery } from "./node-matches-query.js";

const node = (
  overrides: Partial<
    Extract<VisualNode, { kind: typeof NODE_KIND.Variable }>
  > = {},
): VisualNode => ({
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
  ...overrides,
});

describe("nodeMatchesQuery", () => {
  test("kind=line matches when line falls within [start,end]", () => {
    const q = {
      kind: ROOT_QUERY_KIND.Line,
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
      kind: ROOT_QUERY_KIND.LineName,
      line: 5,
      name: "x",
      raw: "5:x",
    } as const satisfies ParsedRootQuery;
    expect(nodeMatchesQuery(node({ line: 5, name: "x" }), q)).toBe(true);
    expect(nodeMatchesQuery(node({ line: 5, name: "y" }), q)).toBe(false);
  });

  test("kind=range overlaps node line range", () => {
    const q = {
      kind: ROOT_QUERY_KIND.Range,
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
      kind: ROOT_QUERY_KIND.RangeName,
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
      kind: ROOT_QUERY_KIND.Name,
      name: "x",
      raw: "x",
    } as const satisfies ParsedRootQuery;
    expect(nodeMatchesQuery(node({ name: "x" }), q)).toBe(true);
    expect(
      nodeMatchesQuery(
        {
          type: VISUAL_ELEMENT_TYPE.Node,
          id: "n1",
          kind: NODE_KIND.WriteOp,
          name: "x",
          line: 5,
          endLine: null,
          isJsxElement: false,
          unused: false,
          declarationKind: null,
        },
        q,
      ),
    ).toBe(false);
    expect(
      nodeMatchesQuery(
        {
          type: VISUAL_ELEMENT_TYPE.Node,
          id: "n1",
          kind: NODE_KIND.ReturnUse,
          name: "x",
          line: 5,
          endLine: null,
          isJsxElement: false,
          unused: false,
        },
        q,
      ),
    ).toBe(false);
  });
});
