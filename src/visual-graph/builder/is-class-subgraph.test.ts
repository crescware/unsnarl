import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { isClassSubgraph } from "./is-class-subgraph.js";
import { baseScope } from "./testing/make-scope.js";

describe("isClassSubgraph", () => {
  test("class scope -> true", () => {
    expect(isClassSubgraph({ ...baseScope(), type: SCOPE_TYPE.Class })).toEqual(
      true,
    );
  });

  test("function scope -> false", () => {
    expect(
      isClassSubgraph({ ...baseScope(), type: SCOPE_TYPE.Function }),
    ).toEqual(false);
  });

  test("block scope -> false", () => {
    expect(isClassSubgraph({ ...baseScope(), type: SCOPE_TYPE.Block })).toEqual(
      false,
    );
  });
});
