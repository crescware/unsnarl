// AST layer (oxc-parser node `type` discriminator strings).

import { picklist, type InferOutput } from "valibot";

// AST_TYPE mirrors the `type` discriminator strings emitted by
// oxc-parser (@oxc-project/types). The intent is exhaustive coverage
// of oxc's vocabulary, plus the `UnknownAstType` sentinel for any
// value the parser emits that this enumeration has not yet caught up
// to. `asAstType()` is the application-layer normalizer that maps a
// raw oxc string to AstType, collapsing unrecognized inputs into
// UnknownAstType so a parser upgrade adding a new node kind degrades
// into a known fallback rather than crashing astType$ at the schema
// boundary.
//
// Codebases that dispatch on AST_TYPE values (visual-graph builders,
// analyzer producers, ...) do not need to handle every entry;
// unmatched values simply do not participate in those dispatches.
//
// BooleanLiteral / NullLiteral / NumericLiteral / StringLiteral are
// retained as legacy dispatch entries in format-case-test.ts even
// though oxc-parser emits the unified `Literal` type instead. They
// are kept so the dispatch keeps compiling; oxc input will not flow
// into those branches.
export const AST_TYPE = {
  AccessorProperty: "AccessorProperty",
  ArrayExpression: "ArrayExpression",
  ArrayPattern: "ArrayPattern",
  ArrowFunctionExpression: "ArrowFunctionExpression",
  AssignmentExpression: "AssignmentExpression",
  AssignmentPattern: "AssignmentPattern",
  AwaitExpression: "AwaitExpression",
  BinaryExpression: "BinaryExpression",
  BlockStatement: "BlockStatement",
  BooleanLiteral: "BooleanLiteral",
  BreakStatement: "BreakStatement",
  CallExpression: "CallExpression",
  CatchClause: "CatchClause",
  ChainExpression: "ChainExpression",
  ClassBody: "ClassBody",
  ClassDeclaration: "ClassDeclaration",
  ClassExpression: "ClassExpression",
  ConditionalExpression: "ConditionalExpression",
  ContinueStatement: "ContinueStatement",
  DebuggerStatement: "DebuggerStatement",
  Decorator: "Decorator",
  DoWhileStatement: "DoWhileStatement",
  EmptyStatement: "EmptyStatement",
  ExportAllDeclaration: "ExportAllDeclaration",
  ExportDefaultDeclaration: "ExportDefaultDeclaration",
  ExportNamedDeclaration: "ExportNamedDeclaration",
  ExportSpecifier: "ExportSpecifier",
  ExpressionStatement: "ExpressionStatement",
  ForInStatement: "ForInStatement",
  ForOfStatement: "ForOfStatement",
  ForStatement: "ForStatement",
  FunctionDeclaration: "FunctionDeclaration",
  FunctionExpression: "FunctionExpression",
  Hashbang: "Hashbang",
  Identifier: "Identifier",
  IfStatement: "IfStatement",
  ImportAttribute: "ImportAttribute",
  ImportDeclaration: "ImportDeclaration",
  ImportDefaultSpecifier: "ImportDefaultSpecifier",
  ImportExpression: "ImportExpression",
  ImportNamespaceSpecifier: "ImportNamespaceSpecifier",
  ImportSpecifier: "ImportSpecifier",
  JSXAttribute: "JSXAttribute",
  JSXClosingElement: "JSXClosingElement",
  JSXClosingFragment: "JSXClosingFragment",
  JSXElement: "JSXElement",
  JSXEmptyExpression: "JSXEmptyExpression",
  JSXExpressionContainer: "JSXExpressionContainer",
  JSXFragment: "JSXFragment",
  JSXIdentifier: "JSXIdentifier",
  JSXMemberExpression: "JSXMemberExpression",
  JSXNamespacedName: "JSXNamespacedName",
  JSXOpeningElement: "JSXOpeningElement",
  JSXOpeningFragment: "JSXOpeningFragment",
  JSXSpreadAttribute: "JSXSpreadAttribute",
  JSXSpreadChild: "JSXSpreadChild",
  JSXText: "JSXText",
  LabeledStatement: "LabeledStatement",
  Literal: "Literal",
  LogicalExpression: "LogicalExpression",
  MemberExpression: "MemberExpression",
  MetaProperty: "MetaProperty",
  MethodDefinition: "MethodDefinition",
  NewExpression: "NewExpression",
  NullLiteral: "NullLiteral",
  NumericLiteral: "NumericLiteral",
  ObjectExpression: "ObjectExpression",
  ObjectPattern: "ObjectPattern",
  ParenthesizedExpression: "ParenthesizedExpression",
  PrivateIdentifier: "PrivateIdentifier",
  Program: "Program",
  Property: "Property",
  PropertyDefinition: "PropertyDefinition",
  RestElement: "RestElement",
  ReturnStatement: "ReturnStatement",
  SequenceExpression: "SequenceExpression",
  SpreadElement: "SpreadElement",
  StaticBlock: "StaticBlock",
  StringLiteral: "StringLiteral",
  Super: "Super",
  SwitchCase: "SwitchCase",
  SwitchStatement: "SwitchStatement",
  TaggedTemplateExpression: "TaggedTemplateExpression",
  TemplateElement: "TemplateElement",
  TemplateLiteral: "TemplateLiteral",
  ThisExpression: "ThisExpression",
  ThrowStatement: "ThrowStatement",
  TryStatement: "TryStatement",
  TSAbstractAccessorProperty: "TSAbstractAccessorProperty",
  TSAbstractMethodDefinition: "TSAbstractMethodDefinition",
  TSAbstractPropertyDefinition: "TSAbstractPropertyDefinition",
  TSAnyKeyword: "TSAnyKeyword",
  TSArrayType: "TSArrayType",
  TSAsExpression: "TSAsExpression",
  TSBigIntKeyword: "TSBigIntKeyword",
  TSBooleanKeyword: "TSBooleanKeyword",
  TSCallSignatureDeclaration: "TSCallSignatureDeclaration",
  TSClassImplements: "TSClassImplements",
  TSConditionalType: "TSConditionalType",
  TSConstructorType: "TSConstructorType",
  TSConstructSignatureDeclaration: "TSConstructSignatureDeclaration",
  TSDeclareFunction: "TSDeclareFunction",
  TSEmptyBodyFunctionExpression: "TSEmptyBodyFunctionExpression",
  TSEnumBody: "TSEnumBody",
  TSEnumDeclaration: "TSEnumDeclaration",
  TSEnumMember: "TSEnumMember",
  TSExportAssignment: "TSExportAssignment",
  TSExternalModuleReference: "TSExternalModuleReference",
  TSFunctionType: "TSFunctionType",
  TSImportEqualsDeclaration: "TSImportEqualsDeclaration",
  TSImportType: "TSImportType",
  TSIndexedAccessType: "TSIndexedAccessType",
  TSIndexSignature: "TSIndexSignature",
  TSInferType: "TSInferType",
  TSInstantiationExpression: "TSInstantiationExpression",
  TSInterfaceBody: "TSInterfaceBody",
  TSInterfaceDeclaration: "TSInterfaceDeclaration",
  TSInterfaceHeritage: "TSInterfaceHeritage",
  TSIntersectionType: "TSIntersectionType",
  TSIntrinsicKeyword: "TSIntrinsicKeyword",
  TSJSDocNonNullableType: "TSJSDocNonNullableType",
  TSJSDocNullableType: "TSJSDocNullableType",
  TSJSDocUnknownType: "TSJSDocUnknownType",
  TSLiteralType: "TSLiteralType",
  TSMappedType: "TSMappedType",
  TSMethodSignature: "TSMethodSignature",
  TSModuleBlock: "TSModuleBlock",
  TSModuleDeclaration: "TSModuleDeclaration",
  TSNamedTupleMember: "TSNamedTupleMember",
  TSNamespaceExportDeclaration: "TSNamespaceExportDeclaration",
  TSNeverKeyword: "TSNeverKeyword",
  TSNonNullExpression: "TSNonNullExpression",
  TSNullKeyword: "TSNullKeyword",
  TSNumberKeyword: "TSNumberKeyword",
  TSObjectKeyword: "TSObjectKeyword",
  TSOptionalType: "TSOptionalType",
  TSParameterProperty: "TSParameterProperty",
  TSParenthesizedType: "TSParenthesizedType",
  TSPropertySignature: "TSPropertySignature",
  TSQualifiedName: "TSQualifiedName",
  TSRestType: "TSRestType",
  TSSatisfiesExpression: "TSSatisfiesExpression",
  TSStringKeyword: "TSStringKeyword",
  TSSymbolKeyword: "TSSymbolKeyword",
  TSTemplateLiteralType: "TSTemplateLiteralType",
  TSThisType: "TSThisType",
  TSTupleType: "TSTupleType",
  TSTypeAliasDeclaration: "TSTypeAliasDeclaration",
  TSTypeAnnotation: "TSTypeAnnotation",
  TSTypeAssertion: "TSTypeAssertion",
  TSTypeLiteral: "TSTypeLiteral",
  TSTypeOperator: "TSTypeOperator",
  TSTypeParameter: "TSTypeParameter",
  TSTypeParameterDeclaration: "TSTypeParameterDeclaration",
  TSTypeParameterInstantiation: "TSTypeParameterInstantiation",
  TSTypePredicate: "TSTypePredicate",
  TSTypeQuery: "TSTypeQuery",
  TSTypeReference: "TSTypeReference",
  TSUndefinedKeyword: "TSUndefinedKeyword",
  TSUnionType: "TSUnionType",
  TSUnknownKeyword: "TSUnknownKeyword",
  TSVoidKeyword: "TSVoidKeyword",
  UnaryExpression: "UnaryExpression",
  UpdateExpression: "UpdateExpression",
  V8IntrinsicExpression: "V8IntrinsicExpression",
  VariableDeclaration: "VariableDeclaration",
  VariableDeclarator: "VariableDeclarator",
  WhileStatement: "WhileStatement",
  WithStatement: "WithStatement",
  YieldExpression: "YieldExpression",

  // Sentinel for AST type strings the parser emits that are not (yet)
  // enumerated above. `asAstType()` collapses any unknown string to
  // this value so downstream code can detect "type we don't know
  // about" instead of seeing a verbatim parser string.
  UnknownAstType: "UnknownAstType",
} as const;

const ALL_AST_TYPES = Object.values(
  AST_TYPE,
) as readonly (typeof AST_TYPE)[keyof typeof AST_TYPE][];

const KNOWN_AST_TYPES: ReadonlySet<string> = new Set(ALL_AST_TYPES);

export const astType$ = picklist(ALL_AST_TYPES);

export type AstType = InferOutput<typeof astType$>;

// Application-layer normalizer. Call this on a raw oxc-parser `type`
// string before handing it to code that expects an AstType (e.g.
// serializer entry points that feed astType$). The schema itself
// stays a pure picklist; the boundary owns the "unknown -> sentinel"
// decision.
export function asAstType(raw: string): AstType {
  return KNOWN_AST_TYPES.has(raw) ? (raw as AstType) : AST_TYPE.UnknownAstType;
}
