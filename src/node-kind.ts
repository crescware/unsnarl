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
  ModuleSink: "ModuleSink",
  ModuleSource: "ModuleSource",
  ImportIntermediate: "ImportIntermediate",
} as const;
export type NodeKind = (typeof NODE_KIND)[keyof typeof NODE_KIND];
