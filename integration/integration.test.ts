import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import { createDefaultPipeline } from "../src/index.js";

const FIXTURE_DIR = fileURLToPath(new URL("./fixtures", import.meta.url));

interface FixtureCase {
  name: string;
  inputPath: string;
  inputFile: string;
  language: "ts" | "tsx" | "js" | "jsx";
}

function discoverFixtures(): FixtureCase[] {
  const out: FixtureCase[] = [];
  for (const entry of readdirSync(FIXTURE_DIR, { withFileTypes: true })) {
    if (!entry.isDirectory()) {
      continue;
    }
    const dir = join(FIXTURE_DIR, entry.name);
    const files = readdirSync(dir, { withFileTypes: true });
    const input = files.find((f) => f.isFile() && f.name.startsWith("input."));
    if (!input) {
      continue;
    }
    const ext = input.name.slice("input.".length);
    if (ext !== "ts" && ext !== "tsx" && ext !== "js" && ext !== "jsx") {
      continue;
    }
    out.push({
      name: entry.name,
      inputPath: join(dir, input.name),
      inputFile: input.name,
      language: ext,
    });
  }
  return out.sort((a, b) => a.name.localeCompare(b.name));
}

const fixtures = discoverFixtures();

describe("fixtures (end-to-end pipeline)", () => {
  test("at least one fixture is present", () => {
    expect(fixtures.length).toBeGreaterThan(0);
  });

  for (const fixture of fixtures) {
    describe(fixture.name, () => {
      const pipeline = createDefaultPipeline();
      const code = readFileSync(fixture.inputPath, "utf8");
      const sourcePath = `integration/fixtures/${fixture.name}/${fixture.inputFile}`;

      test("emits the expected IR JSON", () => {
        const out = pipeline.run(code, {
          format: "ir",
          language: fixture.language,
          sourcePath,
          emit: { pretty: true, prunedGraph: null },
          pruning: null,
        });
        expect(out).toMatchFileSnapshot(
          join(FIXTURE_DIR, fixture.name, "expected.ir.json"),
        );
      });

      test("emits the expected VisualGraph JSON", () => {
        const out = pipeline.run(code, {
          format: "json",
          language: fixture.language,
          sourcePath,
          emit: { pretty: true, prunedGraph: null },
          pruning: null,
        });
        expect(out).toMatchFileSnapshot(
          join(FIXTURE_DIR, fixture.name, "expected.json"),
        );
      });

      test("emits the expected Mermaid flowchart", () => {
        const out = pipeline.run(code, {
          format: "mermaid",
          language: fixture.language,
          sourcePath,
          emit: { pretty: true, prunedGraph: null },
          pruning: null,
        });
        expect(out).toMatchFileSnapshot(
          join(FIXTURE_DIR, fixture.name, "expected.mermaid"),
        );
      });

      test("renders the Markdown preview via the markdown emitter", () => {
        const out = pipeline.run(code, {
          format: "markdown",
          language: fixture.language,
          sourcePath,
          emit: { pretty: true, prunedGraph: null },
          pruning: null,
        });
        expect(out).toMatchFileSnapshot(
          join(FIXTURE_DIR, fixture.name, "preview.md"),
        );
      });

      test("emits the expected stats TSV", () => {
        const out = pipeline.run(code, {
          format: "stats",
          language: fixture.language,
          sourcePath,
          emit: { pretty: true, prunedGraph: null },
          pruning: null,
        });
        expect(out).toMatchFileSnapshot(
          join(FIXTURE_DIR, fixture.name, "expected.stats"),
        );
      });
    });
  }
});
