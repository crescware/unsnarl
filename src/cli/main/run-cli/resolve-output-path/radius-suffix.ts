/**
 * Stable a-b-c ordering keeps the filename deterministic regardless of
 * CLI argument order. Flags the user did not type are omitted, so an
 * implicit -C 10 default produces no suffix. -C is shorthand for setting
 * both -A and -B; if both are typed explicitly, -C has no remaining
 * effect on the run, so we drop it from the filename too.
 */
export function radiusSuffix(inputs: {
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
}): string {
  const parts: /* mutable */ string[] = [];
  if (inputs.descendants !== null) {
    parts.push(`a${inputs.descendants}`);
  }
  if (inputs.ancestors !== null) {
    parts.push(`b${inputs.ancestors}`);
  }
  const bothExplicit = inputs.descendants !== null && inputs.ancestors !== null;
  if (inputs.context !== null && !bothExplicit) {
    parts.push(`c${inputs.context}`);
  }
  return parts.length === 0 ? "" : `-${parts.join("-")}`;
}
