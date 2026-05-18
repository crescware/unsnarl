import type { NormalizedReference } from "../normalized/normalized-reference.js";
import { formatRange } from "./format-range.js";
import { formatResolution } from "./format-resolution.js";
import type { Mismatch } from "./mismatch.js";

export function diffReference(
  scopePath: readonly number[],
  unsnarl: NormalizedReference,
  baseline: NormalizedReference,
): Mismatch[] {
  const result: Mismatch[] = [];
  if (
    unsnarl.flags.read !== baseline.flags.read ||
    unsnarl.flags.write !== baseline.flags.write
  ) {
    result.push({
      kind: "reference-flag-mismatch",
      scopePath,
      message: `ref ${formatRange(unsnarl.identifierRange)}: unsnarl r=${unsnarl.flags.read}/w=${unsnarl.flags.write} baseline r=${baseline.flags.read}/w=${baseline.flags.write}`,
    });
  }
  if (unsnarl.init !== baseline.init) {
    result.push({
      kind: "reference-init-mismatch",
      scopePath,
      message: `ref ${formatRange(unsnarl.identifierRange)}: unsnarl init=${unsnarl.init} baseline init=${baseline.init}`,
    });
  }
  const uRes = unsnarl.resolved;
  const bRes = baseline.resolved;
  const sameResolution =
    (uRes === null && bRes === null) ||
    (uRes !== null &&
      bRes !== null &&
      uRes.varName === bRes.varName &&
      uRes.scopePath.length === bRes.scopePath.length &&
      uRes.scopePath.every((v, i) => v === bRes.scopePath[i]));
  if (!sameResolution) {
    result.push({
      kind: "reference-resolution-mismatch",
      scopePath,
      message: `ref ${formatRange(unsnarl.identifierRange)}: unsnarl=${formatResolution(uRes)} baseline=${formatResolution(bRes)}`,
    });
  }
  return result;
}
