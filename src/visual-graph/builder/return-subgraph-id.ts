import { sanitize } from "./sanitize.js";

export function returnSubgraphId(varId: string, containerKey: string): string {
  return `s_return_${sanitize(varId)}_${sanitize(containerKey)}`;
}
