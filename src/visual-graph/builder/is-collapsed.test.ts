import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import {
  NESTING_KIND,
  uniformNestingDepths,
} from "../../serializer/nesting-kind.js";
import { isCollapsed } from "./is-collapsed.js";
import { baseScope } from "./testing/make-scope.js";

describe("isCollapsed", () => {
  test("returns false when depths option is absent", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      nestingDepths: {
        ...uniformNestingDepths(0),
        [NESTING_KIND.Function]: 99,
      },
    };
    expect(isCollapsed(scope, undefined)).toEqual(false);
  });

  test("returns false when depth equals threshold", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      nestingDepths: { ...uniformNestingDepths(0), [NESTING_KIND.Function]: 1 },
    };
    expect(isCollapsed(scope, uniformNestingDepths(1))).toEqual(false);
  });

  test("returns true when nesting kind depth strictly exceeds threshold", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      nestingDepths: { ...uniformNestingDepths(0), [NESTING_KIND.Function]: 2 },
    };
    expect(isCollapsed(scope, uniformNestingDepths(1))).toEqual(true);
  });

  test("each nesting kind is checked independently", () => {
    const scope = {
      ...baseScope(),
      type: SCOPE_TYPE.For,
      nestingDepths: {
        ...uniformNestingDepths(0),
        [NESTING_KIND.Function]: 99,
        [NESTING_KIND.For]: 1,
      },
    };
    // function depth is far over its threshold, but the scope is a `for`
    // and only its own nesting kind counter is consulted.
    const depths = { ...uniformNestingDepths(10), [NESTING_KIND.For]: 1 };
    expect(isCollapsed(scope, depths)).toEqual(false);

    const tighter = { ...uniformNestingDepths(10), [NESTING_KIND.For]: 0 };
    expect(isCollapsed(scope, tighter)).toEqual(true);
  });

  test("non-counted scopes (module / function-expression-name) never collapse", () => {
    const fnExprName = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
      functionExpressionScope: true,
      nestingDepths: uniformNestingDepths(99),
    };
    expect(isCollapsed(fnExprName, uniformNestingDepths(0))).toEqual(false);
  });
});
