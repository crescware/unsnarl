#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run
// Aggregator for `scripts/bench-parity.ts`. Runs the full
// (format x variant) matrix against every `.ts` / `.tsx` file
// under `ts/src`, collecting each sweep's per-file PASS / FAIL
// counts into a single matrix report.
//
// Sweeps are launched sequentially (one child Deno process per
// (format, variant) pair) so the wall-clock total is the sum of
// the per-sweep runtimes. Each child writes into its own
// `target/parity-bench-<format>-<variant>/` work tree, leaving
// the per-sweep artifacts intact for later inspection. The
// aggregator never stops on failure: every (format, variant)
// completes and the final report flags which ones did not match
// byte-for-byte.
//
// Exit code: 0 if every sweep matched byte-for-byte; 1 otherwise.

const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");

const FORMATS = ["ir", "json", "mermaid", "markdown", "stats"] as const;
const VARIANTS = [
  "baseline",
  "r-a-1",
  "r-b-1",
  "r-c-1",
  "h-mid",
  "depth-1",
] as const;

type SweepResult = Readonly<{
  format: (typeof FORMATS)[number];
  variant: (typeof VARIANTS)[number];
  status: "PASS" | "FAIL" | "ERROR";
  total: number;
  pass: number;
  fail: number;
  rustMs: number;
  tsMs: number;
  wallMs: number;
}>;

function parseSummary(
  format: (typeof FORMATS)[number],
  variant: (typeof VARIANTS)[number],
  summaryPath: string,
  wallMs: number,
  childOk: boolean,
): SweepResult {
  let total = 0;
  let pass = 0;
  let fail = 0;
  let rustMs = 0;
  let tsMs = 0;
  let status: SweepResult["status"] = childOk ? "PASS" : "ERROR";
  try {
    const text = Deno.readTextFileSync(summaryPath);
    for (const line of text.split("\n")) {
      const [k, v] = line.split("=");
      if (v === undefined) continue;
      const value = v.split("\t")[0];
      switch (k) {
        case "files_total":
          total = Number(value);
          break;
        case "files_pass":
          pass = Number(value);
          break;
        case "files_fail":
          fail = Number(value);
          break;
        case "rust_total_ms":
          rustMs = Number(value);
          break;
        case "ts_total_ms":
          tsMs = Number(value);
          break;
        case "status":
          if (value === "PASS" || value === "FAIL") status = value;
          break;
      }
    }
  } catch {
    // No summary file at all means the child crashed before
    // writing one. Treat that as ERROR.
    status = "ERROR";
  }
  return { format, variant, status, total, pass, fail, rustMs, tsMs, wallMs };
}

const results: SweepResult[] = [];
const overallStart = performance.now();
for (const format of FORMATS) {
  for (const variant of VARIANTS) {
    const workDir =
      variant === "baseline"
        ? `${REPO_ROOT}/target/parity-bench-${format}`
        : `${REPO_ROOT}/target/parity-bench-${format}-${variant}`;
    const summaryPath = `${workDir}/summary.txt`;
    console.error(`\n=== sweep: format=${format} variant=${variant} ===`);
    const start = performance.now();
    const child = await new Deno.Command("deno", {
      args: [
        "run",
        "--allow-read",
        "--allow-write",
        "--allow-run",
        `${REPO_ROOT}/scripts/bench-parity.ts`,
        format,
        variant,
      ],
      stdout: "inherit",
      stderr: "inherit",
    }).output();
    const wallMs = Math.round(performance.now() - start);
    results.push(parseSummary(format, variant, summaryPath, wallMs, child.success));
  }
}
const overallWallMs = Math.round(performance.now() - overallStart);

const rows = [
  ["format", "variant", "status", "pass/total", "rust_ms", "ts_ms", "wall_ms"],
];
for (const r of results) {
  rows.push([
    r.format,
    r.variant,
    r.status,
    `${r.pass}/${r.total}`,
    String(r.rustMs),
    String(r.tsMs),
    String(r.wallMs),
  ]);
}

function renderTable(rows: string[][]): string {
  const widths = rows[0].map((_, c) =>
    Math.max(...rows.map((r) => r[c].length)),
  );
  return rows
    .map((r) =>
      r.map((cell, c) => cell.padEnd(widths[c], " ")).join("  "),
    )
    .join("\n");
}

const failed = results.filter((r) => r.status !== "PASS");
console.log("\n=== aggregate ===");
console.log(renderTable(rows));
console.log("");
console.log(`sweeps_total = ${results.length}`);
console.log(`sweeps_pass  = ${results.length - failed.length}`);
console.log(`sweeps_fail  = ${failed.length}`);
console.log(`wallclock_total_ms = ${overallWallMs}`);

if (failed.length > 0) {
  console.error("\nFAILED sweeps:");
  for (const r of failed) {
    console.error(
      `  format=${r.format} variant=${r.variant} status=${r.status} fail_files=${r.fail}`,
    );
  }
  Deno.exit(1);
}
