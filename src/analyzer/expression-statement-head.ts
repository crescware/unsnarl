import type { AstNode } from "../ir/primitive/ast-node.js";
import type { HeadExpression } from "../ir/reference/expression-statement-head.js";
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
  return tryBuild(expression) ?? rawFromNode(expression, fallback);
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
      return { kind: "identifier", name };
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
      return { kind: "member", object: objectHead, property: propertyName };
    }
    case AST_TYPE.CallExpression: {
      const callee = (node as { callee?: unknown }).callee as MaybeAstNode;
      const calleeHead = tryBuild(callee);
      if (calleeHead === null) {
        return null;
      }
      return { kind: "call", callee: calleeHead };
    }
    case AST_TYPE.NewExpression: {
      const callee = (node as { callee?: unknown }).callee as MaybeAstNode;
      const calleeHead = tryBuild(callee);
      if (calleeHead === null) {
        return null;
      }
      return { kind: "new", callee: calleeHead };
    }
    case AST_TYPE.AwaitExpression: {
      const argument = (node as { argument?: unknown })
        .argument as MaybeAstNode;
      const argHead = tryBuild(argument);
      if (argHead === null) {
        return null;
      }
      return { kind: "await", argument: argHead };
    }
    default:
      return null;
  }
}

function rawFromNode(
  node: MaybeAstNode,
  fallback: { startOffset: number; endOffset: number },
): HeadExpression {
  if (node) {
    const start = node.start;
    const end = node.end;
    if (typeof start === "number" && typeof end === "number") {
      return { kind: "raw", startOffset: start, endOffset: end };
    }
  }
  return {
    kind: "raw",
    startOffset: fallback.startOffset,
    endOffset: fallback.endOffset,
  };
}
