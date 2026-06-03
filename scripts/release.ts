#!/usr/bin/env -S deno run --allow-read --allow-run
// Release orchestrator: sync -> dry-run -> guard -> tag -> publish -> Release.
//
// The GitHub Release step used to have no command of its own -- it was
// a manual click after `npm-publish.ts`, and so it got forgotten (e.g.
// v0.3.6 was published to npm but never got a Release). This script
// folds the Release into the same command as publish so that "publish
// succeeded but no Release exists" can no longer happen.
//
// It does NOT bump or merge the bump PR. It is normally run right after
// the bump PR is merged, while the shell is still on the `release-x.y.z`
// branch -- so the script checks out main and fast-forwards it to
// origin/main itself, and the tag is cut from there (not the release
// branch tip).
//
// The run order is the order `main()` calls its step functions at the
// bottom of this file; each step's own doc comment explains what it does
// and why. Any step aborts the whole run on failure, so later steps
// never run.
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

const PUBLISH = `${REPO_ROOT}/scripts/npm-publish.ts`;
const dec = new TextDecoder();

// --- small process helpers ----------------------------------------------

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

// Capture a command's stdout + exit code (no inherited stdio), used by
// the on-latest-main guard to read git plumbing output (branch name,
// commit hashes, porcelain status).
async function capture(
  cmd: string,
  args: string[],
): Promise<{ code: number; out: string }> {
  const { code, stdout } = await new Deno.Command(cmd, {
    args,
    stdout: "piped",
    stderr: "piped",
  }).output();
  return { code, out: dec.decode(stdout).trim() };
}

// Like capture() but a non-zero exit aborts the whole release: a failed
// git query means the world is not as the guard assumes, so the guard
// must not pass by accident on an empty string.
async function captureOk(
  label: string,
  cmd: string,
  args: string[],
): Promise<string> {
  const { code, out } = await capture(cmd, args);
  if (code !== 0) {
    console.error(`[release] ${label} failed (exit ${code}); aborting`);
    Deno.exit(1);
  }
  return out;
}

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

// --- steps (called in order by main()) ----------------------------------

// gh must be installed AND authenticated, checked up front. The GitHub
// Release is the LAST step, but it runs only after the irreversible
// `npm publish` and the tag push. A missing or unauthenticated gh would
// otherwise surface there -- leaving exactly the "published to npm but no
// Release" state this script exists to prevent. The release-absent check
// below also leans on gh and reads any non-zero `gh release view` as
// "release absent", so verifying auth here keeps an auth failure from
// masquerading as absence.
async function assertGhAuthenticated(): Promise<void> {
  try {
    const auth = await new Deno.Command("gh", {
      args: ["auth", "status"],
      stdout: "null",
      stderr: "null",
    }).output();
    if (!auth.success) {
      console.error(
        "[release] gh is not authenticated (run `gh auth login`); aborting",
      );
      Deno.exit(1);
    }
  } catch {
    console.error("[release] gh (GitHub CLI) not found on PATH; aborting");
    Deno.exit(1);
  }
}

// Check out main and fast-forward it to origin/main. This is normally run
// straight after merging the bump PR, while the shell is still on the
// `release-x.y.z` branch, so this is what puts us on main's merge commit
// before anything reads the version, dry-runs, or tags. `--ff-only`
// refuses to create a merge commit: if local main has somehow diverged it
// aborts loudly instead of silently tagging the wrong history.
async function syncToLatestMain(): Promise<void> {
  await run("checkout-main", "git", ["checkout", "main"]);
  await run("pull-main", "git", ["pull", "--ff-only", "origin", "main"]);
}

// Refuse to re-release: if the Release already exists, stop (exit 0,
// nothing to do) before touching tags or npm. `gh release view` exits
// non-zero when the release is absent -- that absence is the state we
// proceed from.
async function stopIfReleaseExists(tag: string): Promise<void> {
  const existing = await new Deno.Command("gh", {
    args: ["release", "view", tag],
    stdout: "null",
    stderr: "null",
  }).output();
  if (existing.success) {
    console.error(`[release] release ${tag} already exists; nothing to do`);
    Deno.exit(0);
  }
}

// Dry-run the publish; only its exit code gates the rest, not its output.
async function dryRunPublish(): Promise<void> {
  await run("dry-run", "deno", [
    "run",
    "--allow-read",
    "--allow-run",
    PUBLISH,
    "--dry-run",
  ]);
}

// Re-verify, immediately before the irreversible tag/push, that we are on
// a clean main still in sync with origin/main. syncToLatestMain() already
// put us here, but the dry-run build can take a while, during which the
// branch or remote could move -- so this re-checks rather than trusting
// the earlier sync, and aborts so the tag can only ever land on main's
// merge commit (never the release branch tip or a stale local main).
async function guardOnLatestMain(): Promise<void> {
  const branch = await captureOk("git rev-parse --abbrev-ref HEAD", "git", [
    "rev-parse",
    "--abbrev-ref",
    "HEAD",
  ]);
  if (branch !== "main") {
    console.error(`[release] not on main (currently on '${branch}'); aborting`);
    Deno.exit(1);
  }
  const status = await captureOk("git status --porcelain", "git", [
    "status",
    "--porcelain",
  ]);
  if (status !== "") {
    console.error("[release] working tree is not clean; aborting");
    Deno.exit(1);
  }
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
    console.error(
      `[release] local main (${
        local.slice(0, 9)
      }) is not in sync with origin/main (${remote.slice(0, 9)}); aborting`,
    );
    Deno.exit(1);
  }
}

// Tag HEAD (main's merge commit) and push the tag. A tag that already
// exists makes `git tag` / `git push` fail, which aborts the run -- a
// built-in double-run guard.
async function tagAndPush(tag: string): Promise<void> {
  await run("tag", "git", ["tag", tag]);
  await run("push-tag", "git", ["push", "origin", tag]);
}

// Publish for real (passkey browser flow preserved via inherited stdio).
async function publishForReal(): Promise<void> {
  await run("publish", "deno", ["run", "--allow-read", "--allow-run", PUBLISH]);
}

// Create the GitHub Release against the tag we just pushed. `--prerelease`
// is added when the version carries a semver pre-release identifier (e.g.
// "0.4.0-beta.0"), mirroring npm-publish.ts treating any non-`latest`
// dist-tag as a pre-release line.
async function createGitHubRelease(
  tag: string,
  isPrerelease: boolean,
): Promise<void> {
  const args = ["release", "create", tag, "--generate-notes", "--verify-tag"];
  if (isPrerelease) args.push("--prerelease");
  await run("release", "gh", args);
}

// --- orchestration: this call order IS the release sequence -------------
async function main(): Promise<void> {
  await assertGhAuthenticated();
  await syncToLatestMain();

  const version = readVersion();
  const tag = `v${version}`;
  const isPrerelease = version.includes("-");

  await stopIfReleaseExists(tag);
  await dryRunPublish();
  await guardOnLatestMain();
  await tagAndPush(tag);
  await publishForReal();
  await createGitHubRelease(tag, isPrerelease);

  console.error(
    `[release] done: ${tag} published to npm and released on GitHub`,
  );
}

await main();
