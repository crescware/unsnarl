import { describe, expect, test } from "vitest";

import { ROOT_QUERY_KIND } from "../../root-query/root-query-kind.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualNode } from "../visual-node.js";
import { nodeMatchesHighlightQuery } from "./node-matches-highlight-query.js";

function variable(
  name: string,
  line: number,
  endLine: number | null = null,
): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    kind: NODE_KIND.LegacyVariable,
    id: "n",
    name,
    line,
    endLine,
    isJsxElement: false,
    unused: false,
    declarationKind: VARIABLE_DECLARATION_KIND.Const,
    initIsFunction: false,
  };
}

function returnUse(name: string, line: number): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    kind: NODE_KIND.LegacyReturnUse,
    id: "n",
    name,
    line,
    endLine: null,
    isJsxElement: false,
    unused: false,
  };
}

function writeOp(name: string, line: number): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    kind: NODE_KIND.LegacyWriteOp,
    id: "n",
    name,
    line,
    endLine: null,
    isJsxElement: false,
    unused: false,
    declarationKind: null,
  };
}

describe("nodeMatchesHighlightQuery", () => {
  test("line query matches when the query line falls within the node's [line, endLine]", () => {
    expect(
      nodeMatchesHighlightQuery(variable("x", 5), {
        kind: ROOT_QUERY_KIND.Line,
        line: 5,
        raw: "5",
      }),
    ).toEqual(true);
    expect(
      nodeMatchesHighlightQuery(variable("x", 3, 7), {
        kind: ROOT_QUERY_KIND.Line,
        line: 5,
        raw: "5",
      }),
    ).toEqual(true);
    expect(
      nodeMatchesHighlightQuery(variable("x", 5), {
        kind: ROOT_QUERY_KIND.Line,
        line: 6,
        raw: "6",
      }),
    ).toEqual(false);
  });

  test("line-name query requires both line membership and the name", () => {
    const q = {
      kind: ROOT_QUERY_KIND.LineName,
      line: 5,
      name: "x",
      raw: "5:x",
    } as const;
    expect(nodeMatchesHighlightQuery(variable("x", 5), q)).toEqual(true);
    expect(nodeMatchesHighlightQuery(variable("y", 5), q)).toEqual(false);
    expect(nodeMatchesHighlightQuery(variable("x", 6), q)).toEqual(false);
  });

  test("range query treats node spans inclusively against [start, end]", () => {
    const q = {
      kind: ROOT_QUERY_KIND.Range,
      start: 3,
      end: 7,
      raw: "3-7",
    } as const;
    expect(nodeMatchesHighlightQuery(variable("x", 5), q)).toEqual(true);
    expect(nodeMatchesHighlightQuery(variable("x", 7, 10), q)).toEqual(true);
    expect(nodeMatchesHighlightQuery(variable("x", 8), q)).toEqual(false);
  });

  test("range-name query requires range overlap AND the name", () => {
    const q = {
      kind: ROOT_QUERY_KIND.RangeName,
      start: 3,
      end: 7,
      name: "x",
      raw: "3-7:x",
    } as const;
    expect(nodeMatchesHighlightQuery(variable("x", 5), q)).toEqual(true);
    expect(nodeMatchesHighlightQuery(variable("y", 5), q)).toEqual(false);
    expect(nodeMatchesHighlightQuery(variable("x", 8), q)).toEqual(false);
  });

  // The whole reason a highlight-specific matcher exists: pruning's
  // `nodeMatchesQuery` would reject WriteOp / ReturnUse here, but
  // highlight wants the use-sites coloured too.
  test("name query DOES match WriteOp and ReturnUse, unlike the prune matcher", () => {
    const q = {
      kind: ROOT_QUERY_KIND.Name,
      name: "counter",
      raw: "counter",
    } as const;
    expect(nodeMatchesHighlightQuery(variable("counter", 1), q)).toEqual(true);
    expect(nodeMatchesHighlightQuery(writeOp("counter", 2), q)).toEqual(true);
    expect(nodeMatchesHighlightQuery(returnUse("counter", 3), q)).toEqual(true);
    expect(nodeMatchesHighlightQuery(variable("other", 1), q)).toEqual(false);
  });

  test("line-or-name is unreachable post-resolution and always returns false", () => {
    const q = {
      kind: ROOT_QUERY_KIND.LineOrName,
      line: 5,
      name: "L5",
      raw: "L5",
    } as const;
    expect(nodeMatchesHighlightQuery(variable("L5", 5), q)).toEqual(false);
  });
});
