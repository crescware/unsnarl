import type { ParseResult } from "./parse-error.js";
import type { ParsedRootQuery } from "./parsed-root-query.js";
import { ROOT_QUERY_KIND } from "./root-query-kind.js";

export function validateEndpointQuery(
  eq: ParsedRootQuery,
): ParseResult<ParsedRootQuery> {
  switch (eq.kind) {
    case ROOT_QUERY_KIND.Line:
    case ROOT_QUERY_KIND.LineName:
    case ROOT_QUERY_KIND.LineOrName: {
      if (eq.line < 1) {
        return {
          ok: false,
          errors: [{ message: `line must be >= 1 in '${eq.raw}'` }],
        };
      }
      return { ok: true, value: eq };
    }
    case ROOT_QUERY_KIND.Range:
    case ROOT_QUERY_KIND.RangeName: {
      if (eq.start < 1) {
        return {
          ok: false,
          errors: [{ message: `line must be >= 1 in '${eq.raw}'` }],
        };
      }
      if (eq.start > eq.end) {
        return {
          ok: false,
          errors: [{ message: `range start must be <= end in '${eq.raw}'` }],
        };
      }
      return { ok: true, value: eq };
    }
    case ROOT_QUERY_KIND.Name: {
      return { ok: true, value: eq };
    }
  }
}
