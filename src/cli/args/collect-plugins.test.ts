import { describe, expect, test } from "vitest";

import { collectPlugins } from "./collect-plugins.js";

describe("collectPlugins", () => {
  test.each<{
    name: string;
    value: string;
    prev: readonly string[];
    expected: readonly string[];
  }>([
    {
      name: "appends a single short name",
      value: "react",
      prev: [],
      expected: ["react"],
    },
    {
      name: "strips the 'unsnarl-plugin-' prefix when present",
      value: "unsnarl-plugin-react",
      prev: [],
      expected: ["react"],
    },
    {
      name: "splits a comma-delimited value into multiple entries",
      value: "react,vue",
      prev: [],
      expected: ["react", "vue"],
    },
    {
      name: "trims surrounding whitespace per entry",
      value: " react , vue ",
      prev: [],
      expected: ["react", "vue"],
    },
    {
      name: "drops empty fragments from consecutive commas",
      value: "react,,vue",
      prev: [],
      expected: ["react", "vue"],
    },
    {
      name: "deduplicates repeated names within a single comma-delimited value",
      value: "react,vue,react",
      prev: [],
      expected: ["react", "vue"],
    },
    {
      name: "deduplicates names already present in the previous list",
      value: "react",
      prev: ["react"],
      expected: ["react"],
    },
    {
      name: "treats the prefixed form as the same name as the short form for dedup",
      value: "unsnarl-plugin-react",
      prev: ["react"],
      expected: ["react"],
    },
    {
      name: "returns the previous list unchanged for an empty value",
      value: "",
      prev: ["react"],
      expected: ["react"],
    },
  ])("$name", ({ value, prev, expected }) => {
    expect(collectPlugins(value, prev)).toEqual(expected);
  });

  test("accumulates across repeated invocations (commander pattern)", () => {
    const first = collectPlugins("react", []);
    const second = collectPlugins("vue", first);
    expect(second).toEqual(["react", "vue"]);
  });
});
