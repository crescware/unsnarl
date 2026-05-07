import type { Category } from "../../serializer/category.js";
import type { BlockContext } from "../scope/block-context.js";

export type CategoryDepths = Readonly<Record<Category, number>>;

export type ScopeAnnotation = Readonly<{
  blockContext: BlockContext | null;
  fallsThrough: boolean;
  exitsFunction: boolean;
  categoryDepths: CategoryDepths;
}>;
