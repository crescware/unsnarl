import { VISUAL_ELEMENT_TYPE } from "../../visual-element-type.js";
import type { VisualElement, VisualSubgraph } from "../model.js";

export function* iterateVisualSubgraphs(
  elements: readonly VisualElement[],
): Generator<VisualSubgraph> {
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Subgraph) {
      yield e;
      yield* iterateVisualSubgraphs(e.elements);
    }
  }
}
