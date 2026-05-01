import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import { blockContextOf } from "./block-context-of.js";
import type { NodeLike } from "./node-like.js";
describe("blockContextOf", () => {
  test("returns null when parent is null", () => {
    expect(blockContextOf(null, "body")).toBeNull();
  });

  test("returns null when key is null", () => {
    const parent = {
      type: AST_TYPE.IfStatement,
      start: 5,
    } as const satisfies NodeLike;
    expect(blockContextOf(parent, null)).toBeNull();
  });

  test("returns parent type, key, and start as parentSpanOffset", () => {
    const parent = {
      type: AST_TYPE.IfStatement,
      start: 12,
    } as const satisfies NodeLike;
    expect(blockContextOf(parent, "consequent")).toEqual({
      parentType: AST_TYPE.IfStatement,
      key: "consequent",
      parentSpanOffset: 12,
    });
  });

  test("falls back to parentSpanOffset 0 when start is undefined", () => {
    const parent = { type: AST_TYPE.Program } as const satisfies NodeLike;
    expect(blockContextOf(parent, "body")).toEqual({
      parentType: AST_TYPE.Program,
      key: "body",
      parentSpanOffset: 0,
    });
  });
});
