import type { NormalizedVariable } from "../normalized/normalized-variable.js";
import { diffVariable } from "./diff-variable.js";
import type { Mismatch } from "./mismatch.js";

export function diffVariables(
  scopePath: readonly number[],
  unsnarl: readonly NormalizedVariable[],
  baseline: readonly NormalizedVariable[],
): Mismatch[] {
  const result: Mismatch[] = [];
  const uMap = new Map(unsnarl.map((v) => [v.name, v]));
  const bMap = new Map(baseline.map((v) => [v.name, v]));
  for (const [name, uv] of uMap) {
    const bv = bMap.get(name);
    if (!bv) {
      result.push({
        kind: "variable-extra",
        scopePath,
        message: `unsnarl has variable '${name}' which baseline lacks`,
      });
      continue;
    }
    result.push(...diffVariable(scopePath, name, uv, bv));
  }
  for (const name of bMap.keys()) {
    if (!uMap.has(name)) {
      result.push({
        kind: "variable-missing",
        scopePath,
        message: `baseline has variable '${name}' which unsnarl lacks`,
      });
    }
  }
  return result;
}
