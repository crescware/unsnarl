import type { MermaidStrategy } from "../strategy/strategy.js";

export function baseStrategy(): MermaidStrategy {
  return {
    preambleLines: [],
    emptySubgraphPlaceholder: () => null,
    trailerLines: () => [],
  };
}
