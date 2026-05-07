import type { BlockContext } from "../scope/block-context.js";

export type ScopeAnnotation = Readonly<{
  blockContext: BlockContext | null;
  fallsThrough: boolean;
  exitsFunction: boolean;
}>;
