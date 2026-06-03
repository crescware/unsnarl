#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env
// One-shot project check.
//
// Runs the canonical verification -- once -- and summarizes it:
//
//   cargo fmt --all
//   cargo clippy --workspace --all-targets --verbose -- -D warnings
//   cargo test --workspace --no-fail-fast --verbose   (a SINGLE run)
//
// The test phase used to launch `cargo test -p <member>` once per
// workspace member so each member's output could be logged separately.
// That gave per-crate logs but was slow: with resolver v2 the feature
// set cargo unifies for a single `-p <member>` differs from the set it
// unifies for the whole workspace, so the per-member invocations kept
// recompiling and relinking the shared internal crates against each
// other -- the same crate built over and over as the selection moved
// from member to member. Collapsing the test phase into ONE
// `cargo test --workspace` run resolves features exactly once, so every
// crate compiles a single time; the combined output is then split back
// into per-crate logs here in the script.
//
// Splitting is exact, not heuristic: cargo prints a header before each
// test binary it runs -- a `Running ...` line naming the
// `target/debug/deps/<name>-<hash>` executable (its exact shape depends
// on `--verbose`; see `headerPackage`) and `Doc-tests <name>` for
// doctests -- where `<name>` is the build TARGET name. `cargo metadata`
// maps every target name back to its owning package, so each header is
// attributed to a crate and everything printed until the next header is
// that crate's log. The combined run's build output (everything before
// the first header) is saved separately as the build log.
//
// The per-crate logs land under `target/check/` (git-ignored), so a
// failure points straight at the crate that owns it -- you read that
// one crate's log instead of scrolling a single giant combined log.
// The dead-code-suppression crate (`no-allow-dead-code`) is just one
// of the workspace members, so its scan runs in the same workspace test
// with no special casing.
//
// The point of the script: ONE run yields everything you'd otherwise
// be tempted to re-run to collect. On failure, read the per-step /
// per-crate log under `target/check/` rather than running anything a
// second time -- the log already has it.
//
// fmt and clippy run fail-fast: a formatting / lint / compile failure
// makes the rest meaningless. The workspace test run uses
// `--no-fail-fast` so one failing binary does not hide the other
// crates' results -- one invocation still reports every crate's result.
// Formatting is applied (not just checked) so a run never leaves the
// tree unformatted.
//
// Usage:
//   mise run check

// Script lives at `scripts/check.ts`; the repo root is two `/`s up.
const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");
Deno.chdir(REPO_ROOT);

const WORK = `${REPO_ROOT}/target/check`;
Deno.mkdirSync(WORK, { recursive: true });

const dec = new TextDecoder();

async function runCargo(
  args: string[],
  log: string,
): Promise<{ ok: boolean; output: string }> {
  // Capture stdout + stderr together: cargo test writes results to
  // stdout, clippy writes diagnostics to stderr, and we want one
  // faithful log per step regardless of which stream a tool picked.
  const { code, stdout, stderr } = await new Deno.Command("cargo", {
    args,
    stdout: "piped",
    stderr: "piped",
    env: Deno.env.toObject(),
  }).output();
  const output = dec.decode(stdout) + dec.decode(stderr);
  Deno.writeTextFileSync(log, output);
  return { ok: code === 0, output };
}

async function runMerged(
  cmd: string,
  log: string,
): Promise<{ ok: boolean; output: string }> {
  // The test phase needs stderr and stdout interleaved in the order
  // they were emitted: the `Running ...` / `Doc-tests ...` headers go
  // to stderr while each binary's `test result: ...` block goes to
  // stdout, and the split below relies on a header being immediately
  // followed by its own binary's output. Reading two separate pipes
  // loses that ordering, so the command is run through `sh -c "... 2>&1"`
  // to fold stderr into stdout as a single ordered stream. `cmd` is a
  // fixed literal here -- no interpolation of external input -- so the
  // shell carries no quoting risk.
  const { code, stdout } = await new Deno.Command("sh", {
    args: ["-c", `${cmd} 2>&1`],
    stdout: "piped",
    stderr: "null",
    env: Deno.env.toObject(),
  }).output();
  const output = dec.decode(stdout);
  Deno.writeTextFileSync(log, output);
  return { ok: code === 0, output };
}

function testCounts(
  output: string,
): { passed: number; failed: number; failedNames: string[] } {
  let passed = 0;
  let failed = 0;
  // One "test result:" line per test binary; sum across the crate's
  // unit + integration + doc test binaries.
  for (
    const m of output.matchAll(/test result: \w+\. (\d+) passed; (\d+) failed/g)
  ) {
    passed += Number(m[1]);
    failed += Number(m[2]);
  }
  const failedNames: string[] = [];
  for (const m of output.matchAll(/^test (.+) \.\.\. FAILED$/gm)) {
    failedNames.push(m[1]);
  }
  return { passed, failed, failedNames };
}

function tail(text: string, n: number): string {
  const lines = text.replace(/\n$/, "").split("\n");
  return lines.slice(Math.max(0, lines.length - n)).join("\n");
}

async function targetNameToPackage(): Promise<Map<string, string>> {
  // `--no-deps` narrows `packages` to the workspace members only. Each
  // member's targets carry the `name` cargo uses for the compiled
  // artifact: the lib's underscored crate name (`unsnarl_emitter_ir`),
  // a bin's name (`uns`), an integration test's file stem (`parity`).
  // Those are exactly the names that appear in the `Running` /
  // `Doc-tests` headers, so a target-name -> package map attributes
  // every header to its crate. `custom-build` (build script) targets
  // never produce a test binary, so they are skipped.
  const { stdout } = await new Deno.Command("cargo", {
    args: ["metadata", "--no-deps", "--format-version", "1"],
    stdout: "piped",
    stderr: "null",
  }).output();
  const meta = JSON.parse(dec.decode(stdout)) as {
    packages: { name: string; targets: { name: string; kind: string[] }[] }[];
  };
  const map = new Map<string, string>();
  for (const p of meta.packages) {
    for (const t of p.targets) {
      if (t.kind.includes("custom-build")) continue;
      map.set(t.name, p.name);
    }
  }
  return map;
}

// Identifies the header cargo prints before each test binary it runs,
// and attributes it to its package, or returns null for a non-header
// line. Two header shapes must be recognized, because the flag set is
// not fixed: without `--verbose` cargo prints
//   Running unittests src/lib.rs (target/debug/deps/<name>-<hash>)
//   Running tests/<file>.rs (target/debug/deps/<name>-<hash>)
// while with `--verbose` it prints the literal command it executes,
//   Running `/abs/.../target/debug/deps/<name>-<hash>`
// and ALSO prints a `Running `rustc ...`` / `Running `<build-script>``
// line for every compile step. The discriminator is the executable
// being run (the parenthesized path, or the first token inside the
// backticks): only an executable under `.../deps/<name>-<hash>` is a
// test binary, so the verbose rustc/build-script lines -- whose first
// token is `rustc` or a path under `.../build/` -- are rejected. The
// first token is used deliberately: a verbose rustc line mentions other
// crates' `.../deps/lib<crate>-<hash>.rlib` in its `--extern` args, and
// matching anywhere would misfire on those. `Doc-tests <name>` is a
// header in both modes (`<name>` is the lib's underscored crate name).
function headerPackage(line: string, map: Map<string, string>): string | null {
  const doc = line.match(/^\s*Doc-tests\s+(\S+)\s*$/);
  if (doc) return map.get(doc[1]) ?? "(unknown)";

  let exe: string | null = null;
  const verbose = line.match(/^\s*Running\s+`([^`]+)`\s*$/);
  if (verbose) {
    exe = verbose[1].trim().split(/\s+/)[0];
  } else {
    const plain = line.match(/^\s*Running\s+.*\(([^()]+)\)\s*$/);
    if (plain) exe = plain[1].trim();
  }
  if (!exe || !exe.includes("/deps/")) return null;

  // `.../deps/<name>-<hash>` -> strip dir, then the trailing `-<hash>`
  // (cargo's metadata hash is hex), leaving the build target name.
  const base = exe.split("/").pop() ?? "";
  const stem = base.replace(/-[0-9a-f]{8,}$/, "");
  return map.get(stem) ?? "(unknown)";
}

type Segment = { pkg: string; lines: string[] };

function splitByBinary(
  output: string,
  map: Map<string, string>,
): { preamble: string; segments: Segment[] } {
  const preamble: string[] = [];
  const segments: Segment[] = [];
  let cur: Segment | null = null;
  for (const line of output.split("\n")) {
    const pkg = headerPackage(line, map);
    if (pkg !== null) {
      cur = { pkg, lines: [line] };
      segments.push(cur);
    } else if (cur) {
      cur.lines.push(line);
    } else {
      preamble.push(line);
    }
  }
  return { preamble: preamble.join("\n"), segments };
}

const summary: string[] = [];
const failures: { label: string; log: string; output: string }[] = [];

function record(
  ok: boolean,
  label: string,
  detail: string,
  log: string,
  output: string,
) {
  summary.push(`  ${ok ? "PASS" : "FAIL"}  ${label.padEnd(30)}${detail}`);
  if (!ok) failures.push({ label, log, output });
}

function finish(): never {
  console.error("");
  console.error("check summary");
  console.error(summary.join("\n"));
  console.error("");
  if (failures.length > 0) {
    for (const f of failures) {
      console.error(`--- ${f.label} failed; tail of ${f.log} ---`);
      console.error(tail(f.output, 30));
      console.error("");
    }
    console.error(`full logs under ${WORK}/`);
    Deno.exit(1);
  }
  console.error(`all green. logs under ${WORK}/`);
  Deno.exit(0);
}

function secsSince(t0: number): string {
  return `(${((performance.now() - t0) / 1000).toFixed(1)}s)`;
}

console.error(`[check] writing step logs to ${WORK}/`);

// 1. fmt -- fail-fast.
{
  const t0 = performance.now();
  const log = `${WORK}/fmt.log`;
  const { ok, output } = await runCargo(["fmt", "--all"], log);
  record(ok, "fmt", secsSince(t0), log, output);
  if (!ok) finish();
}

// 2. clippy -- fail-fast.
{
  const t0 = performance.now();
  const log = `${WORK}/clippy.log`;
  const { ok, output } = await runCargo(
    [
      "clippy",
      "--workspace",
      "--all-targets",
      "--verbose",
      "--",
      "-D",
      "warnings",
    ],
    log,
  );
  record(ok, "clippy", secsSince(t0), log, output);
  if (!ok) finish();
}

// 3. tests -- ONE workspace run; split the combined output back into
//    per-crate logs and a per-crate PASS/FAIL summary.
{
  const t0 = performance.now();
  const map = await targetNameToPackage();
  const combinedLog = `${WORK}/test-combined.log`;
  const { ok: runOk, output } = await runMerged(
    "cargo test --workspace --no-fail-fast --verbose",
    combinedLog,
  );

  const { preamble, segments } = splitByBinary(output, map);
  // The build output (Compiling/Fresh/rustc lines) precedes the first
  // test-binary header; keep it as its own log.
  Deno.writeTextFileSync(`${WORK}/test-build.log`, preamble);

  if (segments.length === 0) {
    // No test binary ever ran -- the build failed before the run phase.
    record(false, "test (build)", secsSince(t0), combinedLog, output);
    finish();
  }

  const byPkg = new Map<
    string,
    { lines: string[]; passed: number; failed: number; failedNames: string[] }
  >();
  for (const seg of segments) {
    const e = byPkg.get(seg.pkg) ??
      { lines: [], passed: 0, failed: 0, failedNames: [] };
    e.lines.push(...seg.lines);
    const c = testCounts(seg.lines.join("\n"));
    e.passed += c.passed;
    e.failed += c.failed;
    e.failedNames.push(...c.failedNames);
    byPkg.set(seg.pkg, e);
  }

  for (const pkg of [...byPkg.keys()].sort()) {
    const e = byPkg.get(pkg)!;
    const text = e.lines.join("\n");
    const log = `${WORK}/test-${pkg}.log`;
    Deno.writeTextFileSync(log, text);
    record(
      e.failed === 0,
      `test ${pkg}`,
      `${e.passed} passed, ${e.failed} failed`,
      log,
      text,
    );
    for (const n of e.failedNames) summary.push(`        ✗ ${n}`);
  }
  summary.push(`        (workspace test phase ${secsSince(t0)})`);

  // cargo exited non-zero but no per-crate `test result:` accounted for
  // it (e.g. a binary aborted before printing a summary) -- surface it
  // rather than report all-green.
  if (!runOk && ![...byPkg.values()].some((e) => e.failed > 0)) {
    record(
      false,
      "test (cargo)",
      "non-zero exit; see combined log",
      combinedLog,
      output,
    );
  }
}

finish();
