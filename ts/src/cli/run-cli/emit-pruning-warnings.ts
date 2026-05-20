import type { PipelineRunDetails } from "../../pipeline/runner/pipeline-run-details.js";

export function emitPruningWarnings(
  pruning: PipelineRunDetails["pruning"],
): void {
  if (pruning === null) {
    return;
  }
  for (const r of pruning) {
    if (r.matched === 0) {
      process.stderr.write(
        `uns: warning: query '${r.query}' matched 0 roots\n`,
      );
    }
  }
}
