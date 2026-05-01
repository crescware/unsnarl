import type { MermaidStrategy } from "../strategy/strategy.js";

type FakeStrategyOptions = {
  preambleLines?: readonly string[];
  trailerLines?: readonly string[];
  // When provided, replaces the default placeholder builder. Returning null
  // signals "no patch", just like a real strategy.
  emptySubgraphPlaceholder?: MermaidStrategy["emptySubgraphPlaceholder"];
};

export function makeStrategy(
  options: FakeStrategyOptions = {},
): MermaidStrategy {
  const trailer = options.trailerLines ?? [];
  return {
    preambleLines: options.preambleLines ?? [],
    emptySubgraphPlaceholder: options.emptySubgraphPlaceholder ?? (() => null),
    trailerLines: () => trailer,
  };
}
