import { describe, expect, test } from "vitest";

import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import {
  identifier$,
  member$,
  call$,
  new$,
  await$,
  raw$,
} from "../ir/reference/expression-statement-head-kind.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { findExpressionStatementContainer } from "./expression-statement-container.js";

function entry(node: AstNode, key: string | null = null): PathEntry {
  return { node, key };
}

describe("findExpressionStatementContainer", () => {
  test("returns a structured `call` head when the expression is a CallExpression with a MemberExpression callee", () => {
    const callee = {
      type: AST_TYPE.MemberExpression,
      start: 0,
      end: 11,
      object: { type: AST_TYPE.Identifier, name: "console", start: 0, end: 7 },
      property: { type: AST_TYPE.Identifier, name: "log", start: 8, end: 11 },
      computed: false,
    } as const satisfies AstNode;
    const callExpr = {
      type: AST_TYPE.CallExpression,
      start: 0,
      end: 14,
      callee,
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 0,
      end: 15,
      expression: callExpr,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 20 }),
      entry(stmt, "body"),
      entry(callExpr, "expression"),
      entry(callee, "callee"),
      entry({ type: AST_TYPE.Identifier, start: 0, end: 7 }, "object"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 0,
      endOffset: 15,
      head: {
        kind: call$.literal,
        callee: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "console" },
          property: "log",
        },
      },
    });
  });

  test("returns an `identifier` head when the expression is a bare identifier", () => {
    const expr = {
      type: AST_TYPE.Identifier,
      name: "a",
      start: 2,
      end: 3,
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 2,
      end: 4,
      expression: expr,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 20 }),
      entry(stmt, "body"),
      entry(expr, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 2,
      endOffset: 4,
      head: { kind: identifier$.literal, name: "a" },
    });
  });

  test("collapses an awaited Promise chain to an `await`/`call`/`member` mini-AST", () => {
    const promiseId = {
      type: AST_TYPE.Identifier,
      name: "Promise",
      start: 6,
      end: 13,
    } as const satisfies AstNode;
    const resolveId = {
      type: AST_TYPE.Identifier,
      name: "resolve",
      start: 14,
      end: 21,
    } as const satisfies AstNode;
    const promiseResolveCallee = {
      type: AST_TYPE.MemberExpression,
      start: 6,
      end: 21,
      object: promiseId,
      property: resolveId,
      computed: false,
    } as const satisfies AstNode;
    const promiseResolveCall = {
      type: AST_TYPE.CallExpression,
      start: 6,
      end: 23,
      callee: promiseResolveCallee,
    } as const satisfies AstNode;
    const thenId = {
      type: AST_TYPE.Identifier,
      name: "then",
      start: 27,
      end: 31,
    } as const satisfies AstNode;
    const thenCallee = {
      type: AST_TYPE.MemberExpression,
      start: 6,
      end: 31,
      object: promiseResolveCall,
      property: thenId,
      computed: false,
    } as const satisfies AstNode;
    const thenCall = {
      type: AST_TYPE.CallExpression,
      start: 6,
      end: 70,
      callee: thenCallee,
    } as const satisfies AstNode;
    const catchId = {
      type: AST_TYPE.Identifier,
      name: "catch",
      start: 74,
      end: 79,
    } as const satisfies AstNode;
    const catchCallee = {
      type: AST_TYPE.MemberExpression,
      start: 6,
      end: 79,
      object: thenCall,
      property: catchId,
      computed: false,
    } as const satisfies AstNode;
    const catchCall = {
      type: AST_TYPE.CallExpression,
      start: 6,
      end: 120,
      callee: catchCallee,
    } as const satisfies AstNode;
    const awaitExpr = {
      type: AST_TYPE.AwaitExpression,
      start: 0,
      end: 120,
      argument: catchCall,
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 0,
      end: 121,
      expression: awaitExpr,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 121 }),
      entry(stmt, "body"),
      entry(awaitExpr, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 0,
      endOffset: 121,
      head: {
        kind: await$.literal,
        argument: {
          kind: call$.literal,
          callee: {
            kind: member$.literal,
            object: {
              kind: call$.literal,
              callee: {
                kind: member$.literal,
                object: {
                  kind: call$.literal,
                  callee: {
                    kind: member$.literal,
                    object: {
                      kind: identifier$.literal,
                      name: "Promise",
                    },
                    property: "resolve",
                  },
                },
                property: "then",
              },
            },
            property: "catch",
          },
        },
      },
    });
  });

  test("returns a `new` head with the constructor identifier when the expression is a NewExpression", () => {
    const ctor = {
      type: AST_TYPE.Identifier,
      name: "C",
      start: 4,
      end: 5,
    } as const satisfies AstNode;
    const newExpr = {
      type: AST_TYPE.NewExpression,
      start: 0,
      end: 7,
      callee: ctor,
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 0,
      end: 8,
      expression: newExpr,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 8 }),
      entry(stmt, "body"),
      entry(newExpr, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 0,
      endOffset: 8,
      head: {
        kind: new$.literal,
        callee: { kind: identifier$.literal, name: "C" },
      },
    });
  });

  test("falls back to a `raw` head for computed MemberExpression because the property is not a static name", () => {
    const obj = {
      type: AST_TYPE.Identifier,
      name: "a",
      start: 0,
      end: 1,
    } as const satisfies AstNode;
    const key = {
      type: AST_TYPE.Literal,
      start: 2,
      end: 3,
    } as const satisfies AstNode;
    const member = {
      type: AST_TYPE.MemberExpression,
      start: 0,
      end: 4,
      object: obj,
      property: key,
      computed: true,
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 0,
      end: 5,
      expression: member,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 5 }),
      entry(stmt, "body"),
      entry(member, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 0,
      endOffset: 5,
      head: { kind: raw$.literal, startOffset: 0, endOffset: 4 },
    });
  });

  test("falls back to a `raw` head with the expression's span when the shape is not in the recognised vocabulary", () => {
    // BinaryExpression is deliberately outside the head vocabulary
    // (the diagram cares about who-references-whom, not about generic
    // arithmetic), so it has to land on the raw fallback path. The
    // previous version of this test used an AssignmentExpression, which
    // now does reduce to an `assign` head — using BinaryExpression
    // restores the test's original "unrecognised shape" intent.
    const left = {
      type: AST_TYPE.Identifier,
      name: "x",
      start: 0,
      end: 1,
    } as const satisfies AstNode;
    const right = {
      type: AST_TYPE.Literal,
      start: 4,
      end: 5,
    } as const satisfies AstNode;
    const binary = {
      type: AST_TYPE.BinaryExpression,
      start: 0,
      end: 5,
      left,
      right,
      operator: "+",
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 0,
      end: 6,
      expression: binary,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 6 }),
      entry(stmt, "body"),
      entry(binary, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 0,
      endOffset: 6,
      head: { kind: raw$.literal, startOffset: 0, endOffset: 5 },
    });
  });

  test("returns null when there is no ExpressionStatement on the path", () => {
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.VariableDeclaration, start: 0, end: 10 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 6, end: 7 }, "id"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual(null);
  });

  test("returns null when the ExpressionStatement is missing offsets", () => {
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ExpressionStatement }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 0, end: 1 }, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual(null);
  });
});
