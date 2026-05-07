import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { buildChildren } from "./build-children.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { describeSubgraph } from "./describe-subgraph.js";
import { isCollapsed } from "./is-collapsed.js";
import { attachLoopTestAnchor } from "./loop-test-anchor.js";
import { makeVariableNode } from "./make-variable-node.js";
import { nodeId } from "./node-id.js";
import { shouldSubgraph } from "./should-subgraph.js";
import { attachSwitchDiscriminantAnchor } from "./switch-discriminant-anchor.js";
import { visibleAncestorSubgraphId } from "./visible-ancestor-subgraph-id.js";
import { writeOpNodeId } from "./write-op-node-id.js";

type Container = Readonly<{
  elements: /* mutable */ VisualElement[];
}>;

function recordCollapsedDescendants(
  scope: SerializedScope,
  rootScopeId: string,
  ctx: BuilderContext,
  state: BuildState,
): void {
  state.collapsedRootByScope?.set(scope.id, rootScopeId);
  for (const childId of scope.childScopes) {
    const child = ctx.scopeMap.get(childId);
    if (!child) {
      continue;
    }
    recordCollapsedDescendants(child, rootScopeId, ctx, state);
  }
}

export function buildScope(
  scope: SerializedScope,
  container: Container,
  ctx: BuilderContext,
  state: BuildState,
): void {
  if (isCollapsed(scope, ctx.depths)) {
    // Nothing rendered for the collapsed subtree itself. The visual
    // anchor for cross-boundary edges is, in priority order:
    //   1. the owning variable of the collapsed scope (e.g. `fnB` for
    //      `function fnB() { ... }`) -- already emitted in the parent;
    //   2. otherwise the closest visible ancestor subgraph -- so that
    //      anonymous callbacks / branch / loop / try bodies / bare
    //      blocks still announce "the read happens somewhere inside
    //      the surviving outer container".
    // The redirect itself happens in build-visual-graph's post-pass via
    // collapsedAnchorByRoot + suppressedPredicateRedirect.
    recordCollapsedDescendants(scope, scope.id, ctx, state);
    const ownerVarId = ctx.subgraphOwnerVar.get(scope.id) ?? null;
    if (ownerVarId !== null) {
      state.collapsedAnchorByRoot?.set(scope.id, nodeId(ownerVarId));
    }
    // If the collapsed scope is the body of a control statement, its
    // test anchor (if-test, for-test, while-test, switch discriminant)
    // could not be created -- record where reads of that predicate
    // should land instead.
    const blockCtx = scope.blockContext;
    if (blockCtx && state.suppressedPredicateRedirect) {
      const visibleAncestor = visibleAncestorSubgraphId(scope, ctx, state);
      if (visibleAncestor !== null) {
        state.suppressedPredicateRedirect.set(
          blockCtx.parentSpanOffset,
          visibleAncestor,
        );
      }
    }
    return;
  }

  const subgraphHere = shouldSubgraph(scope);
  let bodyContainer: Container = container;
  let bodySubgraph: VisualSubgraph | null = null;
  if (subgraphHere) {
    const sg = describeSubgraph(scope, ctx.subgraphOwnerVar, ctx.variableMap);
    container.elements.push(sg);
    bodyContainer = sg;
    bodySubgraph = sg;
    state.subgraphByScope.set(scope.id, sg);
    const ownerVar = ctx.subgraphOwnerVar.get(scope.id);
    if (ownerVar) {
      state.functionSubgraphByFn.set(ownerVar, sg);
    }
  }
  for (const vid of scope.variables) {
    const v = ctx.variableMap.get(vid);
    if (!v) {
      continue;
    }
    // Implicit bindings such as `arguments` (FunctionDeclarationInstantiation,
    // ES spec 9.2.13) carry no source-level identifier or definition; they
    // exist only to satisfy resolution for in-source references. Surfacing
    // them as data-flow nodes would add line-0 phantoms with no incident
    // edges, so skip them here while keeping them in the IR for parity.
    if (v.defs.length === 0 && v.identifiers.length === 0) {
      continue;
    }
    const node = makeVariableNode(v);
    state.nodeIdOriginScope?.set(node.id, scope.id);
    bodyContainer.elements.push(node);
  }
  const ops = ctx.writeOpsByScope.get(scope.id) ?? [];
  for (const op of ops) {
    const ownerVar = ctx.variableMap.get(op.varId);
    const ownerDef = ownerVar?.defs[0];
    const declarationKind =
      ownerDef?.type === DEFINITION_TYPE.Variable
        ? ownerDef.declarationKind
        : null;
    const node = {
      type: VISUAL_ELEMENT_TYPE.Node,
      id: writeOpNodeId(op.refId),
      kind: NODE_KIND.WriteOp,
      name: op.varName,
      line: op.line,
      endLine: null,
      isJsxElement: false,
      unused: false,
      declarationKind,
    } satisfies VisualNode;
    state.nodeIdOriginScope?.set(node.id, scope.id);
    bodyContainer.elements.push(node);
  }
  buildChildren(scope, bodyContainer, ctx, state);
  if (bodySubgraph !== null) {
    attachLoopTestAnchor(scope, bodySubgraph, state);
    attachSwitchDiscriminantAnchor(scope, bodySubgraph, state);
  }
}
