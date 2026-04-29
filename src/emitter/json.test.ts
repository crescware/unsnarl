import { describe, expect, test } from "vitest";

import type { SerializedIR } from "../ir/model.js";
import { JsonEmitter } from "./json.js";

const ir: SerializedIR = {
  version: 1,
  source: { path: "x.ts", language: "ts" },
  scopes: [],
  variables: [],
  references: [],
  unusedVariableIds: [],
  raw: "",
  diagnostics: [],
};

describe("JsonEmitter", () => {
  test("emits pretty JSON by default with a trailing newline", () => {
    const out = new JsonEmitter().emit(ir, {});
    expect(out.endsWith("\n")).toBe(true);
    expect(out).toContain('"version": 1');
    expect(JSON.parse(out)).toEqual(ir);
  });

  test("emits compact JSON when pretty is false", () => {
    const out = new JsonEmitter().emit(ir, { pretty: false });
    expect(out).not.toContain("\n  ");
    expect(JSON.parse(out)).toEqual(ir);
  });

  test("identifies as 'json' with the application/json content type", () => {
    const e = new JsonEmitter();
    expect(e.format).toBe("json");
    expect(e.contentType).toBe("application/json");
  });
});
