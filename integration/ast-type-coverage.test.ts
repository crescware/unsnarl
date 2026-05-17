// Guards every AST_TYPE entry against drift from the integration
// fixture tree: each enumerated type must be exercised by at least
// one fixture, so that a new oxc-parser AST type added through
// ast-type.oxc-parity.test.ts also surfaces a missing-fixture
// failure here until a corresponding input is added under
// integration/fixtures/.
//
// The walk is intentionally structural (not via analyze()):
// src/boundary/eslint-scope/handle-enter.ts skips a number of TS
// subtrees through isTypeOnlySubtree, so analyze() would under-
// count those types. Calling walkNode directly with visitorKeys-
// based descent reproduces only the structural traversal, with no
// semantic skip.

import { readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, test } from "vitest";

import { walkNode } from "../src/boundary/eslint-scope/walk/walk-node.js";
import type { AstNode } from "../src/ir/primitive/ast-node.js";
import { LANGUAGE, type Language } from "../src/language.js";
import { AST_TYPE } from "../src/parser/ast-type.js";
import { OxcParser } from "../src/parser/oxc-parser.js";
import { sourceTypeFromPath } from "../src/pipeline/parse/source-type-from-path.js";

const PROJECT_ROOT = process.cwd();
const FIXTURE_DIR = join(PROJECT_ROOT, "integration", "fixtures");

function languageFromExt(ext: string): Language | null {
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

type FixtureInput = Readonly<{
  language: Language;
  sourcePath: string;
  code: string;
}>;

function findFixtureInputs(root: string): readonly FixtureInput[] {
  const results: FixtureInput[] = [];
  walk(root);
  return results;

  function walk(dir: string): void {
    for (const ent of readdirSync(dir, { withFileTypes: true })) {
      const p = join(dir, ent.name);
      if (ent.isDirectory()) {
        walk(p);
        continue;
      }
      if (!ent.isFile() || !ent.name.startsWith("input.")) {
        continue;
      }
      const ext = ent.name.slice("input.".length);
      const language = languageFromExt(ext);
      if (language === null) {
        continue;
      }
      results.push({
        language,
        sourcePath: relative(PROJECT_ROOT, p),
        code: readFileSync(p, "utf8"),
      });
    }
  }
}

function collectTypesFromFixtures(): ReadonlySet<string> {
  const parser = new OxcParser();
  const seen = new Set<string>();
  for (const f of findFixtureInputs(FIXTURE_DIR)) {
    const parsed = parser.parse(f.code, {
      language: f.language,
      sourcePath: f.sourcePath,
      sourceType: sourceTypeFromPath(f.sourcePath, f.language),
    });
    walkNode(
      parsed.ast as unknown as AstNode,
      null,
      null,
      {
        enter(node) {
          seen.add(node.type);
        },
      },
      [],
    );
  }
  return seen;
}

describe("AST_TYPE fixture coverage", () => {
  const seen = collectTypesFromFixtures();
  const declared = new Set<string>(Object.values(AST_TYPE));

  test("every AST_TYPE entry is exercised by some fixture", () => {
    const missing = [...declared].filter((v) => !seen.has(v)).sort();
    expect(missing).toEqual([]);
  });
});
