import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualNode } from "../visual-node.js";

export function findNodeById(
  elements: readonly VisualElement[],
  id: string,
): VisualNode | null {
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Node) {
      if (e.id === id) {
        return e;
      }
    } else {
      const found = findNodeById(e.elements, id);
      if (found) {
        return found;
      }
    }
  }
  return null;
}
