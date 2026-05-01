import type { VisualElement } from "../../visual-graph/model.js";
import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";

export function collectWrappedOwnerIds(
  elements: readonly VisualElement[],
  out: Set<string>,
): void {
  for (const e of elements) {
    if (e.type !== VISUAL_ELEMENT_TYPE.Subgraph) {
      continue;
    }
    if (e.kind === SUBGRAPH_KIND.Function && e.ownerNodeId !== undefined) {
      out.add(e.ownerNodeId);
    }
    collectWrappedOwnerIds(e.elements, out);
  }
}
