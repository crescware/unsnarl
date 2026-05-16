import type { InferOutput } from "valibot";

import type { return$, throw$ } from "../completion/completion-type.js";
import type { AbruptCompletion, Completion } from "../completion/completion.js";

/**
 * The subset of Completion that a Reference's value can flow into.
 *
 * `break` / `continue` accept only an optional label Identifier
 * syntactically (no Expression argument), and eslint-scope
 * classifies a label as `Label`, not `Reference`. A Reference's
 * value therefore cannot flow into a break / continue completion.
 * The `Extract` narrowing makes the IR type system enforce that
 * invariant: any attempt to assign a break / continue
 * AbruptCompletion to a Reference annotation is a type error.
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
