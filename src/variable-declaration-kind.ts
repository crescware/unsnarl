export const VARIABLE_DECLARATION_KIND = {
  Var: "var",
  Let: "let",
  Const: "const",
} as const;
export type VariableDeclarationKind =
  (typeof VARIABLE_DECLARATION_KIND)[keyof typeof VARIABLE_DECLARATION_KIND];
