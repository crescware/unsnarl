import { NODE_KIND } from "../../constants.js";
import type { NodeKind } from "../model.js";

// Use-site nodes (WriteOp, ReturnUse) are positional: they make sense as
// roots when the user pinpoints a line, but a bare `-r counter` should
// stay declaration-scoped so it does not light up every assignment and
// every JSX usage of `counter`.
export const NAME_QUERY_EXCLUDED: ReadonlySet<NodeKind> = new Set<NodeKind>([
  NODE_KIND.WriteOp,
  NODE_KIND.ReturnUse,
]);
