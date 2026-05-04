import type { PredicateContainerType } from "../../analyzer/predicate-container-type.js";

export type PredicateContainer = Readonly<{
  type: PredicateContainerType;
  offset: number;
}>;
