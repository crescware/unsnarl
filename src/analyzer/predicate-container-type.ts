import { picklist } from "valibot";

export const PREDICATE_CONTAINER_TYPE = {
  IfStatement: "IfStatement",
  SwitchStatement: "SwitchStatement",
  WhileStatement: "WhileStatement",
  DoWhileStatement: "DoWhileStatement",
  ForStatement: "ForStatement",
  ForOfStatement: "ForOfStatement",
  ForInStatement: "ForInStatement",
} as const;

export const predicateContainerType$ = picklist([
  PREDICATE_CONTAINER_TYPE.IfStatement,
  PREDICATE_CONTAINER_TYPE.SwitchStatement,
  PREDICATE_CONTAINER_TYPE.WhileStatement,
  PREDICATE_CONTAINER_TYPE.DoWhileStatement,
  PREDICATE_CONTAINER_TYPE.ForStatement,
  PREDICATE_CONTAINER_TYPE.ForOfStatement,
  PREDICATE_CONTAINER_TYPE.ForInStatement,
]);
