#!/usr/bin/env -S deno run --allow-read --allow-run
// Release orchestrator: dry-run -> tag -> publish -> GitHub Release.
//
// The GitHub Release step used to have no command of its own -- it was
// a manual click after `npm-publish.ts`, and so it got forgotten (e.g.
// v0.3.6 was published to npm but never got a Release). This script
// folds the Release into the same command as publish so that "publish
// succeeded but no Release exists" can no longer happen.
//
// It does NOT bump or merge the bump PR. By the time you run this, the
// bump PR is already merged and pulled, so HEAD is the merge commit the
// tag should point at. The sequence is:
//
//   1. dry-run  -- `npm-publish.ts --dry-run`; proceed only if it exits
//                  0. We gate on the exit code alone, not its output.
//   2. tag      -- `git tag vX.Y.Z` on HEAD, then push the tag. A tag
//                  that already exists makes `git tag`/`git push` fail,
//                  which aborts the run -- a built-in double-run guard.
//   3. publish  -- `npm-publish.ts` for real.
//   4. release  -- `gh release create vX.Y.Z --generate-notes
//                  --verify-tag`, plus `--prerelease` when the version
//                  carries a semver pre-release identifier.
//
// Each step aborts the whole run on failure; later steps never run.
//
// `npm-publish.ts` is run as a SUBPROCESS with stdin/stdout/stderr
// inherited, never imported. That is deliberate: npm opens a browser
// and waits for passkey / WebAuthn 2FA, which only happens when npm
// sees the parent's (TTY) stdin. `npm-publish.ts` already inherits
// stdio into its own `npm` spawn; launching it via a child `deno run`
// with inherited stdio preserves that fd chain end to end. Importing it
// instead would also re-run its top-level body on import and is avoided
// for that reason too.

// Script lives at `scripts/release.ts`; the repo root is two `/`s up.
const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");
Deno.chdir(REPO_ROOT);

function readVersion(): string {
  const pkg = JSON.parse(
    Deno.readTextFileSync(`${REPO_ROOT}/package.json`),
  ) as { version?: string };
  if (!pkg.version) {
    console.error("package.json: version is missing");
    Deno.exit(1);
  }
  return pkg.version;
}

// Run a command with the parent's stdio inherited (so npm's passkey
// browser flow and any git/gh prompts keep working) and abort the whole
// release on a non-zero exit.
async function run(label: string, cmd: string, args: string[]): Promise<void> {
  console.error(`[release] ${label}: ${cmd} ${args.join(" ")}`);
  const { success, code } = await new Deno.Command(cmd, {
    args,
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  }).spawn().status;
  if (!success) {
    console.error(`[release] ${label} failed (exit ${code}); aborting`);
    Deno.exit(code || 1);
  }
}

// --- guard: gh available + authenticated, checked up front ----------
// The GitHub Release is the LAST step, but it runs only after the
// irreversible `npm publish` and the tag push. A missing or
// unauthenticated gh would otherwise surface there -- leaving exactly
// the "published to npm but no Release" state this script exists to
// prevent. The existing-release check below also leans on gh and reads
// any non-zero `gh release view` as "release absent", so verifying auth
// here keeps an auth failure from masquerading as absence.
try {
  const auth = await new Deno.Command("gh", {
    args: ["auth", "status"],
    stdout: "null",
    stderr: "null",
  }).output();
  if (!auth.success) {
    console.error("[release] gh is not authenticated (run `gh auth login`); aborting");
    Deno.exit(1);
  }
} catch {
  console.error("[release] gh (GitHub CLI) not found on PATH; aborting");
  Deno.exit(1);
}

const version = readVersion();
const tag = `v${version}`;
// semver pre-release identifier present (e.g. "0.4.0-beta.0") -> mark
// the Release as a prerelease. Mirrors npm-publish.ts treating any
// non-`latest` dist-tag as a pre-release line.
const isPrerelease = version.includes("-");

const PUBLISH = `${REPO_ROOT}/scripts/npm-publish.ts`;

// Refuse to re-release: if the Release already exists, stop before
// touching tags or npm. `gh release view` exits non-zero when the
// release is absent -- that absence is the state we proceed from.
const existing = await new Deno.Command("gh", {
  args: ["release", "view", tag],
  stdout: "null",
  stderr: "null",
}).output();
if (existing.success) {
  console.error(`[release] release ${tag} already exists; nothing to do`);
  Deno.exit(0);
}

// 1. dry-run -- only its exit code gates the rest.
await run("dry-run", "deno", [
  "run",
  "--allow-read",
  "--allow-run",
  PUBLISH,
  "--dry-run",
]);

// 2. tag HEAD (the merge commit) and push the tag.
await run("tag", "git", ["tag", tag]);
await run("push-tag", "git", ["push", "origin", tag]);

// 3. publish for real (passkey browser flow preserved via inherited stdio).
await run("publish", "deno", [
  "run",
  "--allow-read",
  "--allow-run",
  PUBLISH,
]);

// 4. create the GitHub Release against the tag we just pushed.
const releaseArgs = ["release", "create", tag, "--generate-notes", "--verify-tag"];
if (isPrerelease) releaseArgs.push("--prerelease");
await run("release", "gh", releaseArgs);

console.error(`[release] done: ${tag} published to npm and released on GitHub`);
