import type { NestingDepths } from "../ir/annotations/scope-annotation.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { NESTING_KIND, type NestingKind } from "../serializer/nesting-kind.js";
import { walk } from "./walk/walk.js";

const FUNCTION_TYPES: ReadonlySet<string> = new Set([
  AST_TYPE.FunctionDeclaration,
  AST_TYPE.FunctionExpression,
  AST_TYPE.ArrowFunctionExpression,
]);

const FOR_TYPES: ReadonlySet<string> = new Set([
  AST_TYPE.ForStatement,
  AST_TYPE.ForInStatement,
  AST_TYPE.ForOfStatement,
]);

const WHILE_TYPES: ReadonlySet<string> = new Set([
  AST_TYPE.WhileStatement,
  AST_TYPE.DoWhileStatement,
]);

function emptyCounters(): Record<NestingKind, number> {
  return {
    [NESTING_KIND.Function]: 0,
    [NESTING_KIND.If]: 0,
    [NESTING_KIND.For]: 0,
    [NESTING_KIND.While]: 0,
    [NESTING_KIND.Switch]: 0,
    [NESTING_KIND.TryCatchFinally]: 0,
    [NESTING_KIND.Block]: 0,
  };
}

function snapshot(c: Record<NestingKind, number>): NestingDepths {
  return { ...c };
}

function classifyBlock(
  parent: AstNode | null,
  key: string | null,
): NestingKind | null {
  if (!parent || key === null) {
    return NESTING_KIND.Block;
  }
  if (FUNCTION_TYPES.has(parent.type) && key === "body") {
    return null;
  }
  if (
    parent.type === AST_TYPE.IfStatement &&
    (key === "consequent" || key === "alternate")
  ) {
    return NESTING_KIND.If;
  }
  if (FOR_TYPES.has(parent.type) && key === "body") {
    return NESTING_KIND.For;
  }
  if (WHILE_TYPES.has(parent.type) && key === "body") {
    return NESTING_KIND.While;
  }
  if (
    parent.type === AST_TYPE.TryStatement &&
    (key === "block" || key === "finalizer")
  ) {
    return NESTING_KIND.TryCatchFinally;
  }
  if (parent.type === AST_TYPE.CatchClause && key === "body") {
    return NESTING_KIND.TryCatchFinally;
  }
  return NESTING_KIND.Block;
}

export function computeNestingDepths(
  ast: AstNode,
): ReadonlyMap<number, NestingDepths> {
  const counters = emptyCounters();
  const depthsByOffset = new Map<number, NestingDepths>();
  const enterStack: (NestingKind | null)[] = [];

  walk(ast, {
    enter(node, parent, key) {
      const start = node.start;
      let inc: NestingKind | null = null;
      if (FUNCTION_TYPES.has(node.type)) {
        counters[NESTING_KIND.Function] += 1;
        inc = NESTING_KIND.Function;
      } else if (node.type === AST_TYPE.BlockStatement) {
        const cat = classifyBlock(parent, key);
        if (cat !== null) {
          counters[cat] += 1;
          inc = cat;
        }
      } else if (node.type === AST_TYPE.SwitchStatement) {
        counters[NESTING_KIND.Switch] += 1;
        inc = NESTING_KIND.Switch;
      }
      if (start !== undefined) {
        depthsByOffset.set(start, snapshot(counters));
      }
      enterStack.push(inc);
    },
    leave() {
      const inc = enterStack.pop();
      if (inc !== undefined && inc !== null) {
        counters[inc] -= 1;
      }
    },
  });

  return depthsByOffset;
}
