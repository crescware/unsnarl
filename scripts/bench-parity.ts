#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run
// Byte-for-byte parity + per-file timing between the Rust
// implementation (`target/release/uns`) and the TypeScript
// implementation (`ts/dist/index.js`) for the selected emitter
// format (`-f ir`, `-f json`, or `-f mermaid`).
//
// Iterates every `.ts` / `.tsx` file under `ts/src`, runs both
// implementations with the chosen format, and compares the emitter
// output (stdout) byte stream. Stderr from each implementation is
// captured separately so it never pollutes the comparison.
//
// Per-file wall-clock times in milliseconds are recorded for each
// implementation. Defaults to writing all artifacts under
// `target/parity-bench-<format>/` so the per-format runs do not
// clobber each other (and so the whole tree stays git-ignored
// alongside the Cargo target directory).
//
// For the mermaid format both implementations are launched with
// the CLI defaults (`--mermaid-renderer elk`, `--color-theme dark`,
// `--debug` off), so passing only `-f mermaid` matches the on-disk
// `expected.mermaid` baselines.
//
// Usage:
//   mise run bench:ir-parity                          # ir, default work dir
//   mise run bench:ir-parity      -- path/to/dir      # ir, custom work dir
//   mise run bench:json-parity                        # json, default work dir
//   mise run bench:json-parity    -- path/to/dir      # json, custom work dir
//   mise run bench:mermaid-parity                     # mermaid, default work dir
//   mise run bench:mermaid-parity -- path/to/dir      # mermaid, custom work dir
//
// Outputs (under the work dir):
//   summary.txt         human-readable totals + "smallest diffs first" preview
//   timings.tsv         file<TAB>rust_ms<TAB>ts_ms<TAB>match
//   fail_list.txt       paths whose stdouts differ
//   failures.tsv        file<TAB>diff_line_count, sorted ascending
//   diff/<safe>.diff    unified diff (TS vs Rust) for each mismatch
//   rust/, ts/          raw emitter stdouts (mismatches only; matches are deleted)
//   stderr/             non-empty stderr from either implementation
//
// Exit code: 0 if every file matched byte-for-byte, 1 otherwise.

// Script lives at `scripts/bench-parity.ts`; the repo root is two
// `/`s up.
const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");

const TS_ROOT = `${REPO_ROOT}/ts`;
const RUST_BIN = `${REPO_ROOT}/target/release/uns`;
const TS_BIN = `${TS_ROOT}/dist/index.js`;

// First positional arg picks the emitter; defaults to `ir` so the
// behaviour the previous version of this script had survives any
// caller that still invokes it without an explicit format.
const FORMAT = (Deno.args[0] ?? "ir").toLowerCase();
if (FORMAT !== "ir" && FORMAT !== "json" && FORMAT !== "mermaid") {
  console.error(
    `bench-parity: unsupported format '${FORMAT}' (expected 'ir', 'json', or 'mermaid')`,
  );
  Deno.exit(2);
}
// Second positional arg overrides the work dir. The default work
// dir is suffixed by format so `bench:ir-parity`,
// `bench:json-parity`, and `bench:mermaid-parity` write into
// separate trees.
const WORK = Deno.args[1] ?? `${REPO_ROOT}/target/parity-bench-${FORMAT}`;

function ensureFile(path: string, hint: string) {
  try {
    const s = Deno.statSync(path);
    if (!s.isFile) throw new Error("not a file");
  } catch {
    console.error(`missing ${path} -- ${hint}`);
    Deno.exit(1);
  }
}
ensureFile(RUST_BIN, "run `mise run build` first");
ensureFile(TS_BIN, "run `pnpm -C ts build` first");

function rmrf(path: string) {
  try {
    Deno.removeSync(path, { recursive: true });
  } catch (e) {
    if (!(e instanceof Deno.errors.NotFound)) throw e;
  }
}
rmrf(WORK);
for (const sub of ["rust", "ts", "diff", "stderr"]) {
  Deno.mkdirSync(`${WORK}/${sub}`, { recursive: true });
}

function* walk(dir: string): Generator<string> {
  for (const e of Deno.readDirSync(dir)) {
    const p = `${dir}/${e.name}`;
    if (e.isDirectory) yield* walk(p);
    else if (e.isFile && (e.name.endsWith(".ts") || e.name.endsWith(".tsx"))) yield p;
  }
}

const files = [...walk(`${TS_ROOT}/src`)]
  .map((abs) => abs.slice(TS_ROOT.length + 1))
  .sort();
const total = files.length;

function shQuote(s: string): string {
  return `'${s.replaceAll("'", "'\\''")}'`;
}

// Spawn `cmd` with stdout / stderr redirected directly to the named
// files via the system shell. The previous in-process variant
// (`stdout: "piped"` + `.output()`) truncated `node ... -f ir` to
// 64 KiB: Node's pipe writes are asynchronous and `process.exit()`
// does not wait for the user-space buffer to flush, so anything past
// the OS pipe buffer is discarded. Routing stdout to a file fd from
// the start (the same shape the original .sh harness used)
// sidesteps that path entirely.
async function runToFiles(
  cmd: string[],
  stdoutPath: string,
  stderrPath: string,
): Promise<{ ok: boolean; ms: number }> {
  const cmdline = `${cmd.map(shQuote).join(" ")} > ${shQuote(stdoutPath)} 2> ${shQuote(stderrPath)}`;
  const start = performance.now();
  const out = await new Deno.Command("sh", {
    args: ["-c", cmdline],
    cwd: TS_ROOT,
    stdout: "null",
    stderr: "null",
  }).output();
  const ms = Math.round(performance.now() - start);
  return { ok: out.success, ms };
}

function fileSizeOrZero(path: string): number {
  try {
    return Deno.statSync(path).size;
  } catch {
    return 0;
  }
}

async function filesEqual(a: string, b: string): Promise<boolean> {
  if (fileSizeOrZero(a) !== fileSizeOrZero(b)) return false;
  // Delegate to system `cmp -s`: exit 0 iff identical. Streams the
  // bytes; no need to load either file into memory.
  const res = await new Deno.Command("cmp", {
    args: ["-s", a, b],
    stdout: "null",
    stderr: "null",
  }).output();
  return res.success;
}

const timingsRows: string[] = ["file\trust_ms\tts_ms\tmatch"];
const failList: string[] = [];
let pass = 0;
let fail = 0;
let rustErr = 0;
let tsErr = 0;
let rustTotalMs = 0;
let tsTotalMs = 0;
let rustMaxMs = 0;
let tsMaxMs = 0;
let rustMaxFile = "";
let tsMaxFile = "";

const benchStart = performance.now();
let i = 0;
for (const rel of files) {
  i++;
  const safe = rel.replaceAll("/", "__");
  const rOut = `${WORK}/rust/${safe}.out`;
  const tOut = `${WORK}/ts/${safe}.out`;
  const rErr = `${WORK}/stderr/${safe}.rust.err`;
  const tErr = `${WORK}/stderr/${safe}.ts.err`;

  const r = await runToFiles([RUST_BIN, "-f", FORMAT, rel], rOut, rErr);
  const t = await runToFiles(["node", TS_BIN, "-f", FORMAT, rel], tOut, tErr);

  // Mirror the .sh harness: empty stderr files are pruned so the
  // `stderr/` directory only contains rows that actually said
  // something.
  if (fileSizeOrZero(rErr) === 0) {
    try { Deno.removeSync(rErr); } catch { /* already gone */ }
  }
  if (fileSizeOrZero(tErr) === 0) {
    try { Deno.removeSync(tErr); } catch { /* already gone */ }
  }

  if (!r.ok) rustErr++;
  if (!t.ok) tsErr++;

  rustTotalMs += r.ms;
  tsTotalMs += t.ms;

  if (r.ms > rustMaxMs) {
    rustMaxMs = r.ms;
    rustMaxFile = rel;
  }
  if (t.ms > tsMaxMs) {
    tsMaxMs = t.ms;
    tsMaxFile = rel;
  }

  const match = await filesEqual(rOut, tOut);
  if (match) {
    pass++;
    try { Deno.removeSync(rOut); } catch { /* already gone */ }
    try { Deno.removeSync(tOut); } catch { /* already gone */ }
  } else {
    fail++;
    failList.push(rel);
    // Render the unified diff via the system `diff` binary, which
    // produces the same header / hunk layout the previous zsh
    // version emitted; consumers (`failures.tsv`, the summary
    // preview) compare diff sizes byte-for-byte across runs.
    const diffPath = `${WORK}/diff/${safe}.diff`;
    await new Deno.Command("sh", {
      args: ["-c", `diff -u ${shQuote(tOut)} ${shQuote(rOut)} > ${shQuote(diffPath)} 2>&1; exit 0`],
      stdout: "null",
      stderr: "null",
    }).output();
  }

  timingsRows.push(`${rel}\t${r.ms}\t${t.ms}\t${match ? 1 : 0}`);

  if (i % 50 === 0) {
    console.error(
      `progress: ${i}/${total} pass=${pass} fail=${fail} rust_ms_total=${rustTotalMs} ts_ms_total=${tsTotalMs}`,
    );
  }
}
const wallMs = Math.round(performance.now() - benchStart);

Deno.writeTextFileSync(`${WORK}/timings.tsv`, timingsRows.join("\n") + "\n");
Deno.writeTextFileSync(
  `${WORK}/fail_list.txt`,
  failList.length > 0 ? failList.join("\n") + "\n" : "",
);

// Per-failure diff line count = newline count in the unified diff
// (matches `wc -l` in the original zsh version). Sort ascending so
// the cheapest mismatches surface first.
const failureRows: Array<[string, number]> = [];
if (fail > 0) {
  for (const e of Deno.readDirSync(`${WORK}/diff`)) {
    if (!e.isFile || !e.name.endsWith(".diff")) continue;
    const rel = e.name.slice(0, -".diff".length).replaceAll("__", "/");
    const content = Deno.readTextFileSync(`${WORK}/diff/${e.name}`);
    const n = (content.match(/\n/g) ?? []).length;
    failureRows.push([rel, n]);
  }
  failureRows.sort((a, b) => a[1] - b[1] || a[0].localeCompare(b[0]));
  Deno.writeTextFileSync(
    `${WORK}/failures.tsv`,
    failureRows.map(([f, n]) => `${f}\t${n}`).join("\n") + "\n",
  );
}

const denom = total > 0 ? total : 1;
const status = fail === 0 ? "PASS" : "FAIL";

const summaryLines: string[] = [
  `format=${FORMAT}`,
  `status=${status}`,
  `files_total=${total}`,
  `files_pass=${pass}`,
  `files_fail=${fail}`,
  `rust_nonzero_exit=${rustErr}`,
  `ts_nonzero_exit=${tsErr}`,
  `rust_total_ms=${rustTotalMs}`,
  `ts_total_ms=${tsTotalMs}`,
  `rust_avg_ms_per_file=${Math.floor(rustTotalMs / denom)}`,
  `ts_avg_ms_per_file=${Math.floor(tsTotalMs / denom)}`,
  `rust_max_ms=${rustMaxMs}\t${rustMaxFile}`,
  `ts_max_ms=${tsMaxMs}\t${tsMaxFile}`,
  `wallclock_total_ms=${wallMs}`,
];
if (fail > 0) {
  summaryLines.push("");
  summaryLines.push("failures (smallest diff first; full list in failures.tsv):");
  for (const [f, n] of failureRows.slice(0, 20)) {
    summaryLines.push(`  ${String(n).padStart(5)} lines  ${f}`);
  }
  if (fail > 20) {
    summaryLines.push(`  ... and ${fail - 20} more (see failures.tsv)`);
  }
}
const summary = summaryLines.join("\n") + "\n";
Deno.stdout.writeSync(new TextEncoder().encode(summary));
Deno.writeTextFileSync(`${WORK}/summary.txt`, summary);

if (fail > 0) Deno.exit(1);
