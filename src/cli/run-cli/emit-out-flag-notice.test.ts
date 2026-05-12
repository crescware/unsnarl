import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { CLI_COLOR_THEME } from "../../cli-color-theme.js";
import { defaultDepths } from "../args/depth-options.js";
import { emitOutFlagNotice } from "./emit-out-flag-notice.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

const baseOpts = {
  format: "json",
  stdin: false,
  stdinLang: "ts",
  prettyJson: true,
  mermaidRenderer: null,
  colorTheme: CLI_COLOR_THEME.Dark,
  roots: [],
  descendants: null,
  ancestors: null,
  context: null,
  depths: defaultDepths(),
  out: null,
  debug: false,
  plugins: [],
} as const satisfies NormalizedCliOptions;

describe("emitOutFlagNotice", () => {
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

  test("writes nothing when out is null", () => {
    emitOutFlagNotice(baseOpts);
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes nothing when out.mode is file (--out-file already explicit)", () => {
    emitOutFlagNotice({
      ...baseOpts,
      out: { mode: "file", path: "graph.mmd" },
    });
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes nothing for a dir path without an extension", () => {
    emitOutFlagNotice({
      ...baseOpts,
      out: { mode: "dir", path: "build/out" },
    });
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes nothing for a dotfile-style basename (no extname)", () => {
    emitOutFlagNotice({
      ...baseOpts,
      out: { mode: "dir", path: ".cache" },
    });
    expect(writeSpy).not.toHaveBeenCalled();
  });

  test("writes a notice when a dir path's basename has an extension", () => {
    emitOutFlagNotice({
      ...baseOpts,
      out: { mode: "dir", path: "graph.mmd" },
    });
    expect(written).toEqual([
      "uns: notice: -o 'graph.mmd' is treated as a directory name; use --out-file to write to that path as a file.\n",
    ]);
  });

  test("writes a notice for a deep dir path whose tail has an extension", () => {
    emitOutFlagNotice({
      ...baseOpts,
      out: { mode: "dir", path: "build/out.json" },
    });
    expect(written).toEqual([
      "uns: notice: -o 'build/out.json' is treated as a directory name; use --out-file to write to that path as a file.\n",
    ]);
  });
});
