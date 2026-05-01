import type { ROOT_QUERY_KIND } from "./root-query-kind.js";

export type ParsedRootQuery =
  | Readonly<{ kind: typeof ROOT_QUERY_KIND.Line; line: number; raw: string }>
  | Readonly<{
      kind: typeof ROOT_QUERY_KIND.LineName;
      line: number;
      name: string;
      raw: string;
    }>
  | Readonly<{
      kind: typeof ROOT_QUERY_KIND.Range;
      start: number;
      end: number;
      raw: string;
    }>
  | Readonly<{
      kind: typeof ROOT_QUERY_KIND.RangeName;
      start: number;
      end: number;
      name: string;
      raw: string;
    }>
  | Readonly<{ kind: typeof ROOT_QUERY_KIND.Name; name: string; raw: string }>;
