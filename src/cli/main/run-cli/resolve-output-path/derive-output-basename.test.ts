import { describe, expect, test } from "vitest";

import type { ParsedRootQuery } from "../../../../root-query/parsed-root-query.js";
import { ROOT_QUERY_KIND } from "../../../../root-query/root-query-kind.js";
import { deriveOutputBasename } from "./derive-output-basename.js";

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
    const actual = deriveOutputBasename({
      roots: [name("value")],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("value");
  });

  test("kind=line is l<n>", () => {
    const actual = deriveOutputBasename({
      roots: [line(42)],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("l42");
  });

  test("kind=line-name is l<n>-<id> with single hyphen", () => {
    const actual = deriveOutputBasename({
      roots: [lineName(42, "render")],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("l42-render");
  });

  test("kind=range is l<n>-<m>", () => {
    const actual = deriveOutputBasename({
      roots: [range(10, 12)],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("l10-12");
  });

  test("kind=range-name is l<n>-<m>-<id>", () => {
    const actual = deriveOutputBasename({
      roots: [rangeName(10, 12, "render")],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("l10-12-render");
  });

  test("multiple roots are joined with +", () => {
    const actual = deriveOutputBasename({
      roots: [name("value"), name("foo")],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("value+foo");
  });

  test("multiple roots mixing kinds", () => {
    const actual = deriveOutputBasename({
      roots: [lineName(42, "render"), name("foo")],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("l42-render+foo");
  });
});

describe("deriveOutputBasename: radius suffix", () => {
  test("no radius flag → no suffix", () => {
    const actual = deriveOutputBasename({
      roots: [name("value")],
      ...noRadius,
      inputPath: "",
    });

    expect(actual).toBe("value");
  });

  test("-A only → -a<N>", () => {
    const actual = deriveOutputBasename({
      roots: [name("value")],
      descendants: 1,
      ancestors: null,
      context: null,
      inputPath: "",
    });

    expect(actual).toBe("value-a1");
  });

  test("-B only → -b<N>", () => {
    const actual = deriveOutputBasename({
      roots: [name("param")],
      descendants: null,
      ancestors: 2,
      context: null,
      inputPath: "",
    });

    expect(actual).toBe("param-b2");
  });

  test("-C only → -c<N>", () => {
    const actual = deriveOutputBasename({
      roots: [range(10, 12)],
      descendants: null,
      ancestors: null,
      context: 2,
      inputPath: "",
    });

    expect(actual).toBe("l10-12-c2");
  });

  test("-A and -B → -a<N>-b<M>", () => {
    const actual = deriveOutputBasename({
      roots: [name("v")],
      descendants: 1,
      ancestors: 2,
      context: null,
      inputPath: "",
    });

    expect(actual).toBe("v-a1-b2");
  });

  test("-B and -C → alphabetical -b<N>-c<M>", () => {
    const actual = deriveOutputBasename({
      roots: [name("v")],
      descendants: null,
      ancestors: 2,
      context: 3,
      inputPath: "",
    });

    expect(actual).toBe("v-b2-c3");
  });

  test("-C and -A → alphabetical -a<N>-c<M>", () => {
    const actual = deriveOutputBasename({
      roots: [name("v")],
      descendants: 7,
      ancestors: null,
      context: 3,
      inputPath: "",
    });

    expect(actual).toBe("v-a7-c3");
  });

  test("-A and -B together drop -C from the filename (C is then redundant)", () => {
    const actual = deriveOutputBasename({
      roots: [name("v")],
      descendants: 1,
      ancestors: 2,
      context: 3,
      inputPath: "",
    });

    expect(actual).toBe("v-a1-b2");
  });
});

describe("deriveOutputBasename: input file fallback", () => {
  test("strips .ts extension", () => {
    const actual = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "foo.ts",
    });

    expect(actual).toBe("foo");
  });

  test("preserves camelCase basename", () => {
    const actual = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "fooBar.ts",
    });

    expect(actual).toBe("fooBar");
  });

  test("preserves kebab-case basename", () => {
    const actual = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "foo-bar.ts",
    });

    expect(actual).toBe("foo-bar");
  });

  test("strips .tsx", () => {
    const actual = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "Component.tsx",
    });

    expect(actual).toBe("Component");
  });

  test("strips only the last extension", () => {
    const actual = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "foo.test.ts",
    });

    expect(actual).toBe("foo.test");
  });

  test("no extension → keeps full basename", () => {
    const actual = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "Makefile",
    });

    expect(actual).toBe("Makefile");
  });

  test("strips path components", () => {
    const actual = deriveOutputBasename({
      roots: [],
      ...noRadius,
      inputPath: "src/deep/foo.ts",
    });

    expect(actual).toBe("foo");
  });
});
