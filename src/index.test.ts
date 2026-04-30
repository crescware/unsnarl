import { describe, expect, test } from "vitest";

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

const fakeIR: SerializedIR = {
  version: 1,
  source: { path: "test.ts", language: "ts" },
  scopes: [],
  variables: [],
  references: [],
  unusedVariableIds: [],
  raw: "",
  diagnostics: [],
};

const fakeParser: Parser = {
  id: "fake",
  parse: (code, opts) => ({
    ast: { type: "Program", body: [] },
    language: opts.language,
    sourcePath: opts.sourcePath,
    raw: code,
  }),
};

const fakeAnalyzer: ScopeAnalyzer = {
  id: "fake",
  analyze: () => ({ rootScope: fakeScope, diagnostics: [], raw: "" }),
};

const fakeSerializer: IRSerializer = {
  id: "fake",
  serialize: () => fakeIR,
};

const fakeEmitter: Emitter = {
  format: "fake",
  contentType: "text/plain",
  extension: "txt",
  emit: (ir) => `version=${ir.version}`,
};

function buildRegistry(emitters: Emitter[]): EmitterRegistry {
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
      language: "ts",
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
        language: "ts",
        sourcePath: "x.ts",
      }),
    ).toThrow(/Unknown emitter format/);
  });

  test("allows swapping parser without touching downstream layers", () => {
    let parserCalled = false;
    const swapped: Parser = {
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
    };

    const pipeline = createPipeline({
      parser: swapped,
      analyzer: fakeAnalyzer,
      serializer: fakeSerializer,
      emitters: buildRegistry([fakeEmitter]),
    });

    pipeline.run("", {
      format: "fake",
      language: "ts",
      sourcePath: "x.ts",
    });

    expect(parserCalled).toBe(true);
  });
});
