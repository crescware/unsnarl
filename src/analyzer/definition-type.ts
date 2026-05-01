export const DEFINITION_TYPE = {
  CatchClause: "CatchClause",
  ClassName: "ClassName",
  FunctionName: "FunctionName",
  ImplicitGlobalVariable: "ImplicitGlobalVariable",
  ImportBinding: "ImportBinding",
  Parameter: "Parameter",
  Variable: "Variable",
} as const;
export type DefinitionType =
  (typeof DEFINITION_TYPE)[keyof typeof DEFINITION_TYPE];
