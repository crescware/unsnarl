import { sanitize } from "./sanitize.js";

export function nodeId(varId: string): string {
  return `n_${sanitize(varId)}`;
}
