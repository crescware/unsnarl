import type { InferOutput } from "valibot";

import type { normal$, return$, throw$ } from "./completion-kind.js";

/**
 * Records which ECMAScript Completion category carries the value
 * produced by a Reference's read. Offset-based.
 *
 * ECMA §6.2.4 defines Completion Record [[Type]] as one of
 *   normal | break | continue | return | throw,
 * and: "A Completion Record whose [[Type]] is `normal` is called a
 * normal completion. Every Completion Record other than a normal
 * completion is also known as an abrupt completion."
 *
 * For each Reference, "into which completion does this read's value
 * flow?" has exactly one answer, so the type is a total union of
 * normal and abrupt; null is not used.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export type Completion =
  | AbruptCompletion
  | Readonly<{ kind: InferOutput<typeof normal$> }>;

/**
 * The value-carrying subset of the abrupt completions in
 * ECMA §6.2.4 (break | continue | return | throw).
 *
 * break / continue take no Expression argument syntactically (only an
 * optional label Identifier), and eslint-scope classifies labels as
 * `Label`, not `Reference`. A Reference's value therefore cannot flow
 * into a break / continue completion, so this union is restricted to
 * return / throw.
 *
 * Extend the union when JS grammar adds a new value-carrying abrupt
 * completion.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export type AbruptCompletion =
  | Readonly<{
      kind: InferOutput<typeof return$>;
      startOffset: number;
      endOffset: number;
    }>
  | Readonly<{
      kind: InferOutput<typeof throw$>;
      startOffset: number;
      endOffset: number;
    }>;
