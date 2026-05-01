import type { NodeKind } from "../model.js";

// Every visible node that carries a meaningful source line is eligible as a
// root, including "use" nodes (ReturnUse for JSX/ownerless reads inside a
// return, WriteOp for assignments). `-r N` should pin the root at whatever
// is actually at line N; surrounding declarations are reached via the
// ancestors BFS, not auto-attached as a separate root.
export const ROOT_CANDIDATE_KINDS: ReadonlySet<NodeKind> = new Set<NodeKind>([
  "Variable",
  "FunctionName",
  "ClassName",
  "Parameter",
  "CatchClause",
  "ImportBinding",
  "ImplicitGlobalVariable",
  "ReturnUse",
  "WriteOp",
]);
