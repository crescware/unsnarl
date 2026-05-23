#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run
// Run `uns -f stats` against every `.js` / `.ts` file under
// `ts/node_modules` (excluding `*.d.ts`) and rank them by per-file
// wall-clock time in milliseconds. Sequential by design so the
// timings are not contaminated by parallel CPU contention.
//
// Output (under `target/node-modules-bench/`):
//   timings.tsv   rank<TAB>elapsed_ms<TAB>exit<TAB>path
//   ranking.txt   human-readable top-N preview + totals
//   summary.txt   aggregate stats (total / mean / median / p95 / max)

const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");

const UNS_BIN = `${REPO_ROOT}/target/release/uns`;
const TARGET_DIR = `${REPO_ROOT}/ts/node_modules`;
const WORK = `${REPO_ROOT}/target/node-modules-bench`;
const TOP_N = 50;

function ensureFile(path: string, hint: string) {
  try {
    const s = Deno.statSync(path);
    if (!s.isFile) throw new Error("not a file");
  } catch {
    console.error(`missing ${path} -- ${hint}`);
    Deno.exit(1);
  }
}
function ensureDir(path: string, hint: string) {
  try {
    const s = Deno.statSync(path);
    if (!s.isDirectory) throw new Error("not a directory");
  } catch {
    console.error(`missing ${path} -- ${hint}`);
    Deno.exit(1);
  }
}
ensureFile(UNS_BIN, "run `mise run build` first");
ensureDir(TARGET_DIR, "run `pnpm -C ts install` first");

function rmrf(path: string) {
  try {
    Deno.removeSync(path, { recursive: true });
  } catch (e) {
    if (!(e instanceof Deno.errors.NotFound)) throw e;
  }
}
rmrf(WORK);
Deno.mkdirSync(WORK, { recursive: true });

function* walk(dir: string): Generator<string> {
  for (const e of Deno.readDirSync(dir)) {
    const p = `${dir}/${e.name}`;
    if (e.isDirectory) {
      yield* walk(p);
    } else if (e.isFile) {
      // Exclude `*.d.ts` declaration files; they are not source.
      if (e.name.endsWith(".d.ts")) continue;
      if (e.name.endsWith(".js") || e.name.endsWith(".ts")) yield p;
    }
  }
}

const files = [...walk(TARGET_DIR)].sort();
const total = files.length;
console.error(`target files: ${total}`);

type Row = { ms: number; code: number; path: string };
const rows: Row[] = [];

const benchStart = performance.now();
let i = 0;
for (const abs of files) {
  i++;
  // Run `uns` with stdout / stderr discarded so the I/O cost is
  // dominated by the analyser itself rather than terminal pipe
  // buffering. The per-file wall clock is taken around the spawn
  // exactly the same way `scripts/bench-parity.ts` does it.
  const start = performance.now();
  const out = await new Deno.Command(UNS_BIN, {
    args: ["-f", "stats", abs],
    stdout: "null",
    stderr: "null",
  }).output();
  const ms = performance.now() - start;
  rows.push({ ms, code: out.code, path: abs });

  if (i % 250 === 0) {
    const wall = (performance.now() - benchStart) / 1000;
    const rate = i / wall;
    const eta = (total - i) / rate;
    console.error(
      `progress: ${i}/${total} (${rate.toFixed(1)} files/s, eta ${eta.toFixed(0)}s)`,
    );
  }
}
const wallMs = performance.now() - benchStart;

rows.sort((a, b) => b.ms - a.ms);

const rootPrefix = REPO_ROOT + "/";
function relOf(p: string): string {
  return p.startsWith(rootPrefix) ? p.slice(rootPrefix.length) : p;
}

const timingsRows: string[] = ["rank\telapsed_ms\texit\tpath"];
for (let r = 0; r < rows.length; r++) {
  const row = rows[r];
  timingsRows.push(
    `${r + 1}\t${row.ms.toFixed(3)}\t${row.code}\t${relOf(row.path)}`,
  );
}
Deno.writeTextFileSync(`${WORK}/timings.tsv`, timingsRows.join("\n") + "\n");

const rankingLines: string[] = [];
rankingLines.push(`Top ${TOP_N} slowest files (of ${total})`);
rankingLines.push("rank   elapsed_ms   exit   path");
for (let r = 0; r < Math.min(TOP_N, rows.length); r++) {
  const row = rows[r];
  rankingLines.push(
    `${String(r + 1).padStart(4)}   ${row.ms.toFixed(3).padStart(10)}   ${String(row.code).padStart(4)}   ${relOf(row.path)}`,
  );
}
Deno.writeTextFileSync(`${WORK}/ranking.txt`, rankingLines.join("\n") + "\n");

const totalMs = rows.reduce((a, b) => a + b.ms, 0);
const meanMs = total > 0 ? totalMs / total : 0;
const sortedAsc = rows.map((r) => r.ms).sort((a, b) => a - b);
const median = sortedAsc.length === 0
  ? 0
  : sortedAsc.length % 2 === 1
    ? sortedAsc[(sortedAsc.length - 1) >> 1]
    : (sortedAsc[sortedAsc.length / 2 - 1] + sortedAsc[sortedAsc.length / 2]) / 2;
const p95 = sortedAsc.length === 0
  ? 0
  : sortedAsc[Math.min(sortedAsc.length - 1, Math.floor(sortedAsc.length * 0.95))];
const nonzero = rows.filter((r) => r.code !== 0).length;

const summaryLines = [
  `files_total=${total}`,
  `nonzero_exit=${nonzero}`,
  `wallclock_total_ms=${wallMs.toFixed(0)}`,
  `uns_total_ms=${totalMs.toFixed(0)}`,
  `mean_ms_per_file=${meanMs.toFixed(3)}`,
  `median_ms=${median.toFixed(3)}`,
  `p95_ms=${p95.toFixed(3)}`,
  `max_ms=${rows.length > 0 ? rows[0].ms.toFixed(3) : "0"}\t${rows.length > 0 ? relOf(rows[0].path) : ""}`,
];
const summary = summaryLines.join("\n") + "\n";
Deno.stdout.writeSync(new TextEncoder().encode(summary));
Deno.writeTextFileSync(`${WORK}/summary.txt`, summary);

const preview = rankingLines.slice(0, 1 + 1 + Math.min(TOP_N, 20)).join("\n") + "\n";
Deno.stdout.writeSync(new TextEncoder().encode("\n" + preview));
