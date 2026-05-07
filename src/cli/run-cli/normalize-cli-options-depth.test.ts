import { describe, expect, test } from "vitest";

import { CATEGORY } from "../../serializer/category.js";
import { DEFAULT_DEPTH } from "../args/depth-options.js";
import type { ParsedCliOptions } from "../args/parsed-cli-options.js";
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

describe("normalizeCliOptions depth resolution", () => {
  test("no flag -> every category gets DEFAULT_DEPTH", () => {
    const out = normalizeCliOptions(baseParsed);
    for (const c of Object.values(CATEGORY)) {
      expect(out.depths[c]).toBe(DEFAULT_DEPTH);
    }
  });

  test("--depth alone applies to every category", () => {
    const out = normalizeCliOptions({ ...baseParsed, depth: 3 });
    for (const c of Object.values(CATEGORY)) {
      expect(out.depths[c]).toBe(3);
    }
  });

  test("--depth-function overrides --depth for the function category only", () => {
    const out = normalizeCliOptions({
      ...baseParsed,
      depth: 3,
      depthFunction: 1,
    });
    expect(out.depths[CATEGORY.Function]).toBe(1);
    expect(out.depths[CATEGORY.If]).toBe(3);
    expect(out.depths[CATEGORY.For]).toBe(3);
    expect(out.depths[CATEGORY.Block]).toBe(3);
  });

  test("--depth-block overrides --depth for every non-function category", () => {
    const out = normalizeCliOptions({
      ...baseParsed,
      depth: 3,
      depthBlock: 1,
    });
    expect(out.depths[CATEGORY.Function]).toBe(3);
    expect(out.depths[CATEGORY.If]).toBe(1);
    expect(out.depths[CATEGORY.For]).toBe(1);
    expect(out.depths[CATEGORY.While]).toBe(1);
    expect(out.depths[CATEGORY.Switch]).toBe(1);
    expect(out.depths[CATEGORY.TryCatchFinally]).toBe(1);
    expect(out.depths[CATEGORY.Block]).toBe(1);
  });

  test("--depth-function and --depth-block together (no --depth)", () => {
    const out = normalizeCliOptions({
      ...baseParsed,
      depthFunction: 2,
      depthBlock: 5,
    });
    expect(out.depths[CATEGORY.Function]).toBe(2);
    expect(out.depths[CATEGORY.If]).toBe(5);
    expect(out.depths[CATEGORY.Block]).toBe(5);
  });
});
