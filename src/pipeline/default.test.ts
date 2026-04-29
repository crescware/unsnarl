import { describe, expect, test } from "vitest";

import {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "./default.js";

describe("createDefaultPipeline", () => {
  test("registers ir, json, mermaid, and markdown emitters by default", () => {
    const reg = createDefaultEmitterRegistry();
    expect([...reg.list()].sort()).toEqual([
      "ir",
      "json",
      "markdown",
      "mermaid",
    ]);
  });

  test("end-to-end: emits VisualGraph JSON for the same input", () => {
    const pipeline = createDefaultPipeline();
    const out = pipeline.run("const a = 1;\nconst b = a;\n", {
      format: "json",
      language: "ts",
      sourcePath: "input.ts",
      emit: { pretty: false },
    });
    const graph = JSON.parse(out);
    expect(graph.version).toBe(1);
    expect(graph.direction).toBe("RL");
    expect(
      graph.elements
        .filter((e: { type: string }) => e.type === "node")
        .map((n: { name: string }) => n.name)
        .sort(),
    ).toEqual(["a", "b"]);
  });

  test("end-to-end: parses TS, analyzes, serializes, emits IR JSON", () => {
    const pipeline = createDefaultPipeline();
    const out = pipeline.run("const a = 1;\nconst b = a;\n", {
      format: "ir",
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
    expect(out).toMatch(/^%%\{init:.*"elk".*\}%%\nflowchart RL\n/);
    expect(out).toContain('"a<br/>');
    expect(out).toContain('"b<br/>');
  });

  test("throws for unknown formats with available list", () => {
    const pipeline = createDefaultPipeline();
    expect(() =>
      pipeline.run("", {
        format: "yaml",
        language: "ts",
        sourcePath: "x.ts",
      }),
    ).toThrow(/ir, json/);
  });
});
