import { DEFINITION_TYPE } from "../../../analyzer/definition-type.js";
import type { SerializedDefinition } from "../../../ir/model.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { span } from "./span.js";

export function makeDef(
  overrides: Partial<SerializedDefinition> = {},
): SerializedDefinition {
  return {
    type: DEFINITION_TYPE.Variable,
    name: { name: "x", span: span() },
    node: { type: AST_TYPE.Identifier, span: span() },
    parent: null,
    initType: null,
    initSpan: null,
    importKind: null,
    importSource: null,
    importedName: null,
    declarationKind: null,
    ...overrides,
  };
}
