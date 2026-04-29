// @vitest-environment jsdom

import elkLayouts from "@mermaid-js/layout-elk";
import mermaid from "mermaid";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, test } from "vitest";

import { createDefaultPipeline } from "../src/index.js";

const FIXTURE_DIR = join(process.cwd(), "integration", "fixtures");

interface FixtureCase {
  name: string;
  inputPath: string;
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
      language: ext,
    });
  }
  return out.sort((a, b) => a.name.localeCompare(b.name));
}

mermaid.registerLayoutLoaders(elkLayouts);
mermaid.initialize({ startOnLoad: false });

const pipeline = createDefaultPipeline();
const fixtures = discoverFixtures();

describe("Mermaid output syntax (real parser)", () => {
  test("there is at least one fixture", () => {
    expect(fixtures.length).toBeGreaterThan(0);
  });

  for (const fixture of fixtures) {
    test(`${fixture.name} parses with mermaid.parse`, async () => {
      const code = readFileSync(fixture.inputPath, "utf8");
      const out = pipeline.run(code, {
        format: "mermaid",
        language: fixture.language,
        sourcePath: `integration/fixtures/${fixture.name}/input.${fixture.language}`,
      });
      expect(out).not.toContain('\\"');
      await mermaid.parse(out);
    });
  }
});
