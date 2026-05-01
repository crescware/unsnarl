import { VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { VisualElement } from "../../visual-graph/model.js";

export function collectWrappedOwnerIds(
  elements: readonly VisualElement[],
  out: Set<string>,
): void {
  for (const e of elements) {
    if (e.type !== VISUAL_ELEMENT_TYPE.Subgraph) {
      continue;
    }
    if (e.kind === "function" && e.ownerNodeId !== undefined) {
      out.add(e.ownerNodeId);
    }
    collectWrappedOwnerIds(e.elements, out);
  }
}
