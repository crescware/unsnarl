import { describe, expect, test } from "vitest";

import { lineRangeLabel } from "./line-range-label.js";
import { baseSubgraph } from "./testing/make-subgraph.js";

describe("lineRangeLabel", () => {
  test.each<{
    name: string;
    line: number;
    endLine: number | undefined;
    expected: string;
  }>([
    {
      name: "single line when endLine is undefined",
      line: 5,
      endLine: undefined,
      expected: "L5",
    },
    {
      name: "single line when endLine equals line",
      line: 5,
      endLine: 5,
      expected: "L5",
    },
    {
      name: "range when endLine differs from line",
      line: 5,
      endLine: 10,
      expected: "L5-10",
    },
    {
      name: "range with adjacent lines",
      line: 1,
      endLine: 2,
      expected: "L1-2",
    },
  ])("$name", ({ line, endLine, expected }) => {
    const sg =
      endLine === undefined
        ? { ...baseSubgraph(), line }
        : { ...baseSubgraph(), line, endLine };
    expect(lineRangeLabel(sg)).toBe(expected);
  });
});
