import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { PredicateContainer } from "../ir/reference/predicate-container.js";
import { AST_TYPE } from "../parser/ast-type.js";

const LOOP_HEADER_KEYS_FOR: ReadonlySet<string> = new Set([
  "init",
  "test",
  "update",
]);
const LOOP_HEADER_KEYS_FOR_OF_IN: ReadonlySet<string> = new Set([
  "left",
  "right",
]);

export function findPredicateContainer(
  parent: { type: string; start?: number } | null,
  key: string | null,
  path: readonly PathEntry[],
): PredicateContainer | null {
  let curKey: string | null = key;
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const type = entry.node.type;
    const offset = entry.node.start ?? 0;
    if (type === AST_TYPE.IfStatement && curKey === "test") {
      return { type: AST_TYPE.IfStatement, offset };
    }
    if (type === AST_TYPE.SwitchStatement && curKey === "discriminant") {
      return { type: AST_TYPE.SwitchStatement, offset };
    }
    if (type === AST_TYPE.WhileStatement && curKey === "test") {
      return { type: AST_TYPE.WhileStatement, offset };
    }
    if (type === AST_TYPE.DoWhileStatement && curKey === "test") {
      return { type: AST_TYPE.DoWhileStatement, offset };
    }
    if (
      type === AST_TYPE.ForStatement &&
      curKey !== null &&
      LOOP_HEADER_KEYS_FOR.has(curKey)
    ) {
      return { type: AST_TYPE.ForStatement, offset };
    }
    if (
      type === AST_TYPE.ForOfStatement &&
      curKey !== null &&
      LOOP_HEADER_KEYS_FOR_OF_IN.has(curKey)
    ) {
      return { type: AST_TYPE.ForOfStatement, offset };
    }
    if (
      type === AST_TYPE.ForInStatement &&
      curKey !== null &&
      LOOP_HEADER_KEYS_FOR_OF_IN.has(curKey)
    ) {
      return { type: AST_TYPE.ForInStatement, offset };
    }
    curKey = entry.key;
  }
  if (parent && key === "test" && parent.type === AST_TYPE.IfStatement) {
    return { type: AST_TYPE.IfStatement, offset: parent.start ?? 0 };
  }
  if (
    parent &&
    key === "discriminant" &&
    parent.type === AST_TYPE.SwitchStatement
  ) {
    return { type: AST_TYPE.SwitchStatement, offset: parent.start ?? 0 };
  }
  if (parent && key === "test" && parent.type === AST_TYPE.WhileStatement) {
    return { type: AST_TYPE.WhileStatement, offset: parent.start ?? 0 };
  }
  if (parent && key === "test" && parent.type === AST_TYPE.DoWhileStatement) {
    return { type: AST_TYPE.DoWhileStatement, offset: parent.start ?? 0 };
  }
  if (
    parent &&
    key !== null &&
    LOOP_HEADER_KEYS_FOR.has(key) &&
    parent.type === AST_TYPE.ForStatement
  ) {
    return { type: AST_TYPE.ForStatement, offset: parent.start ?? 0 };
  }
  if (
    parent &&
    key !== null &&
    LOOP_HEADER_KEYS_FOR_OF_IN.has(key) &&
    parent.type === AST_TYPE.ForOfStatement
  ) {
    return { type: AST_TYPE.ForOfStatement, offset: parent.start ?? 0 };
  }
  if (
    parent &&
    key !== null &&
    LOOP_HEADER_KEYS_FOR_OF_IN.has(key) &&
    parent.type === AST_TYPE.ForInStatement
  ) {
    return { type: AST_TYPE.ForInStatement, offset: parent.start ?? 0 };
  }
  return null;
}
