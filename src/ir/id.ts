import type { ReferenceId, ScopeId, VariableId } from "./serialized/ids.js";

export function makeScopeId(index: number): ScopeId {
  return `scope#${index}`;
}

export function makeVariableId(
  scopeId: ScopeId,
  name: string,
  offset: number,
): VariableId {
  return `${scopeId}:${name}@${offset}`;
}

export function makeReferenceId(index: number): ReferenceId {
  return `ref#${index}`;
}
