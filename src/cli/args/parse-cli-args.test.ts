import { describe, expect, test } from "vitest";

import { CLI_MERMAID_RENDERER } from "../cli-mermaid-renderer.js";
import { LANGUAGE } from "../language.js";
import { ROOT_QUERY_KIND } from "../root-query/root-query-kind.js";
import { parseCliArgs } from "./parse-cli-args.js";

describe("parseCliArgs", () => {
  test("uses defaults when only an input file is provided", () => {
    const r = parseCliArgs(["foo.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args).toMatchObject({
        format: "ir",
        stdin: false,
        language: LANGUAGE.Ts,
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
    const elk = parseCliArgs([
      "--mermaid-renderer",
      CLI_MERMAID_RENDERER.Elk,
      "foo.ts",
    ]);
    if (elk.ok) {
      expect(elk.args.mermaidRenderer).toBe(CLI_MERMAID_RENDERER.Elk);
    }
    const dagre = parseCliArgs([
      "--mermaid-renderer",
      CLI_MERMAID_RENDERER.Dagre,
      "foo.ts",
    ]);
    if (dagre.ok) {
      expect(dagre.args.mermaidRenderer).toBe(CLI_MERMAID_RENDERER.Dagre);
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

  test("defaults pruning fields to empty / null", () => {
    const r = parseCliArgs(["foo.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.roots).toEqual([]);
      expect(r.args.descendants).toBeNull();
      expect(r.args.ancestors).toBeNull();
      expect(r.args.context).toBeNull();
    }
  });

  test("-r parses a single root query", () => {
    const r = parseCliArgs(["-r", "10:foo", "x.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.roots).toHaveLength(1);
      expect(r.args.roots[0]).toMatchObject({
        kind: ROOT_QUERY_KIND.LineName,
        line: 10,
      });
    }
  });

  test("--roots accepts comma-separated tokens", () => {
    const r = parseCliArgs(["--roots", "10:foo,42,9-13", "x.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.roots).toHaveLength(3);
      expect(r.args.roots.map((q) => q.kind)).toEqual([
        "line-name",
        "line",
        "range",
      ]);
    }
  });

  test("repeated -r flags accumulate", () => {
    const r = parseCliArgs(["-r", "10", "-r", "20", "x.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.roots).toHaveLength(2);
    }
  });

  test("-r and --roots can be mixed", () => {
    const r = parseCliArgs(["-r", "10:a", "--roots", "20,30", "x.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.roots).toHaveLength(3);
    }
  });

  test("-r rejects bad query and missing value", () => {
    expect(parseCliArgs(["-r"]).ok).toBe(false);
    const bad = parseCliArgs(["-r", "foo-bar", "x.ts"]);
    expect(bad.ok).toBe(false);
  });

  test("-A / -B / -C parse non-negative integers", () => {
    const r = parseCliArgs(["-A", "3", "-B", "2", "-C", "5", "x.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.descendants).toBe(3);
      expect(r.args.ancestors).toBe(2);
      expect(r.args.context).toBe(5);
    }
  });

  test("--descendants / --ancestors / --context aliases work", () => {
    const r = parseCliArgs([
      "--descendants",
      "1",
      "--ancestors",
      "2",
      "--context",
      "3",
      "x.ts",
    ]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.descendants).toBe(1);
      expect(r.args.ancestors).toBe(2);
      expect(r.args.context).toBe(3);
    }
  });

  test("-A 0 is allowed", () => {
    const r = parseCliArgs(["-A", "0", "x.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.descendants).toBe(0);
    }
  });

  test("-A rejects non-integer values", () => {
    expect(parseCliArgs(["-A", "foo", "x.ts"]).ok).toBe(false);
    expect(parseCliArgs(["-A", "-1", "x.ts"]).ok).toBe(false);
    expect(parseCliArgs(["-A", "1.5", "x.ts"]).ok).toBe(false);
  });

  test("-A rejects missing value", () => {
    expect(parseCliArgs(["-A"]).ok).toBe(false);
  });

  test("outDir defaults to null", () => {
    const r = parseCliArgs(["foo.ts"]);
    expect(r.ok).toBe(true);
    if (r.ok) {
      expect(r.args.outDir).toBeNull();
    }
  });

  test("-o / --out-dir parse a directory string", () => {
    const a = parseCliArgs(["-o", "./out", "foo.ts"]);
    const b = parseCliArgs(["--out-dir", "./out", "foo.ts"]);
    expect(a.ok).toBe(true);
    expect(b.ok).toBe(true);
    if (a.ok) {
      expect(a.args.outDir).toBe("./out");
    }
    if (b.ok) {
      expect(b.args.outDir).toBe("./out");
    }
  });

  test("-o rejects missing value", () => {
    expect(parseCliArgs(["-o"]).ok).toBe(false);
    expect(parseCliArgs(["--out-dir"]).ok).toBe(false);
  });
});
