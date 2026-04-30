import type { ParsedRootQuery } from "../cli/root-query.js";
import type { VisualGraph } from "./model.js";

export interface PruneOptions {
  readonly roots: readonly ParsedRootQuery[];
  readonly descendants: number;
  readonly ancestors: number;
}

export interface PruneResult {
  readonly graph: VisualGraph;
  readonly perQuery: ReadonlyArray<{
    readonly query: ParsedRootQuery;
    readonly matched: number;
  }>;
}

export function pruneVisualGraph(
  graph: VisualGraph,
  options: PruneOptions,
): PruneResult {
  void options;
  return { graph, perQuery: [] };
}
