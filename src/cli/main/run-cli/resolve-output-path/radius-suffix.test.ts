import { describe, expect, test } from "vitest";

import { radiusSuffix } from "./radius-suffix.js";

describe("radiusSuffix", () => {
  test.each<{
    name: string;
    descendants: number | null;
    ancestors: number | null;
    context: number | null;
    expected: string;
  }>([
    {
      name: "all null → empty string",
      descendants: null,
      ancestors: null,
      context: null,
      expected: "",
    },
    {
      name: "only descendants → -a<N>",
      descendants: 1,
      ancestors: null,
      context: null,
      expected: "-a1",
    },
    {
      name: "only ancestors → -b<N>",
      descendants: null,
      ancestors: 2,
      context: null,
      expected: "-b2",
    },
    {
      name: "only context → -c<N>",
      descendants: null,
      ancestors: null,
      context: 3,
      expected: "-c3",
    },
    {
      name: "descendants + ancestors → -a<N>-b<M> in alphabetical order",
      descendants: 1,
      ancestors: 2,
      context: null,
      expected: "-a1-b2",
    },
    {
      name: "descendants + context → -a<N>-c<M>",
      descendants: 7,
      ancestors: null,
      context: 3,
      expected: "-a7-c3",
    },
    {
      name: "ancestors + context → -b<N>-c<M>",
      descendants: null,
      ancestors: 2,
      context: 3,
      expected: "-b2-c3",
    },
    {
      name: "descendants + ancestors + context → drops -c (redundant once both -a and -b are explicit)",
      descendants: 1,
      ancestors: 2,
      context: 3,
      expected: "-a1-b2",
    },
    {
      name: "zero is preserved verbatim",
      descendants: 0,
      ancestors: 0,
      context: null,
      expected: "-a0-b0",
    },
  ])("$name", ({ descendants, ancestors, context, expected }) => {
    expect(radiusSuffix({ descendants, ancestors, context })).toBe(expected);
  });
});
