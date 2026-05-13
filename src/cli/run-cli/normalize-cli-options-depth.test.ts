import { describe, expect, test } from "vitest";

import { CLI_COLOR_THEME } from "../../cli-color-theme.js";
import { NESTING_KIND } from "../../serializer/nesting-kind.js";
import { DEFAULT_DEPTH } from "../args/depth-options.js";
import type { ParsedCliOptions } from "../args/parsed-cli-options.js";
import { normalizeCliOptions } from "./normalize-cli-options.js";

const baseParsed = {
  format: "json",
  stdin: false,
  stdinLang: "ts",
  prettyJson: true,
  mermaidRenderer: null,
  colorTheme: CLI_COLOR_THEME.Dark,
  roots: [],
  highlight: false,
  descendants: null,
  ancestors: null,
  context: null,
  depth: null,
  depthFunction: null,
  depthBlock: null,
  outDir: null,
  outFile: null,
  debug: false,
  plugins: [],
} as const satisfies ParsedCliOptions;

describe("normalizeCliOptions depth resolution", () => {
  test("no flag -> every nesting kind gets DEFAULT_DEPTH", () => {
    const out = normalizeCliOptions(baseParsed);
    for (const c of Object.values(NESTING_KIND)) {
      expect(out.depths[c]).toEqual(DEFAULT_DEPTH);
    }
  });

  test("--depth alone applies to every nesting kind", () => {
    const out = normalizeCliOptions({ ...baseParsed, depth: 3 });
    for (const c of Object.values(NESTING_KIND)) {
      expect(out.depths[c]).toEqual(3);
    }
  });

  test("--depth-function overrides --depth for the function nesting kind only", () => {
    const out = normalizeCliOptions({
      ...baseParsed,
      depth: 3,
      depthFunction: 1,
    });
    expect(out.depths[NESTING_KIND.Function]).toEqual(1);
    expect(out.depths[NESTING_KIND.If]).toEqual(3);
    expect(out.depths[NESTING_KIND.For]).toEqual(3);
    expect(out.depths[NESTING_KIND.Block]).toEqual(3);
  });

  test("--depth-block overrides --depth for every non-function nesting kind", () => {
    const out = normalizeCliOptions({
      ...baseParsed,
      depth: 3,
      depthBlock: 1,
    });
    expect(out.depths[NESTING_KIND.Function]).toEqual(3);
    expect(out.depths[NESTING_KIND.If]).toEqual(1);
    expect(out.depths[NESTING_KIND.For]).toEqual(1);
    expect(out.depths[NESTING_KIND.While]).toEqual(1);
    expect(out.depths[NESTING_KIND.Switch]).toEqual(1);
    expect(out.depths[NESTING_KIND.TryCatchFinally]).toEqual(1);
    expect(out.depths[NESTING_KIND.Block]).toEqual(1);
  });

  test("--depth-function and --depth-block together (no --depth)", () => {
    const out = normalizeCliOptions({
      ...baseParsed,
      depthFunction: 2,
      depthBlock: 5,
    });
    expect(out.depths[NESTING_KIND.Function]).toEqual(2);
    expect(out.depths[NESTING_KIND.If]).toEqual(5);
    expect(out.depths[NESTING_KIND.Block]).toEqual(5);
  });
});
