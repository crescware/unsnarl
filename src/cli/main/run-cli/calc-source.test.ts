import type { Command } from "commander";
import { beforeEach, describe, expect, test, vi } from "vitest";

import { readStdin } from "../../io.js";
import { calcSource } from "./calc-source.js";
import { CliUsageError } from "./cli-usage-error.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

vi.mock("../../io.js", () => ({
  readStdin: vi.fn(),
  readSourceFile: vi.fn(),
}));

const baseOpts = {
  format: "json",
  stdin: false,
  stdinLang: "ts",
  prettyJson: true,
  mermaidRenderer: null,
  roots: [],
  descendants: null,
  ancestors: null,
  context: null,
  outDir: null,
} as const satisfies NormalizedCliOptions;

const fakeCommand = {
  helpInformation: () => "USAGE",
} as unknown as Command;

describe("calcSource", () => {
  beforeEach(() => {
    vi.mocked(readStdin).mockReset();
  });

  test("stdin: true → reads stdin and returns { stdin: true, text, stdinLang }", async () => {
    vi.mocked(readStdin).mockResolvedValueOnce("piped contents");

    const result = await calcSource(fakeCommand, null, {
      ...baseOpts,
      stdin: true,
      stdinLang: "tsx",
    });

    expect(result).toEqual({
      stdin: true,
      text: "piped contents",
      stdinLang: "tsx",
    });

    expect(vi.mocked(readStdin)).toHaveBeenCalledTimes(1);
  });

  test("stdin: true ignores any positional file argument", async () => {
    vi.mocked(readStdin).mockResolvedValueOnce("");

    const result = await calcSource(fakeCommand, "ignored.ts", {
      ...baseOpts,
      stdin: true,
      stdinLang: "ts",
    });

    expect(result).toEqual({ stdin: true, text: "", stdinLang: "ts" });
  });

  test("stdin: false with file → returns { stdin: false, path: file }", async () => {
    const result = await calcSource(fakeCommand, "src/foo.ts", baseOpts);

    expect(result).toEqual({ stdin: false, path: "src/foo.ts" });
    expect(vi.mocked(readStdin)).not.toHaveBeenCalled();
  });

  test("stdin: false without file → throws CliUsageError carrying help text", async () => {
    await expect(calcSource(fakeCommand, null, baseOpts)).rejects.toMatchObject(
      {
        name: "CliUsageError",
        message: "no input file (use --stdin or pass a path)",
        help: "USAGE",
      },
    );

    await expect(
      calcSource(fakeCommand, null, baseOpts),
    ).rejects.toBeInstanceOf(CliUsageError);

    expect(vi.mocked(readStdin)).not.toHaveBeenCalled();
  });
});
