import { describe, expect, test } from "vitest";

import type { VisualGraphPruning } from "../../visual-graph/model.js";
import { formatPruningQuery } from "./format-pruning-query.js";

const pruning = (
  roots: ReadonlyArray<{ query: string; matched: number }>,
  descendants: number,
  ancestors: number,
): VisualGraphPruning => ({ roots, descendants, ancestors });

describe("formatPruningQuery", () => {
  test("symmetric radius collapses to -C", () => {
    expect(
      formatPruningQuery(pruning([{ query: "value", matched: 1 }], 2, 2)),
    ).toBe("-r value -C 2");
  });

  test("asymmetric radius keeps -A and -B explicit", () => {
    expect(
      formatPruningQuery(pruning([{ query: "value", matched: 1 }], 1, 3)),
    ).toBe("-r value -A 1 -B 3");
  });

  test("multiple root queries are joined with comma", () => {
    expect(
      formatPruningQuery(
        pruning(
          [
            { query: "value", matched: 1 },
            { query: "10-12", matched: 2 },
          ],
          0,
          0,
        ),
      ),
    ).toBe("-r value,10-12 -C 0");
  });

  test("zero radius still emits -C 0", () => {
    expect(
      formatPruningQuery(pruning([{ query: "v", matched: 1 }], 0, 0)),
    ).toBe("-r v -C 0");
  });
});
