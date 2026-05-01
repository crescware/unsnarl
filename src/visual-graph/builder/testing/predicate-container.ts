import type { PredicateContainer } from "../../../ir/model.js";

export function predicateContainer(
  type: PredicateContainer["type"],
  offset: number,
): PredicateContainer {
  return { type, offset };
}
