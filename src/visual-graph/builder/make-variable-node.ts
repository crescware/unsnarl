import type { SerializedVariable } from "../../ir/model.js";
import type { VisualNode } from "../model.js";
import { nodeId } from "./node-id.js";

export function makeVariableNode(v: SerializedVariable): VisualNode {
  const def = v.defs[0];
  const initType = def?.initType ?? null;
  const isFunctionInit =
    initType === "ArrowFunctionExpression" || initType === "FunctionExpression";
  const declarationKind = def?.declarationKind ?? null;
  const importKind = def?.importKind ?? null;
  const importedName = def?.importedName ?? null;
  const importSource = def?.importSource ?? null;
  const node: VisualNode = {
    type: "node",
    id: nodeId(v.id),
    kind: (def?.type ?? "Variable") as VisualNode["kind"],
    name: v.name,
    line: v.identifiers[0]?.line ?? def?.name.span.line ?? 0,
    isJsxElement: false,
  };
  if (declarationKind) {
    node.declarationKind = declarationKind;
  }
  if (isFunctionInit) {
    node.initIsFunction = true;
  }
  if (importKind) {
    node.importKind = importKind;
  }
  if (def?.type === "ImportBinding") {
    node.importedName = importedName;
    if (importSource) {
      node.importSource = importSource;
    }
  }
  return node;
}
