import { VISUAL_ELEMENT_TYPE } from "../../visual-element-type.js";
import type { VisualElement } from "../model.js";

export function collectIds(elements: readonly VisualElement[]): Set<string> {
  const ids = new Set<string>();
  walk(elements);
  return ids;

  function walk(items: readonly VisualElement[]): void {
    for (const item of items) {
      ids.add(item.id);
      if (item.type === VISUAL_ELEMENT_TYPE.Subgraph) {
        walk(item.elements);
      }
    }
  }
}
