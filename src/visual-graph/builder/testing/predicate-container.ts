import type { PredicateContainer } from "../../../ir/reference/predicate-container.js";

export function predicateContainer(
  type: PredicateContainer["type"],
  offset: number,
): PredicateContainer {
  return { type, offset };
}
