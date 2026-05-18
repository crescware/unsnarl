import { describe, expect, test } from "vitest";

import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import { LANGUAGE } from "../../language.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { IrEmitter } from "./ir.js";

const ir = {
  version: SERIALIZED_IR_VERSION,
  source: { path: "x.ts", language: LANGUAGE.Ts },
  scopes: [],
  variables: [],
  references: [],
  unusedVariableIds: [],
  raw: "",
  diagnostics: [],
} as const satisfies SerializedIR;

describe("IrEmitter", () => {
  test("emits pretty-printed JSON when prettyJson is true with a trailing newline", () => {
    const out = new IrEmitter().emit(ir, {
      prettyJson: true,
      prunedGraph: null,
      resolutions: null,
      highlightIds: null,
      highlight: null,
      debug: false,
    });
    expect(out.endsWith("\n")).toEqual(true);
    expect(out).toContain('"version": 1');
    expect(JSON.parse(out)).toEqual(ir);
  });

  test("emits compact JSON when prettyJson is false", () => {
    const out = new IrEmitter().emit(ir, {
      prettyJson: false,
      prunedGraph: null,
      resolutions: null,
      highlightIds: null,
      highlight: null,
      debug: false,
    });
    expect(out).not.toContain("\n  ");
    expect(JSON.parse(out)).toEqual(ir);
  });

  test("identifies as 'ir' with the application/json content type", () => {
    const e = new IrEmitter();
    expect(e.format).toEqual("ir");
    expect(e.contentType).toEqual("application/json");
  });
});
