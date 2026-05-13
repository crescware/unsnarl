import {
  array,
  boolean,
  nullable,
  object,
  pipe,
  readonly,
  string,
  type InferOutput,
} from "valibot";

import { scopeType$ } from "../../analyzer/scope-type.js";
import { nestingDepths$ } from "../annotations/scope-annotation.js";
import { span$ } from "../primitive/span.js";
import { blockContext$ } from "../scope/block-context.js";
import { referenceId$ } from "./reference-id.js";
import { scopeId$ } from "./scope-id.js";
import { variableId$ } from "./variable-id.js";

export const serializedScope$ = object({
  id: scopeId$,
  type: scopeType$,
  isStrict: boolean(),
  upper: nullable(scopeId$),
  childScopes: pipe(array(scopeId$), readonly()),
  variableScope: scopeId$,
  block: pipe(
    object({ type: string(), span: span$, endSpan: span$ }),
    readonly(),
  ),
  variables: pipe(array(variableId$), readonly()),
  references: pipe(array(referenceId$), readonly()),
  through: pipe(array(referenceId$), readonly()),
  functionExpressionScope: boolean(),
  blockContext: nullable(blockContext$),
  fallsThrough: boolean(),
  exitsFunction: boolean(),
  nestingDepths: nestingDepths$,
});

export type SerializedScope = InferOutput<typeof serializedScope$>;
