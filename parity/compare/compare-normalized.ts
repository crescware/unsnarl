import type { NormalizedScope } from "../normalized/normalized-scope.js";
import { diffReferences } from "./diff-references.js";
import { diffVariables } from "./diff-variables.js";
import { formatRange } from "./format-range.js";
import type { Mismatch } from "./mismatch.js";

export function compareNormalized(
  unsnarl: NormalizedScope,
  baseline: NormalizedScope,
): Mismatch[] {
  const result: Mismatch[] = [];
  walk(unsnarl, baseline);
  return result;

  function walk(u: NormalizedScope, b: NormalizedScope): void {
    if (u.type !== b.type) {
      result.push({
        kind: "scope-type-mismatch",
        scopePath: u.path,
        message: `unsnarl=${u.type} baseline=${b.type}`,
      });
    }
    if (u.blockType !== b.blockType) {
      result.push({
        kind: "scope-block-type-mismatch",
        scopePath: u.path,
        message: `unsnarl=${u.blockType} baseline=${b.blockType}`,
      });
    }
    if (
      u.blockRange[0] !== b.blockRange[0] ||
      u.blockRange[1] !== b.blockRange[1]
    ) {
      result.push({
        kind: "scope-block-range-mismatch",
        scopePath: u.path,
        message: `unsnarl=${formatRange(u.blockRange)} baseline=${formatRange(b.blockRange)}`,
      });
    }
    result.push(...diffVariables(u.path, u.variables, b.variables));
    result.push(...diffReferences(u.path, u.references, b.references));
    if (u.childScopes.length !== b.childScopes.length) {
      result.push({
        kind: "child-scope-count-mismatch",
        scopePath: u.path,
        message: `unsnarl=${u.childScopes.length} (${u.childScopes.map((c) => c.type).join(",")}) baseline=${b.childScopes.length} (${b.childScopes.map((c) => c.type).join(",")})`,
      });
    }
    const pairs = Math.min(u.childScopes.length, b.childScopes.length);
    for (let i = 0; i < pairs; i++) {
      const uc = u.childScopes[i];
      const bc = b.childScopes[i];
      if (uc && bc) {
        walk(uc, bc);
      }
    }
  }
}
