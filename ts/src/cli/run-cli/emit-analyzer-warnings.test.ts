import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { DIAGNOSTIC_KIND } from "../../analyzer/diagnostic-kind.js";
import type { PipelineRunDetails } from "../../pipeline/runner/pipeline-run-details.js";
import { emitAnalyzerWarnings } from "./emit-analyzer-warnings.js";

describe("emitAnalyzerWarnings", () => {
  let writeSpy: ReturnType<typeof vi.spyOn>;
  let written: /* mutable */ string[];

  beforeEach(() => {
    written = [];
    writeSpy = vi
      .spyOn(process.stderr, "write")
      .mockImplementation((chunk: unknown) => {
        written.push(typeof chunk === "string" ? chunk : String(chunk));
        return true;
      });
  });

  afterEach(() => {
    writeSpy.mockRestore();
  });

  test("writes nothing when diagnostics is empty", () => {
    emitAnalyzerWarnings([]);
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes a warning line for each VarDetected entry", () => {
    const diagnostics = [
      {
        kind: DIAGNOSTIC_KIND.VarDetected,
        message: "var declaration detected; rendered as node only (no edges).",
        span: { line: 2, column: 0, offset: 11 },
      },
      {
        kind: DIAGNOSTIC_KIND.VarDetected,
        message: "var declaration detected; rendered as node only (no edges).",
        span: { line: 5, column: 4, offset: 50 },
      },
    ] as const satisfies PipelineRunDetails["diagnostics"];

    emitAnalyzerWarnings(diagnostics);
    expect(written).toEqual([
      "uns: warning: L2:0: var declaration detected; rendered as node only (no edges).\n",
      "uns: warning: L5:4: var declaration detected; rendered as node only (no edges).\n",
    ]);
  });

  test("ignores non-VarDetected diagnostics", () => {
    const diagnostics = [
      {
        kind: DIAGNOSTIC_KIND.UnresolvedIdentifier,
        message: "irrelevant",
        span: { line: 1, column: 0, offset: 0 },
      },
    ] as const satisfies PipelineRunDetails["diagnostics"];

    emitAnalyzerWarnings(diagnostics);
    expect(writeSpy).not.toHaveBeenCalled();
  });
});
