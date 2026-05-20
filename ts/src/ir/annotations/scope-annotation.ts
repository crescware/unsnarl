import { number, object, pipe, readonly, type InferOutput } from "valibot";

import { NESTING_KIND } from "../../serializer/nesting-kind.js";
import type { BlockContext } from "../scope/block-context.js";

export const nestingDepths$ = pipe(
  object({
    [NESTING_KIND.Function]: number(),
    [NESTING_KIND.If]: number(),
    [NESTING_KIND.For]: number(),
    [NESTING_KIND.While]: number(),
    [NESTING_KIND.Switch]: number(),
    [NESTING_KIND.TryCatchFinally]: number(),
    [NESTING_KIND.Block]: number(),
  }),
  readonly(),
);

export type NestingDepths = InferOutput<typeof nestingDepths$>;

export type ScopeAnnotation = Readonly<{
  blockContext: BlockContext | null;
  fallsThrough: boolean;
  exitsFunction: boolean;
  nestingDepths: NestingDepths;
}>;
