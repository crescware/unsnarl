import { describe, expect, test } from "vitest";

import { LANGUAGE } from "../../language.js";
import { defaultSourceTypeFor } from "./default-source-type-for.js";
import { SOURCE_TYPE } from "./source-type.js";

describe("defaultSourceTypeFor", () => {
  test("maps js to script (Node.js default)", () => {
    expect(defaultSourceTypeFor(LANGUAGE.Js)).toEqual(SOURCE_TYPE.Script);
  });

  test.each([[LANGUAGE.Ts], [LANGUAGE.Tsx], [LANGUAGE.Jsx]] as const)(
    "maps %s to module",
    (language) => {
      expect(defaultSourceTypeFor(language)).toEqual(SOURCE_TYPE.Module);
    },
  );
});
