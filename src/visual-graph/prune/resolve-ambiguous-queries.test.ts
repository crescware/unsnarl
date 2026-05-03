import { describe, expect, test } from "vitest";

import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import { ROOT_QUERY_KIND } from "../../cli/root-query/root-query-kind.js";
import { DIRECTION } from "../direction.js";
import type { VisualElement, VisualGraph, VisualNode } from "../model.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { resolveAmbiguousQueries } from "./resolve-ambiguous-queries.js";

const variableNode = (name: string, line = 1): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id: `n-${name}-${String(line)}`,
  kind: NODE_KIND.Variable,
  name,
  line,
  endLine: null,
  isJsxElement: false,
  unused: false,
  declarationKind: null,
  initIsFunction: false,
});

const writeOpNode = (name: string, line = 1): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id: `w-${name}-${String(line)}`,
  kind: NODE_KIND.WriteOp,
  name,
  line,
  endLine: null,
  isJsxElement: false,
  unused: false,
  declarationKind: null,
});

const graphOf = (elements: VisualElement[]): VisualGraph => ({
  version: 1,
  source: { path: "f.ts", language: "ts" },
  direction: DIRECTION.TB,
  elements,
  edges: [],
  boundaryEdges: [],
  pruning: null,
});

const lineOrName = (raw: string, line: number): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.LineOrName,
  line,
  name: raw,
  raw,
});

describe("resolveAmbiguousQueries", () => {
  test("returns input untouched when no LineOrName is present", () => {
    const graph = graphOf([variableNode("foo")]);
    const queries: ParsedRootQuery[] = [
      { kind: ROOT_QUERY_KIND.Line, line: 5, raw: "5" },
      { kind: ROOT_QUERY_KIND.Name, name: "foo", raw: "foo" },
    ];
    const result = resolveAmbiguousQueries(graph, queries);
    expect(result.resolved).toBe(queries);
    expect(result.resolutions).toEqual([]);
  });

  test("silently treats LineOrName as Line when no [Ll]\\d+ identifier exists", () => {
    const graph = graphOf([variableNode("foo"), variableNode("bar", 3)]);
    const result = resolveAmbiguousQueries(graph, [lineOrName("L12", 12)]);
    expect(result.resolved).toEqual([
      { kind: ROOT_QUERY_KIND.Line, line: 12, raw: "L12" },
    ]);
    expect(result.resolutions).toEqual([]);
  });

  test("resolves to Name when an exact match exists, with a resolution log", () => {
    const graph = graphOf([variableNode("L12", 7), variableNode("other", 9)]);
    const result = resolveAmbiguousQueries(graph, [lineOrName("L12", 12)]);
    expect(result.resolved).toEqual([
      { kind: ROOT_QUERY_KIND.Name, name: "L12", raw: "L12" },
    ]);
    expect(result.resolutions).toEqual([
      { raw: "L12", line: 12, name: "L12", resolvedAs: "name" },
    ]);
  });

  test("resolves to Line when other [Ll]<n> identifiers exist but no exact match", () => {
    const graph = graphOf([variableNode("l5"), variableNode("l99", 3)]);
    const result = resolveAmbiguousQueries(graph, [lineOrName("L12", 12)]);
    expect(result.resolved).toEqual([
      { kind: ROOT_QUERY_KIND.Line, line: 12, raw: "L12" },
    ]);
    expect(result.resolutions).toEqual([
      { raw: "L12", line: 12, name: "L12", resolvedAs: "line" },
    ]);
  });

  test("name lookup is case-sensitive", () => {
    // `l1` exists but the query is `L1`; matchableNames contains `l1`,
    // which still satisfies the [Ll]\d+ presence check, so we follow the
    // "exists but no exact match" path -> resolves to Line.
    const graph = graphOf([variableNode("l1")]);
    const result = resolveAmbiguousQueries(graph, [lineOrName("L1", 1)]);
    expect(result.resolved).toEqual([
      { kind: ROOT_QUERY_KIND.Line, line: 1, raw: "L1" },
    ]);
    expect(result.resolutions).toEqual([
      { raw: "L1", line: 1, name: "L1", resolvedAs: "line" },
    ]);
  });

  test("preserves order across a mixed array", () => {
    const graph = graphOf([variableNode("L12", 4), variableNode("l5", 6)]);
    const queries: ParsedRootQuery[] = [
      { kind: ROOT_QUERY_KIND.Line, line: 1, raw: "1" },
      lineOrName("L12", 12),
      { kind: ROOT_QUERY_KIND.Name, name: "x", raw: "x" },
      lineOrName("L99", 99),
    ];
    const result = resolveAmbiguousQueries(graph, queries);
    expect(result.resolved).toEqual([
      { kind: ROOT_QUERY_KIND.Line, line: 1, raw: "1" },
      { kind: ROOT_QUERY_KIND.Name, name: "L12", raw: "L12" },
      { kind: ROOT_QUERY_KIND.Name, name: "x", raw: "x" },
      { kind: ROOT_QUERY_KIND.Line, line: 99, raw: "L99" },
    ]);
    expect(result.resolutions).toEqual([
      { raw: "L12", line: 12, name: "L12", resolvedAs: "name" },
      { raw: "L99", line: 99, name: "L99", resolvedAs: "line" },
    ]);
  });

  test("emits one resolution entry per LineOrName occurrence", () => {
    const graph = graphOf([variableNode("L12")]);
    const result = resolveAmbiguousQueries(graph, [
      lineOrName("L12", 12),
      lineOrName("L12", 12),
    ]);
    expect(result.resolutions).toEqual([
      { raw: "L12", line: 12, name: "L12", resolvedAs: "name" },
      { raw: "L12", line: 12, name: "L12", resolvedAs: "name" },
    ]);
  });

  test("ignores name-query-excluded kinds when collecting matchable names", () => {
    // A WriteOp named `L12` is excluded from `-r <id>` matching, so the
    // resolver must not treat it as an exact match either. Without any
    // other [Ll]\d+ identifier, we fall to the silent Line path.
    const graph = graphOf([writeOpNode("L12"), variableNode("foo")]);
    const result = resolveAmbiguousQueries(graph, [lineOrName("L12", 12)]);
    expect(result.resolved).toEqual([
      { kind: ROOT_QUERY_KIND.Line, line: 12, raw: "L12" },
    ]);
    expect(result.resolutions).toEqual([]);
  });
});
