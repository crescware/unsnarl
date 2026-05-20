import type { MermaidStrategy } from "./strategy.js";

export const elkStrategy = {
  preambleLines: ['%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%'],

  emptySubgraphPlaceholder: ({ subgraphId, indent }) => {
    // Workaround for @mermaid-js/layout-elk@0.2.1: an empty subgraph
    // breaks the renderer two different ways. (1) When the empty
    // subgraph is referenced by an edge, render-TAZW7USW.mjs:847
    // dereferences `labels[0]` on an undefined `labels` because
    // addSubGraphs only registers a subgraph in childrenById when it
    // has at least one direct child, and the labels-assignment loop
    // predicates on that registration. (2) Even without an edge, ELK
    // fails to size the empty cluster, so the title floats out of its
    // own rectangle and overlaps siblings. Inserting a single child
    // labeled "No nodes" restores the bookkeeping invariant AND gives
    // ELK something to measure when sizing the cluster. The label
    // doubles as a visible signal that the scope is intentionally
    // empty rather than a rendering failure.
    const placeholderId = `elk_empty_${subgraphId}`;
    return {
      line: `${indent}${placeholderId}["No nodes"]`,
      placeholderId,
    };
  },

  trailerLines: (placeholderIds, theme) => {
    if (placeholderIds.length === 0) {
      return [];
    }
    const c = theme.elkEmptyPlaceholder;
    // No `color:` segment: the placeholder must remain visible (its
    // "No nodes" label is the whole point), so we let Mermaid's
    // default text color apply. fill / stroke are transparent so no
    // rectangle is drawn around the label — it reads as a floating
    // marker, not a node.
    const out: /* mutable */ string[] = [
      `  classDef elkEmptyPlaceholder fill:${c.fill},stroke:${c.stroke};`,
    ];
    for (const id of placeholderIds) {
      out.push(`  class ${id} elkEmptyPlaceholder;`);
    }
    return out;
  },
} as const satisfies MermaidStrategy;
