import type { NodeKind } from "../node-kind.js";
import { NODE_KIND } from "../node-kind.js";

// Every visible node that carries a meaningful source line is eligible as a
// root, including "use" nodes (ReturnUse for JSX/ownerless reads inside a
// return, WriteOp for assignments). `-r N` should pin the root at whatever
// is actually at line N; surrounding declarations are reached via the
// ancestors BFS, not auto-attached as a separate root.
export const ROOT_CANDIDATE_KINDS: ReadonlySet<NodeKind> = new Set<NodeKind>([
  NODE_KIND.VarBinding,
  NODE_KIND.ConstBinding,
  NODE_KIND.LetBinding,
  NODE_KIND.FunctionDeclaration,
  NODE_KIND.ClassDeclaration,
  NODE_KIND.FormalParameter,
  NODE_KIND.CatchParameter,
  NODE_KIND.NamedImportBinding,
  NODE_KIND.DefaultImportBinding,
  NODE_KIND.NamespaceImportBinding,
  NODE_KIND.SyntheticImplicitGlobal,
  NODE_KIND.ReturnArgumentReference,
  NODE_KIND.WriteReference,
  NODE_KIND.SyntheticExpressionStatement,
]);
