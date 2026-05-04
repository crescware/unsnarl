import { describe, expect, test } from "vitest";

import { renderPruningComment } from "./render-pruning-comment.js";
import { baseGraph } from "./testing/make-graph.js";

describe("renderPruningComment", () => {
  test("does nothing when graph.pruning is null", () => {
    const lines: /* mutable */ string[] = [];
    renderPruningComment(baseGraph(), lines);
    expect(lines).toEqual([]);
  });

  test("emits a single comment summarising roots, ancestors, and descendants", () => {
    const lines: /* mutable */ string[] = [];
    renderPruningComment(
      {
        ...baseGraph(),
        pruning: {
          roots: [
            { query: "L5", matched: 1 },
            { query: "L9", matched: 2 },
          ],
          ancestors: 3,
          descendants: 4,
        },
      },
      lines,
    );
    expect(lines).toEqual([
      "  %% pruning roots L5=1 L9=2 ancestors=3 descendants=4",
    ]);
  });

  test("appends a warning line for each zero-match root", () => {
    const lines: /* mutable */ string[] = [];
    renderPruningComment(
      {
        ...baseGraph(),
        pruning: {
          roots: [
            { query: "L1", matched: 0 },
            { query: "L9", matched: 1 },
            { query: "missing", matched: 0 },
          ],
          ancestors: 0,
          descendants: 0,
        },
      },
      lines,
    );
    expect(lines).toContain("  %% pruning warning query L1 matched 0 roots");
    expect(lines).toContain(
      "  %% pruning warning query missing matched 0 roots",
    );
    expect(lines.some((l) => l.includes("warning") && l.includes("L9"))).toBe(
      false,
    );
  });

  test("empty roots list still emits the summary line", () => {
    const lines: /* mutable */ string[] = [];
    renderPruningComment(
      {
        ...baseGraph(),
        pruning: { roots: [], ancestors: 0, descendants: 0 },
      },
      lines,
    );
    expect(lines).toHaveLength(1);
  });
});
