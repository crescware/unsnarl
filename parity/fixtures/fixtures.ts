import type { ParityInput } from "../parity-input.js";

export const FIXTURES: readonly ParityInput[] = [
  {
    fixtureId: "declaration/var-script",
    code: "var x = 1;\n",
    language: "js",
    sourceType: "script",
  },
  {
    fixtureId: "declaration/function-decl",
    code: "function f(a) { return a; }\n",
    language: "js",
    sourceType: "script",
  },
  {
    fixtureId: "imports/import-named",
    code: 'import { x } from "m";\nx;\n',
    language: "js",
    sourceType: "module",
  },
  {
    fixtureId: "class/declaration-with-self-reference",
    code: "class C { static factory() { return new C(); } }\n",
    language: "js",
    sourceType: "script",
  },
  {
    fixtureId: "class/named-expression-with-self-reference",
    code: "const X = class C { static m() { return C; } };\n",
    language: "js",
    sourceType: "script",
  },
  {
    fixtureId: "class/anonymous-expression",
    code: "const X = class { m() {} };\n",
    language: "js",
    sourceType: "script",
  },
];
