import { describe, expect, test } from "vitest";

import { blockContextOf } from "./block-context-of.js";
import type { NodeLike } from "./node-like.js";

describe("blockContextOf", () => {
  test("returns null when parent is null", () => {
    expect(blockContextOf(null, "body")).toBeNull();
  });

  test("returns null when key is null", () => {
    const parent: NodeLike = { type: "IfStatement", start: 5 };
    expect(blockContextOf(parent, null)).toBeNull();
  });

  test("returns parent type, key, and start as parentSpanOffset", () => {
    const parent: NodeLike = { type: "IfStatement", start: 12 };
    expect(blockContextOf(parent, "consequent")).toEqual({
      parentType: "IfStatement",
      key: "consequent",
      parentSpanOffset: 12,
    });
  });

  test("falls back to parentSpanOffset 0 when start is undefined", () => {
    const parent: NodeLike = { type: "Program" };
    expect(blockContextOf(parent, "body")).toEqual({
      parentType: "Program",
      key: "body",
      parentSpanOffset: 0,
    });
  });
});
