import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { LANGUAGE } from "../../language.js";
import { OxcParser } from "../../parser/oxc-parser.js";
import { runAnalysis } from "../../pipeline/analyze/run-analysis.js";
import { sourceTypeFromPath } from "../../pipeline/parse/source-type-from-path.js";
import { FlatSerializer } from "../../serializer/flat/flat-serializer.js";

import unsnarlPluginReact from "./index.js";

function buildIr(code: string) {
  const parser = new OxcParser();
  const sourcePath = "input.tsx";
  const parsed = parser.parse(code, {
    language: LANGUAGE.Tsx,
    sourcePath,
    sourceType: sourceTypeFromPath(sourcePath, LANGUAGE.Tsx),
  });
  const analyzed = runAnalysis(parsed);
  return new FlatSerializer().serialize({
    rootScope: analyzed.rootScope,
    annotations: analyzed.annotations,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: sourcePath, language: LANGUAGE.Tsx },
  });
}

describe("unsnarl-plugin-react", () => {
  test("exposes meta.name matching the package convention", () => {
    expect(unsnarlPluginReact.meta.name).toEqual("unsnarl-plugin-react");
  });

  test("rewrites useCallback-bound variables so their init points at the inner function", () => {
    const code = [
      'import { useCallback } from "react";',
      "",
      "const Comp = () => {",
      "  const a = useCallback(() => 1, []);",
      "  const b = useCallback(() => a() + 1, [a]);",
      "  const c = useCallback(() => b() + 1, [b]);",
      "  return <button onClick={c}>{c()}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);

    for (const name of ["a", "b", "c"]) {
      const variable = out.variables.find((v) => v.name === name);
      expect(variable !== undefined, `variable ${name} should exist`).toEqual(
        true,
      );
      if (!variable) {
        return;
      }
      const def = variable.defs[0];
      expect(def?.type).toEqual(DEFINITION_TYPE.Variable);
      if (def?.type !== DEFINITION_TYPE.Variable) {
        return;
      }
      expect(def.init?.type).toEqual("ArrowFunctionExpression");
      const innerScope = out.scopes.find(
        (scope) =>
          scope.upper === variable.scope &&
          scope.block.span.offset === def.init?.span.offset,
      );
      expect(
        innerScope !== undefined,
        `inner function scope for ${name} should exist`,
      ).toEqual(true);
    }
  });

  test("removes useCallback callee and dep-array references introduced by the hook call", () => {
    const code = [
      'import { useCallback } from "react";',
      "",
      "const Comp = () => {",
      "  const a = useCallback(() => 1, []);",
      "  const b = useCallback(() => a() + 1, [a]);",
      "  return <button onClick={b}>{b()}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);

    for (const ref of out.references) {
      expect(ref.identifier.name).not.toEqual("useCallback");
    }
    const compScopeId = out.scopes.find((v) => v.block.type !== "Program")?.id;
    expect(compScopeId !== undefined).toEqual(true);
    const aRefs = out.references.filter((v) => v.identifier.name === "a");
    for (const ref of aRefs) {
      if (ref.init) {
        continue;
      }
      expect(ref.from).not.toEqual(compScopeId);
    }
  });

  test("drops the useCallback import entirely once it has no remaining references", () => {
    const code = [
      'import { useCallback } from "react";',
      "",
      "const Comp = () => {",
      "  const a = useCallback(() => 1, []);",
      "  return <button>{a()}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);

    const useCallbackVar = out.variables.find((v) => v.name === "useCallback");
    expect(useCallbackVar).toEqual(undefined);
    for (const scope of out.scopes) {
      for (const vid of scope.variables) {
        expect(vid).not.toContain("useCallback");
      }
    }
  });

  test("is a no-op when no useCallback import is present", () => {
    const code = [
      "const Comp = () => {",
      "  const a = () => 1;",
      "  return <button>{a()}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);
    expect(out === ir).toEqual(true);
  });

  test("leaves a useCallback variable in place when it is referenced for non-call use", () => {
    const code = [
      'import { useCallback } from "react";',
      "",
      "const passthrough = (fn: unknown) => fn;",
      "const ref = passthrough(useCallback);",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);
    const useCallbackVar = out.variables.find((v) => v.name === "useCallback");
    expect(useCallbackVar !== undefined).toEqual(true);
  });

  test("keeps a useMemo-bound variable's init as a CallExpression so it reads as an IIFE", () => {
    const code = [
      'import { useMemo } from "react";',
      "",
      "const Comp = () => {",
      "  const v = useMemo(() => {",
      "    const x = 1;",
      "    return x;",
      "  }, []);",
      "  return <button>{v}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);

    const variable = out.variables.find((v) => v.name === "v");
    if (variable === undefined) {
      throw new Error("variable v should exist after the transform");
    }
    const def = variable.defs[0];
    if (def?.type !== DEFINITION_TYPE.Variable) {
      throw new Error("v's first def should be a Variable");
    }
    expect(def.init?.type).toEqual("CallExpression");
    const innerScope = out.scopes.find(
      (scope) =>
        scope.upper === variable.scope &&
        (scope.block.type === "ArrowFunctionExpression" ||
          scope.block.type === "FunctionExpression"),
    );
    expect(innerScope !== undefined).toEqual(true);
  });

  test("removes useMemo callee and dep-array references introduced by the hook call", () => {
    const code = [
      'import { useMemo } from "react";',
      "",
      "const Comp = ({ start }: { start: number }) => {",
      "  const v = useMemo(() => start * 2, [start]);",
      "  return <button>{v}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);

    for (const ref of out.references) {
      expect(ref.identifier.name).not.toEqual("useMemo");
    }
    const vVar = out.variables.find((v) => v.name === "v");
    if (vVar === undefined) {
      throw new Error("variable v should exist after the transform");
    }
    const refsOwnedByV = out.references.filter((v) =>
      v.owners.includes(vVar.id),
    );
    for (const ref of refsOwnedByV) {
      expect(ref.init).toEqual(true);
    }
  });

  test("drops the useMemo import entirely once it has no remaining references", () => {
    const code = [
      'import { useMemo } from "react";',
      "",
      "const Comp = () => {",
      "  const v = useMemo(() => 1, []);",
      "  return <button>{v}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);

    const useMemoVar = out.variables.find((v) => v.name === "useMemo");
    expect(useMemoVar).toEqual(undefined);
  });

  test("rewrites both useCallback and useMemo within the same module", () => {
    const code = [
      'import { useCallback, useMemo } from "react";',
      "",
      "const Comp = ({ start }: { start: number }) => {",
      "  const inc = useCallback((n: number) => n + start, [start]);",
      "  const v = useMemo(() => start * 2, [start]);",
      "  return <button onClick={inc as never}>{v}</button>;",
      "};",
      "",
    ].join("\n");
    const ir = buildIr(code);
    const out = unsnarlPluginReact.transform(ir);

    expect(out.variables.find((v) => v.name === "useCallback")).toEqual(
      undefined,
    );
    expect(out.variables.find((v) => v.name === "useMemo")).toEqual(undefined);

    const incDef = out.variables.find((v) => v.name === "inc")?.defs[0];
    if (incDef?.type !== DEFINITION_TYPE.Variable) {
      throw new Error("inc's first def should be a Variable");
    }
    expect(incDef.init?.type).toEqual("ArrowFunctionExpression");

    const vDef = out.variables.find((v) => v.name === "v")?.defs[0];
    if (vDef?.type !== DEFINITION_TYPE.Variable) {
      throw new Error("v's first def should be a Variable");
    }
    expect(vDef.init?.type).toEqual("CallExpression");
  });
});
