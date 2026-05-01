import { describe, expect, test } from "vitest";

import { ROOT_QUERY_KIND } from "../../root-query-kind.js";
import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import { deriveOutputBasename } from "./output-name.js";

const noRadius = {
  descendants: null,
  ancestors: null,
  context: null,
} satisfies Record<"descendants" | "ancestors" | "context", number | null>;

const name = (n: string): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.Name,
  name: n,
  raw: n,
});
const line = (n: number): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.Line,
  line: n,
  raw: String(n),
});
const lineName = (n: number, id: string): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.LineName,
  line: n,
  name: id,
  raw: `${n}:${id}`,
});
const range = (s: number, e: number): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.Range,
  start: s,
  end: e,
  raw: `${s}-${e}`,
});
const rangeName = (s: number, e: number, id: string): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.RangeName,
  start: s,
  end: e,
  name: id,
  raw: `${s}-${e}:${id}`,
});

describe("deriveOutputBasename: root tokenization", () => {
  test("kind=name uses the identifier", () => {
    const r = deriveOutputBasename({
      roots: [name("value")],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "value" });
  });

  test("kind=line is l<n>", () => {
    const r = deriveOutputBasename({
      roots: [line(42)],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "l42" });
  });

  test("kind=line-name is l<n>-<id> with single hyphen", () => {
    const r = deriveOutputBasename({
      roots: [lineName(42, "render")],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "l42-render" });
  });

  test("kind=range is l<n>-<m>", () => {
    const r = deriveOutputBasename({
      roots: [range(10, 12)],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "l10-12" });
  });

  test("kind=range-name is l<n>-<m>-<id>", () => {
    const r = deriveOutputBasename({
      roots: [rangeName(10, 12, "render")],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "l10-12-render" });
  });

  test("multiple roots are joined with +", () => {
    const r = deriveOutputBasename({
      roots: [name("value"), name("foo")],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "value+foo" });
  });

  test("multiple roots mixing kinds", () => {
    const r = deriveOutputBasename({
      roots: [lineName(42, "render"), name("foo")],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "l42-render+foo" });
  });
});

describe("deriveOutputBasename: radius suffix", () => {
  test("no radius flag → no suffix", () => {
    const r = deriveOutputBasename({
      roots: [name("value")],
      ...noRadius,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "value" });
  });

  test("-A only → -a<N>", () => {
    const r = deriveOutputBasename({
      roots: [name("value")],
      descendants: 1,
      ancestors: null,
      context: null,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "value-a1" });
  });

  test("-B only → -b<N>", () => {
    const r = deriveOutputBasename({
      roots: [name("param")],
      descendants: null,
      ancestors: 2,
      context: null,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "param-b2" });
  });

  test("-C only → -c<N>", () => {
    const r = deriveOutputBasename({
      roots: [range(10, 12)],
      descendants: null,
      ancestors: null,
      context: 2,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "l10-12-c2" });
  });

  test("-A and -B → -a<N>-b<M>", () => {
    const r = deriveOutputBasename({
      roots: [name("v")],
      descendants: 1,
      ancestors: 2,
      context: null,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "v-a1-b2" });
  });

  test("-B and -C → alphabetical -b<N>-c<M>", () => {
    const r = deriveOutputBasename({
      roots: [name("v")],
      descendants: null,
      ancestors: 2,
      context: 3,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "v-b2-c3" });
  });

  test("-C and -A → alphabetical -a<N>-c<M>", () => {
    const r = deriveOutputBasename({
      roots: [name("v")],
      descendants: 7,
      ancestors: null,
      context: 3,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "v-a7-c3" });
  });

  test("-A and -B together drop -C from the filename (C is then redundant)", () => {
    const r = deriveOutputBasename({
      roots: [name("v")],
      descendants: 1,
      ancestors: 2,
      context: 3,
      inputPath: null,
    });
    expect(r).toEqual({ ok: true, basename: "v-a1-b2" });
  });
});

describe("deriveOutputBasename: input file fallback", () => {
  test("strips .ts extension", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "foo.ts",
    });
    expect(r).toEqual({ ok: true, basename: "foo" });
  });

  test("preserves camelCase basename", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "fooBar.ts",
    });
    expect(r).toEqual({ ok: true, basename: "fooBar" });
  });

  test("preserves kebab-case basename", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "foo-bar.ts",
    });
    expect(r).toEqual({ ok: true, basename: "foo-bar" });
  });

  test("strips .tsx", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "Component.tsx",
    });
    expect(r).toEqual({ ok: true, basename: "Component" });
  });

  test("strips only the last extension", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "foo.test.ts",
    });
    expect(r).toEqual({ ok: true, basename: "foo.test" });
  });

  test("no extension → keeps full basename", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "Makefile",
    });
    expect(r).toEqual({ ok: true, basename: "Makefile" });
  });

  test("strips path components", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "src/deep/foo.ts",
    });
    expect(r).toEqual({ ok: true, basename: "foo" });
  });
});

describe("deriveOutputBasename: errors", () => {
  test("no roots and no input path → error", () => {
    const r = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: null,
    });
    expect(r.ok).toBe(false);
    if (!r.ok) {
      expect(r.error).toMatch(/-r\/--roots/);
    }
  });
});
