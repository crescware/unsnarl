import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { spanOf } from "./span-of.js";

describe("spanOf", () => {
  test("derives the span from node.start using spanFromOffset", () => {
    const raw = "abc\ndef";
    expect(
      spanOf({ type: "X", start: 5 } as unknown as AstNode, raw),
    ).toMatchObject({
      offset: 5,
    });
  });

  test("falls back to offset 0 when node.start is undefined", () => {
    expect(spanOf({ type: "X" } as unknown as AstNode, "abc")).toMatchObject({
      offset: 0,
    });
  });
});
