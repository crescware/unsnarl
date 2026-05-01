import type { AstIdentifier, AstNode } from "../../ir/model.js";
import { classifyIdentifier } from "../classify.js";
import { findJsxElementSpan } from "../jsx-element-span.js";
import type { ScopeManager } from "../manager.js";
import { findReferenceOwners } from "../owner/find-reference-owners.js";
import { findPredicateContainer } from "../predicate.js";
import { bindReference } from "../resolve.js";
import { findReturnContainer } from "../return-container.js";
import { ReferenceImpl } from "../scope.js";
import type { PathEntry } from "../walk/walk.js";
import type { NodeLike } from "./node-like.js";

export function handleIdentifierReference(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: ReadonlyArray<PathEntry>,
  manager: ScopeManager,
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
    writeExpr: result.writeExpr,
  });
  bindReference(manager.current(), ref, manager.globalScope);
  ref.unsnarlOwners = findReferenceOwners(path, manager.current());
  ref.unsnarlPredicateContainer = findPredicateContainer(
    parent as unknown as { type: string; start?: number } | null,
    key,
    path,
  );
  ref.unsnarlReturnContainer = findReturnContainer(path);
  ref.unsnarlJsxElement = findJsxElementSpan(path);
}
