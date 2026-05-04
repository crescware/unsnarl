import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { ScopeImpl } from "../../analyzer/scope.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { collectScopesInOrder } from "./collect-scopes-in-order.js";

const block = (type: string): AstNode => ({ type }) as unknown as AstNode;

describe("collectScopesInOrder", () => {
  test("returns the root alone when there are no children", () => {
    const root = new ScopeImpl({
      type: SCOPE_TYPE.Module,
      isStrict: true,
      upper: null,
      block: block(AST_TYPE.Program),
      blockContext: null,
    });
    expect(collectScopesInOrder(root)).toEqual([root]);
  });

  test("performs a depth-first pre-order traversal of childScopes", () => {
    const root = new ScopeImpl({
      type: SCOPE_TYPE.Module,
      isStrict: true,
      upper: null,
      block: block(AST_TYPE.Program),
      blockContext: null,
    });
    const a = new ScopeImpl({
      type: SCOPE_TYPE.Block,
      isStrict: true,
      upper: root,
      block: block(AST_TYPE.BlockStatement),
      blockContext: null,
    });
    const a1 = new ScopeImpl({
      type: SCOPE_TYPE.Block,
      isStrict: true,
      upper: a,
      block: block(AST_TYPE.BlockStatement),
      blockContext: null,
    });
    const b = new ScopeImpl({
      type: SCOPE_TYPE.Block,
      isStrict: true,
      upper: root,
      block: block(AST_TYPE.BlockStatement),
      blockContext: null,
    });
    expect(collectScopesInOrder(root)).toEqual([root, a, a1, b]);
  });
});
