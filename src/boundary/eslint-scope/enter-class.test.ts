import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { enterClass } from "./enter-class.js";
import { ScopeManager } from "./manager.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";

describe("enterClass", () => {
  test("pushes a class scope and defines the inner ClassName for ClassDeclaration", () => {
    const code = "class C {}";
    const program = parse(code);
    const cls = findFirst(program, AST_TYPE.ClassDeclaration);
    const manager = new ScopeManager("module", program as unknown as AstNode);

    enterClass(cls, null, null, [], manager, {});

    const classScope = manager.current();
    expect(classScope.type).toBe("class");
    expect(classScope.block).toBe(cls);
    expect(classScope.variables.map((v) => v.name)).toEqual(["C"]);
    const inner = classScope.set.get("C");
    expect(inner?.defs.map((v) => v.type)).toEqual([DEFINITION_TYPE.ClassName]);
    expect(inner?.scope).toBe(classScope);
  });

  test("defines the inner ClassName for ClassExpression with id", () => {
    const code = "const X = class C {};";
    const program = parse(code);
    const cls = findFirst(program, AST_TYPE.ClassExpression);
    const manager = new ScopeManager("module", program as unknown as AstNode);

    enterClass(cls, null, null, [], manager, {});

    const classScope = manager.current();
    expect(classScope.type).toBe("class");
    expect(classScope.variables.map((v) => v.name)).toEqual(["C"]);
  });

  test("does not define an inner variable for an anonymous ClassExpression", () => {
    const code = "const X = class {};";
    const program = parse(code);
    const cls = findFirst(program, AST_TYPE.ClassExpression);
    const manager = new ScopeManager("module", program as unknown as AstNode);

    enterClass(cls, null, null, [], manager, {});

    const classScope = manager.current();
    expect(classScope.type).toBe("class");
    expect(classScope.variables).toHaveLength(0);
  });

  test("notifies the visitor with the new class scope", () => {
    const code = "class C {}";
    const program = parse(code);
    const cls = findFirst(program, AST_TYPE.ClassDeclaration);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const seen: /* mutable */ string[] = [];

    enterClass(cls, null, null, [], manager, {
      onScope(input) {
        seen.push(input.scope.type);
      },
    });

    expect(seen).toEqual(["class"]);
  });
});
