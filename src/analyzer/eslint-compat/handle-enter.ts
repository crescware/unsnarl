import { AST_TYPE } from "../../parser/ast-type.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import type { ScopeManager } from "../manager.js";
import { isTypeOnlySubtree } from "../skip-types.js";
import type { PathEntry, WalkAction } from "../walk/walk.js";
import { enterBlock } from "./enter-block.js";
import { enterCatch } from "./enter-catch.js";
import { enterFor } from "./enter-for.js";
import { enterFunction } from "./enter-function.js";
import { enterSwitchCase } from "./enter-switch-case.js";
import { enterSwitch } from "./enter-switch.js";
import { handleIdentifierReference } from "./handle-identifier-reference.js";
import type { NodeLike } from "./node-like.js";
import { skipBlockScope } from "./skip-block-scope.js";

export function handleEnter(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): WalkAction {
  if (isTypeOnlySubtree(node.type, key)) {
    return "skip";
  }
  if (
    node.type === AST_TYPE.Identifier ||
    node.type === AST_TYPE.JSXIdentifier
  ) {
    handleIdentifierReference(node, parent, key, path, manager);
    return;
  }
  switch (node.type) {
    case AST_TYPE.FunctionDeclaration:
    case AST_TYPE.FunctionExpression:
    case AST_TYPE.ArrowFunctionExpression:
      enterFunction(node, manager, raw, diagnostics);
      return;
    case AST_TYPE.BlockStatement:
      if (parent && key === "body" && skipBlockScope(parent.type)) {
        return;
      }
      enterBlock(node, parent, key, path, manager, raw, diagnostics);
      return;
    case AST_TYPE.ForStatement:
    case AST_TYPE.ForOfStatement:
    case AST_TYPE.ForInStatement:
      enterFor(node, parent, key, path, manager, raw, diagnostics);
      return;
    case AST_TYPE.SwitchStatement:
      enterSwitch(node, parent, key, path, manager);
      return;
    case AST_TYPE.SwitchCase:
      enterSwitchCase(node, parent, key, manager, raw, diagnostics);
      return;
    case AST_TYPE.CatchClause:
      enterCatch(node, parent, key, path, manager, raw, diagnostics);
      return;
    default:
      return;
  }
}
