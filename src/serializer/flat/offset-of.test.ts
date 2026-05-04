import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { offsetOf } from "./offset-of.js";

describe("offsetOf", () => {
  test("returns node.start when defined", () => {
    expect(offsetOf({ type: "X", start: 42 } as unknown as AstNode)).toBe(42);
  });

  test("falls back to 0 when start is undefined", () => {
    expect(offsetOf({ type: "X" } as unknown as AstNode)).toBe(0);
  });

  test("preserves start === 0 (does not coerce)", () => {
    expect(offsetOf({ type: "X", start: 0 } as unknown as AstNode)).toBe(0);
  });
});
