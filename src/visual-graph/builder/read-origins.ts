import { branchContainerKey } from "./branch-container-key.js";
import { branchScopeOf } from "./branch-scope-of.js";
import type { BuilderContext } from "./context.js";
import { isAncestorScope } from "./is-ancestor-scope.js";
import { nodeId } from "./node-id.js";
import { writeOpNodeId } from "./write-op-node-id.js";
import type { WriteOp } from "./write-op.js";

export function readOrigins(
  varId: string,
  refOffset: number,
  refScopeId: string,
  ctx: BuilderContext,
): readonly string[] {
  const ops = ctx.writeOpsByVariable.get(varId) ?? [];
  const prev = ops.filter((op) => op.offset < refOffset);
  const last = prev[prev.length - 1];
  if (!last) {
    return [nodeId(varId)];
  }
  if (isAncestorScope(last.scopeId, refScopeId, ctx.scopeMap)) {
    return [writeOpNodeId(last.refId)];
  }
  const lastBranchId = branchScopeOf(last.scopeId, ctx.scopeMap);
  if (!lastBranchId) {
    return [writeOpNodeId(last.refId)];
  }
  const lastBranchScope = ctx.scopeMap.get(lastBranchId);
  if (!lastBranchScope) {
    return [writeOpNodeId(last.refId)];
  }
  const containerKey = branchContainerKey(lastBranchScope);
  if (containerKey === null) {
    return [writeOpNodeId(last.refId)];
  }

  const branchScopeIds: /* mutable */ string[] = [];
  for (const s of ctx.ir.scopes) {
    if (branchContainerKey(s) === containerKey) {
      branchScopeIds.push(s.id);
    }
  }

  const origins: /* mutable */ string[] = [];
  const isSwitch = containerKey.startsWith("switch:");
  for (const branchId of branchScopeIds) {
    const branchScope = ctx.scopeMap.get(branchId);
    if (isSwitch && branchScope !== undefined && branchScope.fallsThrough) {
      const cases = ctx.sortedCasesByContainer.get(containerKey);
      if (cases) {
        const idx = cases.indexOf(branchScope);
        if (idx >= 0 && idx < cases.length - 1) {
          continue;
        }
      }
    }
    // Cases ending in return/throw exit the function, so their writes
    // never reach a read after the switch.
    if (isSwitch && branchScope !== undefined && branchScope.exitsFunction) {
      continue;
    }
    let lastOp: WriteOp | null = null;
    for (const op of prev) {
      if (
        op.scopeId === branchId ||
        isAncestorScope(branchId, op.scopeId, ctx.scopeMap)
      ) {
        lastOp = op;
      }
    }
    if (lastOp) {
      origins.push(writeOpNodeId(lastOp.refId));
    }
  }

  if (containerKey.startsWith("if:")) {
    const hasAlternate = branchScopeIds.some((id) => {
      const s = ctx.scopeMap.get(id);
      return s?.blockContext?.key === "alternate";
    });
    if (!hasAlternate) {
      const ifOffset = lastBranchScope.blockContext?.parentSpanOffset ?? 0;
      const before = ops.filter((op) => op.offset < ifOffset);
      const lastBefore = before[before.length - 1];
      if (lastBefore) {
        origins.push(writeOpNodeId(lastBefore.refId));
      } else {
        origins.push(nodeId(varId));
      }
    }
  }

  if (containerKey.startsWith("try:")) {
    const hasHandler = branchScopeIds.some((id) => {
      const s = ctx.scopeMap.get(id);
      return s?.blockContext?.key === "handler";
    });
    if (!hasHandler) {
      const tryOffset = lastBranchScope.blockContext?.parentSpanOffset ?? 0;
      const before = ops.filter((op) => op.offset < tryOffset);
      const lastBefore = before[before.length - 1];
      if (lastBefore) {
        origins.push(writeOpNodeId(lastBefore.refId));
      } else {
        origins.push(nodeId(varId));
      }
    }
  }

  if (origins.length === 0) {
    return [writeOpNodeId(last.refId)];
  }
  return Array.from(new Set(origins));
}
