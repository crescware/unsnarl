import type { ParseResult } from "./parse-error.js";
import type { RootQuery } from "./root-query.js";
import { validateEndpointQuery } from "./validate-endpoint-query.js";

export function validateRootQuery(rq: RootQuery): ParseResult<RootQuery> {
  switch (rq.kind) {
    case "single": {
      const r = validateEndpointQuery(rq.query);
      if (!r.ok) {
        return r;
      }
      return { ok: true, value: rq };
    }
    case "path": {
      const lhsR = validateEndpointQuery(rq.lhs);
      if (!lhsR.ok) {
        return lhsR;
      }
      const rhsR = validateEndpointQuery(rq.rhs);
      if (!rhsR.ok) {
        return rhsR;
      }
      return { ok: true, value: rq };
    }
    case "direction": {
      const lhsR = validateEndpointQuery(rq.lhs);
      if (!lhsR.ok) {
        return lhsR;
      }
      return { ok: true, value: rq };
    }
  }
}
