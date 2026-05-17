import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/primitive/ast-node.js";
import {
  identifier$,
  member$,
  call$,
  assign$,
  update$,
  elided$,
  raw$,
} from "../ir/reference/expression-statement-head-kind.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { buildHeadExpression } from "./expression-statement-head.js";

const fallback = { startOffset: 0, endOffset: 0 };

function identifier(name: string, start: number, end: number): AstNode {
  return { type: AST_TYPE.Identifier, name, start, end };
}

function memberExpr(
  object: AstNode,
  property: AstNode,
  start: number,
  end: number,
): AstNode {
  return {
    type: AST_TYPE.MemberExpression,
    object,
    property,
    computed: false,
    start,
    end,
  };
}

function literal(start: number, end: number): AstNode {
  return { type: AST_TYPE.Literal, start, end };
}

describe("buildHeadExpression: AssignmentExpression", () => {
  test("reduces both sides when each is in the head vocabulary", () => {
    // `a.z = b.z`
    const left = memberExpr(identifier("a", 0, 1), identifier("z", 2, 3), 0, 3);
    const right = memberExpr(
      identifier("b", 6, 7),
      identifier("z", 8, 9),
      6,
      9,
    );
    const node: AstNode = {
      type: AST_TYPE.AssignmentExpression,
      operator: "=",
      left,
      right,
      start: 0,
      end: 9,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result).toEqual({
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "a" },
          property: "z",
        },
        startOffset: 0,
        endOffset: 3,
      },
      right: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "b" },
          property: "z",
        },
        startOffset: 6,
        endOffset: 9,
      },
    });
  });

  // The point of `elided`: when one side is outside the head vocabulary
  // (here, a numeric literal), the resulting head keeps the operand's
  // span so consumers can still locate the elided range in source. The
  // structural head reduces to `elided` so the renderer's "..." pattern
  // is the only thing that surfaces in the label.
  test("collapses a non-reducible right-hand side to `elided` while keeping its span", () => {
    // `a.z = 1`
    const left = memberExpr(identifier("a", 0, 1), identifier("z", 2, 3), 0, 3);
    const right = literal(6, 7);
    const node: AstNode = {
      type: AST_TYPE.AssignmentExpression,
      operator: "=",
      left,
      right,
      start: 0,
      end: 7,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result).toEqual({
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "a" },
          property: "z",
        },
        startOffset: 0,
        endOffset: 3,
      },
      right: {
        head: { kind: elided$.literal },
        startOffset: 6,
        endOffset: 7,
      },
    });
  });

  test("preserves a compound operator (e.g. `+=`) verbatim", () => {
    const left = identifier("count", 0, 5);
    const right = literal(9, 10);
    const node: AstNode = {
      type: AST_TYPE.AssignmentExpression,
      operator: "+=",
      left,
      right,
      start: 0,
      end: 10,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result.kind).toEqual(assign$.literal);
    if (result.kind === assign$.literal) {
      expect(result.operator).toEqual("+=");
    }
  });

  // Defensive guard: if neither side reduces (e.g. destructuring patterns
  // on both sides), an assign head with two `elided` operands would
  // render as `... = ...` which carries no useful structure. Fall back
  // to raw so the renderer can slice the original source instead.
  test("falls back to raw when neither side fits the head vocabulary", () => {
    // Both sides are literals — neither reduces.
    const left = literal(0, 1);
    const right = literal(4, 5);
    const node: AstNode = {
      type: AST_TYPE.AssignmentExpression,
      operator: "=",
      left,
      right,
      start: 0,
      end: 5,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result.kind).toEqual(raw$.literal);
  });

  test("falls back to raw when the operator is missing", () => {
    const left = identifier("a", 0, 1);
    const right = literal(4, 5);
    const node = {
      type: AST_TYPE.AssignmentExpression,
      left,
      right,
      start: 0,
      end: 5,
    } as unknown as AstNode;
    const result = buildHeadExpression(node, fallback);
    expect(result.kind).toEqual(raw$.literal);
  });
});

describe("buildHeadExpression: UpdateExpression", () => {
  test("captures a prefix update with operator and prefix=true", () => {
    // `++a.z`
    const argument = memberExpr(
      identifier("a", 2, 3),
      identifier("z", 4, 5),
      2,
      5,
    );
    const node: AstNode = {
      type: AST_TYPE.UpdateExpression,
      operator: "++",
      prefix: true,
      argument,
      start: 0,
      end: 5,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result).toEqual({
      kind: update$.literal,
      operator: "++",
      prefix: true,
      argument: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "a" },
          property: "z",
        },
        startOffset: 2,
        endOffset: 5,
      },
    });
  });

  test("captures a postfix update with prefix=false", () => {
    // `a.z--`
    const argument = memberExpr(
      identifier("a", 0, 1),
      identifier("z", 2, 3),
      0,
      3,
    );
    const node: AstNode = {
      type: AST_TYPE.UpdateExpression,
      operator: "--",
      prefix: false,
      argument,
      start: 0,
      end: 5,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result.kind).toEqual(update$.literal);
    if (result.kind === update$.literal) {
      expect(result.prefix).toEqual(false);
      expect(result.operator).toEqual("--");
    }
  });

  // Update only makes sense when the argument is a valid LHS that's
  // also in the head vocabulary. A computed member (`a[k]++`) bottoms
  // out at a non-reducible MemberExpression, so the whole UpdateExpression
  // falls back to raw rather than producing a meaningless update head.
  test("falls back to raw when the argument is not in the head vocabulary", () => {
    // `a[k]++` -- computed member is not in the vocabulary.
    const argument: AstNode = {
      type: AST_TYPE.MemberExpression,
      object: identifier("a", 0, 1),
      property: identifier("k", 2, 3),
      computed: true,
      start: 0,
      end: 4,
    };
    const node: AstNode = {
      type: AST_TYPE.UpdateExpression,
      operator: "++",
      prefix: false,
      argument,
      start: 0,
      end: 6,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result.kind).toEqual(raw$.literal);
  });
});

describe("buildHeadExpression: vocabulary fallthrough", () => {
  // Pure sanity that the head reducer is what gates the new kinds:
  // a CallExpression still reduces to a `call` head exactly as it did
  // before assign/update/elided were added.
  test("a CallExpression still reduces to a `call` head", () => {
    // `f()`
    const callee = identifier("f", 0, 1);
    const node: AstNode = {
      type: AST_TYPE.CallExpression,
      callee,
      start: 0,
      end: 3,
    };
    const result = buildHeadExpression(node, fallback);
    expect(result).toEqual({
      kind: call$.literal,
      callee: { kind: identifier$.literal, name: "f" },
    });
  });
});
