import { describe, expect, test } from "vitest";

import { EslintCompatAnalyzer } from "../analyzer/eslint-compat/eslint-compat.js";
import {
  IMPORT_KIND,
  LANGUAGE,
  SCOPE_TYPE,
  type Language,
} from "../constants.js";
import type {
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
} from "../ir/model.js";
import { OxcParser } from "../parser/oxc.js";
import { FlatSerializer } from "./flat/flat-serializer.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();

function pipe(code: string, language: Language = LANGUAGE.Ts): SerializedIR {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
  });
  const analyzed = analyzer.analyze(parsed);
  return serializer.serialize({
    rootScope: analyzed.rootScope,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: `input.${language}`, language },
  });
}

function varByName(ir: SerializedIR, name: string): SerializedVariable {
  const matches = ir.variables.filter((v) => v.name === name);
  if (matches.length !== 1) {
    throw new Error(
      `expected 1 variable named "${name}", found ${matches.length}`,
    );
  }
  return matches[0] as SerializedVariable;
}

function refsToVar(
  ir: SerializedIR,
  id: string,
): readonly SerializedReference[] {
  return ir.references.filter((r) => r.resolved === id);
}

function caseScopesOf(ir: SerializedIR): readonly SerializedScope[] {
  return ir.scopes.filter(
    (s) =>
      s.type === SCOPE_TYPE.Block &&
      s.blockContext?.parentType === "SwitchStatement",
  );
}

function ifBranchScopesOf(ir: SerializedIR): readonly SerializedScope[] {
  return ir.scopes.filter(
    (s) =>
      s.type === SCOPE_TYPE.Block &&
      s.blockContext?.parentType === "IfStatement",
  );
}

function scopeFromOf(
  ir: SerializedIR,
  ref: SerializedReference,
): SerializedScope {
  const scope = ir.scopes.find((s) => s.id === ref.from);
  if (!scope) {
    throw new Error(`scope ${ref.from} not found`);
  }
  return scope;
}

describe("scenario: switch with break — case scopes are exhaustively non-falling", () => {
  const code = [
    'let label = "";',
    'const kind = "a";',
    "switch (kind) {",
    '  case "a":',
    '    label = "alpha";',
    "    break;",
    '  case "b":',
    '    label = "beta";',
    "    break;",
    "  default:",
    '    label = "other";',
    "    break;",
    "}",
    "const result = label;",
  ].join("\n");
  const ir = pipe(code);

  test("the switch produces three case-block scopes (two cases + default)", () => {
    const cases = caseScopesOf(ir);
    expect(cases).toHaveLength(3);
    const tests = cases.map((s) => s.blockContext?.caseTest);
    expect(tests).toEqual(['"a"', '"b"', null]);
  });

  test("every case scope has fallsThrough === false", () => {
    const cases = caseScopesOf(ir);
    expect(cases.map((s) => s.fallsThrough)).toEqual([false, false, false]);
  });

  test("label has one write reference per case scope (3 writes, all in distinct scopes)", () => {
    const label = varByName(ir, "label");
    const writes = refsToVar(ir, label.id).filter(
      (r) => r.flags.write && !r.init,
    );
    expect(writes).toHaveLength(3);
    const scopes = new Set(writes.map((r) => r.from));
    expect(scopes.size).toBe(3);
    for (const w of writes) {
      const s = scopeFromOf(ir, w);
      expect(s.blockContext?.parentType).toBe("SwitchStatement");
    }
  });

  test("the discriminant identifier carries a SwitchStatement predicate container", () => {
    const kindRefs = ir.references.filter((r) => r.identifier.name === "kind");
    expect(kindRefs).toHaveLength(1);
    expect(kindRefs[0]?.predicateContainer?.type).toBe("SwitchStatement");
  });
});

describe("scenario: switch with implicit fallthrough — every case body falls through", () => {
  const code = [
    'let label = "";',
    'const kind = "a";',
    "switch (kind) {",
    '  case "a":',
    '    label = "alpha";',
    '  case "b":',
    '    label = "beta";',
    "  default:",
    '    label = "other";',
    "}",
  ].join("\n");
  const ir = pipe(code);

  test("every case scope has fallsThrough === true", () => {
    const cases = caseScopesOf(ir);
    expect(cases).toHaveLength(3);
    expect(cases.map((s) => s.fallsThrough)).toEqual([true, true, true]);
  });

  test("the number of writes to label is unchanged from the break-bearing variant", () => {
    const label = varByName(ir, "label");
    const writes = refsToVar(ir, label.id).filter(
      (r) => r.flags.write && !r.init,
    );
    expect(writes).toHaveLength(3);
  });
});

describe("scenario: if/else exposes a predicate and two branch scopes", () => {
  const code = [
    "let counter = 0;",
    "const flag = true;",
    "if (flag) {",
    "  counter = 1;",
    "} else {",
    "  counter = 2;",
    "}",
  ].join("\n");
  const ir = pipe(code);

  test("the if-statement produces two block scopes keyed consequent/alternate", () => {
    const arms = ifBranchScopesOf(ir);
    expect(arms.map((s) => s.blockContext?.key).sort()).toEqual([
      "alternate",
      "consequent",
    ]);
  });

  test("the predicate identifier carries an IfStatement predicate container", () => {
    const flagRefs = ir.references.filter((r) => r.identifier.name === "flag");
    expect(flagRefs).toHaveLength(1);
    expect(flagRefs[0]?.predicateContainer?.type).toBe("IfStatement");
  });

  test("counter receives one write per branch, in distinct branch scopes", () => {
    const counter = varByName(ir, "counter");
    const writes = refsToVar(ir, counter.id).filter(
      (r) => r.flags.write && !r.init,
    );
    expect(writes).toHaveLength(2);
    const scopes = new Set(writes.map((r) => r.from));
    expect(scopes.size).toBe(2);
  });
});

describe("scenario: if without else — only the consequent scope exists", () => {
  const code = [
    "let counter = 0;",
    "const flag = true;",
    "if (flag) {",
    "  counter = 1;",
    "}",
  ].join("\n");
  const ir = pipe(code);

  test("the if-statement produces only a consequent block scope, no alternate", () => {
    const arms = ifBranchScopesOf(ir);
    expect(arms).toHaveLength(1);
    expect(arms[0]?.blockContext?.key).toBe("consequent");
  });

  test("the predicate identifier still carries an IfStatement predicate container", () => {
    const flagRefs = ir.references.filter((r) => r.identifier.name === "flag");
    expect(flagRefs[0]?.predicateContainer?.type).toBe("IfStatement");
  });
});

describe("scenario: try / catch / finally — three child scopes, catch parameter is scoped", () => {
  const code = [
    "let v = 0;",
    "try {",
    "  v = 1;",
    "} catch (err) {",
    "  v = 2;",
    "} finally {",
    "  v = 3;",
    "}",
  ].join("\n");
  const ir = pipe(code);

  test("the try statement produces a try block, a catch scope, and a finalizer block", () => {
    const tryChildren = ir.scopes.filter(
      (s) => s.blockContext?.parentType === "TryStatement",
    );
    expect(tryChildren).toHaveLength(3);
    const layout = tryChildren.map((s) => ({
      type: s.type,
      key: s.blockContext?.key,
    }));
    expect(layout).toEqual([
      { type: SCOPE_TYPE.Block, key: "block" },
      { type: SCOPE_TYPE.Catch, key: "handler" },
      { type: SCOPE_TYPE.Block, key: "finalizer" },
    ]);
  });

  test("the catch parameter `err` is owned by the catch scope, not the surrounding module", () => {
    const err = varByName(ir, "err");
    const catchScope = ir.scopes.find((s) => s.type === SCOPE_TYPE.Catch);
    expect(catchScope).toBeDefined();
    expect(catchScope?.variables).toContain(err.id);
    const moduleScope = ir.scopes.find((s) => s.type === SCOPE_TYPE.Module) as
      | SerializedScope
      | undefined;
    expect(moduleScope?.variables ?? []).not.toContain(err.id);
  });

  test("the catch parameter has a CatchClause definition", () => {
    const err = varByName(ir, "err");
    expect(err.defs[0]?.type).toBe("CatchClause");
  });
});

describe("scenario: import declarations carry kind / source / imported name", () => {
  const code = [
    "import def from 'some-default';",
    "import { named, other as renamed } from 'some-named';",
    "import * as ns from 'some-namespace';",
  ].join("\n");
  const ir = pipe(code);

  test("default imports record the source and a null importedName", () => {
    const def = varByName(ir, "def").defs[0];
    expect(def?.type).toBe("ImportBinding");
    expect(def?.importKind).toBe(IMPORT_KIND.Default);
    expect(def?.importSource).toBe("some-default");
    expect(def?.importedName).toBeNull();
  });

  test("named imports record the imported name as the original symbol", () => {
    const named = varByName(ir, "named").defs[0];
    expect(named?.importKind).toBe(IMPORT_KIND.Named);
    expect(named?.importSource).toBe("some-named");
    expect(named?.importedName).toBe(IMPORT_KIND.Named);
  });

  test("renamed imports keep the local name on the variable but the original on importedName", () => {
    const renamed = varByName(ir, "renamed");
    expect(renamed.name).toBe("renamed");
    expect(renamed.defs[0]?.importKind).toBe(IMPORT_KIND.Named);
    expect(renamed.defs[0]?.importedName).toBe("other");
    expect(renamed.defs[0]?.importSource).toBe("some-named");
  });

  test("namespace imports record kind=namespace and a null importedName", () => {
    const ns = varByName(ir, "ns").defs[0];
    expect(ns?.importKind).toBe(IMPORT_KIND.Namespace);
    expect(ns?.importSource).toBe("some-namespace");
    expect(ns?.importedName).toBeNull();
  });
});

describe("scenario: ImplicitGlobalVariable — receiver flag distinguishes member from direct use", () => {
  test("a global accessed only as a member receiver carries flags.receiver=true", () => {
    const ir = pipe("const xs = Object.keys(arg);\n");
    const objectRefs = ir.references.filter(
      (r) => r.identifier.name === "Object",
    );
    expect(objectRefs).toHaveLength(1);
    expect(objectRefs[0]?.flags.receiver).toBe(true);
    expect(objectRefs[0]?.flags.read).toBe(true);
    const objectDef = varByName(ir, "Object").defs[0];
    expect(objectDef?.type).toBe("ImplicitGlobalVariable");
  });

  test("a global read directly carries flags.receiver=false", () => {
    const ir = pipe("const xs = Object.keys(arg);\n");
    const argRefs = ir.references.filter((r) => r.identifier.name === "arg");
    expect(argRefs).toHaveLength(1);
    expect(argRefs[0]?.flags.receiver).toBe(false);
    expect(argRefs[0]?.flags.read).toBe(true);
    const argDef = varByName(ir, "arg").defs[0];
    expect(argDef?.type).toBe("ImplicitGlobalVariable");
  });
});

describe("scenario: function parameter references are not duplicated", () => {
  test("each parameter has exactly one read reference inside the body", () => {
    const ir = pipe("function add(a, b) { return a + b; }\n");
    const a = varByName(ir, "a");
    const b = varByName(ir, "b");
    expect(a.references).toHaveLength(1);
    expect(b.references).toHaveLength(1);
    const aRef = ir.references.find((r) => r.id === a.references[0]);
    const bRef = ir.references.find((r) => r.id === b.references[0]);
    expect(aRef?.flags.read).toBe(true);
    expect(aRef?.flags.write).toBe(false);
    expect(bRef?.flags.read).toBe(true);
    expect(bRef?.flags.write).toBe(false);
  });

  test("each parameter has a Parameter definition", () => {
    const ir = pipe("function add(a, b) { return a + b; }\n");
    expect(varByName(ir, "a").defs[0]?.type).toBe("Parameter");
    expect(varByName(ir, "b").defs[0]?.type).toBe("Parameter");
  });
});
