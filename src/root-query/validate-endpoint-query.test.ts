import { describe, expect, test } from "vitest";

import { parseEndpointQuery } from "./parse-endpoint-query.js";
import type { ParsedRootQuery } from "./parsed-root-query.js";
import { validateEndpointQuery } from "./validate-endpoint-query.js";

function parse(text: string): ParsedRootQuery {
  const r = parseEndpointQuery(text);
  if (!r.ok) {
    throw new Error(
      `unexpected parse failure for '${text}': ${r.errors[0]?.message ?? "(no message)"}`,
    );
  }
  return r.value;
}

describe("validateEndpointQuery", () => {
  test.each(["0", "0:foo", "0-3", "L0", "l0"])(
    "rejects %s with 'line must be >= 1'",
    (input) => {
      const r = validateEndpointQuery(parse(input));
      if (r.ok) {
        throw new Error(`expected validation failure for '${input}'`);
      }
      expect(r.errors[0]?.message ?? "").toContain("line must be >= 1");
    },
  );

  test.each(["5-1", "L5-1", "l5-1"])(
    "rejects descending range %s with 'range start must be <= end'",
    (input) => {
      const r = validateEndpointQuery(parse(input));
      if (r.ok) {
        throw new Error(`expected validation failure for '${input}'`);
      }
      expect(r.errors[0]?.message ?? "").toContain(
        "range start must be <= end",
      );
    },
  );

  test.each(["1", "1-5", "foo", "L12", "L1-5", "5-5"])(
    "accepts %s",
    (input) => {
      expect(validateEndpointQuery(parse(input))).toMatchObject({ ok: true });
    },
  );

  test.each(["$", "_", "foo1"])(
    "accepts identifier %s without applying numeric validation",
    (input) => {
      expect(validateEndpointQuery(parse(input))).toMatchObject({ ok: true });
    },
  );
});
