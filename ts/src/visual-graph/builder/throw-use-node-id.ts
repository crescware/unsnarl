import { sanitize } from "./sanitize.js";

export function throwUseNodeId(refId: string): string {
  return `throw_use_${sanitize(refId)}`;
}
