import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { asFilledString } from "../../util/filled-string.js";
import { enclosingFunctionVar } from "./enclosing-function-var.js";
import { baseScope } from "./testing/make-scope.js";

const grand = { ...baseScope(), id: asScopeId("grand") };
const parent = {
  ...baseScope(),
  id: asScopeId("parent"),
  upper: asScopeId("grand"),
};
const child = {
  ...baseScope(),
  id: asScopeId("child"),
  upper: asScopeId("parent"),
};
const map = new Map<string, SerializedScope>(
  [grand, parent, child].map((v) => [v.id, v]),
);

describe("enclosingFunctionVar", () => {
  test.each<{
    name: string;
    owners: Map<string, string>;
    start: string;
    expected: string | null;
  }>([
    {
      name: asFilledString(
        "owner found at start scope returns its variable id",
      ),
      owners: new Map([["child", "varChild"]]),
      start: "child",
      expected: "varChild",
    },
    {
      name: asFilledString(
        "owner found higher up returns that ancestor's variable id",
      ),
      owners: new Map([["grand", "varGrand"]]),
      start: "child",
      expected: "varGrand",
    },
    {
      name: asFilledString("no owner anywhere -> null"),
      owners: new Map(),
      start: "child",
      expected: null,
    },
    {
      name: asFilledString("starting scope missing from map -> null"),
      owners: new Map([["x", "v"]]),
      start: "missing",
      expected: null,
    },
  ])("$name", ({ owners, start, expected }) => {
    expect(enclosingFunctionVar(start, map, owners)).toEqual(expected);
  });
});
