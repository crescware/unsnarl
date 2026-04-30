import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import { createDefaultPipeline, parseRootQueries } from "../src/index.js";

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
      const out = pipeline.run(code, {
        format: "json",
        language: "ts",
        sourcePath,
        emit: { pretty: true },
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-r10-c1.json"),
      );
    });

    test("emits the pruned Mermaid flowchart", () => {
      const out = pipeline.run(code, {
        format: "mermaid",
        language: "ts",
        sourcePath,
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-r10-c1.mermaid"),
      );
    });

    test("renders the pruned Markdown preview", () => {
      const out = pipeline.run(code, {
        format: "markdown",
        language: "ts",
        sourcePath,
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "preview.pruned-r10-c1.md"),
      );
    });

    test("emits the pruned stats TSV", () => {
      const out = pipeline.run(code, {
        format: "stats",
        language: "ts",
        sourcePath,
        pruning,
      });
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
      const out = pipeline.run(code, {
        format: "json",
        language: "ts",
        sourcePath,
        emit: { pretty: true },
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-a2.json"),
      );
    });

    test("emits the pruned Mermaid flowchart", () => {
      const out = pipeline.run(code, {
        format: "mermaid",
        language: "ts",
        sourcePath,
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-a2.mermaid"),
      );
    });

    test("renders the pruned Markdown preview", () => {
      const out = pipeline.run(code, {
        format: "markdown",
        language: "ts",
        sourcePath,
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "preview.pruned-counter-a2.md"),
      );
    });

    test("emits the pruned stats TSV", () => {
      const out = pipeline.run(code, {
        format: "stats",
        language: "ts",
        sourcePath,
        pruning,
      });
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
      const out = pipeline.run(code, {
        format: "json",
        language: "ts",
        sourcePath,
        emit: { pretty: true },
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-b2.json"),
      );
    });

    test("emits the pruned Mermaid flowchart", () => {
      const out = pipeline.run(code, {
        format: "mermaid",
        language: "ts",
        sourcePath,
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "expected.pruned-counter-b2.mermaid"),
      );
    });

    test("renders the pruned Markdown preview", () => {
      const out = pipeline.run(code, {
        format: "markdown",
        language: "ts",
        sourcePath,
        pruning,
      });
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, "preview.pruned-counter-b2.md"),
      );
    });

    test("emits the pruned stats TSV", () => {
      const out = pipeline.run(code, {
        format: "stats",
        language: "ts",
        sourcePath,
        pruning,
      });
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
        const out = pipeline.run(code, {
          format: "json",
          language: "ts",
          sourcePath,
          emit: { pretty: true },
          pruning,
        });
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.json`),
        );
      });

      test("emits the pruned Mermaid flowchart", () => {
        const out = pipeline.run(code, {
          format: "mermaid",
          language: "ts",
          sourcePath,
          pruning,
        });
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `expected.pruned-${slug}.mermaid`),
        );
      });

      test("renders the pruned Markdown preview", () => {
        const out = pipeline.run(code, {
          format: "markdown",
          language: "ts",
          sourcePath,
          pruning,
        });
        expect(out).toMatchFileSnapshot(
          join(fixtureDir, `preview.pruned-${slug}.md`),
        );
      });

      test("emits the pruned stats TSV", () => {
        const out = pipeline.run(code, {
          format: "stats",
          language: "ts",
          sourcePath,
          pruning,
        });
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
