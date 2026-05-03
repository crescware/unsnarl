import { beforeEach, describe, expect, test, vi } from "vitest";

import { DEFAULT_GENERATIONS } from "../../args/default-generations.js";
import { readSourceFile } from "../../io.js";
import { buildRunOpts } from "./build-run-opts.js";
import type { ExecuteSource } from "./execute-source.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

vi.mock("../../io.js", () => ({
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

const stdinSrc = {
  stdin: true,
  text: "const piped = 1;",
  stdinLang: "tsx",
} as const satisfies ExecuteSource;

const fileSrc = {
  stdin: false,
  path: "src/foo.tsx",
} as const satisfies ExecuteSource;

describe("buildRunOpts", () => {
  beforeEach(() => {
    vi.mocked(readSourceFile).mockReset();
  });

  describe("file input", () => {
    const text = "const x = 1;";

    beforeEach(() => {
      vi.mocked(readSourceFile).mockReturnValueOnce(text);
    });

    test("returns runOpts with text from file, language from extension, and sourcePath from path", () => {
      const expected = {
        text,
        runOpts: {
          format: "json",
          language: "tsx",
          sourcePath: "src/foo.tsx",
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning: null,
        },
      } as const satisfies ReturnType<typeof buildRunOpts>;

      const actual = buildRunOpts(fileSrc, baseOpts);
      expect(actual).toEqual(expected);
    });

    test("calls readSourceFile with the source path", () => {
      buildRunOpts(fileSrc, baseOpts);
      expect(vi.mocked(readSourceFile)).toHaveBeenCalledWith("src/foo.tsx");
    });

    test("propagates format and prettyJson into runOpts.emit", () => {
      const expected = {
        text,
        runOpts: {
          format: "mermaid",
          language: "tsx",
          sourcePath: "src/foo.tsx",
          emit: { prettyJson: false, prunedGraph: null, resolutions: null },
          pruning: null,
        },
      } as const satisfies ReturnType<typeof buildRunOpts>;

      const actual = buildRunOpts(fileSrc, {
        ...baseOpts,
        format: "mermaid",
        prettyJson: false,
      });

      expect(actual).toEqual(expected);
    });

    test("no roots → pruning is null", () => {
      const expected = {
        text,
        runOpts: {
          format: "json",
          language: "tsx",
          sourcePath: "src/foo.tsx",
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning: null,
        },
      } as const satisfies ReturnType<typeof buildRunOpts>;

      const actual = buildRunOpts(fileSrc, baseOpts);
      expect(actual).toEqual(expected);
    });

    describe("radius flags", () => {
      const roots = [
        { kind: "name", name: "render", raw: "render" },
      ] as const satisfies Parameters<typeof buildRunOpts>[1]["roots"];

      test("explicit descendants only → ancestors falls to 0", () => {
        const expected = {
          text,
          runOpts: {
            format: "json",
            language: "tsx",
            sourcePath: "src/foo.tsx",
            emit: { prettyJson: true, prunedGraph: null, resolutions: null },
            pruning: {
              roots,
              descendants: 2,
              ancestors: 0,
            },
          },
        } as const satisfies ReturnType<typeof buildRunOpts>;

        const actual = buildRunOpts(fileSrc, {
          ...baseOpts,
          roots,
          descendants: 2,
          ancestors: null,
          context: null,
        });

        expect(actual).toEqual(expected);
      });

      test("no flag → both fall back to DEFAULT_GENERATIONS", () => {
        const expected = {
          text,
          runOpts: {
            format: "json",
            language: "tsx",
            sourcePath: "src/foo.tsx",
            emit: { prettyJson: true, prunedGraph: null, resolutions: null },
            pruning: {
              roots,
              descendants: DEFAULT_GENERATIONS,
              ancestors: DEFAULT_GENERATIONS,
            },
          },
        } as const satisfies ReturnType<typeof buildRunOpts>;

        const actual = buildRunOpts(fileSrc, { ...baseOpts, roots });
        expect(actual).toEqual(expected);
      });
    });
  });

  describe("stdin input", () => {
    const opts = {
      ...baseOpts,
      stdin: true,
      stdinLang: "tsx",
    } as const satisfies Parameters<typeof buildRunOpts>[1];

    test("returns runOpts with text/stdinLang from src and sourcePath set to stdin.<stdinLang>", () => {
      const expected = {
        text: "const piped = 1;",
        runOpts: {
          format: "json",
          language: "tsx",
          sourcePath: "stdin.tsx",
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning: null,
        },
      } as const satisfies ReturnType<typeof buildRunOpts>;

      const actual = buildRunOpts(stdinSrc, opts);
      expect(actual).toEqual(expected);
    });

    test("does not call readSourceFile", () => {
      buildRunOpts(stdinSrc, opts);
      expect(vi.mocked(readSourceFile)).not.toHaveBeenCalled();
    });
  });
});
