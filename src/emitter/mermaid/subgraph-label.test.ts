import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/visual-node.js";
import { subgraphLabel } from "./subgraph-label.js";
import { baseNode } from "./testing/make-node.js";
import {
  baseCaseSubgraph,
  baseIfElseContainerSubgraph,
  baseSubgraph,
} from "./testing/make-subgraph.js";

const emptyMap = new Map<string, VisualNode>();

describe("subgraphLabel", () => {
  test("function uses ownerName when present, falling back to nodeMap, then ''", () => {
    const sg = {
      ...baseSubgraph(),
      ownerName: "myFn",
      ownerNodeId: "n_owner",
      line: 2,
      endLine: 5,
    };
    expect(subgraphLabel(sg, emptyMap, false)).toBe("myFn()<br/>L2-5");
  });

  test("function falls back to ownerNode.name when ownerName is empty", () => {
    const sg = {
      ...baseSubgraph(),
      ownerNodeId: "n_owner",
      ownerName: "",
      line: 1,
    };
    const map = new Map([
      ["n_owner", { ...baseNode(), id: "n_owner", name: "fallback" }],
    ]);
    expect(subgraphLabel(sg, map, false)).toBe("fallback()<br/>L1");
  });

  test("function with empty ownerName and unknown ownerNodeId yields an empty name", () => {
    const sg = {
      ...baseSubgraph(),
      ownerNodeId: "n_owner",
      ownerName: "",
      line: 1,
    };
    expect(subgraphLabel(sg, emptyMap, false)).toBe("()<br/>L1");
  });

  test("case with explicit caseTest gets 'case <test>'", () => {
    const sg = { ...baseCaseSubgraph(), caseTest: "1", line: 4 };
    expect(subgraphLabel(sg, emptyMap, false)).toBe("case 1 L4");
  });

  test("case with null caseTest renders as 'default'", () => {
    const sg = { ...baseCaseSubgraph(), caseTest: null, line: 4 };
    expect(subgraphLabel(sg, emptyMap, false)).toBe("default L4");
  });

  test("if-else-container with hasElse=true says 'if-else', otherwise 'if'", () => {
    expect(
      subgraphLabel(
        { ...baseIfElseContainerSubgraph(), hasElse: true },
        emptyMap,
        false,
      ),
    ).toBe("if-else L1");
    expect(
      subgraphLabel(
        { ...baseIfElseContainerSubgraph(), hasElse: false },
        emptyMap,
        false,
      ),
    ).toBe("if L1");
  });

  describe("debug=true", () => {
    test("appends SUBGRAPH_KIND to the standard label", () => {
      const sg = {
        ...baseSubgraph(),
        ownerName: "myFn",
        ownerNodeId: "n_owner",
        line: 2,
        endLine: 5,
      };
      expect(subgraphLabel(sg, emptyMap, true)).toBe(
        "myFn()<br/>L2-5<br/>function",
      );
    });

    test("if-else-container surfaces the kind even when the prefix differs", () => {
      expect(
        subgraphLabel(
          { ...baseIfElseContainerSubgraph(), hasElse: true },
          emptyMap,
          true,
        ),
      ).toBe("if-else L1<br/>if-else-container");
    });
  });
});
