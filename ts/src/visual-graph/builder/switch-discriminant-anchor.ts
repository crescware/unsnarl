import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import { sanitize } from "./sanitize.js";

function switchDiscriminantNodeId(
  parentScopeId: string,
  offset: number,
): string {
  return `switch_discriminant_${sanitize(parentScopeId)}_${offset}`;
}

export function attachSwitchDiscriminantAnchor(
  scope: SerializedScope,
  sg: VisualSubgraph,
  state: BuildState,
): void {
  if (scope.type !== SCOPE_TYPE.Switch) {
    return;
  }
  const offset = scope.block.span.offset;
  if (state.switchDiscriminantAnchorByOffset.has(offset)) {
    return;
  }
  const node = {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: switchDiscriminantNodeId(scope.upper ?? "", offset),
    kind: NODE_KIND.SyntheticSwitchStatementDiscriminant,
    name: "switch-discriminant",
    line: scope.block.span.line,
    endLine: null,
    isJsxElement: false,
    unused: false,
  } satisfies VisualNode;
  state.pendingLoopTestAnchors.push({
    subgraph: sg,
    node,
    position: "first",
  });
  state.switchDiscriminantAnchorByOffset.set(offset, node.id);
}
