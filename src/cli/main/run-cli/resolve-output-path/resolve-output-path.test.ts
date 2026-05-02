import { describe, expect, test } from "vitest";

import type { SerializedIR } from "../../../../ir/model.js";
import type { Emitter, EmitterRegistry } from "../../../../pipeline/types.js";
import type { ParsedRootQuery } from "../../../root-query/parsed-root-query.js";
import { ROOT_QUERY_KIND } from "../../../root-query/root-query-kind.js";
import { CliUsageError } from "../cli-usage-error.js";
import type { ExecuteSource } from "../execute-source.js";
import type { NormalizedCliOptions } from "../normalized-cli-options.js";
import { resolveOutputPath } from "./resolve-output-path.js";

const baseOpts: NormalizedCliOptions = {
  format: "json",
  stdin: false,
  lang: "ts",
  prettyJson: true,
  mermaidRenderer: null,
  roots: [],
  descendants: null,
  ancestors: null,
  context: null,
  outDir: null,
};

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

const fileSrc: ExecuteSource = { stdin: false, path: "src/deep/foo.ts" };
const stdinSrc: ExecuteSource = {
  stdin: true,
  text: "",
  lang: "ts",
};

describe("resolveOutputPath", () => {
  test("returns null when outDir is null", () => {
    const result = resolveOutputPath(
      fileSrc,
      baseOpts,
      makeEmitters([makeEmitter("json", "json")]),
    );
    expect(result).toBeNull();
  });

  test("file input + outDir → joins outDir/<basename>.<ext>", () => {
    const result = resolveOutputPath(
      fileSrc,
      { ...baseOpts, outDir: "out" },
      makeEmitters([makeEmitter("json", "json")]),
    );
    expect(result).toBe("out/foo.json");
  });

  test("roots take precedence over input filename", () => {
    const result = resolveOutputPath(
      fileSrc,
      {
        ...baseOpts,
        outDir: "out",
        roots: [nameRoot("render")],
      },
      makeEmitters([makeEmitter("json", "json")]),
    );
    expect(result).toBe("out/render.json");
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
    const result = resolveOutputPath(
      stdinSrc,
      {
        ...baseOpts,
        outDir: "out",
        roots: [nameRoot("render")],
      },
      makeEmitters([makeEmitter("json", "json")]),
    );
    expect(result).toBe("out/render.json");
  });

  test("uses the emitter's extension, not the format name", () => {
    const result = resolveOutputPath(
      fileSrc,
      { ...baseOpts, outDir: "out", format: "mermaid" },
      makeEmitters([makeEmitter("mermaid", "mmd")]),
    );
    expect(result).toBe("out/foo.mmd");
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
