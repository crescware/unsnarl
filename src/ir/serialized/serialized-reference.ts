import {
  array,
  boolean,
  literal,
  nullable,
  object,
  pipe,
  readonly,
  variant,
  type InferOutput,
} from "valibot";

import { filledString$ } from "../../util/filled-string.js";
import { span$ } from "../primitive/span.js";
import { predicateContainer$ } from "../reference/predicate-container.js";
import { referenceId$ } from "./reference-id.js";
import { scopeId$ } from "./scope-id.js";
import { serializedHeadExpression$ } from "./serialized-expression-statement-head.js";
import { variableId$ } from "./variable-id.js";

/**
 * Reference の値がどの ECMAScript Completion 種別の [[Value]] として
 * 産出されるかを表す serialized (span-based) スキーマ。詳細は
 * `src/ir/reference/completion.ts` のドキュメントを参照。
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
const serializedCompletion$ = variant("kind", [
  pipe(object({ kind: literal("normal") }), readonly()),
  pipe(
    object({ kind: literal("return"), startSpan: span$, endSpan: span$ }),
    readonly(),
  ),
  pipe(
    object({ kind: literal("throw"), startSpan: span$, endSpan: span$ }),
    readonly(),
  ),
]);

export const serializedReference$ = object({
  id: referenceId$,
  identifier: pipe(object({ name: filledString$, span: span$ }), readonly()),
  from: scopeId$,
  resolved: nullable(variableId$),
  owners: pipe(array(variableId$), readonly()),
  init: boolean(),
  flags: pipe(
    object({
      read: boolean(),
      write: boolean(),
      call: boolean(),
      receiver: boolean(),
    }),
    readonly(),
  ),
  predicateContainer: nullable(predicateContainer$),
  completion: serializedCompletion$,
  jsxElement: nullable(
    pipe(object({ startSpan: span$, endSpan: span$ }), readonly()),
  ),
  expressionStatementContainer: nullable(
    pipe(
      object({
        startSpan: span$,
        endSpan: span$,
        head: serializedHeadExpression$,
      }),
      readonly(),
    ),
  ),
});

export type SerializedReference = InferOutput<typeof serializedReference$>;
