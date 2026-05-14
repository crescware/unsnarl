import { picklist, type InferOutput } from "valibot";

export const SCOPE_TYPE = {
  Block: "block",
  Catch: "catch",
  Class: "class",
  ClassFieldInitializer: "class-field-initializer",
  ClassStaticBlock: "class-static-block",
  For: "for",
  Function: "function",
  FunctionExpressionName: "function-expression-name",
  Global: "global",
  Module: "module",
  Switch: "switch",
  With: "with",
} as const;

export const scopeType$ = picklist([
  SCOPE_TYPE.Block,
  SCOPE_TYPE.Catch,
  SCOPE_TYPE.Class,
  SCOPE_TYPE.ClassFieldInitializer,
  SCOPE_TYPE.ClassStaticBlock,
  SCOPE_TYPE.For,
  SCOPE_TYPE.Function,
  SCOPE_TYPE.FunctionExpressionName,
  SCOPE_TYPE.Global,
  SCOPE_TYPE.Module,
  SCOPE_TYPE.Switch,
  SCOPE_TYPE.With,
]);

export type ScopeType = InferOutput<typeof scopeType$>;
