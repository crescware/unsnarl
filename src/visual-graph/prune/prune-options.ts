import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";
import type { VisualGraph } from "../visual-graph.js";

export type PruneOptions = Readonly<{
  roots: readonly ParsedRootQuery[];
  descendants: number;
  ancestors: number;
}>;

export type PruneResult = Readonly<{
  graph: VisualGraph;
  perQuery: readonly Readonly<{
    query: ParsedRootQuery;
    matched: number;
  }>[];
  // The exact id set the prune walk treated as "roots" -- i.e. the
  // nodes the queries matched directly (and any nodes swept in by a
  // bare line query that lands on a subgraph's start line). The BFS
  // descendants/ancestors are NOT included. Exposed so `-H` in
  // roots mode can paint the same id set the user pinpointed via
  // `-r`, inheriting the same use-site exclusions that pruning
  // applies on a bare name query.
  rootIds: ReadonlySet<string>;
}>;
