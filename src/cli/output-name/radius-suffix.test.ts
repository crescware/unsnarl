import { describe, expect, test } from "vitest";

import { radiusSuffix } from "./radius-suffix.js";

describe("radiusSuffix", () => {
  test("all null → empty string", () => {
    expect(
      radiusSuffix({ descendants: null, ancestors: null, context: null }),
    ).toBe("");
  });

  test("only descendants → -a<N>", () => {
    expect(
      radiusSuffix({ descendants: 1, ancestors: null, context: null }),
    ).toBe("-a1");
  });

  test("only ancestors → -b<N>", () => {
    expect(
      radiusSuffix({ descendants: null, ancestors: 2, context: null }),
    ).toBe("-b2");
  });

  test("only context → -c<N>", () => {
    expect(
      radiusSuffix({ descendants: null, ancestors: null, context: 3 }),
    ).toBe("-c3");
  });

  test("descendants + ancestors → -a<N>-b<M> in alphabetical order", () => {
    expect(radiusSuffix({ descendants: 1, ancestors: 2, context: null })).toBe(
      "-a1-b2",
    );
  });

  test("descendants + context → -a<N>-c<M>", () => {
    expect(radiusSuffix({ descendants: 7, ancestors: null, context: 3 })).toBe(
      "-a7-c3",
    );
  });

  test("ancestors + context → -b<N>-c<M>", () => {
    expect(radiusSuffix({ descendants: null, ancestors: 2, context: 3 })).toBe(
      "-b2-c3",
    );
  });

  test("descendants + ancestors + context → drops -c (redundant once both -a and -b are explicit)", () => {
    expect(radiusSuffix({ descendants: 1, ancestors: 2, context: 3 })).toBe(
      "-a1-b2",
    );
  });

  test("zero is preserved verbatim", () => {
    expect(radiusSuffix({ descendants: 0, ancestors: 0, context: null })).toBe(
      "-a0-b0",
    );
  });
});
