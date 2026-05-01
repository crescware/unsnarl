import type { MermaidStrategy } from "./strategy.js";

export const dagreStrategy = {
  preambleLines: [],
  emptySubgraphPlaceholder: (_ctx) => null,
  trailerLines: (_placeholderIds) => [],
} as const satisfies MermaidStrategy;
