import {
  array,
  boolean,
  custom,
  nullable,
  object,
  string,
  type InferOutput,
} from "valibot";

import type { ScopeType } from "../../analyzer/scope-type.js";
import type { NestingDepths } from "../annotations/scope-annotation.js";
import type { Span } from "../primitive/span.js";
import type { BlockContext } from "../scope/block-context.js";
import { referenceId$ } from "./reference-id.js";
import { scopeId$ } from "./scope-id.js";
import { variableId$ } from "./variable-id.js";

const scopeType$ = custom<ScopeType>(() => true);
const span$ = custom<Span>(() => true);
const blockContext$ = custom<BlockContext>(() => true);
const nestingDepths$ = custom<NestingDepths>(() => true);

export const serializedScope$ = object({
  id: scopeId$,
  type: scopeType$,
  isStrict: boolean(),
  upper: nullable(scopeId$),
  childScopes: array(scopeId$),
  variableScope: scopeId$,
  block: object({ type: string(), span: span$, endSpan: span$ }),
  variables: array(variableId$),
  references: array(referenceId$),
  through: array(referenceId$),
  functionExpressionScope: boolean(),
  blockContext: nullable(blockContext$),
  fallsThrough: boolean(),
  exitsFunction: boolean(),
  nestingDepths: nestingDepths$,
});

export type SerializedScope = InferOutput<typeof serializedScope$>;
