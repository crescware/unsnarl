import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";

// "roots" reuses the queries from `pruning.roots` (so `-h` alone follows
// whatever `-r` selects). "queries" carries its own query list, which is
// what `-h <value>` produces.
export type HighlightRunOptions =
  | Readonly<{ kind: "roots" }>
  | Readonly<{ kind: "queries"; queries: readonly ParsedRootQuery[] }>;
