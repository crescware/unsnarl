import type { VisualNode } from "../../visual-graph/visual-node.js";
import { nodeSyntax } from "./node-syntax.js";
import type { RenderState } from "./render-state.js";

export function emitNode(
  state: RenderState,
  node: VisualNode,
  indent: string,
): void {
  state.lines.push(`${indent}${node.id}${nodeSyntax(node)}`);
}
