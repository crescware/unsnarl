#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run
// Regenerate every `expected.*` / `preview.md` baseline under
// `integration/fixtures/` by running the Rust CLI (`uns`).
//
// Walks the fixture tree for directories that contain an `input.*`
// file, then for each fixture:
//
//   1. Runs `uns -f <format> --out-file <expected>` for each of the
//      five emitter formats to overwrite the base baselines in place.
//   2. Reads `crates/unsnarl/tests/fixture-variants/<rel>/variants.json`
//      (if present) and derives CLI flags for each variant directory.
//   3. Auto-discovers `plugin-<slug>/` sibling directories and runs
//      `uns --plugin <slug>` to regenerate plugin-variant baselines.
//
// After the script finishes, `git diff` should be empty when the Rust
// pipeline faithfully reproduces the existing (vitest-generated)
// snapshots.
//
// Usage:
//   mise run regen:fixtures
//   deno run --allow-read --allow-write --allow-run scripts/regen-fixtures.ts

const SCRIPT_PATH = new URL(import.meta.url).pathname;
const REPO_ROOT = SCRIPT_PATH.split("/").slice(0, -2).join("/");

const RUST_BIN = `${REPO_ROOT}/target/release/uns`;
const FIXTURES_ROOT = `${REPO_ROOT}/integration/fixtures`;
const VARIANTS_ROOT = `${REPO_ROOT}/crates/unsnarl/tests/fixture-variants`;

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

interface BaselineSpec {
  format: string;
  fileName: string;
}

const BASE_BASELINES: BaselineSpec[] = [
  { format: "ir", fileName: "expected.ir.json" },
  { format: "json", fileName: "expected.json" },
  { format: "mermaid", fileName: "expected.mermaid" },
  { format: "markdown", fileName: "preview.md" },
  { format: "stats", fileName: "expected.stats" },
];

const VARIANT_BASELINES: BaselineSpec[] = [
  { format: "json", fileName: "expected.json" },
  { format: "mermaid", fileName: "expected.mermaid" },
  { format: "markdown", fileName: "preview.md" },
  { format: "stats", fileName: "expected.stats" },
];

const PLUGIN_BASELINES: BaselineSpec[] = BASE_BASELINES;

interface VariantSpec {
  kind: string;
  slug: string;
  roots?: string;
  descendants?: number;
  ancestors?: number;
  depths?: DepthSpec;
  highlight?: HighlightSpec;
}

type DepthSpec =
  | { uniform: number }
  | { function: number; block: number };

type HighlightSpec =
  | { mode: "roots" }
  | { mode: "queries"; raw: string };

function findInputFile(dir: string): string | null {
  const exts = [".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs"];
  for (const entry of Deno.readDirSync(dir)) {
    if (!entry.isFile || !entry.name.startsWith("input.")) continue;
    for (const ext of exts) {
      if (entry.name === `input${ext}`) return `${dir}/${entry.name}`;
    }
  }
  return null;
}

function discoverPluginSlugs(dir: string): string[] {
  const slugs: string[] = [];
  for (const entry of Deno.readDirSync(dir)) {
    if (!entry.isDirectory || !entry.name.startsWith("plugin-")) continue;
    slugs.push(entry.name.slice("plugin-".length));
  }
  slugs.sort();
  return slugs;
}

function readVariants(fixtureRelPath: string): VariantSpec[] {
  const manifestPath = `${VARIANTS_ROOT}/${fixtureRelPath}/variants.json`;
  let text: string;
  try {
    text = Deno.readTextFileSync(manifestPath);
  } catch {
    return [];
  }
  const parsed = JSON.parse(text);
  const arr = parsed.variants as Array<Record<string, unknown>>;
  return arr.map((v) => ({
    kind: (v.kind as string) ?? "pruned",
    slug: v.slug as string,
    roots: v.roots as string | undefined,
    descendants: v.descendants as number | undefined,
    ancestors: v.ancestors as number | undefined,
    depths: v.depths as DepthSpec | undefined,
    highlight: v.highlight as HighlightSpec | undefined,
  }));
}

function variantDirName(kind: string, slug: string): string {
  const prefix: Record<string, string> = {
    pruned: "pruned",
    depth: "depth",
    "pruned-depth": "pruned-depth",
    highlight: "highlight",
    "pruned-highlight": "pruned-highlight",
  };
  return `${prefix[kind]}-${slug}`;
}

function needsPruning(kind: string): boolean {
  return kind === "pruned" || kind === "pruned-depth" || kind === "pruned-highlight";
}

function needsHighlight(kind: string): boolean {
  return kind === "highlight" || kind === "pruned-highlight";
}

function needsDepth(kind: string): boolean {
  return kind === "depth" || kind === "pruned-depth";
}

function buildVariantFlags(v: VariantSpec): string[] {
  const flags: string[] = [];
  if (needsPruning(v.kind)) {
    flags.push("-r", v.roots!);
    flags.push("-A", String(v.descendants!));
    flags.push("-B", String(v.ancestors!));
  }
  if (needsDepth(v.kind) && v.depths) {
    if ("uniform" in v.depths) {
      flags.push("--depth", String(v.depths.uniform));
    } else {
      flags.push("--depth-function", String(v.depths.function));
      flags.push("--depth-block", String(v.depths.block));
    }
  }
  if (needsHighlight(v.kind) && v.highlight) {
    if (v.highlight.mode === "roots") {
      flags.push("-H");
    } else {
      flags.push("-H", v.highlight.raw);
    }
  }
  return flags;
}

function* walkFixtures(dir: string): Generator<string> {
  let entries: Deno.DirEntry[];
  try {
    entries = [...Deno.readDirSync(dir)];
  } catch {
    return;
  }
  if (findInputFile(dir) !== null) {
    yield dir;
  }
  for (const entry of entries) {
    if (!entry.isDirectory) continue;
    if (
      entry.name.startsWith("pruned-") ||
      entry.name.startsWith("depth-") ||
      entry.name.startsWith("pruned-depth-") ||
      entry.name.startsWith("highlight-") ||
      entry.name.startsWith("pruned-highlight-") ||
      entry.name.startsWith("plugin-")
    ) {
      continue;
    }
    yield* walkFixtures(`${dir}/${entry.name}`);
  }
}

async function runUns(args: string[]): Promise<boolean> {
  const proc = await new Deno.Command(RUST_BIN, {
    args,
    cwd: REPO_ROOT,
    stdout: "null",
    stderr: "piped",
  }).output();
  if (!proc.success) {
    const stderr = new TextDecoder().decode(proc.stderr);
    console.error(`  FAIL: uns ${args.join(" ")}`);
    console.error(`  ${stderr.trim()}`);
  }
  return proc.success;
}

const fixtureDirs = [...walkFixtures(FIXTURES_ROOT)].sort();
const total = fixtureDirs.length;
console.error(`Found ${total} fixture directories`);

let generated = 0;
let failed = 0;
let i = 0;

for (const dir of fixtureDirs) {
  i++;
  const inputPath = findInputFile(dir)!;
  const relInput = inputPath.slice(REPO_ROOT.length + 1);
  const relDir = dir.slice(FIXTURES_ROOT.length + 1);

  for (const b of BASE_BASELINES) {
    const outFile = `${dir}/${b.fileName}`;
    const ok = await runUns(["-f", b.format, "--out-file", outFile, relInput]);
    if (ok) generated++;
    else failed++;
  }

  const variants = readVariants(relDir);
  for (const v of variants) {
    const varDir = `${dir}/${variantDirName(v.kind, v.slug)}`;
    try {
      Deno.statSync(varDir);
    } catch {
      continue;
    }
    const extraFlags = buildVariantFlags(v);
    for (const b of VARIANT_BASELINES) {
      const outFile = `${varDir}/${b.fileName}`;
      try {
        Deno.statSync(outFile);
      } catch {
        continue;
      }
      const ok = await runUns([
        "-f", b.format, ...extraFlags, "--out-file", outFile, relInput,
      ]);
      if (ok) generated++;
      else failed++;
    }
  }

  for (const slug of discoverPluginSlugs(dir)) {
    const pluginDir = `${dir}/plugin-${slug}`;
    for (const b of PLUGIN_BASELINES) {
      const outFile = `${pluginDir}/${b.fileName}`;
      try {
        Deno.statSync(outFile);
      } catch {
        continue;
      }
      const ok = await runUns([
        "-f", b.format, "--plugin", slug, "--out-file", outFile, relInput,
      ]);
      if (ok) generated++;
      else failed++;
    }
  }

  if (i % 50 === 0) {
    console.error(`progress: ${i}/${total} generated=${generated} failed=${failed}`);
  }
}

console.error(`\nDone: ${generated} baselines regenerated, ${failed} failures`);
if (failed > 0) Deno.exit(1);
