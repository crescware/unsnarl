import type { VisualElement } from "../model.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";

export function collectNodeIds(
  elements: readonly VisualElement[],
): readonly string[] {
  const out: /* mutable */ string[] = [];
  walk(elements);
  return out;

  function walk(items: readonly VisualElement[]): void {
    for (const item of items) {
      if (item.type === VISUAL_ELEMENT_TYPE.Node) {
        out.push(item.id);
      } else {
        walk(item.elements);
      }
    }
  }
}
