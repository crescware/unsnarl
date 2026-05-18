import { sanitize } from "./sanitize.js";

export function writeOpNodeId(refId: string): string {
  return `wr_${sanitize(refId)}`;
}
