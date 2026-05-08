import { describe, expect, test } from "vitest";

import { LANGUAGE } from "../../language.js";
import { sourceTypeFromPath } from "./source-type-from-path.js";
import { SOURCE_TYPE } from "./source-type.js";

describe("sourceTypeFromPath", () => {
  test(".mjs → module regardless of language tag", () => {
    expect(sourceTypeFromPath("foo.mjs", LANGUAGE.Js)).toBe(SOURCE_TYPE.Module);
  });

  test(".cjs → script regardless of language tag", () => {
    expect(sourceTypeFromPath("foo.cjs", LANGUAGE.Js)).toBe(SOURCE_TYPE.Script);
  });

  test("nested paths still inspect the trailing suffix", () => {
    expect(sourceTypeFromPath("src/deep/foo.mjs", LANGUAGE.Js)).toBe(
      SOURCE_TYPE.Module,
    );
    expect(sourceTypeFromPath("src/deep/foo.cjs", LANGUAGE.Js)).toBe(
      SOURCE_TYPE.Script,
    );
  });

  test(".js → script (falls back to defaultSourceTypeFor)", () => {
    expect(sourceTypeFromPath("foo.js", LANGUAGE.Js)).toBe(SOURCE_TYPE.Script);
  });

  test.each([[LANGUAGE.Ts], [LANGUAGE.Tsx], [LANGUAGE.Jsx]] as const)(
    "non-js languages fall back to module via defaultSourceTypeFor (%s)",
    (language) => {
      expect(sourceTypeFromPath(`foo.${language}`, language)).toBe(
        SOURCE_TYPE.Module,
      );
    },
  );
});
