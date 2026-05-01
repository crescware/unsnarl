import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { SerializedVariable } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { VisualNode } from "../model.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
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
  const isImportBinding = def?.type === DEFINITION_TYPE.ImportBinding;
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: nodeId(v.id),
    kind: (def?.type ?? DEFINITION_TYPE.Variable) as VisualNode["kind"],
    name: v.name,
    line: v.identifiers[0]?.line ?? def?.name.span.line ?? 0,
    endLine: null,
    isJsxElement: false,
    unused: false,
    declarationKind,
    initIsFunction: isFunctionInit,
    importKind,
    importedName: isImportBinding ? importedName : null,
    importSource: isImportBinding ? importSource : null,
  };
}
