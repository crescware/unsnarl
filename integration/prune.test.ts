import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import { parseRootQueries } from "../src/cli/root-query/parse-root-queries.js";
import { createDefaultPipeline } from "../src/pipeline/default.js";

const FIXTURE_DIR = fileURLToPath(new URL("./fixtures", import.meta.url));

describe("control-if (pruned)", () => {
  const pipeline = createDefaultPipeline();
  const fixtureDir = join(FIXTURE_DIR, "control-if");
  const inputPath = join(fixtureDir, "input.ts");
  const code = readFileSync(inputPath, "utf8");
  const sourcePath = "integration/fixtures/control-if/input.ts";

  describe("--roots 10 -C 1", () => {
    const queries = parseRootQueries("10");
    if (!queries.ok) {
      throw new Error(`unexpected query parse failure: ${queries.error}`);
    }
    const pruning = {
      roots: queries.queries,
      descendants: 1,
      ancestors: 1,
    };

    test("emits the pruned VisualGraph JSON", () => {
      const out = pipeline.runDetailed(code, {
        format: "json",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-r10-c1.json"),
      );
    });

    test("emits the pruned Mermaid flowchart", () => {
      const out = pipeline.runDetailed(code, {
        format: "mermaid",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-r10-c1.mermaid"),
      );
    });

    test("renders the pruned Markdown preview", () => {
      const out = pipeline.runDetailed(code, {
        format: "markdown",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "preview.pruned-r10-c1.md"),
      );
    });

    test("emits the pruned stats TSV", () => {
      const out = pipeline.runDetailed(code, {
        format: "stats",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-r10-c1.stats"),
      );
    });
  });

  describe("--roots counter -B 0 -A 2", () => {
    const queries = parseRootQueries("counter");
    if (!queries.ok) {
      throw new Error(`unexpected query parse failure: ${queries.error}`);
    }
    const pruning = {
      roots: queries.queries,
      descendants: 2,
      ancestors: 0,
    };

    test("emits the pruned VisualGraph JSON", () => {
      const out = pipeline.runDetailed(code, {
        format: "json",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-a2.json"),
      );
    });

    test("emits the pruned Mermaid flowchart", () => {
      const out = pipeline.runDetailed(code, {
        format: "mermaid",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-a2.mermaid"),
      );
    });

    test("renders the pruned Markdown preview", () => {
      const out = pipeline.runDetailed(code, {
        format: "markdown",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "preview.pruned-counter-a2.md"),
      );
    });

    test("emits the pruned stats TSV", () => {
      const out = pipeline.runDetailed(code, {
        format: "stats",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-a2.stats"),
      );
    });
  });

  describe("--roots counter -A 0 -B 2", () => {
    const queries = parseRootQueries("counter");
    if (!queries.ok) {
      throw new Error(`unexpected query parse failure: ${queries.error}`);
    }
    const pruning = {
      roots: queries.queries,
      descendants: 0,
      ancestors: 2,
    };

    test("emits the pruned VisualGraph JSON", () => {
      const out = pipeline.runDetailed(code, {
        format: "json",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-b2.json"),
      );
    });

    test("emits the pruned Mermaid flowchart", () => {
      const out = pipeline.runDetailed(code, {
        format: "mermaid",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-b2.mermaid"),
      );
    });

    test("renders the pruned Markdown preview", () => {
      const out = pipeline.runDetailed(code, {
        format: "markdown",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "preview.pruned-counter-b2.md"),
      );
    });

    test("emits the pruned stats TSV", () => {
      const out = pipeline.runDetailed(code, {
        format: "stats",
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-b2.stats"),
      );
    });
  });
});

describe("const-chain-five (pruned)", () => {
  const pipeline = createDefaultPipeline();
  const fixtureDir = join(FIXTURE_DIR, "const-chain-five");
  const inputPath = join(fixtureDir, "input.ts");
  const code = readFileSync(inputPath, "utf8");
  const sourcePath = "integration/fixtures/const-chain-five/input.ts";

  function emitAll(
    label: string,
    rootName: string,
    slug: string,
    descendants: number,
    ancestors: number,
  ) {
    describe(label, () => {
      const queries = parseRootQueries(rootName);
      if (!queries.ok) {
        throw new Error(`unexpected query parse failure: ${queries.error}`);
      }
      const pruning = { roots: queries.queries, descendants, ancestors };

      test("emits the pruned VisualGraph JSON", () => {
        const out = pipeline.runDetailed(code, {
          format: "json",
          language: "ts",
          sourcePath,
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.json`),
        );
      });

      test("emits the pruned Mermaid flowchart", () => {
        const out = pipeline.runDetailed(code, {
          format: "mermaid",
          language: "ts",
          sourcePath,
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.mermaid`),
        );
      });

      test("renders the pruned Markdown preview", () => {
        const out = pipeline.runDetailed(code, {
          format: "markdown",
          language: "ts",
          sourcePath,
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `preview.pruned-${slug}.md`),
        );
      });

      test("emits the pruned stats TSV", () => {
        const out = pipeline.runDetailed(code, {
          format: "stats",
          language: "ts",
          sourcePath,
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
          pruning,
        }).text;
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.stats`),
        );
      });
    });
  }

  // a -> b -> c -> d -> e (linear read chain), e is unused.
  emitAll("--roots a -C 1", "a", "a-c1", 1, 1);
  emitAll("--roots e -C 1", "e", "e-c1", 1, 1);
  emitAll("--roots c -A 1 -B 0", "c", "c-a1", 1, 0);
  emitAll("--roots c -A 0 -B 1", "c", "c-b1", 0, 1);
});

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
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
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
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
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
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
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
          emit: { prettyJson: true, prunedGraph: null, resolutions: null },
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
