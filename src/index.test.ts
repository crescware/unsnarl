import { describe, expect, test } from "vitest";

import { LANGUAGE } from "./language.js";
import type { Scope } from "./ir/scope/scope.js";
import type { SerializedIR } from "./ir/serialized/serialized-ir.js";
import { AST_TYPE } from "./parser/ast-type.js";
import type { ScopeAnalyzer } from "./pipeline/analyze/scope-analyzer.js";
import type { EmitterRegistry } from "./pipeline/emit/emitter-registry.js";
import type { Emitter } from "./pipeline/emit/emitter.js";
import type { Parser } from "./pipeline/parse/parser.js";
import { createPipeline } from "./pipeline/pipeline.js";
import type { IRSerializer } from "./pipeline/serialize/ir-serializer.js";
import { SERIALIZED_IR_VERSION } from "./serializer/serialized-ir-version.js";

const fakeScope = {} as Scope;

const fakeIR = {
  version: SERIALIZED_IR_VERSION,
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
    ast: { type: AST_TYPE.Program, body: [] },
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
    get: (format) => map.get(format) ?? null,
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

    const result = pipeline.runDetailed("const x = 1;", {
      format: "fake",
      language: LANGUAGE.Ts,
      sourcePath: "test.ts",
      emit: { prettyJson: true, prunedGraph: null, resolutions: null },
      pruning: null,
    }).text;

    expect(result).toBe("version=1");
  });

  test("throws when format is unknown", () => {
    const pipeline = createPipeline({
      parser: fakeParser,
      analyzer: fakeAnalyzer,
      serializer: fakeSerializer,
      emitters: buildRegistry([]),
    });

    expect(
      () =>
        pipeline.runDetailed("", {
          format: "missing",
          language: LANGUAGE.Ts,
          sourcePath: "x.ts",
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning: null,
        }).text,
    ).toThrow(/Unknown emitter format/);
  });

  test("allows swapping parser without touching downstream layers", () => {
    let parserCalled = false;
    const swapped = {
      id: "swapped",
      parse: (code, opts) => {
        parserCalled = true;
        return {
          ast: { type: AST_TYPE.Program, body: [] },
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

    pipeline.runDetailed("", {
      format: "fake",
      language: LANGUAGE.Ts,
      sourcePath: "x.ts",
      emit: { prettyJson: true, prunedGraph: null, resolutions: null },
      pruning: null,
    });

    expect(parserCalled).toBe(true);
  });
});
