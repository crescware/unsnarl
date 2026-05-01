import { sanitize } from "./sanitize.js";

export function retUseNodeId(refId: string): string {
  return `ret_use_${sanitize(refId)}`;
}
