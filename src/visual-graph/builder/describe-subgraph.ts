import { caseClause$ } from "../../ir/scope/block-context-kind.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { DIRECTION } from "../direction.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";
import { isClassSubgraph } from "./is-class-subgraph.js";
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
  if (isFunctionSubgraph(scope)) {
    const ownerVarId = subgraphOwnerVar.get(scope.id) ?? null;
    const ownerVar =
      ownerVarId !== null ? (variableMap.get(ownerVarId) ?? null) : null;
    const startLine = ownerVar?.identifiers[0]?.line ?? scope.block.span.line;
    return {
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id,
      kind: SUBGRAPH_KIND.Function,
      line: startLine,
      endLine,
      direction: DIRECTION.RL,
      ownerNodeId: ownerVarId !== null ? nodeId(ownerVarId) : null,
      ownerName: ownerVar?.name ?? "",
      elements: [],
    };
  }
  if (isClassSubgraph(scope)) {
    // The class's own identifier (`Foo` in `class Foo {}`) lives inside
    // the class scope as the inner ClassName binding. Anonymous
    // `ClassExpression` has no such binding, so the variables list is
    // empty -- the subgraph label falls back to "(anonymous)" then.
    //
    // Invariant: `boundary/eslint-scope/enter-class.ts` declares ONLY
    // the inner ClassName into the class scope (methods / fields /
    // static blocks live in their own nested scopes). So `variables[0]`
    // is either that ClassName or absent. Function subgraphs need the
    // explicit `subgraphOwnerVar` map because the owner lives in the
    // *enclosing* scope; class subgraphs can read the inner binding
    // directly because the ClassName is the only entry in this scope's
    // own `variables`.
    const innerNameVarId = scope.variables[0];
    const innerName =
      innerNameVarId !== undefined
        ? (variableMap.get(innerNameVarId)?.name ?? null)
        : null;
    return {
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id,
      kind: SUBGRAPH_KIND.Class,
      line: scope.block.span.line,
      endLine,
      direction: DIRECTION.RL,
      className: innerName,
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
      const caseTest = ctx?.kind === caseClause$.literal ? ctx.caseTest : null;
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
    case SUBGRAPH_KIND.While:
    case SUBGRAPH_KIND.DoWhile:
    case SUBGRAPH_KIND.Return:
    case SUBGRAPH_KIND.Throw:
    case SUBGRAPH_KIND.Block:
      return { ...common, kind };

    case SUBGRAPH_KIND.Function:
      throw new Error(
        `unexpected function subgraph kind for scope ${scope.id}`,
      );

    case SUBGRAPH_KIND.Class:
      throw new Error(`unexpected class subgraph kind for scope ${scope.id}`);
  }
}
