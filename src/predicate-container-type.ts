export const PREDICATE_CONTAINER_TYPE = {
  IfStatement: "IfStatement",
  SwitchStatement: "SwitchStatement",
} as const;
export type PredicateContainerType =
  (typeof PREDICATE_CONTAINER_TYPE)[keyof typeof PREDICATE_CONTAINER_TYPE];
