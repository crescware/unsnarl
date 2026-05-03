import type {
  DEFINITION_TYPE,
  DefinitionType,
} from "../analyzer/definition-type.js";
import type { DiagnosticKind } from "../analyzer/diagnostic-kind.js";
import type { PredicateContainerType } from "../analyzer/predicate-container-type.js";
import type { ScopeType } from "../analyzer/scope-type.js";
import type { Language } from "../cli/language.js";
import type { AST_TYPE } from "../parser/ast-type.js";
import type { IMPORT_KIND } from "../serializer/import-kind.js";
import type { SerializedIRVersion } from "../serializer/serialized-ir-version.js";
import type { VariableDeclarationKind } from "../serializer/variable-declaration-kind.js";

export type {
  DefinitionType,
  DiagnosticKind,
  Language,
  ScopeType,
  VariableDeclarationKind,
};

export type Span = Readonly<{
  line: number;
  column: number;
  offset: number;
}>;

export type AstNode = Readonly<{
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}>;

export type AstIdentifier = AstNode &
  Readonly<{
    type: typeof AST_TYPE.Identifier | typeof AST_TYPE.JSXIdentifier;
    name: string;
  }>;

export const ReferenceFlags = {
  None: 0,
  Read: 1 << 0,
  Write: 1 << 1,
  Call: 1 << 2,
  Receiver: 1 << 3,
} as const;

export type ReferenceFlagBits = number;

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
  init: boolean;
  isWrite(): boolean;
  isRead(): boolean;
  isReadOnly(): boolean;
  isWriteOnly(): boolean;
  isReadWrite(): boolean;
  isCall(): boolean;
  isReceiver(): boolean;
  unsnarlFlags: ReferenceFlagBits;
  unsnarlOwners: /* mutable */ Variable[];
  unsnarlPredicateContainer: PredicateContainer | null;
  unsnarlReturnContainer: ReturnContainer | null;
  unsnarlJsxElement: JsxElementContainer | null;
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
  unsnarlIsUnused(): boolean;
};

// caseTest is only meaningful when this block is a switch-case clause.
// Other contexts (if/else, try/catch/finally, for body, etc.) carry no
// kind-specific payload, so the `case-clause` variant is the only one
// that adds a field.
export type BlockContext =
  | Readonly<{
      kind: "case-clause";
      parentType: string;
      key: string;
      parentSpanOffset: number;
      caseTest: string | null;
    }>
  | Readonly<{
      kind: "other";
      parentType: string;
      key: string;
      parentSpanOffset: number;
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
  unsnarlBlockContext: BlockContext | null;
  unsnarlFallsThrough: boolean;
  unsnarlExitsFunction: boolean;
};

export type Diagnostic = Readonly<{
  kind: DiagnosticKind;
  message: string;
  span: Span;
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

type CommonDefFields = Readonly<{
  name: Readonly<{ name: string; span: Span }>;
  node: Readonly<{ type: string; span: Span }>;
  parent: Readonly<{ type: string; span: Span }> | null;
}>;

export type SerializedDefinition =
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.Variable;
        init: Readonly<{ type: string; span: Span }> | null;
        declarationKind: VariableDeclarationKind | null;
      }>)
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.ImportBinding;
        importKind: typeof IMPORT_KIND.Named;
        importedName: string;
        importSource: string;
      }>)
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.ImportBinding;
        importKind: typeof IMPORT_KIND.Default;
        importSource: string;
      }>)
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.ImportBinding;
        importKind: typeof IMPORT_KIND.Namespace;
        importSource: string;
      }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.FunctionName }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.ClassName }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.Parameter }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.CatchClause }>)
  | (CommonDefFields &
      Readonly<{ type: typeof DEFINITION_TYPE.ImplicitGlobalVariable }>);

export type SerializedIR = Readonly<{
  version: SerializedIRVersion;
  source: Readonly<{ path: string; language: Language }>;
  raw: string;
  scopes: readonly SerializedScope[];
  variables: readonly SerializedVariable[];
  references: readonly SerializedReference[];
  unusedVariableIds: readonly VariableId[];
  diagnostics: readonly Diagnostic[];
}>;
