import type { RootQueryResolution } from "./resolve-ambiguous-queries.js";

// Returns the three lines a resolution notice consists of, sharing the
// exact wording between the stderr emitter and the markdown Notice
// section so both stay in lock-step.
export function formatResolutionNotice(r: RootQueryResolution): string[] {
  const second =
    r.resolvedAs === "name"
      ? "An exact identifier match was found; interpreting as identifier."
      : "No exact identifier match was found; interpreting as line number.";
  return [
    `uns: '${r.raw}' is ambiguous.`,
    `  ${second}`,
    `  To disambiguate, use '-r ${String(r.line)}'.`,
  ];
}
