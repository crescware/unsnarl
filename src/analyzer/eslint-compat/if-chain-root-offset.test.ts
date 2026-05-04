import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { PathEntry } from "../walk/path-entry.js";
import { ifChainRootOffset } from "./if-chain-root-offset.js";
import type { NodeLike } from "./node-like.js";

function ifNode(start: number): NodeLike {
  return { type: AST_TYPE.IfStatement, start } as const satisfies NodeLike;
}

function blockNode(start: number): NodeLike {
  return { type: AST_TYPE.BlockStatement, start } as const satisfies NodeLike;
}

function entry(node: NodeLike, key: string | null): PathEntry {
  return { node: node as unknown as AstNode, key };
}

describe("ifChainRootOffset", () => {
  test("returns null when parent is null", () => {
    expect(ifChainRootOffset(null, "consequent", [])).toBeNull();
  });

  test("returns null when parent is not an IfStatement", () => {
    expect(ifChainRootOffset(blockNode(0), "consequent", [])).toBeNull();
  });

  test("returns null for keys other than consequent / alternate", () => {
    const parent = ifNode(10);
    expect(ifChainRootOffset(parent, "test", [])).toBeNull();
  });

  test("returns null for a standalone if (parent at top level)", () => {
    const parent = ifNode(10);
    const program = { type: AST_TYPE.Program, start: 0 } as const;
    const path: readonly PathEntry[] = [
      entry(program as unknown as NodeLike, null),
      entry(parent, "body"),
    ];
    expect(ifChainRootOffset(parent, "consequent", path)).toBeNull();
  });

  test("returns outer offset for inner-if consequent in else-if chain (1 step)", () => {
    const outer = ifNode(10);
    const inner = ifNode(40);
    const program = { type: AST_TYPE.Program, start: 0 } as const;
    const path: readonly PathEntry[] = [
      entry(program as unknown as NodeLike, null),
      entry(outer, "body"),
      entry(inner, "alternate"),
    ];
    expect(ifChainRootOffset(inner, "consequent", path)).toBe(10);
  });

  test("returns outer offset for inner-if alternate in else-if chain (1 step)", () => {
    const outer = ifNode(10);
    const inner = ifNode(40);
    const program = { type: AST_TYPE.Program, start: 0 } as const;
    const path: readonly PathEntry[] = [
      entry(program as unknown as NodeLike, null),
      entry(outer, "body"),
      entry(inner, "alternate"),
    ];
    expect(ifChainRootOffset(inner, "alternate", path)).toBe(10);
  });

  test("walks back through multiple chained alternates to the outermost if (2 steps)", () => {
    const outermost = ifNode(5);
    const middle = ifNode(40);
    const innermost = ifNode(80);
    const program = { type: AST_TYPE.Program, start: 0 } as const;
    const path: readonly PathEntry[] = [
      entry(program as unknown as NodeLike, null),
      entry(outermost, "body"),
      entry(middle, "alternate"),
      entry(innermost, "alternate"),
    ];
    expect(ifChainRootOffset(innermost, "consequent", path)).toBe(5);
  });

  test("does not walk past a non-IfStatement ancestor", () => {
    // outer-if's alternate is a BlockStatement which contains a separate
    // inner-if; that inner-if is NOT a chain continuation.
    const outer = ifNode(10);
    const innerBlock = blockNode(40);
    const inner = ifNode(45);
    const program = { type: AST_TYPE.Program, start: 0 } as const;
    const path: readonly PathEntry[] = [
      entry(program as unknown as NodeLike, null),
      entry(outer, "body"),
      entry(innerBlock, "alternate"),
      entry(inner, "body"),
    ];
    expect(ifChainRootOffset(inner, "consequent", path)).toBeNull();
  });

  test("does not walk when current if sits in the consequent slot of an outer if", () => {
    // An IfStatement nested inside an outer if's *consequent* (rather than
    // alternate) is independent, not a chain continuation.
    const outer = ifNode(10);
    const consBlock = blockNode(20);
    const inner = ifNode(25);
    const program = { type: AST_TYPE.Program, start: 0 } as const;
    const path: readonly PathEntry[] = [
      entry(program as unknown as NodeLike, null),
      entry(outer, "body"),
      entry(consBlock, "consequent"),
      entry(inner, "body"),
    ];
    expect(ifChainRootOffset(inner, "consequent", path)).toBeNull();
  });

  test("falls back to 0 when chainTop has no start offset", () => {
    const outer = { type: AST_TYPE.IfStatement } as const satisfies NodeLike;
    const inner = ifNode(40);
    const path: readonly PathEntry[] = [
      entry(outer, "body"),
      entry(inner, "alternate"),
    ];
    expect(ifChainRootOffset(inner, "consequent", path)).toBe(0);
  });
});
