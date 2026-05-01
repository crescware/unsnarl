import { describe, expect, test } from "vitest";

import { bfs } from "./bfs.js";

const adjOf = (
  pairs: readonly (readonly [string, string])[],
): Map<string, string[]> => {
  const out = new Map<string, string[]>();
  for (const [from, to] of pairs) {
    const arr = out.get(from);
    if (arr === undefined) {
      out.set(from, [to]);
    } else {
      arr.push(to);
    }
  }
  return out;
};

describe("bfs", () => {
  test("maxDepth=0 returns the start set unchanged", () => {
    const adj = adjOf([["a", "b"]]);
    expect([...bfs(new Set(["a"]), adj, 0)].sort()).toEqual(["a"]);
  });

  test("maxDepth=1 reaches direct neighbors", () => {
    const adj = adjOf([
      ["a", "b"],
      ["b", "c"],
    ]);
    expect([...bfs(new Set(["a"]), adj, 1)].sort()).toEqual(["a", "b"]);
  });

  test("maxDepth=2 reaches grandchildren", () => {
    const adj = adjOf([
      ["a", "b"],
      ["b", "c"],
    ]);
    expect([...bfs(new Set(["a"]), adj, 2)].sort()).toEqual(["a", "b", "c"]);
  });

  test("multiple start nodes union their reachable sets", () => {
    const adj = adjOf([
      ["a", "x"],
      ["b", "y"],
    ]);
    expect([...bfs(new Set(["a", "b"]), adj, 1)].sort()).toEqual([
      "a",
      "b",
      "x",
      "y",
    ]);
  });

  test("cycles do not loop infinitely", () => {
    const adj = adjOf([
      ["a", "b"],
      ["b", "a"],
    ]);
    expect([...bfs(new Set(["a"]), adj, 10)].sort()).toEqual(["a", "b"]);
  });

  test("disconnected nodes stay unreached", () => {
    const adj = adjOf([["a", "b"]]);
    expect([...bfs(new Set(["a"]), adj, 5)].sort()).toEqual(["a", "b"]);
  });

  test("frontier exhaustion bails early without iterating extra depths", () => {
    const adj = adjOf([["a", "b"]]);
    expect([...bfs(new Set(["a"]), adj, 100)].sort()).toEqual(["a", "b"]);
  });

  test("negative maxDepth behaves like 0", () => {
    const adj = adjOf([["a", "b"]]);
    expect([...bfs(new Set(["a"]), adj, -1)].sort()).toEqual(["a"]);
  });
});
