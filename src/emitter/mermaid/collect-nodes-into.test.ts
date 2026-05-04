import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/visual-node.js";
import { collectNodesInto } from "./collect-nodes-into.js";
import { baseNode } from "./testing/make-node.js";
import { baseSubgraph } from "./testing/make-subgraph.js";

describe("collectNodesInto", () => {
  test("collects flat top-level nodes keyed by id", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto(
      [
        { ...baseNode(), id: "a" },
        { ...baseNode(), id: "b" },
      ],
      out,
    );
    expect([...out.keys()]).toEqual(["a", "b"]);
  });

  test("recursively descends into subgraph elements", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto(
      [
        { ...baseNode(), id: "a" },
        {
          ...baseSubgraph(),
          id: "s1",
          elements: [
            { ...baseNode(), id: "b" },
            {
              ...baseSubgraph(),
              id: "s2",
              elements: [{ ...baseNode(), id: "c" }],
            },
          ],
        },
      ],
      out,
    );
    expect([...out.keys()].sort()).toEqual(["a", "b", "c"]);
  });

  test("does NOT add subgraph ids", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto([{ ...baseSubgraph(), id: "sg" }], out);
    expect([...out.keys()]).toEqual([]);
  });

  test("preserves the latest write when ids collide", () => {
    const out = new Map<string, VisualNode>();
    collectNodesInto(
      [
        { ...baseNode(), id: "a", name: "first" },
        { ...baseNode(), id: "a", name: "second" },
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
