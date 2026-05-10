import { describe, expect, test } from "vitest";

import { LANGUAGE } from "../language.js";
import { defaultSourceTypeFor } from "../pipeline/parse/default-source-type-for.js";
import { SOURCE_TYPE } from "../pipeline/parse/source-type.js";
import { AST_TYPE } from "./ast-type.js";
import { OxcParser } from "./oxc-parser.js";
import { ParseError } from "./parse-error.js";

const parser = new OxcParser();

type MinimalProgram = {
  type: string;
  body: readonly { type: string }[];
  sourceType?: string;
};

function asProgram(ast: unknown): MinimalProgram {
  return ast as MinimalProgram;
}

describe("OxcParser", () => {
  test("identifies itself as 'oxc'", () => {
    expect(parser.id).toEqual("oxc");
  });

  test("parses a simple TS program into an ESTree-compatible Program node", () => {
    const code =
      "const greeting: string = 'hi';\nconst length = greeting.length;\n";
    const parsed = parser.parse(code, {
      language: LANGUAGE.Ts,
      sourcePath: "input.ts",
      sourceType: defaultSourceTypeFor(LANGUAGE.Ts),
    });

    expect(parsed.language).toEqual("ts");
    expect(parsed.sourcePath).toEqual("input.ts");
    expect(parsed.sourceType).toEqual(SOURCE_TYPE.Module);
    expect(parsed.raw).toEqual(code);

    const program = asProgram(parsed.ast);
    expect(program.type).toEqual(AST_TYPE.Program);
    expect(program.body).toHaveLength(2);
    expect(program.body[0]?.type).toEqual(AST_TYPE.VariableDeclaration);
    expect(program.body[1]?.type).toEqual(AST_TYPE.VariableDeclaration);
  });

  test("parses TSX with JSX elements", () => {
    const code =
      'const Hello = () => <div className="x"><span>{"hi"}</span></div>;\n';
    const parsed = parser.parse(code, {
      language: LANGUAGE.Tsx,
      sourcePath: "input.tsx",
      sourceType: defaultSourceTypeFor(LANGUAGE.Tsx),
    });

    const program = asProgram(parsed.ast);
    expect(program.type).toEqual(AST_TYPE.Program);
    expect(program.body).toHaveLength(1);
    expect(program.body[0]?.type).toEqual(AST_TYPE.VariableDeclaration);
  });

  test("parses JS with ESM import", () => {
    const code =
      "import { join } from 'node:path';\nexport const sep = join('a', 'b');\n";
    const parsed = parser.parse(code, {
      language: LANGUAGE.Js,
      sourcePath: "input.js",
      sourceType: SOURCE_TYPE.Module,
    });

    const program = asProgram(parsed.ast);
    expect(program.body[0]?.type).toEqual(AST_TYPE.ImportDeclaration);
    expect(program.body[1]?.type).toEqual(AST_TYPE.ExportNamedDeclaration);
  });

  test("preserves an explicitly requested sourceType regardless of the language extension", () => {
    const code = "var legacy = 1;\n";
    const parsed = parser.parse(code, {
      language: LANGUAGE.Js,
      sourcePath: "input.js",
      sourceType: SOURCE_TYPE.Script,
    });
    expect(parsed.sourceType).toEqual(SOURCE_TYPE.Script);
  });

  test("synthesizes a filename with the correct extension when sourcePath has none", () => {
    const code = "const x = 1;\n";
    expect(() =>
      parser.parse(code, {
        language: LANGUAGE.Ts,
        sourcePath: "",
        sourceType: defaultSourceTypeFor(LANGUAGE.Ts),
      }),
    ).not.toThrow();
  });

  test("throws ParseError on syntactically invalid source", () => {
    const code = "const = 1;\n";
    let captured: unknown;
    try {
      parser.parse(code, {
        language: LANGUAGE.Ts,
        sourcePath: "broken.ts",
        sourceType: defaultSourceTypeFor(LANGUAGE.Ts),
      });
    } catch (e) {
      captured = e;
    }
    expect(captured instanceof ParseError).toEqual(true);
    if (captured instanceof ParseError) {
      expect(captured.errors.length > 0).toEqual(true);
      expect(captured.message).toContain("broken.ts");
    }
  });
});
