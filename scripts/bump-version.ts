#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run
// Align every version field across the workspace + npm packages
// to the version string passed as the first argument, then
// regenerate Cargo.lock by running `cargo build -p unsnarl`.
//
// This is the only sanctioned way to set the project version.
// Editing by hand is error-prone because four fields must be kept
// consistent in lockstep:
//   - Cargo.toml `[workspace.package].version`
//   - package.json `version`
//   - package.json `optionalDependencies["unsnarl-darwin-arm64"]`
//   - npm/unsnarl-darwin-arm64/package.json `version`
//
// The script sets all four to the argument verbatim; it does NOT
// derive "the next version" from semver rules. Pass the exact
// version string you want everywhere.
//
// Usage (via mise):
//   mise run bump -- 0.3.0-rc.2
//   mise run bump -- 0.3.0

const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");
Deno.chdir(REPO_ROOT);

const target = Deno.args[0];
if (!target) {
  console.error("usage: bump-version.ts <new-version>");
  Deno.exit(1);
}

const VERSION_RE = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;
if (!VERSION_RE.test(target)) {
  console.error(
    `'${target}' is not a valid semver string (expected e.g. 0.3.0-rc.1)`,
  );
  Deno.exit(1);
}

function replaceOnce(path: string, pattern: RegExp, replacement: string) {
  const text = Deno.readTextFileSync(path);
  const all = text.match(new RegExp(pattern, pattern.flags + "g"));
  if (!all || all.length === 0) {
    console.error(`${path}: pattern ${pattern} not found`);
    Deno.exit(1);
  }
  if (all.length > 1) {
    console.error(
      `${path}: pattern ${pattern} matched ${all.length} times; aborting`,
    );
    Deno.exit(1);
  }
  const next = text.replace(pattern, replacement);
  Deno.writeTextFileSync(path, next);
  console.error(`  ${path}`);
}

console.error(`Setting all version fields to ${target}...`);

// Cargo.toml — version under [workspace.package]. The header is
// anchored to avoid colliding with any unrelated `version = "..."`
// line elsewhere in the file.
replaceOnce(
  "Cargo.toml",
  /(\[workspace\.package\]\nversion = ")[^"]+(")/,
  `$1${target}$2`,
);

// package.json — main "version" field (top-level, first
// occurrence in the file).
replaceOnce(
  "package.json",
  /"version": "[^"]+"/,
  `"version": "${target}"`,
);

// package.json — optionalDependencies entry must stay in lockstep
// with the sub-package version.
replaceOnce(
  "package.json",
  /"unsnarl-darwin-arm64": "[^"]+"/,
  `"unsnarl-darwin-arm64": "${target}"`,
);

// npm/unsnarl-darwin-arm64/package.json — sub-package "version".
replaceOnce(
  "npm/unsnarl-darwin-arm64/package.json",
  /"version": "[^"]+"/,
  `"version": "${target}"`,
);

console.error("");
console.error("Regenerating Cargo.lock via `cargo build -p unsnarl`...");
const status = await new Deno.Command("cargo", {
  args: ["build", "-p", "unsnarl"],
  stdin: "inherit",
  stdout: "inherit",
  stderr: "inherit",
}).spawn().status;
if (!status.success) {
  console.error("cargo build failed; Cargo.lock may not be regenerated");
  Deno.exit(1);
}

console.error("");
console.error(`All version fields aligned to ${target}.`);
