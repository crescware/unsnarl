import type { CategoryDepths } from "../../ir/annotations/scope-annotation.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { categoryOf } from "./category-of.js";

export function isCollapsed(
  scope: SerializedScope,
  depths: CategoryDepths | undefined,
): boolean {
  if (!depths) {
    return false;
  }
  const cat = categoryOf(scope);
  if (cat === null) {
    return false;
  }
  return scope.categoryDepths[cat] > depths[cat];
}
