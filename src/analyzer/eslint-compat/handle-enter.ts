import type { DiagnosticCollector } from "../../util/diagnostic.js";
import type { ScopeManager } from "../manager.js";
import { isTypeOnlySubtree } from "../skip-types.js";
import type { PathEntry, WalkAction } from "../walk.js";
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
  path: ReadonlyArray<PathEntry>,
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): WalkAction {
  if (isTypeOnlySubtree(node.type, key)) {
    return "skip";
  }
  if (node.type === "Identifier" || node.type === "JSXIdentifier") {
    handleIdentifierReference(node, parent, key, path, manager);
    return;
  }
  switch (node.type) {
    case "FunctionDeclaration":
    case "FunctionExpression":
    case "ArrowFunctionExpression":
      enterFunction(node, manager, raw, diagnostics);
      return;
    case "BlockStatement":
      if (parent && key === "body" && skipBlockScope(parent.type)) {
        return;
      }
      enterBlock(node, parent, key, manager, raw, diagnostics);
      return;
    case "ForStatement":
    case "ForOfStatement":
    case "ForInStatement":
      enterFor(node, parent, key, manager, raw, diagnostics);
      return;
    case "SwitchStatement":
      enterSwitch(node, parent, key, manager);
      return;
    case "SwitchCase":
      enterSwitchCase(node, parent, key, manager, raw, diagnostics);
      return;
    case "CatchClause":
      enterCatch(node, parent, key, manager, raw, diagnostics);
      return;
    default:
      return;
  }
}
