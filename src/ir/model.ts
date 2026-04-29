export interface Span {
  line: number;
  column: number;
  offset: number;
}

export type Language = "ts" | "tsx" | "js" | "jsx";

export type ScopeType =
  | "block"
  | "catch"
  | "class"
  | "class-field-initializer"
  | "class-static-block"
  | "for"
  | "function"
  | "function-expression-name"
  | "global"
  | "module"
  | "switch"
  | "with";

export type DefinitionType =
  | "CatchClause"
  | "ClassName"
  | "FunctionName"
  | "ImplicitGlobalVariable"
  | "ImportBinding"
  | "Parameter"
  | "Variable";

export interface AstNode {
  readonly type: string;
  readonly start?: number;
  readonly end?: number;
}

export interface AstIdentifier extends AstNode {
  readonly type: "Identifier" | "JSXIdentifier";
  readonly name: string;
}

export type AstExpression = AstNode;

export const ReferenceFlags = {
  None: 0,
  Read: 1 << 0,
  Write: 1 << 1,
  Call: 1 << 2,
} as const;

export type ReferenceFlagBits = number;

export interface Reference {
  identifier: AstIdentifier;
  from: Scope;
  resolved: Variable | null;
  writeExpr: AstExpression | null;
  init: boolean;
  isWrite(): boolean;
  isRead(): boolean;
  isReadOnly(): boolean;
  isWriteOnly(): boolean;
  isReadWrite(): boolean;
  isCall?(): boolean;
  unsnarlFlags?: ReferenceFlagBits;
}

export interface Definition {
  type: DefinitionType;
  name: AstIdentifier;
  node: AstNode;
  parent: AstNode | null;
}

export interface Variable {
  name: string;
  scope: Scope;
  identifiers: AstIdentifier[];
  references: Reference[];
  defs: Definition[];
  unsnarlIsUnused?(): boolean;
}

export interface Scope {
  type: ScopeType;
  isStrict: boolean;
  upper: Scope | null;
  childScopes: Scope[];
  variableScope: Scope;
  block: AstNode;
  variables: Variable[];
  set: Map<string, Variable>;
  references: Reference[];
  through: Reference[];
  functionExpressionScope: boolean;
}

export type DiagnosticKind =
  | "var-detected"
  | "unresolved-identifier"
  | "parse-error";

export interface Diagnostic {
  kind: DiagnosticKind;
  message: string;
  span?: Span;
}

export type ScopeId = string;
export type VariableId = string;
export type ReferenceId = string;

export interface SerializedScope {
  id: ScopeId;
  type: ScopeType;
  isStrict: boolean;
  upper: ScopeId | null;
  childScopes: ScopeId[];
  variableScope: ScopeId;
  block: { type: string; span: Span };
  variables: VariableId[];
  references: ReferenceId[];
  through: ReferenceId[];
  functionExpressionScope: boolean;
}

export interface SerializedVariable {
  id: VariableId;
  name: string;
  scope: ScopeId;
  identifiers: Span[];
  references: ReferenceId[];
  defs: SerializedDefinition[];
}

export interface SerializedReference {
  id: ReferenceId;
  identifier: { name: string; span: Span };
  from: ScopeId;
  resolved: VariableId | null;
  writeExpr: Span | null;
  init: boolean;
  flags: { read: boolean; write: boolean; call: boolean };
}

export interface SerializedDefinition {
  type: DefinitionType;
  name: { name: string; span: Span };
  node: { type: string; span: Span };
  parent: { type: string; span: Span } | null;
}

export interface SerializedIR {
  version: 1;
  source: { path: string; language: Language };
  scopes: SerializedScope[];
  variables: SerializedVariable[];
  references: SerializedReference[];
  unusedVariableIds: VariableId[];
  diagnostics: Diagnostic[];
}
