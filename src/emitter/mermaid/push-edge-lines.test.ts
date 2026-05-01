import { describe, expect, test } from "vitest";

import { pushEdgeLines } from "./push-edge-lines.js";
import { makeEdge } from "./testing/make-edge.js";

describe("pushEdgeLines", () => {
  test("formats each edge as '  <from> -->|<label>| <to>'", () => {
    const lines: /* mutable */ string[] = [];
    pushEdgeLines(
      [
        makeEdge({ from: "a", to: "b", label: "read" }),
        makeEdge({ from: "c", to: "d", label: "write" }),
      ],
      lines,
    );
    expect(lines).toEqual(["  a -->|read| b", "  c -->|write| d"]);
  });

  test("preserves the input order in the appended lines", () => {
    const lines: /* mutable */ string[] = ["existing"];
    pushEdgeLines([makeEdge({ from: "x", to: "y", label: "set" })], lines);
    expect(lines).toEqual(["existing", "  x -->|set| y"]);
  });

  test("empty input pushes nothing", () => {
    const lines: /* mutable */ string[] = [];
    pushEdgeLines([], lines);
    expect(lines).toEqual([]);
  });
});
