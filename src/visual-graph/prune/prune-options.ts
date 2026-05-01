import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import type { VisualGraph } from "../model.js";

export type PruneOptions = {
  readonly roots: readonly ParsedRootQuery[];
  readonly descendants: number;
  readonly ancestors: number;
};

export type PruneResult = {
  readonly graph: VisualGraph;
  readonly perQuery: ReadonlyArray<{
    readonly query: ParsedRootQuery;
    readonly matched: number;
  }>;
};
