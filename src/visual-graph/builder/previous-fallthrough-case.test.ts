import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { previousFallthroughCase } from "./previous-fallthrough-case.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

function caseScope(
  id: string,
  parentSpanOffset: number,
  fallsThrough: boolean,
): SerializedScope {
  return {
    ...baseScope(),
    id,
    upper: "switch",
    fallsThrough,
    blockContext: {
      ...baseBlockContext(),
      parentType: AST_TYPE.SwitchStatement,
      key: "cases",
      parentSpanOffset,
    },
  };
}

const c0 = caseScope("c0", 100, true);
const c1 = caseScope("c1", 100, false);
const c2 = caseScope("c2", 100, true);
const c3 = caseScope("c3", 100, false);

const containerKey = "switch:switch:100";
const sortedCases = new Map<string, readonly SerializedScope[]>([
  [containerKey, [c0, c1, c2, c3]],
]);

describe("previousFallthroughCase", () => {
  test.each<{ name: string; target: SerializedScope; expected: string | null }>(
    [
      { name: "first case has no previous", target: c0, expected: null },
      {
        name: "previous fallsThrough -> returns previous",
        target: c1,
        expected: "c0",
      },
      {
        name: "previous does not fall through -> null",
        target: c2,
        expected: null,
      },
      {
        name: "fallsThrough chain works at later positions",
        target: c3,
        expected: "c2",
      },
    ],
  )("$name", ({ target, expected }) => {
    expect(previousFallthroughCase(target, sortedCases)?.id ?? null).toBe(
      expected,
    );
  });

  test("scope without branchContainerKey -> null", () => {
    expect(
      previousFallthroughCase({ ...baseScope(), id: "x" }, sortedCases),
    ).toBeNull();
  });

  test("container key not in map -> null", () => {
    const orphan = caseScope("orphan", 999, true);
    expect(previousFallthroughCase(orphan, sortedCases)).toBeNull();
  });
});
