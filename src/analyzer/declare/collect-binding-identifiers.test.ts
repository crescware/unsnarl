import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { collectBindingIdentifiers } from "./collect-binding-identifiers.js";

const declId = (code: string): AstNode => {
  const program = parseSync("input.ts", code, { lang: "ts" })
    .program as unknown as {
    body: readonly { declarations: readonly { id: AstNode }[] }[];
  };
  const decl = program.body[0]?.declarations[0]?.id ?? null;
  if (decl === null) {
    throw new Error("test fixture missing declarator id");
  }
  return decl;
};

const names = (pattern: AstNode): readonly string[] =>
  collectBindingIdentifiers(pattern).map((i) => i.name);

describe("collectBindingIdentifiers", () => {
  test("Identifier → single name", () => {
    expect(names(declId("const x = 1;"))).toEqual(["x"]);
  });

  test("ObjectPattern destructures property values", () => {
    expect(names(declId("const { a, b } = obj;"))).toEqual(["a", "b"]);
  });

  test("ObjectPattern handles RestElement", () => {
    expect(names(declId("const { a, ...rest } = obj;"))).toEqual(["a", "rest"]);
  });

  test("ArrayPattern destructures elements", () => {
    expect(names(declId("const [x, y] = arr;"))).toEqual(["x", "y"]);
  });

  test("ArrayPattern skips holes", () => {
    expect(names(declId("const [x, , z] = arr;"))).toEqual(["x", "z"]);
  });

  test("ArrayPattern handles RestElement at the tail", () => {
    expect(names(declId("const [head, ...tail] = arr;"))).toEqual([
      "head",
      "tail",
    ]);
  });

  test("AssignmentPattern looks at the left side only", () => {
    expect(names(declId("const { a = 1 } = obj;"))).toEqual(["a"]);
  });

  test("nested patterns descend recursively", () => {
    expect(names(declId("const { a: { b }, c: [d, e] } = obj;"))).toEqual([
      "b",
      "d",
      "e",
    ]);
  });

  test("unsupported node types yield no identifiers", () => {
    const program = parseSync("input.ts", "1;", { lang: "ts" })
      .program as unknown as {
      body: readonly { expression: AstNode }[];
    };
    const literal = program.body[0]?.expression ?? null;
    if (literal === null) {
      throw new Error("test fixture missing literal");
    }
    expect(collectBindingIdentifiers(literal)).toEqual([]);
  });
});
