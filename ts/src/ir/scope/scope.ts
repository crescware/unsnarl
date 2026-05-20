import type { ScopeType } from "../../analyzer/scope-type.js";
import type { AstNode } from "../primitive/ast-node.js";
import type { Reference } from "../reference/reference.js";
import type { Variable } from "./variable.js";

// Mutable: ScopeImpl pushes into childScopes / variables / references /
// through during the eslint-compat analyzer pass.
export type Scope = {
  type: ScopeType;
  isStrict: boolean;
  upper: Scope | null;
  childScopes: /* mutable */ Scope[];
  variableScope: Scope;
  block: AstNode;
  variables: /* mutable */ Variable[];
  set: Map<string, Variable>;
  references: /* mutable */ Reference[];
  through: /* mutable */ Reference[];
  functionExpressionScope: boolean;
};
