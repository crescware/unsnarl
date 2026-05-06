import type { NormalizedDefinition } from "../normalized/normalized-definition.js";
import { formatRange } from "./format-range.js";
import type { Mismatch } from "./mismatch.js";

export function diffDefinition(
  scopePath: readonly number[],
  varName: string,
  index: number,
  unsnarl: NormalizedDefinition,
  baseline: NormalizedDefinition,
): Mismatch[] {
  const result: Mismatch[] = [];
  if (unsnarl.type !== baseline.type) {
    result.push({
      kind: "definition-type-mismatch",
      scopePath,
      message: `'${varName}'.defs[${index}]: unsnarl=${unsnarl.type} baseline=${baseline.type}`,
    });
  }
  if (
    unsnarl.nodeRange[0] !== baseline.nodeRange[0] ||
    unsnarl.nodeRange[1] !== baseline.nodeRange[1]
  ) {
    result.push({
      kind: "definition-node-range-mismatch",
      scopePath,
      message: `'${varName}'.defs[${index}]: unsnarl=${formatRange(unsnarl.nodeRange)} baseline=${formatRange(baseline.nodeRange)}`,
    });
  }
  return result;
}
