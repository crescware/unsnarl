import type { Range } from "../normalized/range.js";

export function formatRange(r: Range): string {
  return `[${r[0]},${r[1]}]`;
}
