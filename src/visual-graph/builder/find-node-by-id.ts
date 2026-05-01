import { VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { VisualElement, VisualNode } from "../model.js";

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
