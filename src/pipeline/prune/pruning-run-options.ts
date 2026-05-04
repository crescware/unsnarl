import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";

export type PruningRunOptions = Readonly<{
  roots: readonly ParsedRootQuery[];
  descendants: number;
  ancestors: number;
}>;
