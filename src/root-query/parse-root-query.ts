import { parseRootQueryAst } from "./parse-root-query-ast.js";
import type { ParsedRootQuery } from "./parsed-root-query.js";
import { ROOT_QUERY_SCOPE_POINT_ONLY } from "./root-query-scope.js";
import { validateRootQuery } from "./validate-root-query.js";

export function parseRootQuery(
  token: string,
): ParsedRootQuery | { error: string } {
  const ast = parseRootQueryAst(token, ROOT_QUERY_SCOPE_POINT_ONLY);
  if (!ast.ok) {
    return { error: ast.errors[0]?.message ?? "(no message)" };
  }
  const validated = validateRootQuery(ast.value);
  if (!validated.ok) {
    return { error: validated.errors[0]?.message ?? "(no message)" };
  }
  if (validated.value.kind !== "single") {
    return {
      error: `internal error: expected 'single' RootQuery, got '${validated.value.kind}'`,
    };
  }
  return validated.value.query;
}
