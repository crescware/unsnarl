import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import type { PathEntry, WalkVisitor } from "./walk.js";
import { walk } from "./walk.js";

const parse = (code: string): AstNode =>
  parseSync("input.ts", code, { lang: "ts" }).program as unknown as AstNode;

describe("walk", () => {
  test("enter is called for the root node first", () => {
    const program = parse("const x = 1;");
    const types: string[] = [];
    walk(program, {
      enter(node) {
        types.push(node.type);
      },
    });
    expect(types[0]).toBe("Program");
  });

  test("enter visits descendants in source order", () => {
    const program = parse("const x = 1; const y = 2;");
    const declared: string[] = [];
    walk(program, {
      enter(node) {
        if (node.type === "VariableDeclarator") {
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
    const entered: string[] = [];
    const left: string[] = [];
    walk(program, {
      enter(node) {
        entered.push(node.type);
        if (node.type === "VariableDeclarator") {
          return "skip";
        }
      },
      leave(node) {
        left.push(node.type);
      },
    });
    expect(entered).toContain("VariableDeclarator");
    expect(entered).not.toContain("Identifier");
    expect(left).toContain("VariableDeclarator");
  });

  test("leave fires in post-order (root last)", () => {
    const program = parse("const x = 1;");
    const left: string[] = [];
    walk(program, {
      leave(node) {
        left.push(node.type);
      },
    });
    expect(left[left.length - 1]).toBe("Program");
  });

  test("path passed to enter ends at the parent of the visited node", () => {
    const program = parse("const x = 1;");
    let identPath: readonly PathEntry[] | null = null;
    walk(program, {
      enter(node, _parent, _key, path) {
        if (node.type === "Identifier" && identPath === null) {
          identPath = path.slice();
        }
      },
    });
    expect(identPath).not.toBeNull();
    const types = (identPath as unknown as PathEntry[]).map((p) => p.node.type);
    expect(types[0]).toBe("Program");
    expect(types[types.length - 1]).toBe("VariableDeclarator");
  });

  test("parent and key are null for the root", () => {
    const program = parse("const x = 1;");
    const visitor: WalkVisitor = {
      enter(node, parent, key) {
        if (node === program) {
          expect(parent).toBeNull();
          expect(key).toBeNull();
        }
      },
    };
    walk(program, visitor);
  });

  test("works without enter or leave (no throw, no call)", () => {
    const program = parse("const x = 1;");
    expect(() => walk(program, {})).not.toThrow();
  });
});
