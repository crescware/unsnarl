import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { PathEntry } from "../walk/walk.js";
import { blockContextOf } from "./block-context-of.js";
import type { NodeLike } from "./node-like.js";

describe("blockContextOf", () => {
  test("returns null when parent is null", () => {
    expect(blockContextOf(null, "body", [])).toBeNull();
  });

  test("returns null when key is null", () => {
    const parent = {
      type: AST_TYPE.IfStatement,
      start: 5,
    } as const satisfies NodeLike;
    expect(blockContextOf(parent, null, [])).toBeNull();
  });

  test("returns parent type, key, and start as parentSpanOffset", () => {
    const parent = {
      type: AST_TYPE.IfStatement,
      start: 12,
    } as const satisfies NodeLike;
    expect(blockContextOf(parent, "consequent", [])).toEqual({
      parentType: AST_TYPE.IfStatement,
      key: "consequent",
      parentSpanOffset: 12,
      kind: "other",
    });
  });

  test("falls back to parentSpanOffset 0 when start is undefined", () => {
    const parent = { type: AST_TYPE.Program } as const satisfies NodeLike;
    expect(blockContextOf(parent, "body", [])).toEqual({
      parentType: AST_TYPE.Program,
      key: "body",
      parentSpanOffset: 0,
      kind: "other",
    });
  });

  test("includes ifChainRootOffset when path indicates an else-if chain", () => {
    const outer = {
      type: AST_TYPE.IfStatement,
      start: 5,
    } as const satisfies NodeLike;

    const inner = {
      type: AST_TYPE.IfStatement,
      start: 40,
    } as const satisfies NodeLike;

    const path: readonly PathEntry[] = [
      {
        node: { type: AST_TYPE.Program, start: 0 } as unknown as AstNode,
        key: null,
      },
      { node: outer as unknown as AstNode, key: "body" },
      { node: inner as unknown as AstNode, key: "alternate" },
    ];
    expect(blockContextOf(inner, "consequent", path)).toEqual({
      parentType: AST_TYPE.IfStatement,
      key: "consequent",
      parentSpanOffset: 40,
      ifChainRootOffset: 5,
      kind: "other",
    });
  });
});
