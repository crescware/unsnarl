import { describe, expect, test } from "vitest";

import {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "./default.js";

describe("createDefaultPipeline", () => {
  test("registers json and mermaid emitters by default", () => {
    const reg = createDefaultEmitterRegistry();
    expect([...reg.list()].sort()).toEqual(["json", "mermaid"]);
  });

  test("end-to-end: parses TS, analyzes, serializes, emits JSON", () => {
    const pipeline = createDefaultPipeline();
    const out = pipeline.run("const a = 1;\nconst b = a;\n", {
      format: "json",
      language: "ts",
      sourcePath: "input.ts",
      emit: { pretty: false },
    });
    const ir = JSON.parse(out);
    expect(ir.version).toBe(1);
    expect(ir.source).toEqual({ path: "input.ts", language: "ts" });
    expect(ir.variables.map((v: { name: string }) => v.name).sort()).toEqual([
      "a",
      "b",
    ]);
  });

  test("end-to-end: emits a Mermaid flowchart for the same input", () => {
    const pipeline = createDefaultPipeline();
    const out = pipeline.run("const a = 1;\nconst b = a;\n", {
      format: "mermaid",
      language: "ts",
      sourcePath: "input.ts",
    });
    expect(out).toMatch(/^flowchart RL\n/);
    expect(out).toContain("a : Variable");
    expect(out).toContain("b : Variable");
  });

  test("throws for unknown formats with available list", () => {
    const pipeline = createDefaultPipeline();
    expect(() =>
      pipeline.run("", {
        format: "yaml",
        language: "ts",
        sourcePath: "x.ts",
      }),
    ).toThrow(/json, mermaid/);
  });
});
