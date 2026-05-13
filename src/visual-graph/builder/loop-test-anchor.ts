import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import {
  doWhileTestNodeId,
  forTestNodeId,
  whileTestNodeId,
} from "./loop-test-node-id.js";

export function attachLoopTestAnchor(
  scope: SerializedScope,
  sg: VisualSubgraph,
  state: BuildState,
): void {
  if (scope.type === SCOPE_TYPE.For) {
    const offset = scope.block.span.offset;
    if (state.forTestAnchorByOffset.has(offset)) {
      return;
    }
    const node = {
      type: VISUAL_ELEMENT_TYPE.Node,
      id: forTestNodeId(scope.upper ?? "", offset),
      kind: NODE_KIND.LegacyForTest,
      name: "for-test",
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
    state.forTestAnchorByOffset.set(offset, node.id);
    return;
  }
  if (scope.type !== SCOPE_TYPE.Block) {
    return;
  }
  const ctx = scope.blockContext;
  if (ctx === null || ctx.key !== "body") {
    return;
  }
  if (ctx.parentType === AST_TYPE.WhileStatement) {
    const offset = ctx.parentSpanOffset;
    if (state.whileTestAnchorByOffset.has(offset)) {
      return;
    }
    const node = {
      type: VISUAL_ELEMENT_TYPE.Node,
      id: whileTestNodeId(scope.upper ?? "", offset),
      kind: NODE_KIND.LegacyWhileTest,
      name: "while-test",
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
    state.whileTestAnchorByOffset.set(offset, node.id);
    return;
  }
  if (ctx.parentType === AST_TYPE.DoWhileStatement) {
    const offset = ctx.parentSpanOffset;
    if (state.doWhileTestAnchorByOffset.has(offset)) {
      return;
    }
    const node = {
      type: VISUAL_ELEMENT_TYPE.Node,
      id: doWhileTestNodeId(scope.upper ?? "", offset),
      kind: NODE_KIND.LegacyDoWhileTest,
      name: "do-while-test",
      line: scope.block.endSpan.line,
      endLine: null,
      isJsxElement: false,
      unused: false,
    } satisfies VisualNode;
    state.pendingLoopTestAnchors.push({
      subgraph: sg,
      node,
      position: "last",
    });
    state.doWhileTestAnchorByOffset.set(offset, node.id);
  }
}
