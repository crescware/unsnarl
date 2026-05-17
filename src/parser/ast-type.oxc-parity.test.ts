// Guards AST_TYPE against silent divergence from the upstream
// @oxc-project/types `type:` discriminator surface.
//
// Unlike the runtime-behavior tests in ast-type.test.ts, this file
// reads the actual `types.d.ts` shipped by the oxc-parser version
// pinned in package.json and asserts member-equality with AST_TYPE.
// A breakage here means one of:
//   - AST_TYPE lists a string oxc does not emit (transcription drift)
//   - oxc added/removed a type and AST_TYPE was not updated
//   - oxc introduced a new string-union alias used as `type:`
//     discriminator and OXC_TYPE_ALIAS_EXPANSIONS needs updating
//
// The intent is intentional tight coupling to the upstream `.d.ts`
// so an oxc-side change surfaces immediately as a failing unit test
// rather than as a downstream runtime miss.

import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname } from "node:path";

import { describe, expect, test } from "vitest";

import { AST_TYPE } from "./ast-type.js";

// Resolve the @oxc-project/types instance that the directly-pinned
// oxc-parser actually pulls in, rather than another copy pnpm may
// have hoisted from indirect dependents (e.g. rolldown).
const requireFromHere = createRequire(import.meta.url);
const oxcParserDir = dirname(
  requireFromHere.resolve("oxc-parser/package.json"),
);
const OXC_TYPES_DTS_PATH = requireFromHere.resolve(
  "@oxc-project/types/types.d.ts",
  { paths: [oxcParserDir] },
);

// Expansion table for the string-union aliases that @oxc-project/types
// uses as `type:` discriminators. If oxc adds a new such alias, the
// `unknownAliases` test fails, forcing this table to be updated rather
// than letting the extraction silently miss types.
const OXC_TYPE_ALIAS_EXPANSIONS: Readonly<Record<string, readonly string[]>> = {
  FunctionType: [
    "FunctionDeclaration",
    "FunctionExpression",
    "TSDeclareFunction",
    "TSEmptyBodyFunctionExpression",
  ],
  ClassType: ["ClassDeclaration", "ClassExpression"],
  MethodDefinitionType: ["MethodDefinition", "TSAbstractMethodDefinition"],
  PropertyDefinitionType: [
    "PropertyDefinition",
    "TSAbstractPropertyDefinition",
  ],
  AccessorPropertyType: ["AccessorProperty", "TSAbstractAccessorProperty"],
};

function extractOxcAstTypes(): {
  types: ReadonlySet<string>;
  unknownAliases: readonly string[];
} {
  const src = readFileSync(OXC_TYPES_DTS_PATH, "utf8");
  const literalDiscriminator = /^\s+type:\s*"([^"]+)";/gm;
  const aliasDiscriminator = /^\s+type:\s*([A-Z][A-Za-z0-9]+);/gm;

  const types = new Set<string>();
  const unknownAliases = new Set<string>();

  for (const m of src.matchAll(literalDiscriminator)) {
    types.add(m[1]);
  }
  for (const m of src.matchAll(aliasDiscriminator)) {
    const aliasName = m[1];
    const expansion = OXC_TYPE_ALIAS_EXPANSIONS[aliasName];
    if (expansion === undefined) {
      unknownAliases.add(aliasName);
      continue;
    }
    for (const t of expansion) {
      types.add(t);
    }
  }
  return { types, unknownAliases: [...unknownAliases].sort() };
}

describe("AST_TYPE parity with @oxc-project/types", () => {
  const { types: oxcTypes, unknownAliases } = extractOxcAstTypes();
  const declared = new Set<string>(Object.values(AST_TYPE));

  test("OXC_TYPE_ALIAS_EXPANSIONS covers every alias used as a `type:` discriminator in types.d.ts", () => {
    // If this fails, oxc introduced a new string-union alias used as
    // a node `type:`. Add the alias and its member strings to
    // OXC_TYPE_ALIAS_EXPANSIONS above.
    expect(unknownAliases).toEqual([]);
  });

  test("AST_TYPE contains no entries that oxc does not emit", () => {
    const extra = [...declared].filter((t) => !oxcTypes.has(t)).sort();
    expect(extra).toEqual([]);
  });

  test("AST_TYPE lists every type string oxc emits", () => {
    const missing = [...oxcTypes].filter((t) => !declared.has(t)).sort();
    expect(missing).toEqual([]);
  });
});
