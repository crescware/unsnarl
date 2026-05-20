import { describe, expect, test } from "vitest";

import { parseDirectionToken } from "./parse-direction-token.js";

describe("parseDirectionToken", () => {
  test.each([
    ["+a", "a"],
    ["+b", "b"],
    ["+c", "c"],
  ] as const)("parses %s with level = null", (input, dir) => {
    expect(parseDirectionToken(input)).toEqual({
      ok: true,
      value: { dir, level: null },
    });
  });

  test.each([
    ["+a3", "a", 3],
    ["+b10", "b", 10],
    ["+c1", "c", 1],
  ] as const)("parses %s with level = %s", (input, dir, level) => {
    expect(parseDirectionToken(input)).toEqual({
      ok: true,
      value: { dir, level },
    });
  });

  test("syntactically accepts +a0 (numeric validation moves to runtime layer)", () => {
    expect(parseDirectionToken("+a0")).toEqual({
      ok: true,
      value: { dir: "a", level: 0 },
    });
  });

  test("rejects bare '+' or empty", () => {
    expect(parseDirectionToken("+")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
    expect(parseDirectionToken("")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });

  test("rejects unknown direction letters", () => {
    expect(parseDirectionToken("+x")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
    expect(parseDirectionToken("+d")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });

  test("rejects multi-letter directions", () => {
    expect(parseDirectionToken("+ab")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
    expect(parseDirectionToken("+aa")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });

  test("rejects trailing garbage after digits", () => {
    expect(parseDirectionToken("+a3b")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
    expect(parseDirectionToken("+a-3")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });

  test("rejects direction-like text without '+'", () => {
    expect(parseDirectionToken("a")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
    expect(parseDirectionToken("a3")).toMatchObject({
      ok: false,
      errors: [
        { message: expect.stringContaining("unexpected direction token") },
      ],
    });
  });
});
