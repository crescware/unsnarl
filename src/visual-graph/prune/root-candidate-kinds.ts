import type { NodeKind } from "../node-kind.js";
import { NODE_KIND } from "../node-kind.js";

// Every visible node that carries a meaningful source line is eligible as a
// root, including "use" nodes (ReturnUse for JSX/ownerless reads inside a
// return, WriteOp for assignments). `-r N` should pin the root at whatever
// is actually at line N; surrounding declarations are reached via the
// ancestors BFS, not auto-attached as a separate root.
export const ROOT_CANDIDATE_KINDS: ReadonlySet<NodeKind> = new Set<NodeKind>([
  NODE_KIND.LegacyVariable,
  NODE_KIND.LegacyFunctionName,
  NODE_KIND.LegacyClassName,
  NODE_KIND.LegacyParameter,
  NODE_KIND.LegacyCatchClause,
  NODE_KIND.LegacyImportBinding,
  NODE_KIND.SyntheticImplicitGlobal,
  NODE_KIND.LegacyReturnUse,
  NODE_KIND.LegacyWriteOp,
  NODE_KIND.SyntheticExpressionStatement,
]);
