import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";

export type PruningRunOptions = Readonly<{
  roots: readonly ParsedRootQuery[];
  descendants: number;
  ancestors: number;
}>;
