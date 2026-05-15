import { parseDirectionToken } from "./parse-direction-token.js";
import { parseEndpointQuery } from "./parse-endpoint-query.js";
import type { ParseResult } from "./parse-error.js";
import type { RootQueryScope } from "./root-query-scope.js";
import type { RootQuery } from "./root-query.js";

export function parseRootQueryAst(
  token: string,
  scope: RootQueryScope,
): ParseResult<RootQuery> {
  if (token === "") {
    return { ok: false, errors: [{ message: "empty root query" }] };
  }

  const parts = token.split("..");

  if (parts.length >= 3) {
    return {
      ok: false,
      errors: [{ message: `unexpected duplicate '..' in '${token}'` }],
    };
  }

  if (parts.length === 1) {
    const lone = parts[0] ?? "";
    const r = parseEndpointQuery(lone);
    if (!r.ok) {
      return r;
    }
    if (!scope.point) {
      return {
        ok: false,
        errors: [{ message: `unexpected token '${token}'` }],
      };
    }
    return {
      ok: true,
      value: { kind: "single", query: r.value, raw: token },
    };
  }

  const lhsText = parts[0] ?? "";
  const rhsText = parts[1] ?? "";

  if (lhsText === "") {
    return {
      ok: false,
      errors: [
        { message: `unexpected empty left-hand side of '..' in '${token}'` },
      ],
    };
  }
  if (rhsText === "") {
    return {
      ok: false,
      errors: [
        { message: `unexpected empty right-hand side of '..' in '${token}'` },
      ],
    };
  }

  const lhsR = parseEndpointQuery(lhsText);
  if (!lhsR.ok) {
    return lhsR;
  }

  if (rhsText.startsWith("+")) {
    const dirR = parseDirectionToken(rhsText);
    if (!dirR.ok) {
      return dirR;
    }
    if (!scope.direction) {
      return {
        ok: false,
        errors: [
          { message: `unexpected direction token '${rhsText}' in '${token}'` },
        ],
      };
    }
    if (dirR.value.level !== null && !scope.directionLevel) {
      return {
        ok: false,
        errors: [
          {
            message: `unexpected level in direction token '${rhsText}'`,
          },
        ],
      };
    }
    return {
      ok: true,
      value: {
        kind: "direction",
        lhs: lhsR.value,
        dir: dirR.value.dir,
        level: dirR.value.level,
        raw: token,
      },
    };
  }

  const rhsR = parseEndpointQuery(rhsText);
  if (!rhsR.ok) {
    return rhsR;
  }
  if (!scope.path) {
    return {
      ok: false,
      errors: [{ message: `unexpected '..' in '${token}'` }],
    };
  }
  return {
    ok: true,
    value: {
      kind: "path",
      lhs: lhsR.value,
      rhs: rhsR.value,
      raw: token,
    },
  };
}
