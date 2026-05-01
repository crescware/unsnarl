import { DEFINITION_TYPE } from "../../../analyzer/definition-type.js";
import type { SerializedDefinition } from "../../../ir/model.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { span } from "./span.js";

const COMMON = {
  name: { name: "x", span: span() },
  node: { type: AST_TYPE.Identifier, span: span() },
  parent: null,
} as const;

export function baseDef(): Extract<
  SerializedDefinition,
  { type: typeof DEFINITION_TYPE.Variable }
> {
  return {
    ...COMMON,
    type: DEFINITION_TYPE.Variable,
    init: null,
    declarationKind: null,
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
