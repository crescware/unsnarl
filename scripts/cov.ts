#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env
// Line / region / function coverage for the Rust workspace.
//
// Builds every workspace test target with rustc's
// `-Cinstrument-coverage`, runs `cargo test --workspace` so each test
// binary writes its own `*.profraw`, merges them, and produces a
// per-source-file report via `llvm-cov report`.
//
// All artifacts live under `target/coverage/` so they stay
// git-ignored alongside the rest of the Cargo target tree.
// `target/cov-build/` is used as a sibling Cargo target dir so the
// instrumented build does not invalidate the default `cargo`
// incremental cache (re-flipping `RUSTFLAGS` between calls would
// otherwise force every dep to recompile in `target/`).
//
// Requires the `llvm-tools-preview` rustup component for
// `llvm-profdata` / `llvm-cov`. Install once with:
//
//   rustup component add llvm-tools-preview
//
// Usage (via mise):
//   mise run cov                                          # full workspace report
//   mise run cov -- --filter build_analysis_visitor.rs    # filter rows by filename regex
//   mise run cov -- --test parity                         # restrict to the `parity` integration test target
//   mise run cov -- --filter visual_graph -- --test parity
//                                                         # combine: parity-only run, rows filtered to visual_graph
//
// Argument convention:
//   --filter <regex>   Filename regex applied to the final llvm-cov report rows.
//   Anything else      Forwarded verbatim to both `cargo test --workspace --no-run`
//                      and the subsequent `cargo test --workspace` run (e.g.
//                      `--test parity`, `-p unsnarl-analyzer`, `--lib`, ...).

// Script lives at `scripts/cov.ts`; the repo root is two `/`s up.
const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");
Deno.chdir(REPO_ROOT);

const WORK = `${REPO_ROOT}/target/coverage`;
const BUILD_DIR = `${REPO_ROOT}/target/cov-build`;
const PROF_DIR = `${WORK}/profraw`;
const MERGED = `${WORK}/merged.profdata`;

async function captureStdout(cmd: string, args: string[]): Promise<string> {
  const { stdout } = await new Deno.Command(cmd, {
    args,
    stdout: "piped",
    stderr: "inherit",
  }).output();
  return new TextDecoder().decode(stdout);
}

async function locateLlvmTools(): Promise<{ profdata: string; llvmcov: string }> {
  const sysroot = (await captureStdout("rustc", ["--print", "sysroot"])).trim();
  const verbose = await captureStdout("rustc", ["-vV"]);
  const host = verbose.match(/^host:\s*(.+)$/m)?.[1].trim();
  if (!host) {
    console.error("could not determine rustc host triple");
    Deno.exit(1);
  }
  const dir = `${sysroot}/lib/rustlib/${host}/bin`;
  const profdata = `${dir}/llvm-profdata`;
  const llvmcov = `${dir}/llvm-cov`;
  for (const bin of [profdata, llvmcov]) {
    try {
      const s = Deno.statSync(bin);
      if (!s.isFile) throw new Error("not a file");
    } catch {
      console.error(`missing ${bin}`);
      console.error("install the rustup component:   rustup component add llvm-tools-preview");
      Deno.exit(1);
    }
  }
  return { profdata, llvmcov };
}

function rmrf(path: string) {
  try {
    Deno.removeSync(path, { recursive: true });
  } catch (e) {
    if (!(e instanceof Deno.errors.NotFound)) throw e;
  }
}

function listProfraws(): string[] {
  const out: string[] = [];
  for (const e of Deno.readDirSync(PROF_DIR)) {
    if (e.isFile && e.name.endsWith(".profraw")) out.push(`${PROF_DIR}/${e.name}`);
  }
  return out;
}

function listInstrumentedTestObjects(): string[] {
  const deps = `${BUILD_DIR}/debug/deps`;
  const out: string[] = [];
  for (const e of Deno.readDirSync(deps)) {
    if (!e.isFile) continue;
    if (e.name.endsWith(".d") || e.name.endsWith(".dSYM")) continue;
    const path = `${deps}/${e.name}`;
    const s = Deno.statSync(path);
    // Skip non-executables; on macOS the depfiles and rmeta artefacts
    // share this directory with the actual test binaries.
    if (((s.mode ?? 0) & 0o111) === 0) continue;
    out.push("-object", path);
  }
  return out;
}

function parseArgs(argv: string[]): { filter: string; cargoExtra: string[] } {
  let filter = "";
  const cargoExtra: string[] = [];
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--filter") {
      const next = argv[i + 1];
      if (next === undefined) {
        console.error("--filter requires a regex argument");
        Deno.exit(2);
      }
      filter = next;
      i++;
    } else if (a.startsWith("--filter=")) {
      filter = a.slice("--filter=".length);
    } else {
      cargoExtra.push(a);
    }
  }
  return { filter, cargoExtra };
}

const { filter, cargoExtra } = parseArgs(Deno.args);
const { profdata, llvmcov } = await locateLlvmTools();

rmrf(WORK);
Deno.mkdirSync(PROF_DIR, { recursive: true });

// Inherit the parent env so subprocess `cargo` / `rustc` can find
// their toolchain (PATH, HOME, etc.), then layer the
// coverage-specific overrides on top.
const subprocessEnv = {
  ...Deno.env.toObject(),
  CARGO_TARGET_DIR: BUILD_DIR,
  RUSTFLAGS: "-Cinstrument-coverage",
  LLVM_PROFILE_FILE: `${PROF_DIR}/cov-%p-%m.profraw`,
};

console.error("[1/3] building instrumented test binaries...");
{
  const { success } = await new Deno.Command("cargo", {
    args: ["test", "--workspace", "--no-run", ...cargoExtra],
    env: subprocessEnv,
    stdout: "null",
    stderr: "inherit",
  }).output();
  if (!success) {
    console.error("cargo test --no-run failed");
    Deno.exit(1);
  }
}

console.error("[2/3] running tests...");
{
  // Test failures must not block report generation -- a partial
  // run is still informative -- so we ignore the exit code here
  // and let the merge / report step decide whether there is
  // anything to summarise.
  await new Deno.Command("cargo", {
    args: ["test", "--workspace", "--quiet", ...cargoExtra],
    env: subprocessEnv,
    stdout: "null",
    stderr: "null",
  }).output();
}

const profraws = listProfraws();
if (profraws.length === 0) {
  console.error(`no .profraw files produced under ${PROF_DIR}`);
  Deno.exit(1);
}
console.error(`      collected ${profraws.length} raw profile(s)`);

console.error("[3/3] merging profiles and rendering report...");
{
  const { success } = await new Deno.Command(profdata, {
    args: ["merge", "-sparse", ...profraws, "-o", MERGED],
    stderr: "inherit",
  }).output();
  if (!success) {
    console.error("llvm-profdata merge failed");
    Deno.exit(1);
  }
}

const objects = listInstrumentedTestObjects();
if (objects.length === 0) {
  console.error(`no executables found in ${BUILD_DIR}/debug/deps`);
  Deno.exit(1);
}

const ignoreRegex = "/.cargo/|/rustc/|/target/";
const { stdout: reportBytes } = await new Deno.Command(llvmcov, {
  args: [
    "report",
    ...objects,
    `-instr-profile=${MERGED}`,
    `-ignore-filename-regex=${ignoreRegex}`,
    "-show-functions=0",
    "-use-color=false",
  ],
  stdout: "piped",
  stderr: "inherit",
}).output();
const report = new TextDecoder().decode(reportBytes);
const reportPath = `${WORK}/report.txt`;
Deno.writeTextFileSync(reportPath, report);

if (filter) {
  // Always include the first two rows (header + separator) and the
  // trailing TOTAL line so the filtered view is self-describing.
  const re = new RegExp(filter);
  const lines = report.split("\n");
  const out = lines.filter((line, i) => i < 2 || re.test(line) || /^TOTAL/.test(line));
  console.log(out.join("\n"));
} else {
  Deno.stdout.writeSync(reportBytes);
}
