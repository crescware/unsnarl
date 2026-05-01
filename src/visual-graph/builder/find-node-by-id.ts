import type { VisualElement, VisualNode } from "../model.js";

export function findNodeById(
  elements: VisualElement[],
  id: string,
): VisualNode | null {
  for (const e of elements) {
    if (e.type === "node") {
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
