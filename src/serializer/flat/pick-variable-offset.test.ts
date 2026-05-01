import { describe, expect, test } from "vitest";

import type {
  AstIdentifier,
  AstNode,
  Definition,
  Variable,
} from "../../ir/model.js";
import { pickVariableOffset } from "./pick-variable-offset.js";

const ident = (name: string, start?: number): AstIdentifier =>
  ({ type: "Identifier", name, start }) as unknown as AstIdentifier;

const def = (nameStart?: number): Definition => ({
  type: "Variable",
  name: ident("x", nameStart),
  node: { type: "VariableDeclarator" } as unknown as AstNode,
  parent: null,
});

const variable = (
  identifiers: AstIdentifier[],
  defs: Definition[] = [],
): Variable => ({ identifiers, defs }) as unknown as Variable;

describe("pickVariableOffset", () => {
  test("uses first identifier's start when present", () => {
    expect(pickVariableOffset(variable([ident("x", 7), ident("x", 9)]))).toBe(
      7,
    );
  });

  test("falls back to defs[0].name.start when identifiers is empty", () => {
    expect(pickVariableOffset(variable([], [def(15)]))).toBe(15);
  });

  test("returns 0 when both sources are missing or have no start", () => {
    expect(pickVariableOffset(variable([]))).toBe(0);
    expect(pickVariableOffset(variable([], [def(undefined)]))).toBe(0);
    expect(pickVariableOffset(variable([ident("x", undefined)]))).toBe(0);
  });
});
