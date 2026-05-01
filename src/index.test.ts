import { describe, expect, test } from "vitest";

import { LANGUAGE } from "./constants.js";
import type { Scope, SerializedIR } from "./ir/model.js";
import { createPipeline } from "./pipeline/pipeline.js";
import type {
  Emitter,
  EmitterRegistry,
  IRSerializer,
  Parser,
  ScopeAnalyzer,
} from "./pipeline/types.js";

const fakeScope = {} as Scope;

const fakeIR = {
  version: 1,
  source: { path: "test.ts", language: LANGUAGE.Ts },
  scopes: [],
  variables: [],
  references: [],
  unusedVariableIds: [],
  raw: "",
  diagnostics: [],
} as const satisfies SerializedIR;

const fakeParser = {
  id: "fake",
  parse: (code, opts) => ({
    ast: { type: "Program", body: [] },
    language: opts.language,
    sourcePath: opts.sourcePath,
    raw: code,
  }),
} as const satisfies Parser;

const fakeAnalyzer = {
  id: "fake",
  analyze: () => ({ rootScope: fakeScope, diagnostics: [], raw: "" }),
} as const satisfies ScopeAnalyzer;

const fakeSerializer = {
  id: "fake",
  serialize: () => fakeIR,
} as const satisfies IRSerializer;

const fakeEmitter = {
  format: "fake",
  contentType: "text/plain",
  extension: "txt",
  emit: (ir) => `version=${ir.version}`,
} as const satisfies Emitter;

function buildRegistry(emitters: readonly Emitter[]): EmitterRegistry {
  const map = new Map<string, Emitter>();
  for (const e of emitters) {
    map.set(e.format, e);
  }
  return {
    register: (e) => {
      map.set(e.format, e);
    },
    get: (format) => map.get(format),
    list: () => [...map.keys()],
  };
}

describe("createPipeline", () => {
  test("connects parser, analyzer, serializer, and emitter", () => {
    const pipeline = createPipeline({
      parser: fakeParser,
      analyzer: fakeAnalyzer,
      serializer: fakeSerializer,
      emitters: buildRegistry([fakeEmitter]),
    });

    const result = pipeline.run("const x = 1;", {
      format: "fake",
      language: LANGUAGE.Ts,
      sourcePath: "test.ts",
    });

    expect(result).toBe("version=1");
  });

  test("throws when format is unknown", () => {
    const pipeline = createPipeline({
      parser: fakeParser,
      analyzer: fakeAnalyzer,
      serializer: fakeSerializer,
      emitters: buildRegistry([]),
    });

    expect(() =>
      pipeline.run("", {
        format: "missing",
        language: LANGUAGE.Ts,
        sourcePath: "x.ts",
      }),
    ).toThrow(/Unknown emitter format/);
  });

  test("allows swapping parser without touching downstream layers", () => {
    let parserCalled = false;
    const swapped = {
      id: "swapped",
      parse: (code, opts) => {
        parserCalled = true;
        return {
          ast: { type: "Program", body: [] },
          language: opts.language,
          sourcePath: opts.sourcePath,
          raw: code,
        };
      },
    } as const satisfies Parser;

    const pipeline = createPipeline({
      parser: swapped,
      analyzer: fakeAnalyzer,
      serializer: fakeSerializer,
      emitters: buildRegistry([fakeEmitter]),
    });

    pipeline.run("", {
      format: "fake",
      language: LANGUAGE.Ts,
      sourcePath: "x.ts",
    });

    expect(parserCalled).toBe(true);
  });
});
