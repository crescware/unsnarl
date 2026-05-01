import type { VisualGraph } from "../../visual-graph/model.js";

export function renderBoundaryEdges(
  graph: VisualGraph,
  lines: /* mutable */ string[],
  stubIds: /* mutable */ string[],
): void {
  if (graph.boundaryEdges === undefined || graph.boundaryEdges.length === 0) {
    return;
  }
  // Pruning detected one or more neighbors past the requested radius.
  // Mermaid cannot draw a truly dangling edge, so each boundary edge gets
  // a faint stub node "(...)" attached via a dashed arrow. The stub stands
  // in for "more graph keeps going beyond here". The label question follows
  // the edge semantics `from -label-> to`, where the label describes the
  // action `to` performs on `from`:
  //
  // - "out" (`inside -> stub`): the actor is the stub, which is unknown,
  //   so we cannot honestly attach a label.
  // - "in"  (`stub -> inside`): the actor is the kept inside node, so we
  //   keep the original label.
  let stubCounter = 0;
  for (const be of graph.boundaryEdges) {
    stubCounter += 1;
    const stubId = `boundary_stub_${stubCounter}`;
    stubIds.push(stubId);
    // ASCII "..." instead of U+2026 -- some Mermaid renderers stumble
    // on multibyte glyphs inside node shape syntax.
    lines.push(`  ${stubId}((...))`);
    if (be.direction === "out") {
      lines.push(`  ${be.inside} -.-> ${stubId}`);
    } else {
      lines.push(`  ${stubId} -.->|${be.label}| ${be.inside}`);
    }
  }
}
