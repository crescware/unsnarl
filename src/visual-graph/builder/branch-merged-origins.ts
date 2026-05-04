import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { branchContainerKey } from "./branch-container-key.js";
import type { BuilderContext } from "./context.js";
import { isAncestorScope } from "./is-ancestor-scope.js";
import { outermostBranchUnder } from "./outermost-branch-under.js";
import { writeOpNodeId } from "./write-op-node-id.js";
import type { WriteOp } from "./write-op.js";

// Given an outer branch (an if/case/try arm), collect every reachable
// last-write op by recursing into branch containers nested inside it.
// Without this, a `case 0` whose body is a fully-covering inner if/else
// flattens its branches into a single linear lastOp and drops every
// inner sibling that is not the source-textual last writer.
export function branchMergedOrigins(
  branchId: string,
  prev: readonly WriteOp[],
  ctx: BuilderContext,
): readonly string[] {
  const insideOps = prev.filter(
    (op) =>
      op.scopeId === branchId ||
      isAncestorScope(branchId, op.scopeId, ctx.scopeMap),
  );
  if (insideOps.length === 0) {
    return [];
  }
  const last = insideOps[insideOps.length - 1];
  if (!last) {
    return [];
  }
  const innerBranchId = outermostBranchUnder(
    branchId,
    last.scopeId,
    ctx.scopeMap,
  );
  if (innerBranchId === null) {
    return [writeOpNodeId(last.refId)];
  }
  const innerScope = ctx.scopeMap.get(innerBranchId);
  if (!innerScope) {
    return [writeOpNodeId(last.refId)];
  }
  const innerKey = branchContainerKey(innerScope);
  if (innerKey === null) {
    return [writeOpNodeId(last.refId)];
  }

  const innerSiblings: SerializedScope[] = [];
  for (const s of ctx.ir.scopes) {
    if (branchContainerKey(s) !== innerKey) {
      continue;
    }
    if (!isAncestorScope(branchId, s.id, ctx.scopeMap)) {
      continue;
    }
    innerSiblings.push(s);
  }

  const isSwitch = innerKey.startsWith("switch:");
  const sortedCases = isSwitch
    ? ctx.sortedCasesByContainer.get(innerKey)
    : undefined;
  const merged: /* mutable */ string[] = [];
  for (const sib of innerSiblings) {
    if (isSwitch && sib.fallsThrough && sortedCases) {
      const idx = sortedCases.indexOf(sib);
      if (idx >= 0 && idx < sortedCases.length - 1) {
        continue;
      }
    }
    if (isSwitch && sib.exitsFunction) {
      continue;
    }
    const sub = branchMergedOrigins(sib.id, insideOps, ctx);
    for (const id of sub) {
      merged.push(id);
    }
  }

  // When the inner container can be skipped (an if without alternate, or a
  // try without catch), the last write that ran *before* the inner container
  // remains a possible last writer for the outer branch and must be kept.
  // A switch without default at this nested level is intentionally not
  // covered here; that mirrors the top-level read-origins behaviour and is
  // tracked separately as a visualisation tuning concern.
  const lacksFallback =
    (innerKey.startsWith("if:") &&
      !innerSiblings.some((s) => s.blockContext?.key === "alternate")) ||
    (innerKey.startsWith("try:") &&
      !innerSiblings.some((s) => s.blockContext?.key === "handler"));
  if (lacksFallback) {
    const innerOffset = innerScope.blockContext?.parentSpanOffset ?? 0;
    const beforeInner = insideOps.filter((op) => op.offset < innerOffset);
    const lastBefore = beforeInner[beforeInner.length - 1];
    if (lastBefore) {
      merged.push(writeOpNodeId(lastBefore.refId));
    }
  }

  if (merged.length === 0) {
    return [writeOpNodeId(last.refId)];
  }
  return merged;
}
