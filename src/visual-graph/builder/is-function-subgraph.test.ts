import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { isFunctionSubgraph } from "./is-function-subgraph.js";
import { baseScope } from "./testing/make-scope.js";

describe("isFunctionSubgraph", () => {
  test("function scope -> true", () => {
    expect(
      isFunctionSubgraph({ ...baseScope(), type: SCOPE_TYPE.Function }),
    ).toBe(true);
  });

  test("function-expression-name scope -> false (the named-function-expression name binding never renders as a subgraph)", () => {
    expect(
      isFunctionSubgraph({
        ...baseScope(),
        type: SCOPE_TYPE.Function,
        functionExpressionScope: true,
      }),
    ).toBe(false);
  });

  test("non-function scope -> false", () => {
    expect(isFunctionSubgraph({ ...baseScope(), type: SCOPE_TYPE.Block })).toBe(
      false,
    );
  });
});
