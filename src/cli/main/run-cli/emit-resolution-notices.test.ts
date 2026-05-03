import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import type { PipelineRunDetails } from "../../../pipeline/types.js";
import { emitResolutionNotices } from "./emit-resolution-notices.js";

describe("emitResolutionNotices", () => {
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

  test("writes nothing when resolutions is null", () => {
    emitResolutionNotices(null);
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes nothing when resolutions is empty", () => {
    emitResolutionNotices([]);
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes the identifier-match notice exactly", () => {
    const resolutions = [
      { raw: "L12", line: 12, name: "L12", resolvedAs: "name" },
    ] as const satisfies PipelineRunDetails["resolutions"];

    emitResolutionNotices(resolutions);
    expect(written).toEqual([
      [
        "uns: 'L12' is ambiguous.",
        "  An exact identifier match was found; interpreting as identifier.",
        "  To disambiguate, use '-r 12'.",
        "",
      ].join("\n"),
    ]);
  });

  test("writes the line-fallback notice exactly", () => {
    const resolutions = [
      { raw: "L12", line: 12, name: "L12", resolvedAs: "line" },
    ] as const satisfies PipelineRunDetails["resolutions"];

    emitResolutionNotices(resolutions);
    expect(written).toEqual([
      [
        "uns: 'L12' is ambiguous.",
        "  No exact identifier match was found; interpreting as line number.",
        "  To disambiguate, use '-r 12'.",
        "",
      ].join("\n"),
    ]);
  });

  test("writes one notice per resolution, preserving order", () => {
    const resolutions = [
      { raw: "L12", line: 12, name: "L12", resolvedAs: "name" },
      { raw: "l5", line: 5, name: "l5", resolvedAs: "line" },
    ] as const satisfies PipelineRunDetails["resolutions"];

    emitResolutionNotices(resolutions);
    expect(written).toHaveLength(2);
    expect(written[0]).toContain("'L12' is ambiguous");
    expect(written[0]).toContain("interpreting as identifier");
    expect(written[1]).toContain("'l5' is ambiguous");
    expect(written[1]).toContain("interpreting as line number");
    expect(written[1]).toContain("-r 5");
  });
});
