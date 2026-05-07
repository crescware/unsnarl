import type { NestingDepths } from "../../ir/annotations/scope-annotation.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { nestingKindOf } from "./nesting-kind-of.js";

export function isCollapsed(
  scope: SerializedScope,
  depths: NestingDepths | undefined,
): boolean {
  if (!depths) {
    return false;
  }
  const cat = nestingKindOf(scope);
  if (cat === null) {
    return false;
  }
  return scope.nestingDepths[cat] > depths[cat];
}
