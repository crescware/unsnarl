import type { PipelineRunDetails } from "../../../pipeline/types.js";

export function emitResolutionNotices(
  resolutions: PipelineRunDetails["resolutions"],
): void {
  if (resolutions === null) {
    return;
  }
  for (const r of resolutions) {
    const second =
      r.resolvedAs === "name"
        ? "An exact identifier match was found; interpreting as identifier."
        : "No exact identifier match was found; interpreting as line number.";
    process.stderr.write(
      `uns: '${r.raw}' is ambiguous.\n  ${second}\n  To disambiguate, use '-r ${String(r.line)}'.\n`,
    );
  }
}
