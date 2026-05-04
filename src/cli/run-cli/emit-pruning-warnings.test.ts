import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import type { PipelineRunDetails } from "../../pipeline/runner/pipeline-run-details.js";
import { emitPruningWarnings } from "./emit-pruning-warnings.js";

describe("emitPruningWarnings", () => {
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

  test("writes nothing when pruning is null", () => {
    emitPruningWarnings(null);
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes nothing when pruning is empty", () => {
    emitPruningWarnings([]);
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes nothing when every entry has matched > 0", () => {
    const pruning = [
      { query: "render", matched: 3 },
      { query: "init", matched: 1 },
    ] as const satisfies PipelineRunDetails["pruning"];

    emitPruningWarnings(pruning);
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes a warning line for an entry with matched === 0", () => {
    const pruning = [
      { query: "render", matched: 0 },
    ] as const satisfies PipelineRunDetails["pruning"];

    emitPruningWarnings(pruning);
    expect(written).toEqual(["uns: warning: query 'render' matched 0 roots\n"]);
  });

  test("writes one warning line per zero-match entry, skipping the matched ones", () => {
    const pruning = [
      { query: "render", matched: 0 },
      { query: "init", matched: 2 },
      { query: "boot", matched: 0 },
    ] as const satisfies PipelineRunDetails["pruning"];

    emitPruningWarnings(pruning);
    expect(written).toEqual([
      "uns: warning: query 'render' matched 0 roots\n",
      "uns: warning: query 'boot' matched 0 roots\n",
    ]);
  });
});
