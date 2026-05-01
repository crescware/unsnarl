import { DIRECTION, VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { SerializedScope } from "../../ir/model.js";
import type { VisualElement, VisualSubgraph } from "../model.js";
import { branchContainerKey } from "./branch-container-key.js";
import { buildScope } from "./build-scope.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { ifContainerSubgraphId } from "./if-container-subgraph-id.js";
import { lineForOffset } from "./line-for-offset.js";

type Container = Readonly<{
  elements: /* mutable */ VisualElement[];
}>;

export function buildChildren(
  parentScope: SerializedScope,
  container: Container,
  ctx: BuilderContext,
  state: BuildState,
): void {
  const children: /* mutable */ SerializedScope[] = [];
  for (const id of parentScope.childScopes) {
    const c = ctx.scopeMap.get(id);
    if (c) {
      children.push(c);
    }
  }
  let i = 0;
  while (i < children.length) {
    const child = children[i];
    if (!child) {
      i++;
      continue;
    }
    const ckey = branchContainerKey(child);
    if (ckey === null || !ckey.startsWith("if:")) {
      buildScope(child, container, ctx, state);
      i++;
      continue;
    }
    const group: /* mutable */ SerializedScope[] = [child];
    let j = i + 1;
    while (j < children.length) {
      const next = children[j];
      if (!next || branchContainerKey(next) !== ckey) {
        break;
      }
      group.push(next);
      j++;
    }
    if (group.length < 2) {
      for (const g of group) {
        buildScope(g, container, ctx, state);
      }
      i = j;
      continue;
    }
    const offset = child.blockContext?.parentSpanOffset ?? 0;
    const containerId = ifContainerSubgraphId(child.upper ?? "", offset);
    const hasElse = group.some((g) => g.blockContext?.key === "alternate");
    const containerSubgraph = {
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id: containerId,
      kind: "if-else-container",
      line: lineForOffset(ctx.ir.raw, offset),
      direction: DIRECTION.RL,
      hasElse,
      elements: [] as VisualElement[],
    } satisfies VisualSubgraph as VisualSubgraph;
    container.elements.push(containerSubgraph);
    for (const g of group) {
      buildScope(g, containerSubgraph, ctx, state);
    }
    let containerEndLine = containerSubgraph.line;
    for (const elem of containerSubgraph.elements) {
      if (
        elem.type === VISUAL_ELEMENT_TYPE.Subgraph &&
        elem.endLine !== undefined
      ) {
        containerEndLine = Math.max(containerEndLine, elem.endLine);
      }
    }
    if (containerEndLine !== containerSubgraph.line) {
      containerSubgraph.endLine = containerEndLine;
    }
    i = j;
  }
}
