import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import type { VisualGraph } from "../model.js";

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
}>;
