import { describe, expect, test } from "vitest";

import { OxcParser, ParseError } from "./oxc.js";

const parser = new OxcParser();

interface MinimalProgram {
  type: string;
  body: ReadonlyArray<{ type: string }>;
  sourceType?: string;
}

function asProgram(ast: unknown): MinimalProgram {
  return ast as MinimalProgram;
}

describe("OxcParser", () => {
  test("identifies itself as 'oxc'", () => {
    expect(parser.id).toBe("oxc");
  });

  test("parses a simple TS program into an ESTree-compatible Program node", () => {
    const code =
      "const greeting: string = 'hi';\nconst length = greeting.length;\n";
    const parsed = parser.parse(code, {
      language: "ts",
      sourcePath: "input.ts",
    });

    expect(parsed.language).toBe("ts");
    expect(parsed.sourcePath).toBe("input.ts");
    expect(parsed.raw).toBe(code);

    const program = asProgram(parsed.ast);
    expect(program.type).toBe("Program");
    expect(program.body).toHaveLength(2);
    expect(program.body[0]?.type).toBe("VariableDeclaration");
    expect(program.body[1]?.type).toBe("VariableDeclaration");
  });

  test("parses TSX with JSX elements", () => {
    const code =
      'const Hello = () => <div className="x"><span>{"hi"}</span></div>;\n';
    const parsed = parser.parse(code, {
      language: "tsx",
      sourcePath: "input.tsx",
    });

    const program = asProgram(parsed.ast);
    expect(program.type).toBe("Program");
    expect(program.body).toHaveLength(1);
    expect(program.body[0]?.type).toBe("VariableDeclaration");
  });

  test("parses JS with ESM import", () => {
    const code =
      "import { join } from 'node:path';\nexport const sep = join('a', 'b');\n";
    const parsed = parser.parse(code, {
      language: "js",
      sourcePath: "input.js",
    });

    const program = asProgram(parsed.ast);
    expect(program.body[0]?.type).toBe("ImportDeclaration");
    expect(program.body[1]?.type).toBe("ExportNamedDeclaration");
  });

  test("synthesizes a filename with the correct extension when sourcePath has none", () => {
    const code = "const x = 1;\n";
    expect(() =>
      parser.parse(code, { language: "ts", sourcePath: "" }),
    ).not.toThrow();
  });

  test("throws ParseError on syntactically invalid source", () => {
    const code = "const = 1;\n";
    let captured: unknown;
    try {
      parser.parse(code, { language: "ts", sourcePath: "broken.ts" });
    } catch (e) {
      captured = e;
    }
    expect(captured).toBeInstanceOf(ParseError);
    if (captured instanceof ParseError) {
      expect(captured.errors.length).toBeGreaterThan(0);
      expect(captured.message).toContain("broken.ts");
    }
  });
});
