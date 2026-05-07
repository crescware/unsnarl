import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/primitive/ast-node.js";
import { OxcParser } from "../parser/oxc-parser.js";
import { NESTING_KIND } from "../serializer/nesting-kind.js";
import { computeNestingDepths } from "./compute-nesting-depths.js";

const parser = new OxcParser();

function depthsAt(
  source: string,
  offset: number,
): Record<string, number> | undefined {
  const ast = parser.parse(source, {
    language: "ts",
    sourcePath: "x.ts",
    sourceType: "module",
  }).ast as AstNode;
  const map = computeNestingDepths(ast);
  return map.get(offset);
}

describe("computeNestingDepths", () => {
  test("function body increments function counter", () => {
    const code = "function f() { let x = 1; }";
    const fnIdx = code.indexOf("function");
    const at = depthsAt(code, fnIdx);
    expect(at?.[NESTING_KIND.Function]).toBe(1);
  });

  test("if body increments if independently of function", () => {
    const code = "function f() { if (a) { let y = 1; } }";
    const innerIdx = code.indexOf("{ let y");
    const at = depthsAt(code, innerIdx);
    expect(at?.[NESTING_KIND.Function]).toBe(1);
    expect(at?.[NESTING_KIND.If]).toBe(1);
  });

  test("for-init binding lives at parent depth (() outside {})", () => {
    const code = "for (let i = 0; i < 1; i++) { i; }";
    const forIdx = code.indexOf("for ");
    const forDepths = depthsAt(code, forIdx);
    expect(forDepths?.[NESTING_KIND.For]).toBe(0);

    const bodyIdx = code.indexOf("{ i;");
    const bodyDepths = depthsAt(code, bodyIdx);
    expect(bodyDepths?.[NESTING_KIND.For]).toBe(1);
  });

  test("nested if inside if increments if to 2", () => {
    const code = "if (a) { if (b) { 1; } }";
    const innerIdx = code.indexOf("{ 1;");
    const at = depthsAt(code, innerIdx);
    expect(at?.[NESTING_KIND.If]).toBe(2);
  });

  test("each nesting kind counts independently: if inside for stays at if=1, for=1", () => {
    const code = "for (;;) { if (b) { 1; } }";
    const innerIdx = code.indexOf("{ 1;");
    const at = depthsAt(code, innerIdx);
    expect(at?.[NESTING_KIND.For]).toBe(1);
    expect(at?.[NESTING_KIND.If]).toBe(1);
  });

  test("empty if body still counts (ECMA-strict, not eslint-scope-dependent)", () => {
    // The if's body block has no lexical declarations, but per ECMA-262 14.2
    // it still introduces a Block scope -> if depth must be 1 inside it.
    const code = "if (a) { foo(); }";
    const innerIdx = code.indexOf("{ foo");
    const at = depthsAt(code, innerIdx);
    expect(at?.[NESTING_KIND.If]).toBe(1);
  });
});
