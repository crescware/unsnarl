import type { InferOutput } from "valibot";

import type { return$, throw$ } from "../completion/completion-type.js";
import type { AbruptCompletion, Completion } from "../completion/completion.js";

/**
 * The subset of Completion that a Reference's value can flow into.
 *
 * `break` / `continue` take no Expression argument syntactically
 * (only an optional label Identifier), and eslint-scope classifies
 * labels as `Label`, not `Reference`. A Reference's value therefore
 * cannot flow into a break / continue completion; the IR enforces
 * this by narrowing through `Extract` here.
 *
 * Extend the union when JS grammar adds a new value-carrying abrupt
 * completion.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export type ReferenceCompletion =
  | Exclude<Completion, AbruptCompletion>
  | Extract<
      AbruptCompletion,
      { type: InferOutput<typeof return$> | InferOutput<typeof throw$> }
    >;
