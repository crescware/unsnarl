//! oxc-mirrored value sets.
//!
//! Each item in this crate is a value set whose membership and
//! spelling must match `oxc_ast` (or, transitively, ECMA strings
//! that `oxc_ast` echoes verbatim) value-for-value. Concentrating
//! that responsibility here keeps the IR contract crate
//! (`unsnarl-ir`) from silently absorbing oxc upgrades: if oxc
//! renames or adds a value, the change driver is an oxc upgrade
//! against this crate, not an IR contract redesign.
//!
//! Membership criterion: a type lives here iff (a) its values come
//! from oxc-emitted strings (directly, or via ECMA spellings that
//! oxc echoes), and (b) the value set is meaningful as a curated
//! Rust enum at type level (so downstream `match` can be
//! exhaustive). Items that are merely "a subset of `AstType`
//! checked at construction" do not need a re-declared enum here;
//! they can stay at the IR site as `AstType` plus `assert!`.
//!
//! Current contents:
//!
//! * [`AstType`]: the full `type` discriminator. Folds together
//!   parser-emitted variants and a tail of nodes `oxc_ast`
//!   declares but no public `parse` input can produce in practice
//!   (`Hashbang`, `TSJSDocUnknownType`, `V8IntrinsicExpression`);
//!   downstream serialization treats them identically.
//!   [`UnknownAstType`](AstType::UnknownAstType) is the sentinel
//!   for any value we don't recognise.
//! * [`VariableDeclarationKind`]: `var` / `let` / `const` read
//!   directly off `VariableDeclaration.kind`.
//! * [`PredicateContainerType`]: the seven predicate-owning
//!   statement types; a curated subset of `AstType` values exposed
//!   as its own enum because downstream code matches on it
//!   exhaustively.
//! * [`AssignOperator`]: the 16 assignment operators
//!   (`=`, `+=`, ..., `??=`) read off
//!   `AssignmentExpression.operator`.
//! * [`UpdateOperator`]: the 2 update operators (`++`, `--`) read
//!   off `UpdateExpression.operator`.
//!
//! Other crates (the IR, the boundary, the analyzer, ...) depend
//! on this crate rather than redeclaring these types.

use serde::Serialize;

pub mod as_ast_type;
pub mod assign_operator;
pub mod predicate_container_type;
pub mod skip_types;
pub mod update_operator;
pub mod variable_declaration_kind;

pub use as_ast_type::as_ast_type;
pub use assign_operator::AssignOperator;
pub use predicate_container_type::PredicateContainerType;
pub use skip_types::is_type_only_subtree;
pub use update_operator::UpdateOperator;
pub use variable_declaration_kind::VariableDeclarationKind;

pub const UNKNOWN_AST_TYPE: &str = "UnknownAstType";

include!(concat!(env!("OUT_DIR"), "/generated_parity.rs"));

#[derive(Clone, Serialize)]
pub enum AstType {
    AccessorProperty,
    ArrayExpression,
    ArrayPattern,
    ArrowFunctionExpression,
    AssignmentExpression,
    AssignmentPattern,
    AwaitExpression,
    BinaryExpression,
    BlockStatement,
    BreakStatement,
    CallExpression,
    CatchClause,
    ChainExpression,
    ClassBody,
    ClassDeclaration,
    ClassExpression,
    ConditionalExpression,
    ContinueStatement,
    DebuggerStatement,
    Decorator,
    DoWhileStatement,
    EmptyStatement,
    ExportAllDeclaration,
    ExportDefaultDeclaration,
    ExportNamedDeclaration,
    ExportSpecifier,
    ExpressionStatement,
    ForInStatement,
    ForOfStatement,
    ForStatement,
    FunctionDeclaration,
    FunctionExpression,
    Identifier,
    IfStatement,
    ImportAttribute,
    ImportDeclaration,
    ImportDefaultSpecifier,
    ImportExpression,
    ImportNamespaceSpecifier,
    ImportSpecifier,
    JSXAttribute,
    JSXClosingElement,
    JSXClosingFragment,
    JSXElement,
    JSXEmptyExpression,
    JSXExpressionContainer,
    JSXFragment,
    JSXIdentifier,
    JSXMemberExpression,
    JSXNamespacedName,
    JSXOpeningElement,
    JSXOpeningFragment,
    JSXSpreadAttribute,
    JSXSpreadChild,
    JSXText,
    LabeledStatement,
    Literal,
    LogicalExpression,
    MemberExpression,
    MetaProperty,
    MethodDefinition,
    NewExpression,
    ObjectExpression,
    ObjectPattern,
    ParenthesizedExpression,
    PrivateIdentifier,
    Program,
    Property,
    PropertyDefinition,
    RestElement,
    ReturnStatement,
    SequenceExpression,
    SpreadElement,
    StaticBlock,
    Super,
    SwitchCase,
    SwitchStatement,
    TaggedTemplateExpression,
    TemplateElement,
    TemplateLiteral,
    ThisExpression,
    ThrowStatement,
    TryStatement,
    TSAbstractAccessorProperty,
    TSAbstractMethodDefinition,
    TSAbstractPropertyDefinition,
    TSAnyKeyword,
    TSArrayType,
    TSAsExpression,
    TSBigIntKeyword,
    TSBooleanKeyword,
    TSCallSignatureDeclaration,
    TSClassImplements,
    TSConditionalType,
    TSConstructorType,
    TSConstructSignatureDeclaration,
    TSDeclareFunction,
    TSEmptyBodyFunctionExpression,
    TSEnumBody,
    TSEnumDeclaration,
    TSEnumMember,
    TSExportAssignment,
    TSExternalModuleReference,
    TSFunctionType,
    TSImportEqualsDeclaration,
    TSImportType,
    TSIndexedAccessType,
    TSIndexSignature,
    TSInferType,
    TSInstantiationExpression,
    TSInterfaceBody,
    TSInterfaceDeclaration,
    TSInterfaceHeritage,
    TSIntersectionType,
    TSIntrinsicKeyword,
    TSJSDocNonNullableType,
    TSJSDocNullableType,
    TSLiteralType,
    TSMappedType,
    TSMethodSignature,
    TSModuleBlock,
    TSModuleDeclaration,
    TSNamedTupleMember,
    TSNamespaceExportDeclaration,
    TSNeverKeyword,
    TSNonNullExpression,
    TSNullKeyword,
    TSNumberKeyword,
    TSObjectKeyword,
    TSOptionalType,
    TSParameterProperty,
    TSParenthesizedType,
    TSPropertySignature,
    TSQualifiedName,
    TSRestType,
    TSSatisfiesExpression,
    TSStringKeyword,
    TSSymbolKeyword,
    TSTemplateLiteralType,
    TSThisType,
    TSTupleType,
    TSTypeAliasDeclaration,
    TSTypeAnnotation,
    TSTypeAssertion,
    TSTypeLiteral,
    TSTypeOperator,
    TSTypeParameter,
    TSTypeParameterDeclaration,
    TSTypeParameterInstantiation,
    TSTypePredicate,
    TSTypeQuery,
    TSTypeReference,
    TSUndefinedKeyword,
    TSUnionType,
    TSUnknownKeyword,
    TSVoidKeyword,
    UnaryExpression,
    UpdateExpression,
    VariableDeclaration,
    VariableDeclarator,
    WhileStatement,
    WithStatement,
    YieldExpression,
    // Declared by `oxc_ast` but unreachable through any public
    // `parse` -> walker descent: kept here so the discriminator is
    // total over the upstream surface.
    Hashbang,
    TSJSDocUnknownType,
    V8IntrinsicExpression,
    // Internal sentinel for parser strings we don't recognize.
    #[serde(rename = "UnknownAstType")]
    UnknownAstType,
}

#[cfg(test)]
#[path = "ast_type_parity_test.rs"]
mod ast_type_parity_test;
