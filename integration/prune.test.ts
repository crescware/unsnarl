import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import { createDefaultPipeline } from "../src/pipeline/create-default-pipeline.js";
import { parseRootQueries } from "../src/root-query/parse-root-queries.js";

const FIXTURE_DIR = fileURLToPath(new URL("./fixtures", import.meta.url));

describe("jsx-nested-five (pruned)", () => {
  const pipeline = createDefaultPipeline();
  const fixtureDir = join(FIXTURE_DIR, "jsx-nested-five");
  const inputPath = join(fixtureDir, "input.tsx");
  const code = readFileSync(inputPath, "utf8");
  const sourcePath = "integration/fixtures/jsx-nested-five/input.tsx";

  function emitAll(
    label: string,
    rootSpec: string,
    slug: string,
    descendants: number,
    ancestors: number,
  ) {
    describe(label, () => {
      const queries = parseRootQueries(rootSpec);
      if (!queries.ok) {
        throw new Error(`unexpected query parse failure: ${queries.error}`);
      }
      const pruning = { roots: queries.queries, descendants, ancestors };

      test("emits the pruned VisualGraph JSON", () => {
        const out = pipeline.runDetailed(code, {
          format: "json",
          language: "tsx",
          sourcePath,
          emit: {
            prettyJson: true,
            prunedGraph: null,
            resolutions: null,
            debug: false,
          },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.json`),
        );
      });

      test("emits the pruned Mermaid flowchart", () => {
        const out = pipeline.runDetailed(code, {
          format: "mermaid",
          language: "tsx",
          sourcePath,
          emit: {
            prettyJson: true,
            prunedGraph: null,
            resolutions: null,
            debug: false,
          },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.mermaid`),
        );
      });

      test("renders the pruned Markdown preview", () => {
        const out = pipeline.runDetailed(code, {
          format: "markdown",
          language: "tsx",
          sourcePath,
          emit: {
            prettyJson: true,
            prunedGraph: null,
            resolutions: null,
            debug: false,
          },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `preview.pruned-${slug}.md`),
        );
      });

      test("emits the pruned stats TSV", () => {
        const out = pipeline.runDetailed(code, {
          format: "stats",
          language: "tsx",
          sourcePath,
          emit: {
            prettyJson: true,
            prunedGraph: null,
            resolutions: null,
            debug: false,
          },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.stats`),
        );
      });
    });
  }

  // -C 10 (both 10) is the implicit default when -r is given without
  // -A/-B/-C; -A 1 (descendants only) and -B 1 (ancestors only) probe the
  // single-direction radii. -A and -B are intentionally never combined here.
  for (const [rootSpec, slugBase] of [
    ["10", "r10"],
    ["10-11", "r10-11"],
    ["10-12", "r10-12"],
    ["19", "r19"],
    ["23", "r23"],
    ["24", "r24"],
  ] as const) {
    emitAll(`--roots ${rootSpec}`, rootSpec, slugBase, 10, 10);
    emitAll(`--roots ${rootSpec} -A 1`, rootSpec, `${slugBase}-a1`, 1, 0);
    emitAll(`--roots ${rootSpec} -B 1`, rootSpec, `${slugBase}-b1`, 0, 1);
  }
});
