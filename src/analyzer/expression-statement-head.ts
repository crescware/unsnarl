import { parse } from "valibot";

import type { AstNode } from "../ir/primitive/ast-node.js";
import {
  identifier$,
  member$,
  call$,
  new$,
  await$,
  assign$,
  update$,
  elided$,
  raw$,
} from "../ir/reference/expression-statement-head-kind.js";
import {
  headExpression$,
  type HeadExpression,
  type HeadOperand,
} from "../ir/reference/expression-statement-head.js";
import { AST_TYPE } from "../parser/ast-type.js";

type MaybeAstNode = AstNode | null | undefined;

// Reduce an ExpressionStatement's `expression` to the small mini-AST defined
// in `src/ir/reference/expression-statement-head.ts`. Recognised shapes
// (identifier / member access / call / new / await) are mapped to their
// structural counterpart, dropping call arguments so an awaited promise
// chain like `await Promise.resolve().then(cb).catch(cb)` collapses to a
// 3-segment chain in the mini-AST.
//
// Shapes outside the recognised vocabulary are returned as `raw` with the
// node's source span; the emitter slices the original source to render
// them. This keeps non-chain forms (assignments, updates, etc.) on the
// existing slice-based path without putting synthesised text into the IR.
export function buildHeadExpression(
  expression: MaybeAstNode,
  fallback: { startOffset: number; endOffset: number },
): HeadExpression {
  return parse(
    headExpression$,
    tryBuild(expression) ?? rawFromNode(expression, fallback),
  );
}

function tryBuild(node: MaybeAstNode): HeadExpression | null {
  if (!node) {
    return null;
  }
  switch (node.type) {
    case AST_TYPE.Identifier: {
      const name = (node as { name?: unknown }).name;
      if (typeof name !== "string") {
        return null;
      }
      return { kind: identifier$.literal, name };
    }
    case AST_TYPE.MemberExpression: {
      const object = (node as { object?: unknown }).object as MaybeAstNode;
      const property = (node as { property?: unknown })
        .property as MaybeAstNode;
      const computed = (node as { computed?: unknown }).computed === true;
      if (computed) {
        return null;
      }
      const objectHead = tryBuild(object);
      if (objectHead === null) {
        return null;
      }
      const propertyName = (property as { name?: unknown } | null)?.name;
      if (typeof propertyName !== "string") {
        return null;
      }
      return {
        kind: member$.literal,
        object: objectHead,
        property: propertyName,
      };
    }
    case AST_TYPE.CallExpression: {
      const callee = (node as { callee?: unknown }).callee as MaybeAstNode;
      const calleeHead = tryBuild(callee);
      if (calleeHead === null) {
        return null;
      }
      return { kind: call$.literal, callee: calleeHead };
    }
    case AST_TYPE.NewExpression: {
      const callee = (node as { callee?: unknown }).callee as MaybeAstNode;
      const calleeHead = tryBuild(callee);
      if (calleeHead === null) {
        return null;
      }
      return { kind: new$.literal, callee: calleeHead };
    }
    case AST_TYPE.AwaitExpression: {
      const argument = (node as { argument?: unknown })
        .argument as MaybeAstNode;
      const argHead = tryBuild(argument);
      if (argHead === null) {
        return null;
      }
      return { kind: await$.literal, argument: argHead };
    }
    case AST_TYPE.AssignmentExpression: {
      // The diagram is "how a reference is used", so both sides are
      // independently reduced. A side that doesn't fit the head
      // vocabulary (literal, computed member, destructuring pattern,
      // binary expression, etc.) collapses to `elided` so the surrounding
      // assignment structure still reads as one. Without this, an
      // expression like `C.z = 1` would fall through to the raw source
      // slice and surface `C.z = 1` as the node label, dragging the
      // RHS literal into a label that exists to show how `C` is used.
      //
      // Each operand carries its own source span so the per-side
      // position information that the older `raw` head used to provide
      // for the whole assignment expression is still recoverable from
      // the IR -- especially important for the `elided` side, whose
      // structural head has no positions of its own.
      const left = (node as { left?: unknown }).left as MaybeAstNode;
      const right = (node as { right?: unknown }).right as MaybeAstNode;
      const operator = (node as { operator?: unknown }).operator;
      if (typeof operator !== "string") {
        return null;
      }
      const leftOperand = buildOperand(left);
      const rightOperand = buildOperand(right);
      if (leftOperand === null || rightOperand === null) {
        return null;
      }
      // If neither side reduced, the AssignmentExpression carries no
      // structural information worth keeping. Fall back to raw so the
      // caller can slice the original source instead of showing `... = ...`.
      if (
        leftOperand.head.kind === elided$.literal &&
        rightOperand.head.kind === elided$.literal
      ) {
        return null;
      }
      return {
        kind: assign$.literal,
        operator,
        left: leftOperand,
        right: rightOperand,
      };
    }
    case AST_TYPE.UpdateExpression: {
      const argument = (node as { argument?: unknown })
        .argument as MaybeAstNode;
      const operator = (node as { operator?: unknown }).operator;
      const prefix = (node as { prefix?: unknown }).prefix === true;
      if (typeof operator !== "string") {
        return null;
      }
      const argHead = tryBuild(argument);
      if (argHead === null) {
        return null;
      }
      const argOperand = operandFromHead(argument, argHead);
      if (argOperand === null) {
        return null;
      }
      return {
        kind: update$.literal,
        operator,
        prefix,
        argument: argOperand,
      };
    }
    default:
      return null;
  }
}

// Build a HeadOperand pair: reduce the AST node to a head (or `elided`
// if not reducible), and pair it with the node's source span. Returns
// null only when the AST node is missing offsets -- without offsets the
// span would be ungrounded and the resulting IR could not be relied on
// for position lookups.
function buildOperand(node: MaybeAstNode): HeadOperand | null {
  const head = tryBuild(node) ?? { kind: elided$.literal };
  return operandFromHead(node, head);
}

function operandFromHead(
  node: MaybeAstNode,
  head: HeadExpression,
): HeadOperand | null {
  if (!node) {
    return null;
  }
  const start = node.start;
  const end = node.end;
  if (typeof start !== "number" || typeof end !== "number") {
    return null;
  }
  return { head, startOffset: start, endOffset: end };
}

function rawFromNode(
  node: MaybeAstNode,
  fallback: { startOffset: number; endOffset: number },
): HeadExpression {
  if (node) {
    const start = node.start;
    const end = node.end;
    if (typeof start === "number" && typeof end === "number") {
      return {
        kind: raw$.literal,
        startOffset: start,
        endOffset: end,
      };
    }
  }
  return {
    kind: raw$.literal,
    startOffset: fallback.startOffset,
    endOffset: fallback.endOffset,
  };
}
