import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";

// "off"     -- no `-h` flag given.
// "roots"   -- `-h` given without a value; highlight reuses the `-r` queries.
// "queries" -- `-h <value>` given; highlight uses its own queries instead of
//              the `-r` queries (so a `-r value -h other` combination keeps
//              `-r` driving pruning while `-h` controls the colour).
export type HighlightSpec =
  | Readonly<{ mode: "off" }>
  | Readonly<{ mode: "roots" }>
  | Readonly<{ mode: "queries"; queries: readonly ParsedRootQuery[] }>;
