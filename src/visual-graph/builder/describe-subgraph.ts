import type { SerializedScope, SerializedVariable } from "../../ir/model.js";
import { DIRECTION } from "../direction.js";
import type { VisualElement, VisualSubgraph } from "../model.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";
import { isFunctionSubgraph } from "./is-function-subgraph.js";
import { nodeId } from "./node-id.js";
import { subgraphScopeId } from "./subgraph-scope-id.js";

export function describeSubgraph(
  scope: SerializedScope,
  subgraphOwnerVar: ReadonlyMap<string, string>,
  variableMap: ReadonlyMap<string, SerializedVariable>,
): VisualSubgraph {
  const id = subgraphScopeId(scope);
  const endLine = scope.block.endSpan.line;
  if (isFunctionSubgraph(scope, subgraphOwnerVar)) {
    const ownerVarId = subgraphOwnerVar.get(scope.id);
    if (!ownerVarId) {
      throw new Error(
        `expected owner variable for function subgraph ${scope.id}`,
      );
    }
    const ownerVar = variableMap.get(ownerVarId);
    const startLine = ownerVar?.identifiers[0]?.line ?? scope.block.span.line;
    return {
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id,
      kind: SUBGRAPH_KIND.Function,
      line: startLine,
      endLine,
      direction: DIRECTION.RL,
      ownerNodeId: nodeId(ownerVarId),
      ownerName: ownerVar?.name ?? "",
      elements: [],
    };
  }
  const kind = controlSubgraphKindOf(scope);
  if (kind === null) {
    throw new Error(
      `expected control subgraph kind for scope ${scope.id} (type=${scope.type})`,
    );
  }
  const sg = {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id,
    kind,
    line: scope.block.span.line,
    endLine,
    direction: DIRECTION.RL,
    elements: [] as VisualElement[],
  } satisfies VisualSubgraph as VisualSubgraph;
  if (kind === "case") {
    sg.caseTest = scope.blockContext?.caseTest ?? null;
  }
  return sg;
}
