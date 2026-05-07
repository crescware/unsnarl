import { classifyIdentifier } from "../../analyzer/classify/classify-identifier.js";
import type { ScopeManager } from "../../analyzer/manager.js";
import { ReferenceImpl } from "../../analyzer/reference-impl.js";
import { bindReference } from "../../analyzer/resolve.js";
import type { PathEntry } from "../../analyzer/walk/path-entry.js";
import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { AnnotationBuilder } from "./annotation-builder.js";
import type { NodeLike } from "./node-like.js";

export function handleIdentifierReference(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  annotationBuilder: AnnotationBuilder,
): void {
  const result = classifyIdentifier(
    parent as unknown as AstNode | null,
    key,
    path,
  );
  if (result.kind !== "reference") {
    return;
  }
  const ref = new ReferenceImpl({
    identifier: node as unknown as AstIdentifier,
    from: manager.current(),
    flags: result.flags,
    init: result.init,
  });
  bindReference(manager.current(), ref, manager.globalScope);
  const annotation = annotationBuilder.buildReferenceAnnotation({
    parent,
    key,
    path,
    scope: manager.current(),
  });
  manager.annotations.setReference(ref, annotation);
}
