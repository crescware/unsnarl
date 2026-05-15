import type { ParsedRootQuery } from "./parsed-root-query.js";

export type RootQuery =
  | Readonly<{ kind: "single"; query: ParsedRootQuery; raw: string }>
  | Readonly<{
      kind: "path";
      lhs: ParsedRootQuery;
      rhs: ParsedRootQuery;
      raw: string;
    }>
  | Readonly<{
      kind: "direction";
      lhs: ParsedRootQuery;
      dir: "a" | "b" | "c";
      level: number | null;
      raw: string;
    }>;
