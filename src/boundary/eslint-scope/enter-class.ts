import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { declareVariable } from "./declare/declare-variable.js";
import { isIdentifierNode } from "./hoisting/is-identifier-node.js";
import type { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";
import type { PathEntry } from "./walk/path-entry.js";

// Mirror of eslint-scope's `Referencer#visitClass` for the inner class
// scope (`__nestClassScope` + inner ClassName define). The outer
// ClassName for ClassDeclaration is already added by the hoisting pass
// before the walk reaches the class, so this only handles the inner
// definition that lives within the new class scope.
export function enterClass(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  visitor: AnalysisVisitor,
): void {
  const scope = manager.push("class", node as unknown as AstNode);
  visitor.onScope?.({ scope, parent, key, path });
  const id = node["id"];
  if (isIdentifierNode(id)) {
    declareVariable(
      scope,
      id,
      DEFINITION_TYPE.ClassName,
      node as unknown as AstNode,
      null,
    );
  }
}
