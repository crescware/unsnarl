import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/model.js";
import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { subgraphLabel } from "./subgraph-label.js";
import { makeNode } from "./testing/make-node.js";
import { makeSubgraph } from "./testing/make-subgraph.js";

const emptyMap = new Map<string, VisualNode>();

describe("subgraphLabel", () => {
  test("function uses ownerName when present, falling back to nodeMap, then ''", () => {
    const sg = makeSubgraph({
      kind: SUBGRAPH_KIND.Function,
      ownerName: "myFn",
      ownerNodeId: "n_owner",
      line: 2,
      endLine: 5,
    });
    expect(subgraphLabel(sg, emptyMap)).toBe("myFn()<br/>L2-5");
  });

  test("function falls back to ownerNode.name when ownerName is missing", () => {
    const sg = makeSubgraph({
      kind: SUBGRAPH_KIND.Function,
      ownerNodeId: "n_owner",
      line: 1,
    });
    const map = new Map([
      ["n_owner", makeNode({ id: "n_owner", name: "fallback" })],
    ]);
    expect(subgraphLabel(sg, map)).toBe("fallback()<br/>L1");
  });

  test("function with neither ownerName nor a known ownerNodeId yields an empty name", () => {
    const sg = makeSubgraph({ kind: SUBGRAPH_KIND.Function, line: 1 });
    expect(subgraphLabel(sg, emptyMap)).toBe("()<br/>L1");
  });

  test("case with explicit caseTest gets 'case <test>'", () => {
    const sg = makeSubgraph({
      kind: SUBGRAPH_KIND.Case,
      caseTest: "1",
      line: 4,
    });
    expect(subgraphLabel(sg, emptyMap)).toBe("case 1 L4");
  });

  test("case with null caseTest renders as 'default'", () => {
    const sg = makeSubgraph({
      kind: SUBGRAPH_KIND.Case,
      caseTest: null,
      line: 4,
    });
    expect(subgraphLabel(sg, emptyMap)).toBe("default L4");
  });

  test("case with undefined caseTest also renders as 'default'", () => {
    const sg = makeSubgraph({ kind: SUBGRAPH_KIND.Case, line: 4 });
    expect(subgraphLabel(sg, emptyMap)).toBe("default L4");
  });

  test("if-else-container with hasElse=true says 'if-else', otherwise 'if'", () => {
    expect(
      subgraphLabel(
        makeSubgraph({ kind: SUBGRAPH_KIND.IfElseContainer, hasElse: true }),
        emptyMap,
      ),
    ).toBe("if-else L1");
    expect(
      subgraphLabel(
        makeSubgraph({ kind: SUBGRAPH_KIND.IfElseContainer, hasElse: false }),
        emptyMap,
      ),
    ).toBe("if L1");
  });
});
