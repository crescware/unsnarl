import { VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { VisualElement, VisualNode } from "../../visual-graph/model.js";

export function collectNodes(
  elements: readonly VisualElement[],
): readonly VisualNode[] {
  const out: /* mutable */ VisualNode[] = [];
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Node) {
      out.push(e);
    } else {
      out.push(...collectNodes(e.elements));
    }
  }
  return out;
}
