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
  });
});
