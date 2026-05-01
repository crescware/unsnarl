import { existsSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { DEFAULT_GENERATIONS } from "../args/cli-args.js";
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

describe("runCli (end-to-end)", () => {
  let tmpDir: string;
  beforeEach(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "unsnarl-cli-"));
  });
  afterEach(() => {
    // Vitest 終了時に OS が回収するので削除不要
  });

  test("--version prints 0.0.0 and exits 0", async () => {
    const r = await captureRun(["--version"]);
    expect(r.exitCode).toBe(0);
    expect(r.stdout.trim()).toBe("0.0.0");
  });

  test("--help prints usage and exits 0", async () => {
    const r = await captureRun(["--help"]);
    expect(r.exitCode).toBe(0);
    expect(r.stdout).toMatch(/Usage:/);
    expect(r.stdout).toMatch(/--format/);
  });

  test("--list-formats lists ir, json, mermaid, markdown, and stats", async () => {
    const r = await captureRun(["--list-formats"]);
    expect(r.exitCode).toBe(0);
    expect(r.stdout).toContain("ir");
    expect(r.stdout).toContain("json");
    expect(r.stdout).toContain("mermaid");
    expect(r.stdout).toContain("markdown");
    expect(r.stdout).toContain("stats");
  });

  test("happy path: analyzes a file and prints JSON IR", async () => {
    const inputPath = join(tmpDir, "input.ts");
    writeFileSync(
      inputPath,
      "const used = 1;\nconst answer = used;\nconst ignored = 2;\n",
    );
    const r = await captureRun([inputPath, "--no-pretty"]);
    expect(r.exitCode).toBe(0);
    const ir = JSON.parse(r.stdout);
    expect(ir.version).toBe(1);
    expect(ir.source.path).toBe(inputPath);
    expect(ir.variables.map((v: { name: string }) => v.name).sort()).toEqual([
      "answer",
      "ignored",
      "used",
    ]);
    expect(ir.unusedVariableIds.length).toBe(2);
  });

  test("happy path: emits stats TSV", async () => {
    const inputPath = join(tmpDir, "stats.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun(["--format", "stats", inputPath]);
    expect(r.exitCode).toBe(0);
    expect(r.stderr).toBe("");
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
    expect(r.exitCode).toBe(0);
    expect(r.stdout).toMatch(/^%%\{init:.*"elk".*\}%%\nflowchart RL\n/);
    expect(r.stdout).toContain('"a<br/>');
  });

  test("missing input returns exit 2 with usage", async () => {
    const r = await captureRun([]);
    expect(r.exitCode).toBe(2);
    expect(r.stderr).toMatch(/no input file/);
    expect(r.stderr).toMatch(/Usage:/);
  });

  test("unknown option returns exit 2", async () => {
    const r = await captureRun(["--whatever"]);
    expect(r.exitCode).toBe(2);
    expect(r.stderr).toMatch(/Unknown option/);
  });

  test("parse error returns exit 1", async () => {
    const inputPath = join(tmpDir, "broken.ts");
    writeFileSync(inputPath, "const = 1;\n");
    const r = await captureRun([inputPath]);
    expect(r.exitCode).toBe(1);
    expect(r.stderr).toMatch(/parse error/);
  });

  test("unknown emitter format returns exit 1", async () => {
    const inputPath = join(tmpDir, "ok.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun(["--format", "yaml", inputPath]);
    expect(r.exitCode).toBe(1);
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
    expect(r.exitCode).toBe(0);
    expect(r.stderr).toBe("");
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning).toBeDefined();
    expect(graph.pruning.descendants).toBe(1);
    expect(graph.pruning.ancestors).toBe(1);
    expect(graph.pruning.roots).toEqual([{ query: "1", matched: 1 }]);
    const names = graph.elements
      .filter((e: { type: string }) => e.type === "node")
      .map((e: { name: string }) => e.name);
    // Inner radius keeps a (root) and b (1 hop). c is past the requested
    // radius and is NOT in the kept node list; instead the outgoing edge
    // toward c shows up in boundaryEdges as a "more graph beyond here"
    // hint, and d (2 hops out) doesn't appear at all.
    expect(names).toContain("a");
    expect(names).toContain("b");
    expect(names).not.toContain("c");
    expect(names).not.toContain("d");
    expect(graph.boundaryEdges).toEqual([
      expect.objectContaining({ direction: "out" }),
    ]);
    // out-direction boundary edges are intentionally label-less because
    // the action's actor (the unseen node beyond the boundary) is unknown.
    expect(graph.boundaryEdges[0]).not.toHaveProperty("label");
  });

  test("--roots emits a stderr warning for queries that match nothing", async () => {
    const inputPath = join(tmpDir, "tiny.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun(["--format", "mermaid", "-r", "999", inputPath]);
    expect(r.exitCode).toBe(0);
    expect(r.stderr).toMatch(/unsnarl: warning: query '999' matched 0 roots/);
    // Mermaid comment uses bracket-free, quote-free wording so older
    // Mermaid versions don't get tripped by `[` / `'` inside `%% ...`.
    expect(r.stdout).toMatch(/%% pruning warning query 999 matched 0 roots/);
  });

  test("ir format ignores --roots (no pruning, no warning)", async () => {
    const inputPath = join(tmpDir, "tiny2.ts");
    writeFileSync(inputPath, "const a = 1;\n");
    const r = await captureRun(["-r", "999", inputPath, "--no-pretty"]);
    expect(r.exitCode).toBe(0);
    expect(r.stderr).toBe("");
    const ir = JSON.parse(r.stdout);
    expect(ir.variables.map((v: { name: string }) => v.name)).toEqual(["a"]);
  });

  test("--roots default generations match DEFAULT_GENERATIONS when no -A/-B/-C given", async () => {
    const inputPath = join(tmpDir, "default.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\nconst c = b;\n");
    const r = await captureRun(["--format", "json", "-r", "1", inputPath]);
    expect(r.exitCode).toBe(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toBe(DEFAULT_GENERATIONS);
    expect(graph.pruning.ancestors).toBe(DEFAULT_GENERATIONS);
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
    expect(r.exitCode).toBe(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toBe(6);
    expect(graph.pruning.ancestors).toBe(0);
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
    expect(r.exitCode).toBe(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toBe(0);
    expect(graph.pruning.ancestors).toBe(5);
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
    expect(r.exitCode).toBe(0);
    const graph = JSON.parse(r.stdout);
    expect(graph.pruning.descendants).toBe(7);
    expect(graph.pruning.ancestors).toBe(3);
  });

  // One full-dressing happy path: nested out-dir + -A -B -C all set +
  // non-default format. Demonstrates that args parsing -> name derivation
  // -> emitter extension lookup -> mkdir(recursive) -> writeFile is wired
  // end-to-end. Naming permutations (other -A/-B/-C combos, query forms)
  // are pure string transforms and live in output-name.test.ts.
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
    expect(r.exitCode).toBe(0);
    expect(r.stdout).toBe("");
    // -C is dropped because -A and -B are both explicit; -C has no effect
    // once both sides are pinned.
    const expected = join(outDir, "value-a1-b2.md");
    expect(existsSync(expected)).toBe(true);
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
    expect(r.exitCode).toBe(0);
    expect(existsSync(join(outDir, "fooBar.mmd"))).toBe(true);
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
    expect(first.exitCode).toBe(0);
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
    expect(second.exitCode).toBe(0);
    const after = readFileSync(target, "utf8");
    expect(after).not.toBe(before);
  });

  test("--out-dir with --stdin and no -r exits with 2 (no naming basis)", async () => {
    const outDir = join(tmpDir, "stdin-out");
    const r = await captureRun(["--stdin", "--lang", "ts", "-o", outDir]);
    expect(r.exitCode).toBe(2);
    expect(r.stderr).toMatch(/-r\/--roots|input file/);
    expect(existsSync(outDir)).toBe(false);
  });
});
