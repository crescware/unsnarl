import type { NormalizedReference } from "../normalized/normalized-reference.js";
import { diffReference } from "./diff-reference.js";
import { formatRange } from "./format-range.js";
import type { Mismatch } from "./mismatch.js";

function keyOf(r: NormalizedReference): string {
  return `${r.identifierRange[0]},${r.identifierRange[1]}`;
}

export function diffReferences(
  scopePath: readonly number[],
  unsnarl: readonly NormalizedReference[],
  baseline: readonly NormalizedReference[],
): Mismatch[] {
  const result: Mismatch[] = [];
  const uMap = new Map(unsnarl.map((r) => [keyOf(r), r]));
  const bMap = new Map(baseline.map((r) => [keyOf(r), r]));
  for (const [k, ur] of uMap) {
    const br = bMap.get(k);
    if (!br) {
      result.push({
        kind: "reference-extra",
        scopePath,
        message: `unsnarl has reference at ${formatRange(ur.identifierRange)} which baseline lacks`,
      });
      continue;
    }
    result.push(...diffReference(scopePath, ur, br));
  }
  for (const [k, br] of bMap) {
    if (!uMap.has(k)) {
      result.push({
        kind: "reference-missing",
        scopePath,
        message: `baseline has reference at ${formatRange(br.identifierRange)} which unsnarl lacks`,
      });
    }
  }
  return result;
}
