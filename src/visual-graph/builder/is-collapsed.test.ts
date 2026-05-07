import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { CATEGORY, makeDepths } from "../../serializer/category.js";
import { isCollapsed } from "./is-collapsed.js";
import { baseScope } from "./testing/make-scope.js";

describe("isCollapsed", () => {
  test("returns false when depths option is absent", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      categoryDepths: { ...makeDepths(0), [CATEGORY.Function]: 99 },
    };
    expect(isCollapsed(scope, undefined)).toBe(false);
  });

  test("returns false when depth equals threshold", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      categoryDepths: { ...makeDepths(0), [CATEGORY.Function]: 1 },
    };
    expect(isCollapsed(scope, makeDepths(1))).toBe(false);
  });

  test("returns true when category depth strictly exceeds threshold", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      categoryDepths: { ...makeDepths(0), [CATEGORY.Function]: 2 },
    };
    expect(isCollapsed(scope, makeDepths(1))).toBe(true);
  });

  test("each category is checked independently", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.For,
      categoryDepths: {
        ...makeDepths(0),
        [CATEGORY.Function]: 99,
        [CATEGORY.For]: 1,
      },
    };
    // function depth is far over its threshold, but the scope is a `for`
    // and only its own category counter is consulted.
    const depths = { ...makeDepths(10), [CATEGORY.For]: 1 };
    expect(isCollapsed(scope, depths)).toBe(false);

    const tighter = { ...makeDepths(10), [CATEGORY.For]: 0 };
    expect(isCollapsed(scope, tighter)).toBe(true);
  });

  test("non-counted scopes (module / function-expression-name) never collapse", () => {
    const fnExprName = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      functionExpressionScope: true,
      categoryDepths: makeDepths(99),
    };
    expect(isCollapsed(fnExprName, makeDepths(0))).toBe(false);
  });
});
