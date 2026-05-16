import type { InferOutput } from "valibot";

import type {
  break$,
  continue$,
  normal$,
  return$,
  throw$,
} from "./completion-type.js";

/**
 * Records which ECMAScript Completion category produced a statement
 * or expression result. Offset-based.
 *
 * ECMA §6.2.4 defines Completion Record [[Type]] as one of
 *   normal | break | continue | return | throw,
 * and: "A Completion Record whose [[Type]] is `normal` is called a
 * normal completion. Every Completion Record other than a normal
 * completion is also known as an abrupt completion."
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export type Completion =
  | AbruptCompletion
  | Readonly<{ type: InferOutput<typeof normal$> }>;

/**
 * The ECMA §6.2.4 abrupt completion records, mirrored to the IR for
 * all four `[[Type]]` values (return, throw, break, continue).
 *
 * Field mapping:
 *
 * - `[[Type]]` -> the `type` discriminator.
 * - `[[Target]]` -> the `target` field on `break` / `continue` (label
 *   identifier, or `null` if the statement has no label). Read from
 *   `BreakStatement.label?.name` / `ContinueStatement.label?.name`
 *   at construction time. `return` and `throw` carry no `[[Target]]`
 *   in the spec, so the field is absent on those variants.
 * - `[[Value]]` -> intentionally NOT mirrored. unsnarl carries value
 *   flow through the existence of a Reference in the IR; an
 *   additional `value` field on AbruptCompletion would duplicate
 *   that information.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export type AbruptCompletion =
  | Readonly<{
      type: InferOutput<typeof return$>;
      startOffset: number;
      endOffset: number;
    }>
  | Readonly<{
      type: InferOutput<typeof throw$>;
      startOffset: number;
      endOffset: number;
    }>
  | Readonly<{
      type: InferOutput<typeof break$>;
      target: string | null;
      startOffset: number;
      endOffset: number;
    }>
  | Readonly<{
      type: InferOutput<typeof continue$>;
      target: string | null;
      startOffset: number;
      endOffset: number;
    }>;
