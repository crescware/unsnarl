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
export type ScopeType = (typeof SCOPE_TYPE)[keyof typeof SCOPE_TYPE];
