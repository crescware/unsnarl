import {
  literal,
  nullable,
  object,
  pipe,
  readonly,
  union,
  type InferOutput,
} from "valibot";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { astType$ } from "../../parser/ast-type.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { variableDeclarationKind$ } from "../../serializer/variable-declaration-kind.js";
import { filledString$ } from "../../util/filled-string.js";
import { span$ } from "../primitive/span.js";

const definitionName$ = object({ name: filledString$, span: span$ });
const definitionNode$ = object({ type: astType$, span: span$ });
const definitionParent$ = nullable(definitionNode$);

const commonDefFields = {
  name: definitionName$,
  node: definitionNode$,
  parent: definitionParent$,
} as const;

// SerializedDefinition is a 9-way union: 7 def types (Variable plus six
// no-extra-fields kinds) and 3 ImportBinding sub-shapes keyed by importKind.
// variant() on `type` alone can't discriminate the three ImportBinding shapes,
// so the whole set is expressed as a flat union -- parse() tries each schema
// in turn until one matches the concrete shape of the input.
export const serializedDefinition$ = union([
  pipe(
    object({
      ...commonDefFields,
      type: literal(DEFINITION_TYPE.Variable),
      init: nullable(object({ type: astType$, span: span$ })),
      declarationKind: variableDeclarationKind$,
    }),
    readonly(),
  ),
  pipe(
    object({
      ...commonDefFields,
      type: literal(DEFINITION_TYPE.ImportBinding),
      importKind: literal(IMPORT_KIND.Named),
      importedName: filledString$,
      importSource: filledString$,
    }),
    readonly(),
  ),
  pipe(
    object({
      ...commonDefFields,
      type: literal(DEFINITION_TYPE.ImportBinding),
      importKind: literal(IMPORT_KIND.Default),
      importSource: filledString$,
    }),
    readonly(),
  ),
  pipe(
    object({
      ...commonDefFields,
      type: literal(DEFINITION_TYPE.ImportBinding),
      importKind: literal(IMPORT_KIND.Namespace),
      importSource: filledString$,
    }),
    readonly(),
  ),
  pipe(
    object({ ...commonDefFields, type: literal(DEFINITION_TYPE.FunctionName) }),
    readonly(),
  ),
  pipe(
    object({ ...commonDefFields, type: literal(DEFINITION_TYPE.ClassName) }),
    readonly(),
  ),
  pipe(
    object({ ...commonDefFields, type: literal(DEFINITION_TYPE.Parameter) }),
    readonly(),
  ),
  pipe(
    object({ ...commonDefFields, type: literal(DEFINITION_TYPE.CatchClause) }),
    readonly(),
  ),
  pipe(
    object({
      ...commonDefFields,
      type: literal(DEFINITION_TYPE.ImplicitGlobalVariable),
    }),
    readonly(),
  ),
]);

export type SerializedDefinition = InferOutput<typeof serializedDefinition$>;
