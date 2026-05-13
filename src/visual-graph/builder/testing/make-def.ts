import { DEFINITION_TYPE } from "../../../analyzer/definition-type.js";
import type { SerializedDefinition } from "../../../ir/serialized/serialized-definition.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import type { VariableDeclarationKind } from "../../../serializer/variable-declaration-kind.js";
import { asFilledString } from "../../../util/filled-string.js";
import { span } from "./span.js";

const COMMON = {
  name: { name: asFilledString("x"), span: span() },
  node: { type: AST_TYPE.Identifier, span: span() },
  parent: null,
} as const;

export function baseDef(
  declarationKind: VariableDeclarationKind,
): Extract<SerializedDefinition, { type: typeof DEFINITION_TYPE.Variable }> {
  return {
    ...COMMON,
    type: DEFINITION_TYPE.Variable,
    init: null,
    declarationKind,
  };
}

type SimpleDefType =
  | typeof DEFINITION_TYPE.FunctionName
  | typeof DEFINITION_TYPE.ClassName
  | typeof DEFINITION_TYPE.Parameter
  | typeof DEFINITION_TYPE.CatchClause
  | typeof DEFINITION_TYPE.ImplicitGlobalVariable;

export function baseSimpleDef(
  type: SimpleDefType,
): Extract<SerializedDefinition, { type: SimpleDefType }> {
  return { ...COMMON, type };
}
