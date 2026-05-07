import type { NestingKind } from "../../serializer/nesting-kind.js";
import type { BlockContext } from "../scope/block-context.js";

export type NestingDepths = Readonly<Record<NestingKind, number>>;

export type ScopeAnnotation = Readonly<{
  blockContext: BlockContext | null;
  fallsThrough: boolean;
  exitsFunction: boolean;
  nestingDepths: NestingDepths;
}>;
