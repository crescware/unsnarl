import type { SerializedScope } from "../../ir/model.js";
import type { VisualElement, VisualNode } from "../model.js";
import { buildChildren } from "./build-children.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { describeSubgraph } from "./describe-subgraph.js";
import { makeVariableNode } from "./make-variable-node.js";
import { shouldSubgraph } from "./should-subgraph.js";
import { writeOpNodeId } from "./write-op-node-id.js";

interface Container {
  elements: VisualElement[];
}

export function buildScope(
  scope: SerializedScope,
  container: Container,
  ctx: BuilderContext,
  state: BuildState,
): void {
  const subgraphHere = shouldSubgraph(scope, ctx.subgraphOwnerVar);
  let bodyContainer: Container = container;
  if (subgraphHere) {
    const sg = describeSubgraph(scope, ctx.subgraphOwnerVar, ctx.variableMap);
    container.elements.push(sg);
    bodyContainer = sg;
    state.subgraphByScope.set(scope.id, sg);
    const ownerVar = ctx.subgraphOwnerVar.get(scope.id);
    if (ownerVar) {
      state.functionSubgraphByFn.set(ownerVar, sg);
    }
  }
  for (const vid of scope.variables) {
    if (ctx.hiddenVariables.has(vid)) {
      continue;
    }
    const v = ctx.variableMap.get(vid);
    if (!v) {
      continue;
    }
    bodyContainer.elements.push(makeVariableNode(v));
  }
  const ops = ctx.writeOpsByScope.get(scope.id) ?? [];
  for (const op of ops) {
    const ownerVar = ctx.variableMap.get(op.varId);
    const declarationKind = ownerVar?.defs[0]?.declarationKind;
    const node: VisualNode = {
      type: "node",
      id: writeOpNodeId(op.refId),
      kind: "WriteOp",
      name: op.varName,
      line: op.line,
      isJsxElement: false,
    };
    if (declarationKind) {
      node.declarationKind = declarationKind;
    }
    bodyContainer.elements.push(node);
  }
  buildChildren(scope, bodyContainer, ctx, state);
}
