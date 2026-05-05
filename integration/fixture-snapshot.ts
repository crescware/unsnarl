import elkLayouts from "@mermaid-js/layout-elk";
import mermaid from "mermaid";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import { createDefaultPipeline } from "../src/pipeline/create-default-pipeline.js";

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

export function fixtureSnapshot(metaUrl: string): void {
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
    emit: {
      prettyJson: true,
      prunedGraph: null,
      resolutions: null,
      debug: false,
    },
    pruning: null,
  } as const;

  describe(name, () => {
    test("emits the expected IR JSON", () => {
      const out = pipeline.runDetailed(code, { ...opts, format: "ir" }).text;
      expect(out).toMatchFileSnapshot(join(here, "expected.ir.json"));
    });
    test("emits the expected VisualGraph JSON", () => {
      const out = pipeline.runDetailed(code, { ...opts, format: "json" }).text;
      expect(out).toMatchFileSnapshot(join(here, "expected.json"));
    });
    test("emits the expected Mermaid flowchart", () => {
      const out = pipeline.runDetailed(code, {
        ...opts,
        format: "mermaid",
      }).text;
      expect(out).toMatchFileSnapshot(join(here, "expected.mermaid"));
    });
    test("renders the Markdown preview", () => {
      const out = pipeline.runDetailed(code, {
        ...opts,
        format: "markdown",
      }).text;
      expect(out).toMatchFileSnapshot(join(here, "preview.md"));
    });
    test("emits the expected stats TSV", () => {
      const out = pipeline.runDetailed(code, { ...opts, format: "stats" }).text;
      expect(out).toMatchFileSnapshot(join(here, "expected.stats"));
    });
    test("Mermaid output parses with mermaid.parse", async () => {
      const out = pipeline.runDetailed(code, {
        ...opts,
        format: "mermaid",
      }).text;
      expect(out).not.toContain('\\"');
      await mermaid.parse(out);
    });
  });
}
