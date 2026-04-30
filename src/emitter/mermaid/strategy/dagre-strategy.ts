import type { MermaidStrategy } from "./strategy.js";

export const dagreStrategy: MermaidStrategy = {
  preambleLines: [],
  emptySubgraphPlaceholder: () => null,
  trailerLines: () => [],
};
