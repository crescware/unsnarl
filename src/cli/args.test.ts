import { describe, expect, test } from "vitest";

import { parseCliArgs } from "./args.js";

describe("parseCliArgs", () => {
  test("uses defaults when only an input file is provided", () => {
    const r = parseCliArgs(["foo.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args).toMatchObject({
        format: "ir",
        stdin: false,
        language: "ts",
        pretty: true,
        listFormats: false,
        help: false,
        version: false,
        inputFile: "foo.ts",
      });
    }
  });

  test("parses --format / --lang / --no-pretty", () => {
    const r = parseCliArgs([
      "--format",
      "mermaid",
      "--lang",
      "tsx",
      "--no-pretty",
      "x.tsx",
    ]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.format).toBe("mermaid");
      expect(r.args.language).toBe("tsx");
      expect(r.args.pretty).toBe(false);
      expect(r.args.inputFile).toBe("x.tsx");
    }
  });

  test("--stdin / --list-formats / --help / --version flags", () => {
    expect(parseCliArgs(["--stdin"]).ok).toBe(true);
    const r = parseCliArgs(["--stdin"]);
    if (r.ok) {
      expect(r.args.stdin).toBe(true);
    }
    const lf = parseCliArgs(["--list-formats"]);
    if (lf.ok) {
      expect(lf.args.listFormats).toBe(true);
    }
    const hp = parseCliArgs(["-h"]);
    if (hp.ok) {
      expect(hp.args.help).toBe(true);
    }
    const v = parseCliArgs(["-v"]);
    if (v.ok) {
      expect(v.args.version).toBe(true);
    }
  });

  test("rejects unknown options", () => {
    const r = parseCliArgs(["--unknown"]);
    expect(r.ok).toBe(false);
    if (!r.ok) {
      expect(r.error).toMatch(/Unknown option/);
    }
  });

  test("rejects missing values for --format / --lang", () => {
    expect(parseCliArgs(["--format"]).ok).toBe(false);
    expect(parseCliArgs(["--lang"]).ok).toBe(false);
  });

  test("rejects invalid --lang values", () => {
    const r = parseCliArgs(["--lang", "rust"]);
    expect(r.ok).toBe(false);
    if (!r.ok) {
      expect(r.error).toMatch(/Invalid language/);
    }
  });

  test("rejects multiple input files", () => {
    const r = parseCliArgs(["a.ts", "b.ts"]);
    expect(r.ok).toBe(false);
    if (!r.ok) {
      expect(r.error).toMatch(/Multiple input files/);
    }
  });

  test("--mermaid-renderer accepts dagre/elk and defaults to null", () => {
    const def = parseCliArgs(["foo.ts"]);
    if (def.ok) {
      expect(def.args.mermaidRenderer).toBeNull();
    }
    const elk = parseCliArgs(["--mermaid-renderer", "elk", "foo.ts"]);
    if (elk.ok) {
      expect(elk.args.mermaidRenderer).toBe("elk");
    }
    const dagre = parseCliArgs(["--mermaid-renderer", "dagre", "foo.ts"]);
    if (dagre.ok) {
      expect(dagre.args.mermaidRenderer).toBe("dagre");
    }
  });

  test("--mermaid-renderer rejects unknown values and missing arg", () => {
    expect(parseCliArgs(["--mermaid-renderer"]).ok).toBe(false);
    const bad = parseCliArgs(["--mermaid-renderer", "graphviz"]);
    expect(bad.ok).toBe(false);
    if (!bad.ok) {
      expect(bad.error).toMatch(/Invalid mermaid renderer/);
    }
  });
});
