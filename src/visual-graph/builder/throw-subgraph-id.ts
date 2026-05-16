import { sanitize } from "./sanitize.js";

export function throwSubgraphId(varId: string, containerKey: string): string {
  return `s_throw_${sanitize(varId)}_${sanitize(containerKey)}`;
}
