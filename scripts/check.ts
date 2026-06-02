#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env
// One-shot project check.
//
// Runs the canonical verification -- once -- and summarizes it:
//
//   cargo fmt --all
//   cargo clippy --workspace --all-targets -- -D warnings
//   cargo test -p <member>          for every workspace member
//
// The test phase runs each workspace member separately and writes a
// SEPARATE log per member under `target/check/` (git-ignored), so a
// failure points straight at the crate that owns it -- you read that
// one crate's log instead of scrolling a single giant combined log.
// The dead-code-suppression crate (`no-allow-dead-code`) is just one
// of those members, so its scan runs exactly once with no special
// casing.
//
// The point of the script: ONE run yields everything you'd otherwise
// be tempted to re-run to collect. On failure, read the per-step /
// per-crate log under `target/check/` rather than running anything a
// second time -- the log already has it.
//
// fmt and clippy run fail-fast: a formatting / lint / compile failure
// makes the rest meaningless. The per-crate test phase does NOT stop
// at the first failing crate -- it runs them all so one invocation
// reports every crate's result. Formatting is applied (not just
// checked) so a run never leaves the tree unformatted.
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

async function runCargo(args: string[], log: string): Promise<{ ok: boolean; output: string }> {
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

function testCounts(output: string): { passed: number; failed: number; failedNames: string[] } {
  let passed = 0;
  let failed = 0;
  // One "test result:" line per test binary; sum across the crate's
  // unit + integration + doc test binaries.
  for (const m of output.matchAll(/test result: \w+\. (\d+) passed; (\d+) failed/g)) {
    passed += Number(m[1]);
    failed += Number(m[2]);
  }
  const failedNames: string[] = [];
  for (const m of output.matchAll(/^test (.+) \.\.\. FAILED$/gm)) failedNames.push(m[1]);
  return { passed, failed, failedNames };
}

function tail(text: string, n: number): string {
  const lines = text.replace(/\n$/, "").split("\n");
  return lines.slice(Math.max(0, lines.length - n)).join("\n");
}

async function workspaceMembers(): Promise<string[]> {
  // `--no-deps` narrows `packages` to the workspace members only, so
  // their `name` fields are exactly the `-p <name>` targets to test.
  const { stdout } = await new Deno.Command("cargo", {
    args: ["metadata", "--no-deps", "--format-version", "1"],
    stdout: "piped",
    stderr: "null",
  }).output();
  const meta = JSON.parse(dec.decode(stdout)) as { packages: { name: string }[] };
  return meta.packages.map((p) => p.name).sort();
}

const summary: string[] = [];
const failures: { label: string; log: string; output: string }[] = [];

function record(ok: boolean, label: string, detail: string, log: string, output: string) {
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
    ["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"],
    log,
  );
  record(ok, "clippy", secsSince(t0), log, output);
  if (!ok) finish();
}

// 3. tests -- one workspace member at a time, separate log each, and
//    keep going so one run reports every crate's result.
for (const member of await workspaceMembers()) {
  const t0 = performance.now();
  const log = `${WORK}/test-${member}.log`;
  const { ok, output } = await runCargo(["test", "-p", member], log);
  const { passed, failed, failedNames } = testCounts(output);
  record(ok, `test ${member}`, `${passed} passed, ${failed} failed  ${secsSince(t0)}`, log, output);
  for (const n of failedNames) summary.push(`        ✗ ${n}`);
}

finish();
