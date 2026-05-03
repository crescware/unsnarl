import type { NodeKind } from "../model.js";
import { NODE_KIND } from "../node-kind.js";

// Every visible node that carries a meaningful source line is eligible as a
// root, including "use" nodes (ReturnUse for JSX/ownerless reads inside a
// return, WriteOp for assignments). `-r N` should pin the root at whatever
// is actually at line N; surrounding declarations are reached via the
// ancestors BFS, not auto-attached as a separate root.
export const ROOT_CANDIDATE_KINDS: ReadonlySet<NodeKind> = new Set<NodeKind>([
  NODE_KIND.Variable,
  NODE_KIND.FunctionName,
  NODE_KIND.ClassName,
  NODE_KIND.Parameter,
  NODE_KIND.CatchClause,
  NODE_KIND.ImportBinding,
  NODE_KIND.ImplicitGlobalVariable,
  NODE_KIND.ReturnUse,
  NODE_KIND.WriteOp,
  NODE_KIND.ExpressionStatement,
]);
