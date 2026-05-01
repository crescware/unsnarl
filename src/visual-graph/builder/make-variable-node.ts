import {
  AST_TYPE,
  DEFINITION_TYPE,
  VISUAL_ELEMENT_TYPE,
} from "../../constants.js";
import type { SerializedVariable } from "../../ir/model.js";
import type { VisualNode } from "../model.js";
import { nodeId } from "./node-id.js";

export function makeVariableNode(v: SerializedVariable): VisualNode {
  const def = v.defs[0];
  const initType = def?.initType ?? null;
  const isFunctionInit =
    initType === AST_TYPE.ArrowFunctionExpression ||
    initType === AST_TYPE.FunctionExpression;
  const declarationKind = def?.declarationKind ?? null;
  const importKind = def?.importKind ?? null;
  const importedName = def?.importedName ?? null;
  const importSource = def?.importSource ?? null;
  const node = {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: nodeId(v.id),
    kind: (def?.type ?? DEFINITION_TYPE.Variable) as VisualNode["kind"],
    name: v.name,
    line: v.identifiers[0]?.line ?? def?.name.span.line ?? 0,
    isJsxElement: false,
  } satisfies VisualNode as VisualNode;
  if (declarationKind) {
    node.declarationKind = declarationKind;
  }
  if (isFunctionInit) {
    node.initIsFunction = true;
  }
  if (importKind) {
    node.importKind = importKind;
  }
  if (def?.type === DEFINITION_TYPE.ImportBinding) {
    node.importedName = importedName;
    if (importSource) {
      node.importSource = importSource;
    }
  }
  return node;
}
