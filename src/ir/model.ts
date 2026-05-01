export type Span = {
  line: number;
  column: number;
  offset: number;
};

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

export type AstNode = {
  readonly type: string;
  readonly start?: number;
  readonly end?: number;
  readonly [key: string]: unknown;
};

export type AstIdentifier = AstNode & {
  readonly type: "Identifier" | "JSXIdentifier";
  readonly name: string;
};

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

export type PredicateContainer = {
  type: PredicateContainerType;
  offset: number;
};

export type ReturnContainer = {
  startOffset: number;
  endOffset: number;
};

export type JsxElementContainer = {
  startOffset: number;
  endOffset: number;
};

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
  unsnarlOwners?: Variable[];
  unsnarlPredicateContainer?: PredicateContainer | null;
  unsnarlReturnContainer?: ReturnContainer | null;
  unsnarlJsxElement?: JsxElementContainer | null;
};

export type Definition = {
  type: DefinitionType;
  name: AstIdentifier;
  node: AstNode;
  parent: AstNode | null;
};

export type Variable = {
  name: string;
  scope: Scope;
  identifiers: AstIdentifier[];
  references: Reference[];
  defs: Definition[];
  unsnarlIsUnused?(): boolean;
};

export type BlockContext = {
  parentType: string;
  key: string;
  parentSpanOffset: number;
  caseTest?: string | null;
};

export type Scope = {
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
  unsnarlBlockContext?: BlockContext | null;
  unsnarlFallsThrough?: boolean;
  unsnarlExitsFunction?: boolean;
};

export type DiagnosticKind =
  | "var-detected"
  | "unresolved-identifier"
  | "parse-error";

export type Diagnostic = {
  kind: DiagnosticKind;
  message: string;
  span?: Span;
};

export type ScopeId = string;
export type VariableId = string;
export type ReferenceId = string;

export type SerializedScope = {
  id: ScopeId;
  type: ScopeType;
  isStrict: boolean;
  upper: ScopeId | null;
  childScopes: ScopeId[];
  variableScope: ScopeId;
  block: { type: string; span: Span; endSpan: Span };
  variables: VariableId[];
  references: ReferenceId[];
  through: ReferenceId[];
  functionExpressionScope: boolean;
  blockContext: BlockContext | null;
  fallsThrough: boolean;
  exitsFunction: boolean;
};

export type SerializedVariable = {
  id: VariableId;
  name: string;
  scope: ScopeId;
  identifiers: Span[];
  references: ReferenceId[];
  defs: SerializedDefinition[];
};

export type SerializedReference = {
  id: ReferenceId;
  identifier: { name: string; span: Span };
  from: ScopeId;
  resolved: VariableId | null;
  owners: VariableId[];
  writeExpr: Span | null;
  init: boolean;
  flags: {
    read: boolean;
    write: boolean;
    call: boolean;
    receiver: boolean;
  };
  predicateContainer: PredicateContainer | null;
  returnContainer: { startSpan: Span; endSpan: Span } | null;
  jsxElement: { startSpan: Span; endSpan: Span } | null;
};

export type ImportKind = "default" | "named" | "namespace";

export type VariableDeclarationKind = "var" | "let" | "const";

export type SerializedDefinition = {
  type: DefinitionType;
  name: { name: string; span: Span };
  node: { type: string; span: Span };
  parent: { type: string; span: Span } | null;
  initType: string | null;
  initSpan: Span | null;
  importKind: ImportKind | null;
  importSource: string | null;
  importedName: string | null;
  declarationKind: VariableDeclarationKind | null;
};

export type SerializedIR = {
  version: 1;
  source: { path: string; language: Language };
  raw: string;
  scopes: SerializedScope[];
  variables: SerializedVariable[];
  references: SerializedReference[];
  unusedVariableIds: VariableId[];
  diagnostics: Diagnostic[];
};
