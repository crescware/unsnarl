import { sanitize } from "./sanitize.js";

export function ifContainerSubgraphId(
  parentScopeId: string,
  offset: number,
): string {
  return `cont_if_${sanitize(parentScopeId)}_${offset}`;
}
