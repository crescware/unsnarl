import type { VisualNode } from "../../visual-graph/model.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeHead } from "./node-head.js";

export function nodeLabel(n: VisualNode): string {
  if (n.kind === NODE_KIND.IfTest) {
    return `if L${n.line}`;
  }
  const head = nodeHead(n);
  if (n.kind === NODE_KIND.ModuleSink) {
    return "module";
  }
  // Unused declarations are surfaced via a textual prefix instead of a
  // dashed border. This keeps the visual cue legible even when the node
  // already has another classDef applied (boundary stub, fnWrap, ...).
  const prefixed = n.unused ? `unused ${head}` : head;
  const range =
    n.endLine !== null && n.endLine !== n.line
      ? `L${n.line}-${n.endLine}`
      : `L${n.line}`;
  return `${prefixed}<br/>${range}`;
}
