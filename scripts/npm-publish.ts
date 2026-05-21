#!/usr/bin/env -S deno run --allow-read --allow-run --allow-env
// Publish unsnarl + unsnarl-darwin-arm64 to npm in the right order.
//
// The dist-tag for each package is read from its own package.json's
// `publishConfig.tag` and passed to `npm publish --tag <value>`
// explicitly. We cannot rely on npm picking the tag up from
// publishConfig directly: as of npm 11.12.x the publish command
// silently ignores publishConfig.tag and falls back to `latest`,
// even though the rest of publishConfig (e.g. `access: "public"`)
// is honored. Verified locally with `npm publish --dry-run` on a
// minimal package; the workaround is the explicit `--tag` flag,
// fed from the same publishConfig.tag value so there is still only
// one source of truth per package.
//
// The sub-package is published first so that the root package's
// optionalDependency on unsnarl-darwin-arm64@<version> resolves at
// install time. Both `npm publish` invocations run from each
// package's source directory (not from a pre-built tarball) so the
// `prepack` lifecycle hook fires and rebuilds the native binary
// via `mise run npm:build-darwin-arm64`.
//
// For 2FA, pass `NPM_OTP=<code>` as an env var. Without it, npm
// will prompt interactively per package, which forces typing the
// code twice (once per publish) inside the OTP window.
//
// Pass `--dry-run` (after `--` when invoked via mise) to forward
// `--dry-run` to both publish invocations and skip the trailing
// `npm view` verification.
//
// Usage (via mise):
//   mise run npm:publish
//   mise run npm:publish -- --dry-run
//   NPM_OTP=123456 mise run npm:publish

// Script lives at `scripts/npm-publish.ts`; the repo root is two
// `/`s up.
const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");
Deno.chdir(REPO_ROOT);

const dryRun = Deno.args.includes("--dry-run");
const otp = Deno.env.get("NPM_OTP") ?? "";

interface Pkg {
  name?: string;
  version?: string;
  publishConfig?: { tag?: string };
}

function readPkg(pkgJsonPath: string): Pkg {
  return JSON.parse(Deno.readTextFileSync(pkgJsonPath)) as Pkg;
}

function tagOf(pkg: Pkg, pkgJsonPath: string): string {
  const tag = pkg.publishConfig?.tag;
  if (!tag) {
    console.error(`${pkgJsonPath}: publishConfig.tag is missing`);
    Deno.exit(1);
  }
  return tag;
}

async function npmPublish(cwd: string, label: string, tag: string): Promise<void> {
  const args = ["publish", "--tag", tag];
  if (dryRun) args.push("--dry-run");
  if (otp) args.push(`--otp=${otp}`);
  const printable = args.map((a) =>
    a.startsWith("--otp=") ? "--otp=***" : a,
  ).join(" ");
  console.error(`[${label}] npm ${printable}`);
  const { success } = await new Deno.Command("npm", {
    args,
    cwd,
    stdout: "inherit",
    stderr: "inherit",
  }).output();
  if (!success) {
    console.error(`npm publish failed in ${cwd}`);
    Deno.exit(1);
  }
}

const SUB_DIR = `${REPO_ROOT}/npm/unsnarl-darwin-arm64`;
const ROOT_DIR = REPO_ROOT;

const subPkg = readPkg(`${SUB_DIR}/package.json`);
const rootPkg = readPkg(`${ROOT_DIR}/package.json`);
const subTag = tagOf(subPkg, `${SUB_DIR}/package.json`);
const rootTag = tagOf(rootPkg, `${ROOT_DIR}/package.json`);

await npmPublish(SUB_DIR, `${subPkg.name}@${subPkg.version}`, subTag);
await npmPublish(ROOT_DIR, `${rootPkg.name}@${rootPkg.version}`, rootTag);

if (!dryRun) {
  console.error("");
  console.error("--- npm view unsnarl dist-tags ---");
  const { success } = await new Deno.Command("npm", {
    args: ["view", "unsnarl", "dist-tags"],
    stdout: "inherit",
    stderr: "inherit",
  }).output();
  if (!success) {
    console.error("npm view failed");
    Deno.exit(1);
  }
}
