import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/model.js";
import { collectNodesInto } from "./collect-nodes-into.js";
import { makeNode } from "./testing/make-node.js";
import { makeSubgraph } from "./testing/make-subgraph.js";

describe("collectNodesInto", () => {
  test("collects flat top-level nodes keyed by id", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto([makeNode({ id: "a" }), makeNode({ id: "b" })], out);
    expect([...out.keys()]).toEqual(["a", "b"]);
  });

  test("recursively descends into subgraph elements", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto(
      [
        makeNode({ id: "a" }),
        makeSubgraph({
          id: "s1",
          elements: [
            makeNode({ id: "b" }),
            makeSubgraph({ id: "s2", elements: [makeNode({ id: "c" })] }),
          ],
        }),
      ],
      out,
    );
    expect([...out.keys()].sort()).toEqual(["a", "b", "c"]);
  });

  test("does NOT add subgraph ids", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto([makeSubgraph({ id: "sg" })], out);
    expect([...out.keys()]).toEqual([]);
  });

  test("preserves the latest write when ids collide", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto(
      [
        makeNode({ id: "a", name: "first" }),
        makeNode({ id: "a", name: "second" }),
      ],
      out,
    );
    expect(out.get("a")?.name).toBe("second");
  });

  test("empty input list -> empty out", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto([], out);
    expect(out.size).toBe(0);
  });
});
