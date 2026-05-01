import type { VisualNode } from "../../../visual-graph/model.js";
import { NODE_KIND } from "../../../visual-graph/node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../../visual-graph/visual-element-type.js";

export function baseNode(): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: "n_v",
    kind: NODE_KIND.Variable,
    name: "x",
    line: 1,
    endLine: null,
    isJsxElement: false,
    unused: false,
    declarationKind: null,
    initIsFunction: false,
    importKind: null,
    importedName: null,
    importSource: null,
  };
}
