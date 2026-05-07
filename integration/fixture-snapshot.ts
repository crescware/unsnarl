import elkLayouts from "@mermaid-js/layout-elk";
import mermaid from "mermaid";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import type { NestingDepths } from "../src/ir/annotations/scope-annotation.js";
import { createDefaultPipeline } from "../src/pipeline/create-default-pipeline.js";
import { defaultSourceTypeFor } from "../src/pipeline/parse/default-source-type-for.js";
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

type PruneVariant = Readonly<{
  // Raw --roots argument; passed verbatim to parseRootQueries.
  roots: string;
  descendants: number;
  ancestors: number;
  // Filename slug; output goes under pruned-<slug>/. Not auto-derived
  // because existing fixtures use ad-hoc conventions (`r10-c1`,
  // `counter-a2`, `<base>-a1`) that diverge per fixture.
  slug: string;
  // describe label; defaults to `pruned: ${slug}`.
  label?: string;
}>;

export function fixtureSnapshot(metaUrl: string, variant?: PruneVariant): void {
  const here = metaUrlToDir(metaUrl);
  const inputFile = readdirSync(here).find((f) => f.startsWith("input."));
  if (!inputFile) {
    throw new Error(`no input.* file under ${here}`);
  }
  const ext = inputFile.slice("input.".length);
  if (ext !== "ts" && ext !== "tsx" && ext !== "js" && ext !== "jsx") {
    throw new Error(`unsupported input extension: ${inputFile}`);
  }
  const code = readFileSync(join(here, inputFile), "utf8");
  const sourcePath = relative(PROJECT_ROOT, join(here, inputFile));
  const name = relative(FIXTURE_DIR, here);
  const pipeline = createDefaultPipeline();
  ensureMermaid();

  const baseOpts = {
    language: ext,
    sourcePath,
    sourceType: defaultSourceTypeFor(ext),
    emit: {
      prettyJson: true,
      prunedGraph: null,
      resolutions: null,
      debug: false,
    },
  } as const;

  type Opts = Omit<PipelineRunOptions, "format">;

  function makeSnap(dir: string, opts: Opts) {
    return (title: string, format: SnapshotFormat): void => {
      test(title, () => {
        const out = pipeline.runDetailed(code, { ...opts, format }).text;
        expect(out).toMatchFileSnapshot(join(dir, FORMAT_FILE[format]));
      });
    };
  }

  if (variant === undefined) {
    const opts = { ...baseOpts, pruning: null } as const;
    describe(name, () => {
      const snap = makeSnap(here, opts);
      snap("emits the expected IR JSON", "ir");
      snap("emits the expected VisualGraph JSON", "json");
      snap("emits the expected Mermaid flowchart", "mermaid");
      snap("renders the Markdown preview", "markdown");
      snap("emits the expected stats TSV", "stats");
      test("Mermaid output parses with mermaid.parse", async () => {
        const out = pipeline.runDetailed(code, {
          ...opts,
          format: "mermaid",
        }).text;
        expect(out).not.toContain('\\"');
        await mermaid.parse(out);
      });
    });
    return;
  }

  const queries = parseRootQueries(variant.roots);
  if (!queries.ok) {
    throw new Error(
      `unexpected --roots parse failure for "${variant.roots}": ${queries.error}`,
    );
  }
  const opts: Opts = {
    ...baseOpts,
    pruning: {
      roots: queries.queries,
      descendants: variant.descendants,
      ancestors: variant.ancestors,
    },
  };
  const label = variant.label ?? `pruned: ${variant.slug}`;
  const variantDir = join(here, `pruned-${variant.slug}`);
  describe(`${name} (${label})`, () => {
    const snap = makeSnap(variantDir, opts);
    snap("emits the pruned VisualGraph JSON", "json");
    snap("emits the pruned Mermaid flowchart", "mermaid");
    snap("renders the pruned Markdown preview", "markdown");
    snap("emits the pruned stats TSV", "stats");
  });
}

type DepthVariant = Readonly<{
  // The full per-NestingKind threshold map applied at build time. Tests
  // typically build it from `uniformNestingDepths(N)` (sugar for setting
  // all kinds to N) or override one kind on top of that.
  depths: NestingDepths;
  // Filename slug; output goes under `depth-<slug>/`.
  slug: string;
  // describe label; defaults to `depth: ${slug}`.
  label?: string;
}>;

export function fixtureSnapshotDepth(
  metaUrl: string,
  variant: DepthVariant,
): void {
  const here = metaUrlToDir(metaUrl);
  const inputFile = readdirSync(here).find((f) => f.startsWith("input."));
  if (!inputFile) {
    throw new Error(`no input.* file under ${here}`);
  }
  const ext = inputFile.slice("input.".length);
  if (ext !== "ts" && ext !== "tsx" && ext !== "js" && ext !== "jsx") {
    throw new Error(`unsupported input extension: ${inputFile}`);
  }
  const code = readFileSync(join(here, inputFile), "utf8");
  const sourcePath = relative(PROJECT_ROOT, join(here, inputFile));
  const name = relative(FIXTURE_DIR, here);
  const pipeline = createDefaultPipeline();
  ensureMermaid();

  const opts = {
    language: ext,
    sourcePath,
    sourceType: defaultSourceTypeFor(ext),
    emit: {
      prettyJson: true,
      prunedGraph: null,
      resolutions: null,
      debug: false,
      depths: variant.depths,
    },
    pruning: null,
    depths: variant.depths,
  } as const satisfies Omit<PipelineRunOptions, "format">;

  const label = variant.label ?? `depth: ${variant.slug}`;
  const variantDir = join(here, `depth-${variant.slug}`);
  describe(`${name} (${label})`, () => {
    function snap(title: string, format: SnapshotFormat): void {
      test(title, () => {
        const out = pipeline.runDetailed(code, { ...opts, format }).text;
        expect(out).toMatchFileSnapshot(join(variantDir, FORMAT_FILE[format]));
      });
    }
    snap("emits the depth-bounded VisualGraph JSON", "json");
    snap("emits the depth-bounded Mermaid flowchart", "mermaid");
    snap("renders the depth-bounded Markdown preview", "markdown");
    snap("emits the depth-bounded stats TSV", "stats");
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
  const here = metaUrlToDir(metaUrl);
  const inputFile = readdirSync(here).find((f) => f.startsWith("input."));
  if (!inputFile) {
    throw new Error(`no input.* file under ${here}`);
  }
  const ext = inputFile.slice("input.".length);
  if (ext !== "ts" && ext !== "tsx" && ext !== "js" && ext !== "jsx") {
    throw new Error(`unsupported input extension: ${inputFile}`);
  }
  const code = readFileSync(join(here, inputFile), "utf8");
  const sourcePath = relative(PROJECT_ROOT, join(here, inputFile));
  const name = relative(FIXTURE_DIR, here);
  const queries = parseRootQueries(v.roots);
  if (!queries.ok) {
    throw new Error(
      `unexpected --roots parse failure for "${v.roots}": ${queries.error}`,
    );
  }
  const pipeline = createDefaultPipeline();
  const label = v.label ?? `resolves --roots ${v.roots}`;
  describe(`${name} (${label})`, () => {
    test("logs the expected resolution entries", () => {
      const result = pipeline.runDetailed(code, {
        format: "json",
        language: ext,
        sourcePath,
        sourceType: defaultSourceTypeFor(ext),
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
