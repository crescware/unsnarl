import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import { parseRootQueries } from "../src/root-query/parse-root-queries.js";
import { createDefaultPipeline } from "../src/pipeline/create-default-pipeline.js";

const FIXTURE_DIR = fileURLToPath(new URL("./fixtures", import.meta.url));

function parseOrThrow(input: string) {
  const r = parseRootQueries(input);
  if (!r.ok) {
    throw new Error(
      `unexpected query parse failure for '${input}': ${r.error}`,
    );
  }
  return r.queries;
}

const FORMATS = [
  { format: "json", ext: "json" },
  { format: "mermaid", ext: "mermaid" },
  { format: "markdown", ext: "md", filePrefix: "preview" },
  { format: "stats", ext: "stats" },
] as const;

describe("lprefix-no-collision (-r L12 → silent line)", () => {
  const pipeline = createDefaultPipeline();
  const fixtureDir = join(FIXTURE_DIR, "lprefix-no-collision");
  const code = readFileSync(join(fixtureDir, "input.ts"), "utf8");
  const sourcePath = "integration/fixtures/lprefix-no-collision/input.ts";
  const pruning = {
    roots: parseOrThrow("L12"),
    descendants: 1,
    ancestors: 1,
  };

  test("returns no resolution log entries when no [Ll]<n> identifier exists", () => {
    const result = pipeline.runDetailed(code, {
      format: "json",
      language: "ts",
      sourcePath,
      emit: { prettyJson: true, prunedGraph: null, resolutions: null },
      pruning,
    });
    expect(result.resolutions).toEqual([]);
  });

  for (const fmt of FORMATS) {
    test(`emits the pruned ${fmt.format} output`, () => {
      const out = pipeline.runDetailed(code, {
        format: fmt.format,
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      const prefix = "filePrefix" in fmt ? fmt.filePrefix : "expected";
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, `${prefix}.pruned-L12-c1.${fmt.ext}`),
      );
    });
  }
});

describe("lprefix-exact-match (-r L12 → identifier)", () => {
  const pipeline = createDefaultPipeline();
  const fixtureDir = join(FIXTURE_DIR, "lprefix-exact-match");
  const code = readFileSync(join(fixtureDir, "input.ts"), "utf8");
  const sourcePath = "integration/fixtures/lprefix-exact-match/input.ts";
  const pruning = {
    roots: parseOrThrow("L12"),
    descendants: 1,
    ancestors: 1,
  };

  test("logs a name resolution when the source declares the exact identifier", () => {
    const result = pipeline.runDetailed(code, {
      format: "json",
      language: "ts",
      sourcePath,
      emit: { prettyJson: true, prunedGraph: null, resolutions: null },
      pruning,
    });
    expect(result.resolutions).toEqual([
      { raw: "L12", line: 12, name: "L12", resolvedAs: "name" },
    ]);
  });

  for (const fmt of FORMATS) {
    test(`emits the pruned ${fmt.format} output`, () => {
      const out = pipeline.runDetailed(code, {
        format: fmt.format,
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      const prefix = "filePrefix" in fmt ? fmt.filePrefix : "expected";
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, `${prefix}.pruned-L12-c1.${fmt.ext}`),
      );
    });
  }
});

describe("lprefix-other-match (-r L12 → line, with notice)", () => {
  const pipeline = createDefaultPipeline();
  const fixtureDir = join(FIXTURE_DIR, "lprefix-other-match");
  const code = readFileSync(join(fixtureDir, "input.ts"), "utf8");
  const sourcePath = "integration/fixtures/lprefix-other-match/input.ts";
  const pruning = {
    roots: parseOrThrow("L12"),
    descendants: 1,
    ancestors: 1,
  };

  test("logs a line resolution when [Ll]<n> exists but the exact name does not", () => {
    const result = pipeline.runDetailed(code, {
      format: "json",
      language: "ts",
      sourcePath,
      emit: { prettyJson: true, prunedGraph: null, resolutions: null },
      pruning,
    });
    expect(result.resolutions).toEqual([
      { raw: "L12", line: 12, name: "L12", resolvedAs: "line" },
    ]);
  });

  for (const fmt of FORMATS) {
    test(`emits the pruned ${fmt.format} output`, () => {
      const out = pipeline.runDetailed(code, {
        format: fmt.format,
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      const prefix = "filePrefix" in fmt ? fmt.filePrefix : "expected";
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, `${prefix}.pruned-L12-c1.${fmt.ext}`),
      );
    });
  }
});

describe("lprefix-exact-match (-r L12-34 → range, no resolution)", () => {
  // The hyphenated form is unambiguous: it is parsed as Range and
  // bypasses the resolver entirely, regardless of source contents.
  const pipeline = createDefaultPipeline();
  const fixtureDir = join(FIXTURE_DIR, "lprefix-exact-match");
  const code = readFileSync(join(fixtureDir, "input.ts"), "utf8");
  const sourcePath = "integration/fixtures/lprefix-exact-match/input.ts";
  const pruning = {
    roots: parseOrThrow("L1-3"),
    descendants: 1,
    ancestors: 1,
  };

  test("emits no resolution log for the L-prefixed range form", () => {
    const result = pipeline.runDetailed(code, {
      format: "json",
      language: "ts",
      sourcePath,
      emit: { prettyJson: true, prunedGraph: null, resolutions: null },
      pruning,
    });
    expect(result.resolutions).toEqual([]);
  });

  for (const fmt of FORMATS) {
    test(`emits the pruned ${fmt.format} output for the range`, () => {
      const out = pipeline.runDetailed(code, {
        format: fmt.format,
        language: "ts",
        sourcePath,
        emit: { prettyJson: true, prunedGraph: null, resolutions: null },
        pruning,
      }).text;
      const prefix = "filePrefix" in fmt ? fmt.filePrefix : "expected";
      expect(out).toMatchFileSnapshot(
        join(fixtureDir, `${prefix}.pruned-L1-3-c1.${fmt.ext}`),
      );
    });
  }
});
