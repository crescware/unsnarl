import type { MermaidStrategy } from "./strategy.js";

export const elkStrategy = {
  preambleLines: ['%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%'],

  emptySubgraphPlaceholder: ({ subgraphId, indent, referencedByEdge }) => {
    // Workaround for @mermaid-js/layout-elk@0.2.1: an empty subgraph that
    // is referenced by an edge crashes the renderer at
    // render-TAZW7USW.mjs:847 because addSubGraphs only registers a
    // subgraph in childrenById when it has at least one direct child, and
    // the labels-assignment loop predicates on that registration — so the
    // edge-handling code reads `labels[0]` on an undefined `labels`.
    // Inserting a single hidden child restores the bookkeeping invariant.
    // Subgraphs that no edge references don't hit the buggy path, so
    // those stay clean.
    if (!referencedByEdge) {
      return null;
    }
    const placeholderId = `elk_empty_${subgraphId}`;
    return {
      line: `${indent}${placeholderId}[" "]`,
      placeholderId,
    };
  },

  trailerLines: (placeholderIds) => {
    if (placeholderIds.length === 0) {
      return [];
    }
    const out: /* mutable */ string[] = [
      "  classDef elkEmptyPlaceholder fill:transparent,stroke:transparent,color:transparent;",
    ];
    for (const id of placeholderIds) {
      out.push(`  class ${id} elkEmptyPlaceholder;`);
    }
    return out;
  },
} as const satisfies MermaidStrategy;
