import { describe, expect, test } from "vitest";

import { collectEdgeEndpointIds } from "./collect-edge-endpoint-ids.js";
import { makeEdge } from "./testing/make-edge.js";

describe("collectEdgeEndpointIds", () => {
  test("collects both endpoints of every edge", () => {
    const got = collectEdgeEndpointIds([
      makeEdge({ from: "a", to: "b" }),
      makeEdge({ from: "c", to: "d" }),
    ]);
    expect([...got].sort()).toEqual(["a", "b", "c", "d"]);
  });

  test("deduplicates ids that appear in multiple edges", () => {
    const got = collectEdgeEndpointIds([
      makeEdge({ from: "a", to: "b" }),
      makeEdge({ from: "b", to: "c" }),
    ]);
    expect([...got].sort()).toEqual(["a", "b", "c"]);
  });

  test("empty edge list -> empty set", () => {
    expect(collectEdgeEndpointIds([]).size).toBe(0);
  });

  test("self-loop counts the id once", () => {
    const got = collectEdgeEndpointIds([makeEdge({ from: "a", to: "a" })]);
    expect([...got]).toEqual(["a"]);
  });
});
