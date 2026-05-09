import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { classifyIdentifier } from "./classify/classify-identifier.js";
import type { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import { ReferenceImpl } from "./reference-impl.js";
import { bindReference } from "./resolve.js";
import type { AnalysisVisitor } from "./visitor.js";
import type { PathEntry } from "./walk/path-entry.js";

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
