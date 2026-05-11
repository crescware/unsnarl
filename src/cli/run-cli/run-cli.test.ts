import { existsSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { BOUNDARY_EDGE_DIRECTION } from "../../visual-graph/prune/boundary-edge-direction.js";
import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import { DEFAULT_GENERATIONS } from "../args/default-generations.js";
import { runCli } from "./run-cli.js";

type CapturedOutput = Readonly<{
  stdout: string;
  stderr: string;
}>;

async function captureRun(
  argv: readonly string[],
): Promise<{ exitCode: number } & CapturedOutput> {
  const stdout: /* mutable */ string[] = [];
  const stderr: /* mutable */ string[] = [];
  const stdoutSpy = vi
    .spyOn(process.stdout, "write")
    .mockImplementation((chunk: unknown) => {
      stdout.push(typeof chunk === "string" ? chunk : String(chunk));
      return true;
    });
  const stderrSpy = vi
    .spyOn(process.stderr, "write")
    .mockImplementation((chunk: unknown) => {
      stderr.push(typeof chunk === "string" ? chunk : String(chunk));
      return true;
    });
  try {
    const exitCode = await runCli(argv);
    return { exitCode, stdout: stdout.join(""), stderr: stderr.join("") };
  } finally {
    stdoutSpy.mockRestore();
    stderrSpy.mockRestore();
  }
}

describe("runCli (integration)", () => {
  let tmpDir: string;
  beforeEach(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "unsnarl-cli-"));
  });
  afterEach(() => {
    // Vitest 終了時に OS が回収するので削除不要
  });

  test("--help prints usage and exits 0", async () => {
    const r = await captureRun(["--help"]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toMatch(/Usage:/);
    expect(r.stdout).toMatch(/--format/);
  });

  test("happy path: analyzes a file and prints JSON IR", async () => {
    const inputPath = join(tmpDir, "input.ts");
    writeFileSync(
      inputPath,
      "const used = 1;\nconst answer = used;\nconst ignored = 2;\n",
    );
    const r = await captureRun([
      "--format",
      "ir",
      inputPath,
      "--no-pretty-json",
    ]);
    expect(r.exitCode).toEqual(0);
    const ir = JSON.parse(r.stdout);
    expect(ir.version).toEqual(SERIALIZED_IR_VERSION);
    expect(ir.source.path).toEqual(inputPath);
    expect(ir.variables.map((v: { name: string }) => v.name).sort()).toEqual([
      "answer",
      "ignored",
      "used",
    ]);
    expect(ir.unusedVariableIds.length).toEqual(2);
  });

  test("happy path: emits stats TSV", async () => {
    const inputPath = join(tmpDir, "stats.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun(["--format", "stats", inputPath]);
    expect(r.exitCode).toEqual(0);
    expect(r.stderr).toEqual("");
    const lines = r.stdout.trimEnd().split("\n");
    expect(lines).toEqual([
      `1\t0\t${inputPath}:1 a`,
      `0\t1\t${inputPath}:2 unused b`,
      "1\t1\t2 total",
    ]);
  });

  test("happy path: emits Mermaid output", async () => {
    const inputPath = join(tmpDir, "small.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun(["--format", "mermaid", inputPath]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toMatch(/^%%\{init:.*"elk".*\}%%\nflowchart RL\n/);
    expect(r.stdout).toContain('"a<br/>');
  });

  test("--mermaid-renderer dagre selects the dagre strategy (no elk init directive)", async () => {
    const inputPath = join(tmpDir, "dagre.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun([
      "--format",
      "mermaid",
      "--mermaid-renderer",
      "dagre",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout.startsWith("flowchart ")).toEqual(true);
    expect(r.stdout).not.toMatch(/^%%\{init:/);
  });

  test("--mermaid-renderer elk selects the elk strategy", async () => {
    const inputPath = join(tmpDir, "elk-explicit.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun([
      "--format",
      "mermaid",
      "--mermaid-renderer",
      "elk",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toMatch(/^%%\{init:.*"elk".*\}%%\nflowchart RL\n/);
  });

  test("--color-theme dark uses the dark palette literals (default)", async () => {
    const inputPath = join(tmpDir, "theme-dark.ts");
    writeFileSync(inputPath, "function f() { return 1; }\n");
    const r = await captureRun([
      "--format",
      "mermaid",
      "--color-theme",
      "dark",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    // The dark theme's fnWrap fill is #1a2030; without --color-theme the
    // same literal must appear because dark is the default.
    expect(r.stdout).toMatch(/classDef fnWrap fill:#1a2030,stroke:#5a7d99;/);
  });

  test("--color-theme light switches every classDef to the light palette", async () => {
    const inputPath = join(tmpDir, "theme-light.ts");
    writeFileSync(inputPath, "function f() { return 1; }\n");
    const r = await captureRun([
      "--format",
      "mermaid",
      "--color-theme",
      "light",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    // Dark literal must NOT appear under the light theme.
    expect(r.stdout).not.toMatch(/fill:#1a2030/);
    // Light theme fnWrap fill (#e0e8f0) must appear instead.
    expect(r.stdout).toMatch(/classDef fnWrap fill:#e0e8f0/);
  });

  test("--color-theme omitted defaults to dark", async () => {
    const inputPath = join(tmpDir, "theme-default.ts");
    writeFileSync(inputPath, "function f() { return 1; }\n");
    const r = await captureRun(["--format", "mermaid", inputPath]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toMatch(/classDef fnWrap fill:#1a2030,stroke:#5a7d99;/);
  });

  test("--color-theme with an unknown value exits with 2", async () => {
    const inputPath = join(tmpDir, "theme-invalid.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun([
      "--format",
      "mermaid",
      "--color-theme",
      "neon",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(2);
    expect(r.stderr).toMatch(/Invalid color theme: neon/);
  });

  test("--mermaid-renderer omitted falls back to elk", async () => {
    const inputPath = join(tmpDir, "renderer-default.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun(["--format", "mermaid", inputPath]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toMatch(/^%%\{init:.*"elk".*\}%%\nflowchart RL\n/);
  });

  test("default format is mermaid (no --format flag)", async () => {
    const inputPath = join(tmpDir, "default-format.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun([inputPath]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toMatch(/^%%\{init:.*"elk".*\}%%\nflowchart RL\n/);
  });

  test("--debug appends NODE_KIND to Mermaid node labels", async () => {
    const inputPath = join(tmpDir, "debug.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\nconsole.log(b);\n");
    const noDebug = await captureRun(["--format", "mermaid", inputPath]);
    const debug = await captureRun([
      "--format",
      "mermaid",
      "--debug",
      inputPath,
    ]);
    expect(noDebug.exitCode).toEqual(0);
    expect(debug.exitCode).toEqual(0);
    expect(noDebug.stdout).not.toContain("<br/>Variable");
    expect(debug.stdout).toContain('"a<br/>L1<br/>Variable"');
    expect(debug.stdout).toContain('"b<br/>L2<br/>Variable"');
  });

  test("missing input returns exit 2 with usage", async () => {
    const r = await captureRun([]);
    expect(r.exitCode).toEqual(2);
    expect(r.stderr).toMatch(/no input file/);
    expect(r.stderr).toMatch(/Usage:/);
  });

  test("unknown option returns exit 2", async () => {
    const r = await captureRun(["--whatever"]);
    expect(r.exitCode).toEqual(2);
    expect(r.stderr).toMatch(/unknown option/i);
  });

  test("parse error returns exit 1", async () => {
    const inputPath = join(tmpDir, "broken.ts");
    writeFileSync(inputPath, "const = 1;\n");
    const r = await captureRun([inputPath]);
    expect(r.exitCode).toEqual(1);
    expect(r.stderr).toMatch(/parse error/);
  });

  test("unknown emitter format returns exit 1", async () => {
    const inputPath = join(tmpDir, "ok.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun(["--format", "yaml", inputPath]);
    expect(r.exitCode).toEqual(1);
    expect(r.stderr).toMatch(/Unknown emitter format/);
  });

  test("--roots prunes the JSON output and adds pruning metadata", async () => {
    const inputPath = join(tmpDir, "chain.ts");
    writeFileSync(
      inputPath,
      "const a = 1;\nconst b = a;\nconst c = b;\nconst d = c;\n",
    );
    const r = await captureRun([
      "--format",
      "json",
      "-r",
      "1",
      "-C",
      "1",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stderr).toEqual("");
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning !== null && graph.pruning !== undefined).toEqual(true);
    expect(graph.pruning.descendants).toEqual(1);
    expect(graph.pruning.ancestors).toEqual(1);
    expect(graph.pruning.roots).toEqual([{ query: "1", matched: 1 }]);
    const names = graph.elements
      .filter((v: { type: string }) => v.type === VISUAL_ELEMENT_TYPE.Node)
      .map((v: { name: string }) => v.name);
    // Inner radius keeps a (root) and b (1 hop). c is past the requested
    // radius and is NOT in the kept node list; instead the outgoing edge
    // toward c shows up in boundaryEdges as a "more graph beyond here"
    // hint, and d (2 hops out) doesn't appear at all.
    expect(names).toContain("a");
    expect(names).toContain("b");
    expect(names).not.toContain("c");
    expect(names).not.toContain("d");
    expect(graph.boundaryEdges).toEqual([
      expect.objectContaining({ direction: BOUNDARY_EDGE_DIRECTION.Out }),
    ]);
    // out-direction boundary edges are intentionally label-less because
    // the action's actor (the unseen node beyond the boundary) is unknown.
    expect(graph.boundaryEdges[0]).not.toHaveProperty("label");
  });

  test("--roots emits a stderr warning for queries that match nothing", async () => {
    const inputPath = join(tmpDir, "tiny.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun(["--format", "mermaid", "-r", "999", inputPath]);
    expect(r.exitCode).toEqual(0);
    expect(r.stderr).toMatch(/uns: warning: query '999' matched 0 roots/);
    // Mermaid comment uses bracket-free, quote-free wording so older
    // Mermaid versions don't get tripped by `[` / `'` inside `%% ...`.
    expect(r.stdout).toMatch(/%% pruning warning query 999 matched 0 roots/);
  });

  test("ir format ignores --roots (no pruning, no warning)", async () => {
    const inputPath = join(tmpDir, "tiny2.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun([
      "--format",
      "ir",
      "-r",
      "999",
      inputPath,
      "--no-pretty-json",
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stderr).toEqual("");
    const ir = JSON.parse(r.stdout);
    expect(ir.variables.map((v: { name: string }) => v.name)).toEqual(["a"]);
  });

  test("--roots default generations match DEFAULT_GENERATIONS when no -A/-B/-C given", async () => {
    const inputPath = join(tmpDir, "default.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\nconst c = b;\n");
    const r = await captureRun(["--format", "json", "-r", "1", inputPath]);
    expect(r.exitCode).toEqual(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toEqual(DEFAULT_GENERATIONS);
    expect(graph.pruning.ancestors).toEqual(DEFAULT_GENERATIONS);
  });

  test("--roots with only -A gives -B 0 (asymmetric, like grep)", async () => {
    const inputPath = join(tmpDir, "only-a.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun([
      "--format",
      "json",
      "-r",
      "1",
      "-A",
      "6",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toEqual(6);
    expect(graph.pruning.ancestors).toEqual(0);
  });

  test("--roots with only -B gives -A 0 (asymmetric, like grep)", async () => {
    const inputPath = join(tmpDir, "only-b.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun([
      "--format",
      "json",
      "-r",
      "1",
      "-B",
      "5",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toEqual(0);
    expect(graph.pruning.ancestors).toEqual(5);
  });

  test("--roots with -C and -A: -A overrides one side, -C fills the other", async () => {
    const inputPath = join(tmpDir, "c-and-a.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun([
      "--format",
      "json",
      "-r",
      "1",
      "-C",
      "3",
      "-A",
      "7",
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toEqual(7);
    expect(graph.pruning.ancestors).toEqual(3);
  });

  // One full-dressing happy path: nested out-dir + -A -B -C all set +
  // non-default format. Demonstrates that args parsing -> name derivation
  // -> emitter extension lookup -> mkdir(recursive) -> writeFile is wired
  // through. Naming permutations (other -A/-B/-C combos, query forms)
  // are pure string transforms and live in resolve-output-path/derive-output-basename.test.ts.
  test("--out-dir writes a file under a not-yet-existing nested directory with the derived name", async () => {
    const inputPath = join(tmpDir, "smoke.ts");
    writeFileSync(inputPath, "const value = 1;\nconst other = value;\n");
    const outDir = join(tmpDir, "deeply", "nested", "out");
    const r = await captureRun([
      "--format",
      "markdown",
      "-r",
      "value",
      "-A",
      "1",
      "-B",
      "2",
      "-C",
      "3",
      "-o",
      outDir,
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toEqual("");
    // -C is dropped because -A and -B are both explicit; -C has no effect
    // once both sides are pinned.
    const expected = join(outDir, "value-a1-b2.md");
    expect(existsSync(expected)).toEqual(true);
    expect(readFileSync(expected, "utf8")).toMatch(/```mermaid/);
  });

  test("--out-dir without -r falls back to the input filename", async () => {
    const inputPath = join(tmpDir, "fooBar.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const outDir = join(tmpDir, "no-roots-out");
    const r = await captureRun([
      "--format",
      "mermaid",
      "-o",
      outDir,
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(existsSync(join(outDir, "fooBar.mmd"))).toEqual(true);
  });

  test("--out-dir overwrites an existing file", async () => {
    const inputPath = join(tmpDir, "overwrite.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const outDir = join(tmpDir, "overwrite-out");
    const first = await captureRun([
      "--format",
      "mermaid",
      "-o",
      outDir,
      inputPath,
    ]);
    expect(first.exitCode).toEqual(0);
    const target = join(outDir, "overwrite.mmd");
    const before = readFileSync(target, "utf8");

    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const second = await captureRun([
      "--format",
      "mermaid",
      "-o",
      outDir,
      inputPath,
    ]);
    expect(second.exitCode).toEqual(0);
    const after = readFileSync(target, "utf8");
    expect(after).not.toEqual(before);
  });

  test("--out-dir with --stdin and no -r exits with 2 (no naming basis)", async () => {
    const outDir = join(tmpDir, "stdin-out");
    // run-cli reads stdin before validating --out-dir naming, so we have to
    // feed it an immediate EOF or the await blocks on the test harness's
    // open stdin.
    const stdinSpy = vi
      .spyOn(
        process.stdin as unknown as AsyncIterable<Buffer>,
        Symbol.asyncIterator,
      )
      .mockImplementation(
        () => (async function* () {})() as AsyncIterator<Buffer>,
      );
    try {
      const r = await captureRun([
        "--stdin",
        "--stdin-lang",
        "ts",
        "-o",
        outDir,
      ]);
      expect(r.exitCode).toEqual(2);
      expect(r.stderr).toMatch(/-r\/--roots|input file/);
      expect(existsSync(outDir)).toEqual(false);
    } finally {
      stdinSpy.mockRestore();
    }
  });

  test("--out-file writes the result to the exact given path", async () => {
    const inputPath = join(tmpDir, "explicit.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const outFile = join(tmpDir, "explicit-out", "graph.mmd");
    const r = await captureRun([
      "--format",
      "mermaid",
      "--out-file",
      outFile,
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stdout).toEqual("");
    expect(existsSync(outFile)).toEqual(true);
    expect(readFileSync(outFile, "utf8")).toMatch(/```mermaid|flowchart/);
  });

  test("-o with a dot in the basename emits a notice but still treats it as a directory", async () => {
    const inputPath = join(tmpDir, "withdot.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const outDir = join(tmpDir, "looks-like-file.json");
    const r = await captureRun([
      "--format",
      "mermaid",
      "-o",
      outDir,
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stderr).toMatch(/uns: notice: -o/);
    expect(r.stderr).toMatch(/--out-file/);
    expect(existsSync(join(outDir, "withdot.mmd"))).toEqual(true);
  });

  test("-o without a dot does not emit the --out-file notice", async () => {
    const inputPath = join(tmpDir, "nodot.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const outDir = join(tmpDir, "plain-out");
    const r = await captureRun([
      "--format",
      "mermaid",
      "-o",
      outDir,
      inputPath,
    ]);
    expect(r.exitCode).toEqual(0);
    expect(r.stderr).not.toMatch(/uns: notice: -o/);
  });

  test("-o and --out-file together exits with 2", async () => {
    const inputPath = join(tmpDir, "both.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun([
      "--format",
      "mermaid",
      "-o",
      join(tmpDir, "out"),
      "--out-file",
      join(tmpDir, "out.mmd"),
      inputPath,
    ]);
    expect(r.exitCode).toEqual(2);
    expect(r.stderr).toMatch(/mutually exclusive/);
  });
});
