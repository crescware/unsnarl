import { describe, expect, test } from "vitest";

import type { SerializedIR } from "../ir/model.js";
import type { Emitter } from "../pipeline/types.js";
import { DefaultEmitterRegistry } from "./registry.js";

const fakeIR: SerializedIR = {
  version: 1,
  source: { path: "x.ts", language: "ts" },
  scopes: [],
  variables: [],
  references: [],
  unusedVariableIds: [],
  raw: "",
  diagnostics: [],
};

const fakeEmitter: Emitter = {
  format: "fake",
  contentType: "text/plain",
  emit: () => "",
};

const otherEmitter: Emitter = {
  format: "other",
  contentType: "text/plain",
  emit: () => "",
};

describe("DefaultEmitterRegistry", () => {
  test("registers, looks up, and lists emitters", () => {
    const reg = new DefaultEmitterRegistry();
    reg.register(fakeEmitter);
    reg.register(otherEmitter);
    expect(reg.get("fake")).toBe(fakeEmitter);
    expect(reg.get("other")).toBe(otherEmitter);
    expect(reg.list()).toEqual(["fake", "other"]);
  });

  test("returns undefined for unknown formats", () => {
    const reg = new DefaultEmitterRegistry();
    expect(reg.get("missing")).toBeUndefined();
  });

  test("rejects duplicate formats", () => {
    const reg = new DefaultEmitterRegistry();
    reg.register(fakeEmitter);
    expect(() => {
      reg.register(fakeEmitter);
    }).toThrow(/Duplicate emitter format/);
  });

  test("does not depend on a specific IR shape", () => {
    const reg = new DefaultEmitterRegistry();
    reg.register({
      format: "callable",
      contentType: "text/plain",
      emit: (ir) => `version=${ir.version}`,
    });
    expect(reg.get("callable")?.emit(fakeIR, {})).toBe("version=1");
  });
});
