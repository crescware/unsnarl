import type { PipelineRunDetails } from "../../../pipeline/types.js";
import { formatResolutionNotice } from "../../../visual-graph/prune/format-resolution-notice.js";

export function emitResolutionNotices(
  resolutions: PipelineRunDetails["resolutions"],
): void {
  if (resolutions === null) {
    return;
  }
  for (const r of resolutions) {
    process.stderr.write(`${formatResolutionNotice(r).join("\n")}\n`);
  }
}
