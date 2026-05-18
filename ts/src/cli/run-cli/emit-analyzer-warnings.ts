import { DIAGNOSTIC_KIND } from "../../analyzer/diagnostic-kind.js";
import { formatVarDiagnostic } from "../../analyzer/format-var-diagnostic.js";
import type { PipelineRunDetails } from "../../pipeline/runner/pipeline-run-details.js";

export function emitAnalyzerWarnings(
  diagnostics: PipelineRunDetails["diagnostics"],
): void {
  for (const diagnostic of diagnostics) {
    if (diagnostic.kind !== DIAGNOSTIC_KIND.VarDetected) {
      continue;
    }
    process.stderr.write(`${formatVarDiagnostic(diagnostic).join("\n")}\n`);
  }
}
