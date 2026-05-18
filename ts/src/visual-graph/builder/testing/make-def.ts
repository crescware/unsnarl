import { parse } from "valibot";

import { DEFINITION_TYPE } from "../../../analyzer/definition-type.js";
import {
  serializedDefinition$,
  type SerializedDefinition,
} from "../../../ir/serialized/serialized-definition.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import type { VariableDeclarationKind } from "../../../serializer/variable-declaration-kind.js";
import { span } from "./span.js";

const COMMON = {
  name: { name: "x", span: span() },
  node: { type: AST_TYPE.Identifier, span: span() },
  parent: null,
} as const;

export function baseDef(
  declarationKind: VariableDeclarationKind,
): Extract<SerializedDefinition, { type: typeof DEFINITION_TYPE.Variable }> {
  return parse(serializedDefinition$, {
    ...COMMON,
    type: DEFINITION_TYPE.Variable,
    init: null,
    declarationKind,
  }) as Extract<
    SerializedDefinition,
    { type: typeof DEFINITION_TYPE.Variable }
  >;
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
  return parse(serializedDefinition$, { ...COMMON, type }) as Extract<
    SerializedDefinition,
    { type: SimpleDefType }
  >;
}
