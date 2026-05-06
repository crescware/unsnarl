import type { NormalizedVariable } from "../normalized/normalized-variable.js";
import { diffDefinition } from "./diff-definition.js";
import type { Mismatch } from "./mismatch.js";

export function diffVariable(
  scopePath: readonly number[],
  name: string,
  unsnarl: NormalizedVariable,
  baseline: NormalizedVariable,
): Mismatch[] {
  if (unsnarl.defs.length !== baseline.defs.length) {
    return [
      {
        kind: "definition-count-mismatch",
        scopePath,
        message: `'${name}': unsnarl=${unsnarl.defs.length} baseline=${baseline.defs.length}`,
      },
    ];
  }
  const result: Mismatch[] = [];
  unsnarl.defs.forEach((ud, i) => {
    const bd = baseline.defs[i];
    if (!bd) {
      return;
    }
    result.push(...diffDefinition(scopePath, name, i, ud, bd));
  });
  return result;
}
