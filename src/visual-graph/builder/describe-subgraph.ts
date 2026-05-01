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
  const common = {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id,
    line: scope.block.span.line,
    endLine,
    direction: DIRECTION.RL,
    elements: [] as VisualElement[],
  } as const;
  switch (kind) {
    case SUBGRAPH_KIND.Case: {
      const ctx = scope.blockContext;
      const caseTest = ctx?.kind === "case-clause" ? ctx.caseTest : null;
      return { ...common, kind: SUBGRAPH_KIND.Case, caseTest };
    }
    case SUBGRAPH_KIND.IfElseContainer:
      return { ...common, kind: SUBGRAPH_KIND.IfElseContainer, hasElse: false };
    case SUBGRAPH_KIND.Switch:
    case SUBGRAPH_KIND.If:
    case SUBGRAPH_KIND.Else:
    case SUBGRAPH_KIND.Try:
    case SUBGRAPH_KIND.Catch:
    case SUBGRAPH_KIND.Finally:
    case SUBGRAPH_KIND.For:
    case SUBGRAPH_KIND.Return:
      return { ...common, kind };
    case SUBGRAPH_KIND.Function:
      throw new Error(
        `unexpected function subgraph kind for scope ${scope.id}`,
      );
  }
}
