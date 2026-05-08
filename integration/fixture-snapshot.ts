import elkLayouts from "@mermaid-js/layout-elk";
import mermaid from "mermaid";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import type { NestingDepths } from "../src/ir/annotations/scope-annotation.js";
import { LANGUAGE, type Language } from "../src/language.js";
import { createDefaultPipeline } from "../src/pipeline/create-default-pipeline.js";
import { sourceTypeFromPath } from "../src/pipeline/parse/source-type-from-path.js";
import type { PruningRunOptions } from "../src/pipeline/prune/pruning-run-options.js";
import type { PipelineRunOptions } from "../src/pipeline/runner/pipeline-run-options.js";
import { parseRootQueries } from "../src/root-query/parse-root-queries.js";
import type { RootQueryResolution } from "../src/visual-graph/prune/root-query-resolution.js";

// Under the jsdom Vitest environment, import.meta.url is not a file:// URL,
// so fileURLToPath would throw. Resolve via cwd, which Vitest sets to the
// project root.
const PROJECT_ROOT = process.cwd();
const FIXTURE_DIR = join(PROJECT_ROOT, "integration", "fixtures");

function metaUrlToDir(metaUrl: string): string {
  try {
    return dirname(fileURLToPath(metaUrl));
  } catch {
    return dirname(new URL(metaUrl).pathname);
  }
}

let mermaidReady = false;
function ensureMermaid(): void {
  if (mermaidReady) {
    return;
  }
  mermaid.registerLayoutLoaders(elkLayouts);
  mermaid.initialize({ startOnLoad: false });
  mermaidReady = true;
}

type SnapshotFormat = "ir" | "json" | "mermaid" | "markdown" | "stats";

const FORMAT_FILE: Readonly<Record<SnapshotFormat, string>> = {
  ir: "expected.ir.json",
  json: "expected.json",
  mermaid: "expected.mermaid",
  markdown: "preview.md",
  stats: "expected.stats",
};

type FixtureContext = Readonly<{
  here: string;
  // Resolved IR language. `.mjs` and `.cjs` both map to `js` because
  // they are JavaScript at the parser level; their distinction lives in
  // the source path that's handed to `sourceTypeFromPath`.
  language: Language;
  code: string;
  sourcePath: string;
  name: string;
}>;

function fixtureLanguageFromExt(ext: string): Language | null {
  switch (ext) {
    case "ts":
      return LANGUAGE.Ts;
    case "tsx":
      return LANGUAGE.Tsx;
    case "jsx":
      return LANGUAGE.Jsx;
    case "js":
    case "mjs":
    case "cjs":
      return LANGUAGE.Js;
    default:
      return null;
  }
}

function loadFixture(metaUrl: string): FixtureContext {
  const here = metaUrlToDir(metaUrl);
  const inputFile = readdirSync(here).find((f) => f.startsWith("input."));
  if (!inputFile) {
    throw new Error(`no input.* file under ${here}`);
  }
  const ext = inputFile.slice("input.".length);
  const language = fixtureLanguageFromExt(ext);
  if (language === null) {
    throw new Error(`unsupported input extension: ${inputFile}`);
  }
  const code = readFileSync(join(here, inputFile), "utf8");
  const sourcePath = relative(PROJECT_ROOT, join(here, inputFile));
  const name = relative(FIXTURE_DIR, here);
  return { here, language, code, sourcePath, name };
}

type FixturePruning = Readonly<{
  // Raw --roots argument; passed verbatim to parseRootQueries.
  roots: string;
  descendants: number;
  ancestors: number;
}>;

type FixtureVariantBase = Readonly<{
  // Filename slug; output goes under `<mode>-<slug>/`, where <mode>
  // tracks which restrictions are active (`pruned`, `depth`,
  // `pruned-depth`). Slugs are not auto-derived because existing
  // fixtures use ad-hoc conventions (`r10-c1`, `counter-a2`, etc.)
  // that diverge per fixture.
  slug: string;
  // describe label; defaults to `<mode>: ${slug}`.
  label?: string;
}>;

// At least one of `pruning` / `depths` must be set; otherwise the run
// would produce the same output as the baseline call. The union of
// "pruning required (depths optional)" with "depths required (pruning
// optional)" enforces that at the type level.
type FixtureVariant =
  | (FixtureVariantBase &
      Readonly<{ pruning: FixturePruning; depths?: NestingDepths }>)
  | (FixtureVariantBase &
      Readonly<{ pruning?: FixturePruning; depths: NestingDepths }>);

type VariantMode = "pruned" | "depth" | "pruned-depth";

function variantMode(variant: FixtureVariant): VariantMode {
  if (variant.pruning && variant.depths) {
    return "pruned-depth";
  }
  if (variant.pruning) {
    return "pruned";
  }
  return "depth";
}

const VARIANT_LABEL: Readonly<Record<VariantMode, string>> = {
  pruned: "pruned",
  depth: "depth",
  "pruned-depth": "pruned+depth",
};

const VARIANT_ADJECTIVE: Readonly<Record<VariantMode, string>> = {
  pruned: "pruned",
  depth: "depth-bounded",
  "pruned-depth": "pruned + depth-bounded",
};

function buildPruning(p: FixturePruning): PruningRunOptions {
  const queries = parseRootQueries(p.roots);
  if (!queries.ok) {
    throw new Error(
      `unexpected --roots parse failure for "${p.roots}": ${queries.error}`,
    );
  }
  return {
    roots: queries.queries,
    descendants: p.descendants,
    ancestors: p.ancestors,
  };
}

type Opts = Omit<PipelineRunOptions, "format">;

function buildBaseOpts(
  ctx: FixtureContext,
): Pick<Opts, "language" | "sourcePath" | "sourceType" | "emit"> {
  return {
    language: ctx.language,
    sourcePath: ctx.sourcePath,
    sourceType: sourceTypeFromPath(ctx.sourcePath, ctx.language),
    emit: {
      prettyJson: true,
      prunedGraph: null,
      resolutions: null,
      debug: false,
    },
  };
}

export function fixtureSnapshot(
  metaUrl: string,
  variant?: FixtureVariant,
): void {
  const ctx = loadFixture(metaUrl);
  const pipeline = createDefaultPipeline();
  ensureMermaid();
  const baseOpts = buildBaseOpts(ctx);

  function snapWith(dir: string, opts: Opts) {
    return (title: string, format: SnapshotFormat): void => {
      test(title, () => {
        const out = pipeline.runDetailed(ctx.code, { ...opts, format }).text;
        expect(out).toMatchFileSnapshot(join(dir, FORMAT_FILE[format]));
      });
    };
  }

  if (variant === undefined) {
    const opts: Opts = { ...baseOpts, pruning: null };
    describe(ctx.name, () => {
      const snap = snapWith(ctx.here, opts);
      snap("emits the expected IR JSON", "ir");
      snap("emits the expected VisualGraph JSON", "json");
      snap("emits the expected Mermaid flowchart", "mermaid");
      snap("renders the Markdown preview", "markdown");
      snap("emits the expected stats TSV", "stats");
      test("Mermaid output parses with mermaid.parse", async () => {
        const out = pipeline.runDetailed(ctx.code, {
          ...opts,
          format: "mermaid",
        }).text;
        expect(out).not.toContain('\\"');
        await mermaid.parse(out);
      });
    });
    return;
  }

  const mode = variantMode(variant);
  const opts: Opts = {
    ...baseOpts,
    pruning: variant.pruning ? buildPruning(variant.pruning) : null,
    ...(variant.depths !== undefined ? { depths: variant.depths } : {}),
  };
  const adjective = VARIANT_ADJECTIVE[mode];
  const label = variant.label ?? `${VARIANT_LABEL[mode]}: ${variant.slug}`;
  const variantDir = join(ctx.here, `${mode}-${variant.slug}`);
  describe(`${ctx.name} (${label})`, () => {
    const snap = snapWith(variantDir, opts);
    snap(`emits the ${adjective} VisualGraph JSON`, "json");
    snap(`emits the ${adjective} Mermaid flowchart`, "mermaid");
    snap(`renders the ${adjective} Markdown preview`, "markdown");
    snap(`emits the ${adjective} stats TSV`, "stats");
  });
}

type ResolutionsAssertion = Readonly<{
  roots: string;
  descendants: number;
  ancestors: number;
  expected: readonly RootQueryResolution[];
  label?: string;
}>;

export function fixtureResolutions(
  metaUrl: string,
  v: ResolutionsAssertion,
): void {
  const ctx = loadFixture(metaUrl);
  const queries = parseRootQueries(v.roots);
  if (!queries.ok) {
    throw new Error(
      `unexpected --roots parse failure for "${v.roots}": ${queries.error}`,
    );
  }
  const pipeline = createDefaultPipeline();
  const label = v.label ?? `resolves --roots ${v.roots}`;
  describe(`${ctx.name} (${label})`, () => {
    test("logs the expected resolution entries", () => {
      const result = pipeline.runDetailed(ctx.code, {
        format: "json",
        language: ctx.language,
        sourcePath: ctx.sourcePath,
        sourceType: sourceTypeFromPath(ctx.sourcePath, ctx.language),
        emit: {
          prettyJson: true,
          prunedGraph: null,
          resolutions: null,
          debug: false,
        },
        pruning: {
          roots: queries.queries,
          descendants: v.descendants,
          ancestors: v.ancestors,
        },
      });
      expect(result.resolutions).toEqual(v.expected);
    });
  });
}
