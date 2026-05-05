export const PREDICATE_CONTAINER_TYPE = {
  IfStatement: "IfStatement",
  SwitchStatement: "SwitchStatement",
  WhileStatement: "WhileStatement",
  DoWhileStatement: "DoWhileStatement",
  ForStatement: "ForStatement",
  ForOfStatement: "ForOfStatement",
  ForInStatement: "ForInStatement",
} as const;
export type PredicateContainerType =
  (typeof PREDICATE_CONTAINER_TYPE)[keyof typeof PREDICATE_CONTAINER_TYPE];
