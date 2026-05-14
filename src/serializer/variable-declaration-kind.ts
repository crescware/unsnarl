import { picklist, type InferOutput } from "valibot";

export const VARIABLE_DECLARATION_KIND = {
  Var: "var",
  Let: "let",
  Const: "const",
} as const;

export const variableDeclarationKind$ = picklist([
  VARIABLE_DECLARATION_KIND.Var,
  VARIABLE_DECLARATION_KIND.Let,
  VARIABLE_DECLARATION_KIND.Const,
]);

export type VariableDeclarationKind = InferOutput<
  typeof variableDeclarationKind$
>;
