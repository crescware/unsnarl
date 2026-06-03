#!/usr/bin/env -S deno run --allow-read --allow-run
// Prepare a release: bump the version and open the release PR in one
// no-look command.
//
// Pairs with release.ts: this is the FIRST half (bump -> PR); release.ts
// is the second half (tag -> publish -> GitHub Release) run after the PR
// is merged. Neither is a child of the other -- `bump` (bump-version.ts)
// is an independent part that this script invokes.
//
// "No-look" means a human does not eyeball the diff before committing --
// so the script MUST verify the bump mechanically. After `bump` runs it
// aborts unless ALL of these hold:
//
//   (a) only the expected files changed: Cargo.toml, Cargo.lock,
//       package.json, npm/unsnarl-darwin-arm64/package.json.
//   (b) every version / publishConfig.tag field now equals the new
//       version / its derived dist-tag.
//   (c) every changed diff line (a `+`/`-` line, excluding file headers
//       and context) contains the old or new version, or the old or new
//       dist-tag. I.e. the change is PURELY version/tag text and nothing
//       slipped in -- e.g. via the Cargo.lock regeneration `bump` does.
//
// Guard: must run on a clean `main` that is in sync with origin/main.
// Does NOT run `mise run check` (by request) and does NOT merge the PR.
//
// Pass `--dry-run` to rehearse without outward effects: it runs the SAME
// guards as a real run (clean main in sync with origin/main, gh
// authenticated) and the same bump + mechanical verify (so the no-look
// safety net is exercised), then reverts the bump and prints what a real
// run would commit / push / open -- creating no branch, commit, push, or
// PR. The guards are deliberately identical: rehearsing from the wrong
// branch or a stale main would be misleading and accident-prone. (If the
// verify itself fails, the bump is left in place for inspection -- abort
// exits immediately, so the revert does not run.)
//
// The run order is the order `main()` calls its step functions at the
// bottom of this file; each step's own doc comment explains what it does
// and why. Any step aborts the whole run on failure, so later steps
// never run.
//
// Usage (via mise):
//   mise run prepare-release -- 0.4.0-beta.1
//   mise run prepare-release -- 0.4.0-beta.1 --dry-run

// Script lives at `scripts/prepare-release.ts`; the repo root is two
// `/`s up.
const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");
Deno.chdir(REPO_ROOT);

const dec = new TextDecoder();

// --- small helpers ------------------------------------------------------

// Capture a command's stdout + exit code (no inherited stdio). `out` is
// trimmed for the common "read one value" case; `raw` is the verbatim
// stdout for callers where leading/trailing bytes are significant (e.g.
// `git status --porcelain`, whose records begin with a status code that
// can itself start with a space).
async function capture(
  cmd: string,
  args: string[],
): Promise<{ code: number; out: string; raw: string }> {
  const { code, stdout } = await new Deno.Command(cmd, {
    args,
    stdout: "piped",
    stderr: "piped",
  }).output();
  const raw = dec.decode(stdout);
  return { code, out: raw.trim(), raw };
}

// Like capture() but a non-zero exit is fatal: abort the whole run
// instead of silently returning an empty string. Use this wherever a
// failed command means the world is not as assumed (so the guard does
// not pass by accident). Use the raw capture() -- which exposes
// `code` -- only where a non-zero exit is itself the expected signal,
// e.g. `git rev-parse --verify --quiet` probing for a branch.
async function captureOk(
  label: string,
  cmd: string,
  args: string[],
): Promise<string> {
  const { code, out } = await capture(cmd, args);
  if (code !== 0) abort(`${label} failed (exit ${code})`);
  return out;
}

// Like captureOk but returns stdout verbatim (no trim). Use where
// leading/trailing bytes carry meaning -- chiefly `git status
// --porcelain`, whose first record begins with the status code's left
// column (a space for a working-tree-only change, e.g. " M"); trimming
// would swallow it and shift that record's path parse by one byte.
async function captureRawOk(
  label: string,
  cmd: string,
  args: string[],
): Promise<string> {
  const { code, raw } = await capture(cmd, args);
  if (code !== 0) abort(`${label} failed (exit ${code})`);
  return raw;
}

// Run a command with the parent's stdio inherited (so git/gh prompts
// work) and abort the whole run on a non-zero exit.
async function run(label: string, cmd: string, args: string[]): Promise<void> {
  console.error(`[prepare-release] ${label}: ${cmd} ${args.join(" ")}`);
  const { success, code } = await new Deno.Command(cmd, {
    args,
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  }).spawn().status;
  if (!success) {
    console.error(`[prepare-release] ${label} failed (exit ${code}); aborting`);
    Deno.exit(code || 1);
  }
}

function abort(msg: string): never {
  console.error(`[prepare-release] ${msg}; aborting`);
  Deno.exit(1);
}

// publishConfig.tag derivation -- identical rule to bump-version.ts:
// no pre-release -> "latest"; "rc" pre-release -> "rc"; otherwise
// ("beta"/"alpha"/...) -> "beta".
function tagFor(version: string): string {
  const pre = version.includes("-")
    ? version.split("-", 2)[1].split(".", 1)[0]
    : null;
  return pre === null ? "latest" : pre === "rc" ? "rc" : "beta";
}

interface Pkg {
  version?: string;
  optionalDependencies?: Record<string, string>;
  publishConfig?: { tag?: string };
}

function readPkg(path: string): Pkg {
  return JSON.parse(Deno.readTextFileSync(`${REPO_ROOT}/${path}`)) as Pkg;
}

// The exact set of files `bump` rewrites: the workspace + npm version
// fields, plus the Cargo.lock it regenerates. The verify keys its "only
// these changed" check off this list, and the dry-run revert restores
// exactly these.
const BUMP_FILES = [
  "Cargo.toml",
  "Cargo.lock",
  "package.json",
  "npm/unsnarl-darwin-arm64/package.json",
];

// --- steps (called in order by main()) ----------------------------------

// Read and validate argv: the target version (the same shape
// bump-version.ts accepts) and an optional `--dry-run` flag.
function parseArgs(): { target: string; dryRun: boolean } {
  const dryRun = Deno.args.includes("--dry-run");
  const target = Deno.args.find((a) => a !== "--dry-run");
  if (!target) {
    console.error("usage: prepare-release.ts <new-version> [--dry-run]");
    Deno.exit(1);
  }
  const VERSION_RE = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;
  if (!VERSION_RE.test(target)) {
    console.error(
      `'${target}' is not a valid semver string (expected e.g. 0.4.0-beta.1)`,
    );
    Deno.exit(1);
  }
  return { target, dryRun };
}

// gh must be installed AND authenticated, checked up front: the PR is the
// last step, so a missing or unauthenticated gh would otherwise only
// surface after we have already branched, committed and pushed.
async function assertGhAuthenticated(): Promise<void> {
  try {
    const auth = await new Deno.Command("gh", {
      args: ["auth", "status"],
      stdout: "null",
      stderr: "null",
    }).output();
    if (!auth.success) abort("gh is not authenticated (run `gh auth login`)");
  } catch {
    abort("gh (GitHub CLI) not found on PATH");
  }
}

// Must run on a clean main that is in sync with origin/main, so the branch
// is cut from -- and the bump diff measured against -- the real tip. A
// dry-run runs this same guard: rehearsing from the wrong branch or a
// stale main would be misleading and accident-prone, so it is held to the
// identical preconditions even though it never branches or pushes.
async function guardCleanMainInSync(): Promise<void> {
  const branch = await captureOk("git rev-parse --abbrev-ref HEAD", "git", [
    "rev-parse",
    "--abbrev-ref",
    "HEAD",
  ]);
  if (branch !== "main") abort(`must run on main (currently on ${branch})`);
  const status = await captureOk("git status --porcelain", "git", [
    "status",
    "--porcelain",
  ]);
  if (status !== "") abort("working tree is not clean");
  await run("fetch", "git", ["fetch", "origin", "main"]);
  const local = await captureOk("git rev-parse HEAD", "git", [
    "rev-parse",
    "HEAD",
  ]);
  const remote = await captureOk("git rev-parse origin/main", "git", [
    "rev-parse",
    "origin/main",
  ]);
  if (local !== remote) {
    abort("local main is not in sync with origin/main (pull first)");
  }
}

// Read the current version from package.json (used for the commit message
// and the diff verification) and refuse to "bump" to the same value.
function readOldVersion(target: string): string {
  const oldVersion = readPkg("package.json").version;
  if (!oldVersion) abort("package.json: version is missing");
  if (oldVersion === target) abort(`package.json is already at ${target}`);
  return oldVersion;
}

// Create the release branch, refusing to clobber an existing one.
async function createReleaseBranch(branch: string): Promise<void> {
  const exists = await capture("git", [
    "rev-parse",
    "--verify",
    "--quiet",
    branch,
  ]);
  if (exists.code === 0) abort(`branch ${branch} already exists`);
  await run("branch", "git", ["switch", "-c", branch]);
}

// Run `bump` (bump-version.ts), which also regenerates Cargo.lock.
async function runBump(target: string): Promise<void> {
  await run("bump", "deno", [
    "run",
    "--allow-read",
    "--allow-write",
    "--allow-run",
    `${REPO_ROOT}/scripts/bump-version.ts`,
    target,
  ]);
}

// Undo the working-tree edits `bump` made, returning the tree to its
// pre-bump state. Used by the dry-run after a successful verify. Safe
// because requireCleanWorktree() ran first, so these files held no other
// changes; `git restore` on an unmodified file is a no-op.
async function restoreBumpedFiles(): Promise<void> {
  await run("restore", "git", ["restore", ...BUMP_FILES]);
}

// Verify the bump mechanically (no human eyeballs the diff): (a) only the
// expected files changed, (b) every version/tag field now holds the new
// value, (c) every changed diff line is purely version/tag text.
async function verifyBumpIsVersionOnly(
  target: string,
  oldVersion: string,
  oldTag: string,
  newTag: string,
): Promise<void> {
  // (a) only the expected files changed.
  const EXPECTED = new Set(BUMP_FILES);
  // A `git status --porcelain` record is `XY PATH`: a 2-column status code,
  // one separator space, then the path. Read it NUL-terminated (`-z`) so
  // records split cleanly and paths are never quoted, and capture it raw so
  // the leading status column survives. Derive the path from that structure
  // -- not a bare offset -- and abort on any record that does not match, so
  // a format surprise fails loudly instead of silently truncating a name.
  const PORCELAIN_RECORD = /^.. (.*)$/s; // XY, separator space, then PATH
  const changed = (await captureRawOk("git status --porcelain -z", "git", [
    "status",
    "--porcelain",
    "-z",
  ]))
    .split("\0")
    .filter((record) => record.length > 0)
    .map((record) => {
      const m = PORCELAIN_RECORD.exec(record);
      if (!m) {
        abort(`could not parse git status record: ${JSON.stringify(record)}`);
      }
      return m[1];
    });
  if (changed.length === 0) abort("bump produced no changes");
  for (const f of changed) {
    if (!EXPECTED.has(f)) abort(`unexpected file changed by bump: ${f}`);
  }

  // (b) every version / tag field now holds the new value.
  const root = readPkg("package.json");
  const sub = readPkg("npm/unsnarl-darwin-arm64/package.json");
  const cargoToml = Deno.readTextFileSync(`${REPO_ROOT}/Cargo.toml`);
  const esc = target.replace(/[.+]/g, "\\$&");
  if (root.version !== target) {
    abort(`package.json version is ${root.version}, expected ${target}`);
  }
  if (root.optionalDependencies?.["unsnarl-darwin-arm64"] !== target) {
    abort("package.json optionalDependencies.unsnarl-darwin-arm64 mismatch");
  }
  if (root.publishConfig?.tag !== newTag) {
    abort(
      `package.json publishConfig.tag is ${root.publishConfig?.tag}, expected ${newTag}`,
    );
  }
  if (sub.version !== target) {
    abort(`sub-package version is ${sub.version}, expected ${target}`);
  }
  if (sub.publishConfig?.tag !== newTag) {
    abort("sub-package publishConfig.tag mismatch");
  }
  if (
    !new RegExp(`\\[workspace\\.package\\]\\nversion = "${esc}"`).test(
      cargoToml,
    )
  ) {
    abort(`Cargo.toml [workspace.package].version is not ${target}`);
  }

  // (c) every changed diff line is purely version/tag text.
  const diff = await captureOk("git diff", "git", ["diff"]);
  const tokens = [oldVersion, target, oldTag, newTag];
  for (const line of diff.split("\n")) {
    if (!/^[+-]/.test(line)) continue; // context / blank lines
    if (/^(\+\+\+|---)/.test(line)) continue; // file headers
    if (!tokens.some((t) => line.includes(t))) {
      abort(`unexpected diff line (not version/tag): ${line}`);
    }
  }
}

// Commit the bump, push the branch, and open the PR.
async function commitPushAndOpenPr(
  branch: string,
  oldVersion: string,
  target: string,
): Promise<void> {
  await run("add", "git", ["add", "-A"]);
  await run("commit", "git", [
    "commit",
    "-m",
    `Bump ${oldVersion} -> ${target}`,
  ]);
  await run("push", "git", ["push", "-u", "origin", branch]);
  await run("pr", "gh", [
    "pr",
    "create",
    "--base",
    "main",
    "--title",
    `Bump ${oldVersion} -> ${target}`,
    "--body",
    "Version-only bump produced by `mise run bump`.",
  ]);
}

// --- orchestration: this call order IS the prepare-release sequence -----
async function main(): Promise<void> {
  const { target, dryRun } = parseArgs();

  // A dry-run is held to the same preconditions as a real run -- being on
  // a clean, in-sync main -- so a rehearsal can never be run (and trusted)
  // from the wrong branch or a stale main.
  await assertGhAuthenticated();
  await guardCleanMainInSync();

  const oldVersion = readOldVersion(target);
  const branch = `release-${target}`;
  const oldTag = tagFor(oldVersion);
  const newTag = tagFor(target);

  if (dryRun) {
    console.error(`[prepare-release] dry-run: would create branch ${branch}`);
  } else {
    await createReleaseBranch(branch);
  }

  await runBump(target);
  await verifyBumpIsVersionOnly(target, oldVersion, oldTag, newTag);

  if (dryRun) {
    // Verify passed -> the bump is a clean version-only change. Revert it
    // so the rehearsal leaves no trace, and print what a real run would do
    // next instead of committing / pushing / opening the PR.
    await restoreBumpedFiles();
    console.error(
      `[prepare-release] dry-run: bump verified as version-only and reverted; a real run would commit "Bump ${oldVersion} -> ${target}", push ${branch}, and open the PR`,
    );
    return;
  }

  await commitPushAndOpenPr(branch, oldVersion, target);

  console.error(
    `[prepare-release] done: ${branch} pushed and PR opened (${oldVersion} -> ${target})`,
  );
}

await main();
