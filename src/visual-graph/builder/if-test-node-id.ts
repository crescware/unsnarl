import { sanitize } from "./sanitize.js";

export function ifTestNodeId(parentScopeId: string, offset: number): string {
  return `if_test_${sanitize(parentScopeId)}_${offset}`;
}
