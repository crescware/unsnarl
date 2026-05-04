import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { VisualElement, VisualNode } from "../model.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { BuildState } from "./build-state.js";
import { expressionStatementNodeId } from "./expression-statement-node-id.js";

export function ensureExpressionStatementNode(
  ref: SerializedReference,
  raw: string,
  graphElements: /* mutable */ VisualElement[],
  state: BuildState,
): string | null {
  const c = ref.expressionStatementContainer;
  if (!c) {
    return null;
  }
  const offset = c.startSpan.offset;
  const existing = state.expressionStatementByOffset.get(offset);
  if (existing) {
    return existing;
  }
  const id = expressionStatementNodeId(offset);
  const head = raw.slice(c.headStartSpan.offset, c.headEndSpan.offset);
  const name = c.isCall ? `${head}()` : head;
  const startLine = c.startSpan.line;
  const endLine = c.endSpan.line !== startLine ? c.endSpan.line : null;
  const node = {
    type: VISUAL_ELEMENT_TYPE.Node,
    id,
    kind: NODE_KIND.ExpressionStatement,
    name,
    line: startLine,
    endLine,
    isJsxElement: false,
    unused: false,
  } satisfies VisualNode;
  graphElements.push(node);
  state.expressionStatementByOffset.set(offset, id);
  return id;
}
