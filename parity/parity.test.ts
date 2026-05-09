import { describe, expect, test } from "vitest";

import { FIXTURES } from "./fixtures/fixtures.js";
import { runParity } from "./run-parity.js";

describe("eslint-scope parity", () => {
  test.each(FIXTURES)("$fixtureId", (fixture) => {
    const mismatches = runParity(fixture);
    if (mismatches.length > 0) {
      const lines = mismatches.map(
        (v) => `  [${v.kind}] ${v.scopePath.join(".") || "root"}: ${v.message}`,
      );
      throw new Error(
        `Parity mismatch for ${fixture.fixtureId} (${mismatches.length} entries):\n${lines.join("\n")}`,
      );
    }
    expect(mismatches).toEqual([]);
  });
});
