// Single source of truth for every string-literal and numeric tag used as
// a discriminator across the codebase. Each `* as const` block owns a
// related family of values; the matching `type` alias is derived from
// `(typeof X)[keyof typeof X]` so the union and the lookup table cannot
// drift apart. Implementation sites import the constant rather than
// retyping the literal.

// ---------------------------------------------------------------------------
// AST layer (oxc-parser node `type` discriminator strings)
// ---------------------------------------------------------------------------

export const AST_TYPE = {
  AccessorProperty: "AccessorProperty",
  ArrayExpression: "ArrayExpression",
  ArrayPattern: "ArrayPattern",
  ArrowFunctionExpression: "ArrowFunctionExpression",
  AssignmentExpression: "AssignmentExpression",
  AssignmentPattern: "AssignmentPattern",
  BinaryExpression: "BinaryExpression",
  BlockStatement: "BlockStatement",
  BooleanLiteral: "BooleanLiteral",
  BreakStatement: "BreakStatement",
  CallExpression: "CallExpression",
  CatchClause: "CatchClause",
  ClassBody: "ClassBody",
  ClassDeclaration: "ClassDeclaration",
  ClassExpression: "ClassExpression",
  ContinueStatement: "ContinueStatement",
  ExportDefaultDeclaration: "ExportDefaultDeclaration",
  ExportNamedDeclaration: "ExportNamedDeclaration",
  ExportSpecifier: "ExportSpecifier",
  ExpressionStatement: "ExpressionStatement",
  ForInStatement: "ForInStatement",
  ForOfStatement: "ForOfStatement",
  ForStatement: "ForStatement",
  FunctionDeclaration: "FunctionDeclaration",
  FunctionExpression: "FunctionExpression",
  Identifier: "Identifier",
  IfStatement: "IfStatement",
  ImportDeclaration: "ImportDeclaration",
  ImportDefaultSpecifier: "ImportDefaultSpecifier",
  ImportNamespaceSpecifier: "ImportNamespaceSpecifier",
  ImportSpecifier: "ImportSpecifier",
  JSXAttribute: "JSXAttribute",
  JSXClosingElement: "JSXClosingElement",
  JSXElement: "JSXElement",
  JSXIdentifier: "JSXIdentifier",
  JSXMemberExpression: "JSXMemberExpression",
  JSXOpeningElement: "JSXOpeningElement",
  Literal: "Literal",
  MemberExpression: "MemberExpression",
  MethodDefinition: "MethodDefinition",
  NewExpression: "NewExpression",
  NullLiteral: "NullLiteral",
  NumericLiteral: "NumericLiteral",
  ObjectExpression: "ObjectExpression",
  ObjectPattern: "ObjectPattern",
  Program: "Program",
  Property: "Property",
  PropertyDefinition: "PropertyDefinition",
  RestElement: "RestElement",
  ReturnStatement: "ReturnStatement",
  StringLiteral: "StringLiteral",
  SwitchCase: "SwitchCase",
  SwitchStatement: "SwitchStatement",
  TemplateLiteral: "TemplateLiteral",
  ThrowStatement: "ThrowStatement",
  TryStatement: "TryStatement",
  UpdateExpression: "UpdateExpression",
  VariableDeclaration: "VariableDeclaration",
  VariableDeclarator: "VariableDeclarator",
} as const;
export type AstType = (typeof AST_TYPE)[keyof typeof AST_TYPE];

// ---------------------------------------------------------------------------
// IR layer
// ---------------------------------------------------------------------------

export const SCOPE_TYPE = {
  Block: "block",
  Catch: "catch",
  Class: "class",
  ClassFieldInitializer: "class-field-initializer",
  ClassStaticBlock: "class-static-block",
  For: "for",
  Function: "function",
  FunctionExpressionName: "function-expression-name",
  Global: "global",
  Module: "module",
  Switch: "switch",
  With: "with",
} as const;
export type ScopeType = (typeof SCOPE_TYPE)[keyof typeof SCOPE_TYPE];

export const DEFINITION_TYPE = {
  CatchClause: "CatchClause",
  ClassName: "ClassName",
  FunctionName: "FunctionName",
  ImplicitGlobalVariable: "ImplicitGlobalVariable",
  ImportBinding: "ImportBinding",
  Parameter: "Parameter",
  Variable: "Variable",
} as const;
export type DefinitionType =
  (typeof DEFINITION_TYPE)[keyof typeof DEFINITION_TYPE];

export const IMPORT_KIND = {
  Default: "default",
  Named: "named",
  Namespace: "namespace",
} as const;
export type ImportKind = (typeof IMPORT_KIND)[keyof typeof IMPORT_KIND];

export const VARIABLE_DECLARATION_KIND = {
  Var: "var",
  Let: "let",
  Const: "const",
} as const;
export type VariableDeclarationKind =
  (typeof VARIABLE_DECLARATION_KIND)[keyof typeof VARIABLE_DECLARATION_KIND];

export const DIAGNOSTIC_KIND = {
  VarDetected: "var-detected",
  UnresolvedIdentifier: "unresolved-identifier",
  ParseError: "parse-error",
} as const;
export type DiagnosticKind =
  (typeof DIAGNOSTIC_KIND)[keyof typeof DIAGNOSTIC_KIND];

export const PREDICATE_CONTAINER_TYPE = {
  IfStatement: "IfStatement",
  SwitchStatement: "SwitchStatement",
} as const;
export type PredicateContainerType =
  (typeof PREDICATE_CONTAINER_TYPE)[keyof typeof PREDICATE_CONTAINER_TYPE];

// SerializedIR.version is a numeric discriminator: bump it every time the
// on-disk shape changes and consumers can switch on it.
export const SERIALIZED_IR_VERSION = 1;
export type SerializedIRVersion = typeof SERIALIZED_IR_VERSION;

// ---------------------------------------------------------------------------
// Visual graph layer
// ---------------------------------------------------------------------------

export const DIRECTION = {
  RL: "RL",
  LR: "LR",
  TB: "TB",
  BT: "BT",
} as const;
export type Direction = (typeof DIRECTION)[keyof typeof DIRECTION];

export const NODE_KIND = {
  Variable: "Variable",
  FunctionName: "FunctionName",
  ClassName: "ClassName",
  Parameter: "Parameter",
  CatchClause: "CatchClause",
  ImportBinding: "ImportBinding",
  ImplicitGlobalVariable: "ImplicitGlobalVariable",
  WriteOp: "WriteOp",
  ReturnUse: "ReturnUse",
  ModuleSink: "ModuleSink",
  ModuleSource: "ModuleSource",
  ImportIntermediate: "ImportIntermediate",
} as const;
export type NodeKind = (typeof NODE_KIND)[keyof typeof NODE_KIND];

export const SUBGRAPH_KIND = {
  Function: "function",
  Switch: "switch",
  Case: "case",
  If: "if",
  Else: "else",
  IfElseContainer: "if-else-container",
  Try: "try",
  Catch: "catch",
  Finally: "finally",
  For: "for",
  Return: "return",
} as const;
export type SubgraphKind = (typeof SUBGRAPH_KIND)[keyof typeof SUBGRAPH_KIND];

export const VISUAL_ELEMENT_TYPE = {
  Node: "node",
  Subgraph: "subgraph",
} as const;
export type VisualElementType =
  (typeof VISUAL_ELEMENT_TYPE)[keyof typeof VISUAL_ELEMENT_TYPE];

export const BOUNDARY_EDGE_DIRECTION = {
  Out: "out",
  In: "in",
} as const;
export type BoundaryEdgeDirection =
  (typeof BOUNDARY_EDGE_DIRECTION)[keyof typeof BOUNDARY_EDGE_DIRECTION];

// ---------------------------------------------------------------------------
// CLI layer
// ---------------------------------------------------------------------------

// Source language tag, shared between the IR (`Language`) and the CLI
// (`CliLanguage`). They alias the same value set so callers on either
// side don't import across domains.
export const LANGUAGE = {
  Ts: "ts",
  Tsx: "tsx",
  Js: "js",
  Jsx: "jsx",
} as const;
export type Language = (typeof LANGUAGE)[keyof typeof LANGUAGE];
export type CliLanguage = Language;

export const CLI_MERMAID_RENDERER = {
  Dagre: "dagre",
  Elk: "elk",
} as const;
export type CliMermaidRenderer =
  (typeof CLI_MERMAID_RENDERER)[keyof typeof CLI_MERMAID_RENDERER];

export const ROOT_QUERY_KIND = {
  Line: "line",
  LineName: "line-name",
  Range: "range",
  RangeName: "range-name",
  Name: "name",
} as const;
export type RootQueryKind =
  (typeof ROOT_QUERY_KIND)[keyof typeof ROOT_QUERY_KIND];
