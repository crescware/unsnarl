import { describe, expect, test } from "vitest";

import { ReferenceFlags } from "../../ir/reference/reference-flags.js";
import { reference } from "./reference.js";

describe("reference factory", () => {
  test("returns a reference-kind result with the given fields", () => {
    expect(reference(ReferenceFlags.Read, false)).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Read,
      init: false,
    });
  });

  test("preserves init=true", () => {
    expect(reference(ReferenceFlags.Write, true)).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Write,
      init: true,
    });
  });
});
