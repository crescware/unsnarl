import type { Range } from "./range.js";

export function compareRange(a: Range, b: Range): number {
  if (a[0] !== b[0]) {
    return a[0] - b[0];
  }
  return a[1] - b[1];
}
