import { describe, expect, test } from "vitest";

import { ScopeImpl } from "../../analyzer/scope.js";
import { AST_TYPE } from "../../ast-type.js";
import type { AstNode } from "../../ir/model.js";
import { SCOPE_TYPE } from "../../scope-type.js";
import { collectScopesInOrder } from "./collect-scopes-in-order.js";

const block = (type: string): AstNode => ({ type }) as unknown as AstNode;

describe("collectScopesInOrder", () => {
  test("returns the root alone when there are no children", () => {
    const root = new ScopeImpl({
      type: SCOPE_TYPE.Module,
      isStrict: true,
      upper: null,
      block: block(AST_TYPE.Program),
    });
    expect(collectScopesInOrder(root)).toEqual([root]);
  });

  test("performs a depth-first pre-order traversal of childScopes", () => {
    const root = new ScopeImpl({
      type: SCOPE_TYPE.Module,
      isStrict: true,
      upper: null,
      block: block(AST_TYPE.Program),
    });
    const a = new ScopeImpl({
      type: SCOPE_TYPE.Block,
      isStrict: true,
      upper: root,
      block: block(AST_TYPE.BlockStatement),
    });
    const a1 = new ScopeImpl({
      type: SCOPE_TYPE.Block,
      isStrict: true,
      upper: a,
      block: block(AST_TYPE.BlockStatement),
    });
    const b = new ScopeImpl({
      type: SCOPE_TYPE.Block,
      isStrict: true,
      upper: root,
      block: block(AST_TYPE.BlockStatement),
    });
    expect(collectScopesInOrder(root)).toEqual([root, a, a1, b]);
  });
});
