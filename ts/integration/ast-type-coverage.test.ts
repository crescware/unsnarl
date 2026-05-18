// Walks fixtures via walkNode directly rather than analyze(): the
// analyzer skips TS subtrees through isTypeOnlySubtree, which
// would under-count those types.

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
