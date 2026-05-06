import type { Range } from "../normalized/range.js";

export function rangeOf(
  node: { start?: number; end?: number } | null | undefined,
): Range {
  return [node?.start ?? 0, node?.end ?? 0];
}
