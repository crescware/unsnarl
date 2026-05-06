import { describe, expect, test } from "vitest";

import { DIAGNOSTIC_KIND } from "./diagnostic-kind.js";
import { formatVarDiagnostic } from "./format-var-diagnostic.js";

describe("formatVarDiagnostic", () => {
  test("renders a single-line warning with line:column and the diagnostic message", () => {
    const lines = formatVarDiagnostic({
      kind: DIAGNOSTIC_KIND.VarDetected,
      message: "var declaration detected; rendered as node only (no edges).",
      span: { line: 3, column: 0, offset: 11 },
    });
    expect(lines).toEqual([
      "uns: warning: L3:0: var declaration detected; rendered as node only (no edges).",
    ]);
  });
});
