export type Span = Readonly<{
  line: number;
  column: number;
  offset: number;
}>;

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

export type AstNode = Readonly<{
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}>;

export type AstIdentifier = AstNode &
  Readonly<{
    type: "Identifier" | "JSXIdentifier";
    name: string;
  }>;

export type AstExpression = AstNode;

export const ReferenceFlags = {
  None: 0,
  Read: 1 << 0,
  Write: 1 << 1,
  Call: 1 << 2,
  Receiver: 1 << 3,
} as const;

export type ReferenceFlagBits = number;

export type PredicateContainerType = "IfStatement" | "SwitchStatement";

export type PredicateContainer = Readonly<{
  type: PredicateContainerType;
  offset: number;
}>;

export type ReturnContainer = Readonly<{
  startOffset: number;
  endOffset: number;
}>;

export type JsxElementContainer = Readonly<{
  startOffset: number;
  endOffset: number;
}>;

// Reference / Variable / Scope keep mutable fields and arrays because the
// builder mutates them in place during scope analysis (ScopeImpl pushes
// onto `variables`, `references`, etc.; bindReference reassigns
// `resolved`). Wrapping in Readonly<...> would break those algorithms.
export type Reference = {
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
  isReceiver?(): boolean;
  unsnarlFlags?: ReferenceFlagBits;
  unsnarlOwners?: /* mutable */ Variable[];
  unsnarlPredicateContainer?: PredicateContainer | null;
  unsnarlReturnContainer?: ReturnContainer | null;
  unsnarlJsxElement?: JsxElementContainer | null;
};

export type Definition = Readonly<{
  type: DefinitionType;
  name: AstIdentifier;
  node: AstNode;
  parent: AstNode | null;
}>;

// Mutable: declareVariable pushes into identifiers/defs and bindReference
// pushes into references during analysis.
export type Variable = {
  name: string;
  scope: Scope;
  identifiers: /* mutable */ AstIdentifier[];
  references: /* mutable */ Reference[];
  defs: /* mutable */ Definition[];
  unsnarlIsUnused?(): boolean;
};

export type BlockContext = Readonly<{
  parentType: string;
  key: string;
  parentSpanOffset: number;
  caseTest?: string | null;
}>;

// Mutable: ScopeImpl pushes into childScopes / variables / references /
// through and reassigns the unsnarl* annotation fields throughout the
// eslint-compat analyzer pass.
export type Scope = {
  type: ScopeType;
  isStrict: boolean;
  upper: Scope | null;
  childScopes: /* mutable */ Scope[];
  variableScope: Scope;
  block: AstNode;
  variables: /* mutable */ Variable[];
  set: Map<string, Variable>;
  references: /* mutable */ Reference[];
  through: /* mutable */ Reference[];
  functionExpressionScope: boolean;
  unsnarlBlockContext?: BlockContext | null;
  unsnarlFallsThrough?: boolean;
  unsnarlExitsFunction?: boolean;
};

export type DiagnosticKind =
  | "var-detected"
  | "unresolved-identifier"
  | "parse-error";

export type Diagnostic = Readonly<{
  kind: DiagnosticKind;
  message: string;
  span?: Span;
}>;

export type ScopeId = string;
export type VariableId = string;
export type ReferenceId = string;

export type SerializedScope = Readonly<{
  id: ScopeId;
  type: ScopeType;
  isStrict: boolean;
  upper: ScopeId | null;
  childScopes: readonly ScopeId[];
  variableScope: ScopeId;
  block: Readonly<{ type: string; span: Span; endSpan: Span }>;
  variables: readonly VariableId[];
  references: readonly ReferenceId[];
  through: readonly ReferenceId[];
  functionExpressionScope: boolean;
  blockContext: BlockContext | null;
  fallsThrough: boolean;
  exitsFunction: boolean;
}>;

export type SerializedVariable = Readonly<{
  id: VariableId;
  name: string;
  scope: ScopeId;
  identifiers: readonly Span[];
  references: readonly ReferenceId[];
  defs: readonly SerializedDefinition[];
}>;

export type SerializedReference = Readonly<{
  id: ReferenceId;
  identifier: Readonly<{ name: string; span: Span }>;
  from: ScopeId;
  resolved: VariableId | null;
  owners: readonly VariableId[];
  writeExpr: Span | null;
  init: boolean;
  flags: Readonly<{
    read: boolean;
    write: boolean;
    call: boolean;
    receiver: boolean;
  }>;
  predicateContainer: PredicateContainer | null;
  returnContainer: Readonly<{ startSpan: Span; endSpan: Span }> | null;
  jsxElement: Readonly<{ startSpan: Span; endSpan: Span }> | null;
}>;

export type ImportKind = "default" | "named" | "namespace";

export type VariableDeclarationKind = "var" | "let" | "const";

export type SerializedDefinition = Readonly<{
  type: DefinitionType;
  name: Readonly<{ name: string; span: Span }>;
  node: Readonly<{ type: string; span: Span }>;
  parent: Readonly<{ type: string; span: Span }> | null;
  initType: string | null;
  initSpan: Span | null;
  importKind: ImportKind | null;
  importSource: string | null;
  importedName: string | null;
  declarationKind: VariableDeclarationKind | null;
}>;

export type SerializedIR = Readonly<{
  version: 1;
  source: Readonly<{ path: string; language: Language }>;
  raw: string;
  scopes: readonly SerializedScope[];
  variables: readonly SerializedVariable[];
  references: readonly SerializedReference[];
  unusedVariableIds: readonly VariableId[];
  diagnostics: readonly Diagnostic[];
}>;
