import { classifyIdentifier } from "../../analyzer/classify/classify-identifier.js";
import type { ScopeManager } from "../../analyzer/manager.js";
import { ReferenceImpl } from "../../analyzer/reference-impl.js";
import { bindReference } from "../../analyzer/resolve.js";
import type { PathEntry } from "../../analyzer/walk/path-entry.js";
import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";

export function handleIdentifierReference(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  visitor: AnalysisVisitor,
): void {
  const result = classifyIdentifier(
    parent as unknown as AstNode | null,
    key,
    path,
  );
  if (result.kind !== "reference") {
    return;
  }
  const scope = manager.current();
  const ref = new ReferenceImpl({
    identifier: node as unknown as AstIdentifier,
    from: scope,
    flags: result.flags,
    init: result.init,
  });
  bindReference(scope, ref, manager.globalScope);
  visitor.onReference?.({ ref, parent, key, path, scope });
}
