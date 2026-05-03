export const NODE_KIND = {
  Variable: "Variable",
  FunctionName: "FunctionName",
  ClassName: "ClassName",
  Parameter: "Parameter",
  CatchClause: "CatchClause",
  ImportBinding: "ImportBinding",
  ImplicitGlobalVariable: "ImplicitGlobalVariable",
  WriteOp: "WriteOp",
  ReturnUse: "ReturnUse",
  IfTest: "IfTest",
  ModuleSink: "ModuleSink",
  ModuleSource: "ModuleSource",
  ImportIntermediate: "ImportIntermediate",
  ExpressionStatement: "ExpressionStatement",
} as const;
export type NodeKind = (typeof NODE_KIND)[keyof typeof NODE_KIND];
