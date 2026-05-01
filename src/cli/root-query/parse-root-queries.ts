import { parseRootQuery } from "./parse-root-query.js";
import type { ParsedRootQuery } from "./parsed-root-query.js";

export type RootQueryParseSuccess = Readonly<{
  ok: true;
  queries: readonly ParsedRootQuery[];
}>;

export type RootQueryParseFailure = Readonly<{
  ok: false;
  error: string;
}>;

export type RootQueryParseResult =
  | RootQueryParseSuccess
  | RootQueryParseFailure;

export function parseRootQueries(value: string): RootQueryParseResult {
  if (value === "") {
    return { ok: false, error: "empty --roots value" };
  }
  const tokens = value.split(",");
  const queries: ParsedRootQuery[] = [];
  for (const token of tokens) {
    const r = parseRootQuery(token);
    if ("error" in r) {
      return { ok: false, error: r.error };
    }
    queries.push(r);
  }
  return { ok: true, queries };
}
