import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import { nodeHead } from "./node-head.js";

export function nodeLabel(n: VisualNode, debug: boolean): string {
  const base = baseLabel(n);
  return debug ? `${base}<br/>${n.kind}` : base;
}

function baseLabel(n: VisualNode): string {
  if (n.kind === NODE_KIND.LegacyIfTest) {
    return `if ()<br/>L${n.line}`;
  }
  if (n.kind === NODE_KIND.LegacySwitchDiscriminant) {
    return `switch ()<br/>L${n.line}`;
  }
  if (n.kind === NODE_KIND.LegacyWhileTest) {
    return `while ()<br/>L${n.line}`;
  }
  if (n.kind === NODE_KIND.LegacyDoWhileTest) {
    return `do while ()<br/>L${n.line}`;
  }
  if (n.kind === NODE_KIND.LegacyForTest) {
    return `for ()<br/>L${n.line}`;
  }
  if (n.kind === NODE_KIND.SyntheticBeyondDepth) {
    // The stub stands in for an arbitrary range of source lines that
    // collapsed past the queried depth; printing a single line number
    // here would be misleading, and printing the full range would
    // duplicate the surrounding subgraph's L<x>-<y> label.
    return nodeHead(n);
  }
  const head = nodeHead(n);
  if (n.kind === NODE_KIND.LegacyModuleSink) {
    return "module";
  }
  if (n.kind === NODE_KIND.LegacyImplicitGlobalVariable) {
    return head;
  }
  // Unused declarations are surfaced via a textual prefix instead of a
  // dashed border. This keeps the visual cue legible even when the node
  // already has another classDef applied (boundary stub, nest level, ...).
  const prefixed = n.unused ? `unused ${head}` : head;
  const range =
    n.endLine !== null && n.endLine !== n.line
      ? `L${n.line}-${n.endLine}`
      : `L${n.line}`;
  return `${prefixed}<br/>${range}`;
}
