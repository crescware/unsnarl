import type { CategoryDepths } from "../ir/annotations/scope-annotation.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { CATEGORY, type Category } from "../serializer/category.js";
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

function emptyCounters(): Record<Category, number> {
  return {
    [CATEGORY.Function]: 0,
    [CATEGORY.If]: 0,
    [CATEGORY.For]: 0,
    [CATEGORY.While]: 0,
    [CATEGORY.Switch]: 0,
    [CATEGORY.TryCatchFinally]: 0,
    [CATEGORY.Block]: 0,
  };
}

function snapshot(c: Record<Category, number>): CategoryDepths {
  return { ...c };
}

function classifyBlock(
  parent: AstNode | null,
  key: string | null,
): Category | null {
  if (!parent || key === null) {
    return CATEGORY.Block;
  }
  if (FUNCTION_TYPES.has(parent.type) && key === "body") {
    return null;
  }
  if (
    parent.type === AST_TYPE.IfStatement &&
    (key === "consequent" || key === "alternate")
  ) {
    return CATEGORY.If;
  }
  if (FOR_TYPES.has(parent.type) && key === "body") {
    return CATEGORY.For;
  }
  if (WHILE_TYPES.has(parent.type) && key === "body") {
    return CATEGORY.While;
  }
  if (
    parent.type === AST_TYPE.TryStatement &&
    (key === "block" || key === "finalizer")
  ) {
    return CATEGORY.TryCatchFinally;
  }
  if (parent.type === AST_TYPE.CatchClause && key === "body") {
    return CATEGORY.TryCatchFinally;
  }
  return CATEGORY.Block;
}

export function computeCategoryDepths(
  ast: AstNode,
): ReadonlyMap<number, CategoryDepths> {
  const counters = emptyCounters();
  const depthsByOffset = new Map<number, CategoryDepths>();
  const enterStack: (Category | null)[] = [];

  walk(ast, {
    enter(node, parent, key) {
      const start = node.start;
      let inc: Category | null = null;
      if (FUNCTION_TYPES.has(node.type)) {
        counters[CATEGORY.Function] += 1;
        inc = CATEGORY.Function;
      } else if (node.type === AST_TYPE.BlockStatement) {
        const cat = classifyBlock(parent, key);
        if (cat !== null) {
          counters[cat] += 1;
          inc = cat;
        }
      } else if (node.type === AST_TYPE.SwitchStatement) {
        counters[CATEGORY.Switch] += 1;
        inc = CATEGORY.Switch;
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
