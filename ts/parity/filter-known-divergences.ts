import type { MismatchKind } from "./compare/mismatch-kind.js";
import type { Mismatch } from "./compare/mismatch.js";

type KnownDivergence = Readonly<{
  fixtureId: string;
  scopePath: readonly number[];
  kind: MismatchKind;
  reason: string;
}>;

const KNOWN_DIVERGENCES: readonly KnownDivergence[] = [];

function samePath(a: readonly number[], b: readonly number[]): boolean {
  return a.length === b.length && a.every((v, i) => v === b[i]);
}

export function filterKnownDivergences(
  fixtureId: string,
  mismatches: readonly Mismatch[],
): Mismatch[] {
  return mismatches.filter((mismatch) => {
    return !KNOWN_DIVERGENCES.some(
      (v) =>
        v.fixtureId === fixtureId &&
        v.kind === mismatch.kind &&
        samePath(v.scopePath, mismatch.scopePath),
    );
  });
}
