import { describe, expect, test } from "vitest";

import { EslintCompatAnalyzer } from "../../analyzer/eslint-compat/eslint-compat.js";
import { BOUNDARY_EDGE_DIRECTION } from "../../constants.js";
import { OxcParser } from "../../parser/oxc.js";
import { FlatSerializer } from "../../serializer/flat/flat-serializer.js";
import { buildVisualGraph } from "../../visual-graph/builder.js";
import type { VisualGraph } from "../../visual-graph/model.js";
import { StatsEmitter } from "./stats.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();
const emitter = new StatsEmitter();

function emit(code: string): string {
  const parsed = parser.parse(code, {
    language: "ts",
    sourcePath: "x.ts",
  });
  const analyzed = analyzer.analyze(parsed);
  const ir = serializer.serialize({
    rootScope: analyzed.rootScope,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: "x.ts", language: "ts" },
  });
  return emitter.emit(ir, {});
}

function emitWithBoundary(
  code: string,
  patch: (graph: VisualGraph) => VisualGraph,
): string {
  const parsed = parser.parse(code, {
    language: "ts",
    sourcePath: "x.ts",
  });
  const analyzed = analyzer.analyze(parsed);
  const ir = serializer.serialize({
    rootScope: analyzed.rootScope,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: "x.ts", language: "ts" },
  });
  const prunedGraph = patch(buildVisualGraph(ir));
  return emitter.emit(ir, { prunedGraph });
}

describe("StatsEmitter", () => {
  test("identifies as 'stats' with TSV content type", () => {
    expect(emitter.format).toBe("stats");
    expect(emitter.contentType).toBe("text/tab-separated-values");
  });

  test("emits one TSV row per node followed by a total summary", () => {
    const out = emit("const a = 1;\nconst b = a;\n");
    const lines = out.trimEnd().split("\n");
    expect(lines).toEqual([
      "1\t0\tx.ts:1 a",
      "0\t1\tx.ts:2 unused b",
      "1\t1\t2 total",
    ]);
  });

  test("zero-edge nodes report 0/0", () => {
    const out = emit("const a = 1;\n");
    expect(out.trimEnd().split("\n")[0]).toBe("0\t0\tx.ts:1 unused a");
  });

  test("renders ? for the direction touched by a boundary edge", () => {
    const out = emitWithBoundary(
      "const a = 1;\nconst b = a;\nconst c = b;\nconst d = c;\n",
      (graph) => {
        // Pretend pruning kept only {a, b} and clipped the chain after b.
        const keepIds = new Set(["n_scope_0_a_6", "n_scope_0_b_19"]);
        return {
          ...graph,
          elements: graph.elements.filter(
            (e) => e.type === "node" && keepIds.has(e.id),
          ),
          edges: graph.edges.filter(
            (e) => keepIds.has(e.from) && keepIds.has(e.to),
          ),
          boundaryEdges: [
            {
              inside: "n_scope_0_b_19",
              direction: BOUNDARY_EDGE_DIRECTION.Out,
            },
          ],
        };
      },
    );
    const lines = out.trimEnd().split("\n");
    // a still has its full a -> b edge in the kept graph.
    expect(lines[0]).toBe("1\t0\tx.ts:1 a");
    // b's outbound side is cut by the boundary so it's "?", not 0.
    // (b is read by c in the source so it's NOT marked unused.)
    expect(lines[1]).toBe("?\t1\tx.ts:2 b");
    // The unknown propagates into the total's descendants column.
    expect(lines[2]).toBe("?\t1\t2 total");
  });

  test("output is newline-terminated for shell-friendly piping", () => {
    const out = emit("const a = 1;\n");
    expect(out.endsWith("\n")).toBe(true);
  });

  test("rows are sorted by line, ascending", () => {
    // The IR happens to surface these in declaration order, but force
    // a noticeable line jump (1 -> 3 -> 2) to make sure rows really
    // come out 1 -> 2 -> 3 regardless of what the IR walk yields.
    const out = emit("const a = 1;\nconst c = 2;\nconst b = a + c;\n");
    const lines = out.trimEnd().split("\n").slice(0, -1);
    const lineNumbers = lines.map((l) => {
      const m = /:(\d+) /.exec(l);
      if (m === null) {
        throw new Error(`no line number in row: ${l}`);
      }
      return Number.parseInt(m[1] ?? "0", 10);
    });
    expect(lineNumbers).toEqual([...lineNumbers].sort((x, y) => x - y));
  });

  test("preserves source order for nodes that share a line (stable sort)", () => {
    // a and b both live on line 1; the stable line-only sort must keep
    // them in their original IR order so editors jumping to the row
    // for "a" don't accidentally land on "b" first.
    const out = emit("const a = 1, b = 2;\n");
    const lines = out.trimEnd().split("\n").slice(0, -1);
    expect(lines).toEqual(["0\t0\tx.ts:1 unused a", "0\t0\tx.ts:1 unused b"]);
  });
});
