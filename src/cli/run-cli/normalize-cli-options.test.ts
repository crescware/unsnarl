import { describe, expect, test } from "vitest";

import type { ParsedCliOptions } from "../args/parsed-cli-options.js";
import { CliUsageError } from "./cli-usage-error.js";
import { normalizeCliOptions } from "./normalize-cli-options.js";

const baseParsed = {
  format: "json",
  stdin: false,
  stdinLang: "ts",
  prettyJson: true,
  mermaidRenderer: null,
  roots: [],
  descendants: null,
  ancestors: null,
  context: null,
  depth: null,
  depthFunction: null,
  depthBlock: null,
  outDir: null,
  outFile: null,
  debug: false,
} as const satisfies ParsedCliOptions;

describe("normalizeCliOptions", () => {
  test("out is null when neither -o nor --out-file is set", () => {
    expect(normalizeCliOptions(baseParsed).out).toEqual(null);
  });

  test("outDir maps to { mode: 'dir', path }", () => {
    expect(normalizeCliOptions({ ...baseParsed, outDir: "build" }).out).toEqual(
      { mode: "dir", path: "build" },
    );
  });

  test("outFile maps to { mode: 'file', path }", () => {
    expect(
      normalizeCliOptions({ ...baseParsed, outFile: "build/graph.mmd" }).out,
    ).toEqual({ mode: "file", path: "build/graph.mmd" });
  });

  test("setting both outDir and outFile throws CliUsageError", () => {
    expect(() =>
      normalizeCliOptions({
        ...baseParsed,
        outDir: "build",
        outFile: "build/graph.mmd",
      }),
    ).toThrow(CliUsageError);
  });
});
