import { describe, expect, test } from "vitest";

import type { SerializedIR } from "../../../../ir/serialized/serialized-ir.js";
import type { Emitter, EmitterRegistry } from "../../../../pipeline/types.js";
import type { ParsedRootQuery } from "../../../root-query/parsed-root-query.js";
import { ROOT_QUERY_KIND } from "../../../root-query/root-query-kind.js";
import { CliUsageError } from "../cli-usage-error.js";
import type { ExecuteSource } from "../execute-source.js";
import type { NormalizedCliOptions } from "../normalized-cli-options.js";
import { resolveOutputPath } from "./resolve-output-path.js";

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

function makeEmitter(format: string, extension: string): Emitter {
  return {
    format,
    contentType: "text/plain",
    extension,
    emit: (_ir: SerializedIR) => "",
  };
}

function makeEmitters(emitters: readonly Emitter[]): EmitterRegistry {
  const map = new Map(emitters.map((e) => [e.format, e]));
  return {
    register: () => {
      throw new Error("not implemented in fake");
    },
    get: (format: string) => map.get(format),
    list: () => Array.from(map.keys()),
  };
}

const nameRoot = (n: string): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.Name,
  name: n,
  raw: n,
});

const stdinSrc = {
  stdin: true,
  text: "",
  stdinLang: "ts",
} as const satisfies ExecuteSource;

const fileSrc = {
  stdin: false,
  path: "src/deep/foo.ts",
} as const satisfies ExecuteSource;

describe("resolveOutputPath", () => {
  test("returns null when outDir is null", () => {
    const actual = resolveOutputPath(
      fileSrc,
      baseOpts,
      makeEmitters([makeEmitter("json", "json")]),
    );

    expect(actual).toBeNull();
  });

  test("file input + outDir → joins outDir/<basename>.<ext>", () => {
    const actual = resolveOutputPath(
      fileSrc,
      { ...baseOpts, outDir: "out" },
      makeEmitters([makeEmitter("json", "json")]),
    );

    expect(actual).toBe("out/foo.json");
  });

  test("roots take precedence over input filename", () => {
    const actual = resolveOutputPath(
      fileSrc,
      {
        ...baseOpts,
        outDir: "out",
        roots: [nameRoot("render")],
      },
      makeEmitters([makeEmitter("json", "json")]),
    );

    expect(actual).toBe("out/render.json");
  });

  test("stdin without roots throws CliUsageError (no usable filename)", () => {
    expect(() =>
      resolveOutputPath(
        stdinSrc,
        { ...baseOpts, outDir: "out" },
        makeEmitters([makeEmitter("json", "json")]),
      ),
    ).toThrow(CliUsageError);
  });

  test("stdin with roots succeeds (positional path is irrelevant)", () => {
    const actual = resolveOutputPath(
      stdinSrc,
      {
        ...baseOpts,
        outDir: "out",
        roots: [nameRoot("render")],
      },
      makeEmitters([makeEmitter("json", "json")]),
    );

    expect(actual).toBe("out/render.json");
  });

  test("uses the emitter's extension, not the format name", () => {
    const actual = resolveOutputPath(
      fileSrc,
      { ...baseOpts, outDir: "out", format: "mermaid" },
      makeEmitters([makeEmitter("mermaid", "mmd")]),
    );

    expect(actual).toBe("out/foo.mmd");
  });

  test("unknown format throws Error listing available formats", () => {
    expect(() =>
      resolveOutputPath(
        fileSrc,
        { ...baseOpts, outDir: "out", format: "ghost" },
        makeEmitters([
          makeEmitter("json", "json"),
          makeEmitter("mermaid", "mmd"),
        ]),
      ),
    ).toThrow(/Unknown emitter format: ghost\. Available: json, mermaid/);
  });
});
