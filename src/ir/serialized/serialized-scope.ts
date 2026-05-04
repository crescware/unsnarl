import type { ScopeType } from "../../analyzer/scope-type.js";
import type { Span } from "../primitive/span.js";
import type { BlockContext } from "../scope/block-context.js";
import type { ReferenceId, ScopeId, VariableId } from "./ids.js";

export type SerializedScope = Readonly<{
  id: ScopeId;
  type: ScopeType;
  isStrict: boolean;
  upper: ScopeId | null;
  childScopes: readonly ScopeId[];
  variableScope: ScopeId;
  block: Readonly<{ type: string; span: Span; endSpan: Span }>;
  variables: readonly VariableId[];
  references: readonly ReferenceId[];
  through: readonly ReferenceId[];
  functionExpressionScope: boolean;
  blockContext: BlockContext | null;
  fallsThrough: boolean;
  exitsFunction: boolean;
}>;
