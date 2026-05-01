import { describe, expect, test } from "vitest";

import { ScopeImpl } from "../../analyzer/scope.js";
import type { AstNode } from "../../ir/model.js";
import { collectScopesInOrder } from "./collect-scopes-in-order.js";

const block = (type: string): AstNode => ({ type }) as unknown as AstNode;

describe("collectScopesInOrder", () => {
  test("returns the root alone when there are no children", () => {
    const root = new ScopeImpl({
      type: "module",
      isStrict: true,
      upper: null,
      block: block("Program"),
    });
    expect(collectScopesInOrder(root)).toEqual([root]);
  });

  test("performs a depth-first pre-order traversal of childScopes", () => {
    const root = new ScopeImpl({
      type: "module",
      isStrict: true,
      upper: null,
      block: block("Program"),
    });
    const a = new ScopeImpl({
      type: "block",
      isStrict: true,
      upper: root,
      block: block("BlockStatement"),
    });
    const a1 = new ScopeImpl({
      type: "block",
      isStrict: true,
      upper: a,
      block: block("BlockStatement"),
    });
    const b = new ScopeImpl({
      type: "block",
      isStrict: true,
      upper: root,
      block: block("BlockStatement"),
    });
    expect(collectScopesInOrder(root)).toEqual([root, a, a1, b]);
  });
});
