import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { PathEntry, WalkVisitor } from "./walk.js";
import { walk } from "./walk.js";

const parse = (code: string): AstNode =>
  parseSync("input.ts", code, { lang: "ts" }).program as unknown as AstNode;

describe("walk", () => {
  test("enter is called for the root node first", () => {
    const program = parse("const x = 1;");
    const types: /* mutable */ string[] = [];
    walk(program, {
      enter(node) {
        types.push(node.type);
      },
    });
    expect(types[0]).toBe(AST_TYPE.Program);
  });

  test("enter visits descendants in source order", () => {
    const program = parse("const x = 1; const y = 2;");
    const declared: /* mutable */ string[] = [];
    walk(program, {
      enter(node) {
        if (node.type === AST_TYPE.VariableDeclarator) {
          const id = node["id"];
          if (
            id !== null &&
            typeof id === "object" &&
            (id as { name?: unknown }).name !== undefined
          ) {
            declared.push((id as { name: string }).name);
          }
        }
      },
    });
    expect(declared).toEqual(["x", "y"]);
  });

  test("returning 'skip' from enter prevents descent but still calls leave", () => {
    const program = parse("const x = 1;");
    const entered: /* mutable */ string[] = [];
    const left: /* mutable */ string[] = [];
    walk(program, {
      enter(node) {
        entered.push(node.type);
        if (node.type === AST_TYPE.VariableDeclarator) {
          return "skip";
        }
        return undefined;
      },
      leave(node) {
        left.push(node.type);
      },
    });
    expect(entered).toContain(AST_TYPE.VariableDeclarator);
    expect(entered).not.toContain(AST_TYPE.Identifier);
    expect(left).toContain(AST_TYPE.VariableDeclarator);
  });

  test("leave fires in post-order (root last)", () => {
    const program = parse("const x = 1;");
    const left: /* mutable */ string[] = [];
    walk(program, {
      leave(node) {
        left.push(node.type);
      },
    });
    expect(left[left.length - 1]).toBe(AST_TYPE.Program);
  });

  test("path passed to enter ends at the parent of the visited node", () => {
    const program = parse("const x = 1;");
    let identPath: readonly PathEntry[] | null = null;
    walk(program, {
      enter(node, _parent, _key, path) {
        if (node.type === AST_TYPE.Identifier && identPath === null) {
          identPath = path.slice();
        }
      },
    });
    expect(identPath).not.toBeNull();
    const types = (identPath as unknown as PathEntry[]).map((p) => p.node.type);
    expect(types[0]).toBe(AST_TYPE.Program);
    expect(types[types.length - 1]).toBe(AST_TYPE.VariableDeclarator);
  });

  test("parent and key are null for the root", () => {
    const program = parse("const x = 1;");
    const visitor = {
      enter(node, parent, key) {
        if (node === program) {
          expect(parent).toBeNull();
          expect(key).toBeNull();
        }
      },
    } satisfies WalkVisitor;
    walk(program, visitor);
  });

  test("works without enter or leave (no throw, no call)", () => {
    const program = parse("const x = 1;");
    expect(() => walk(program, {})).not.toThrow();
  });
});
