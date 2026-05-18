import { matchInfoMap } from "./match-info-map";

const toBePattern = /\.toBe(?:[A-Z][a-zA-Z]*)?\(/g;

export function getDisplayLabel(match: string): string {
  return matchInfoMap.get(match)?.label ?? match;
}

export function getErrorMessage(match: string): string {
  return (
    matchInfoMap.get(match)?.message ?? `Use .toEqual() instead of ${match})`
  );
}

export function findToBeMatches(content: string): readonly string[] | null {
  const lines = content.split(/\r?\n/);
  const matches: string[] = [];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const lineMatches = Array.from(line.matchAll(toBePattern), (m) => m[0]);
    if (lineMatches.length === 0) {
      continue;
    }

    const prevLine = i > 0 ? lines[i - 1] : null;
    if (
      prevLine !== null &&
      prevLine.trim().startsWith("// no-to-be-disable-next-line")
    ) {
      continue;
    }

    matches.push(...lineMatches);
  }

  return 0 < matches.length ? matches : null;
}
