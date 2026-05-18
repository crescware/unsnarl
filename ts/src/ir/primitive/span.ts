import { number, object, pipe, readonly, type InferOutput } from "valibot";

export const span$ = pipe(
  object({
    line: number(),
    column: number(),
    offset: number(),
  }),
  readonly(),
);

export type Span = InferOutput<typeof span$>;
