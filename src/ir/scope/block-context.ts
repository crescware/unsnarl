import {
  literal,
  nullable,
  number,
  object,
  pipe,
  readonly,
  string,
  variant,
  type InferOutput,
} from "valibot";

// caseTest is only meaningful when this block is a switch-case clause.
// Other contexts (if/else, try/catch/finally, for body, etc.) carry no
// kind-specific payload, so the `case-clause` variant is the only one
// that adds a field. ifChainRootOffset is set on if-consequent / if-alternate
// blocks that participate in an `else if` chain; it points to the start of
// the outermost IfStatement so all branches in the chain share a merge key.
export const blockContext$ = variant("kind", [
  pipe(
    object({
      kind: literal("case-clause"),
      parentType: string(),
      key: string(),
      parentSpanOffset: number(),
      caseTest: nullable(string()),
    }),
    readonly(),
  ),
  pipe(
    object({
      kind: literal("other"),
      parentType: string(),
      key: string(),
      parentSpanOffset: number(),
      ifChainRootOffset: nullable(number()),
    }),
    readonly(),
  ),
]);

export type BlockContext = InferOutput<typeof blockContext$>;
