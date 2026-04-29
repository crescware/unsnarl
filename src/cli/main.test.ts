import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { runCli } from "./main.js";

interface CapturedOutput {
  stdout: string;
  stderr: string;
}

async function captureRun(
  argv: ReadonlyArray<string>,
): Promise<{ exitCode: number } & CapturedOutput> {
  const stdout: string[] = [];
  const stderr: string[] = [];
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

  test("--list-formats lists json and mermaid", async () => {
    const r = await captureRun(["--list-formats"]);
    expect(r.exitCode).toBe(0);
    expect(r.stdout).toContain("json");
    expect(r.stdout).toContain("mermaid");
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

  test("happy path: emits Mermaid output", async () => {
    const inputPath = join(tmpDir, "small.ts");
    writeFileSync(inputPath, "const a = 1;\nconst b = a;\n");
    const r = await captureRun(["--format", "mermaid", inputPath]);
    expect(r.exitCode).toBe(0);
    expect(r.stdout).toMatch(/^flowchart RL\n/);
    expect(r.stdout).toContain("a : Variable");
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
});
