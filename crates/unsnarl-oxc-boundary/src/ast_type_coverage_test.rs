use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use oxc_allocator::Allocator;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::Language;

use crate::materialise::ast_type_of;
use crate::parser::{default_source_type_for, OxcParser, ParseOptions, SourceType};

fn workspace_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("workspace root is two levels above crate dir")
        .to_path_buf()
}

fn fixtures_root() -> PathBuf {
    workspace_root().join("integration/fixtures")
}

fn language_from_ext(ext: &str) -> Option<Language> {
    match ext {
        "ts" => Some(Language::Ts),
        "tsx" => Some(Language::Tsx),
        "jsx" => Some(Language::Jsx),
        "js" | "mjs" | "cjs" => Some(Language::Js),
        _ => None,
    }
}

fn source_type_from_path(path: &str, language: Language) -> SourceType {
    if path.ends_with(".mjs") {
        return SourceType::Module;
    }
    if path.ends_with(".cjs") {
        return SourceType::Script;
    }
    default_source_type_for(language)
}

struct FixtureInput {
    language: Language,
    source_path: String,
    code: String,
}

fn find_fixture_inputs(root: &Path) -> Vec<FixtureInput> {
    let mut results = Vec::new();
    walk_dir(root, root, &mut results);
    results
}

fn walk_dir(base: &Path, dir: &Path, results: &mut Vec<FixtureInput>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_dir(base, &path, results);
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let Some(ext) = name.strip_prefix("input.") else {
            continue;
        };
        let Some(language) = language_from_ext(ext) else {
            continue;
        };
        let Ok(code) = fs::read_to_string(&path) else {
            continue;
        };
        let source_path = path
            .strip_prefix(base)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        results.push(FixtureInput {
            language,
            source_path,
            code,
        });
    }
}

// AstType variants that are unreachable through the `semantic.nodes()` +
// `ast_type_of()` collection path used by this test. Each has a
// corresponding fixture under `app-behavior/ast-type-coverage/`, so the
// fixture corpus IS complete — these are excluded purely because the
// Rust boundary layer's materialisation collapses or remaps them.
//
// * Literal — oxc exposes BooleanLiteral / NumericLiteral / StringLiteral /
//   etc. as separate AstKind variants; `ast_type_of` falls back to
//   `as_ast_type(debug_name)` which produces UnknownAstType for those
//   names. The "Literal" spelling is only produced by
//   `ast_node_of_expression`, a different code path.
//
// * TSDeclareFunction — `ast_type_of` matches AstKind::Function and
//   maps FunctionType::TSDeclareFunction → AstType::FunctionDeclaration.
//
// * TSEmptyBodyFunctionExpression — same pattern; mapped to
//   AstType::FunctionExpression.
//
// * TSAbstractAccessorProperty — `ast_type_of` fallback uses
//   `kind.ty()` which returns "AccessorProperty" for all accessor
//   properties regardless of the abstract flag.
//
// * TSJSDocNonNullableType / TSJSDocNullableType — the oxc semantic
//   tree does not produce AstKind nodes for these TSJSDoc type-only
//   positions.
//
// * TSParameterProperty — oxc semantic does not expose a separate
//   AstKind for parameter properties (they appear as FormalParameter).
const WALKER_UNREACHABLE: &[&str] = &[
    "Literal",
    "TSAbstractAccessorProperty",
    "TSDeclareFunction",
    "TSEmptyBodyFunctionExpression",
    "TSJSDocNonNullableType",
    "TSJSDocNullableType",
    "TSParameterProperty",
];

// Reachable AST_TYPE entries — every AstType variant except:
// * the three parser-unreachable ones (Hashbang, TSJSDocUnknownType,
//   V8IntrinsicExpression),
// * the internal sentinel (UnknownAstType), and
// * the walker-unreachable ones listed above.
const REACHABLE_AST_TYPES: &[&str] = &[
    "AccessorProperty",
    "ArrayExpression",
    "ArrayPattern",
    "ArrowFunctionExpression",
    "AssignmentExpression",
    "AssignmentPattern",
    "AwaitExpression",
    "BinaryExpression",
    "BlockStatement",
    "BreakStatement",
    "CallExpression",
    "CatchClause",
    "ChainExpression",
    "ClassBody",
    "ClassDeclaration",
    "ClassExpression",
    "ConditionalExpression",
    "ContinueStatement",
    "DebuggerStatement",
    "Decorator",
    "DoWhileStatement",
    "EmptyStatement",
    "ExportAllDeclaration",
    "ExportDefaultDeclaration",
    "ExportNamedDeclaration",
    "ExportSpecifier",
    "ExpressionStatement",
    "ForInStatement",
    "ForOfStatement",
    "ForStatement",
    "FunctionDeclaration",
    "FunctionExpression",
    "Identifier",
    "IfStatement",
    "ImportAttribute",
    "ImportDeclaration",
    "ImportDefaultSpecifier",
    "ImportExpression",
    "ImportNamespaceSpecifier",
    "ImportSpecifier",
    "JSXAttribute",
    "JSXClosingElement",
    "JSXClosingFragment",
    "JSXElement",
    "JSXEmptyExpression",
    "JSXExpressionContainer",
    "JSXFragment",
    "JSXIdentifier",
    "JSXMemberExpression",
    "JSXNamespacedName",
    "JSXOpeningElement",
    "JSXOpeningFragment",
    "JSXSpreadAttribute",
    "JSXSpreadChild",
    "JSXText",
    "LabeledStatement",
    "LogicalExpression",
    "MemberExpression",
    "MetaProperty",
    "MethodDefinition",
    "NewExpression",
    "ObjectExpression",
    "ObjectPattern",
    "ParenthesizedExpression",
    "PrivateIdentifier",
    "Program",
    "Property",
    "PropertyDefinition",
    "RestElement",
    "ReturnStatement",
    "SequenceExpression",
    "SpreadElement",
    "StaticBlock",
    "Super",
    "SwitchCase",
    "SwitchStatement",
    "TaggedTemplateExpression",
    "TemplateElement",
    "TemplateLiteral",
    "ThisExpression",
    "ThrowStatement",
    "TryStatement",
    "TSAbstractMethodDefinition",
    "TSAbstractPropertyDefinition",
    "TSAnyKeyword",
    "TSArrayType",
    "TSAsExpression",
    "TSBigIntKeyword",
    "TSBooleanKeyword",
    "TSCallSignatureDeclaration",
    "TSClassImplements",
    "TSConditionalType",
    "TSConstructorType",
    "TSConstructSignatureDeclaration",
    "TSEnumBody",
    "TSEnumDeclaration",
    "TSEnumMember",
    "TSExportAssignment",
    "TSExternalModuleReference",
    "TSFunctionType",
    "TSImportEqualsDeclaration",
    "TSImportType",
    "TSIndexedAccessType",
    "TSIndexSignature",
    "TSInferType",
    "TSInstantiationExpression",
    "TSInterfaceBody",
    "TSInterfaceDeclaration",
    "TSInterfaceHeritage",
    "TSIntersectionType",
    "TSIntrinsicKeyword",
    "TSLiteralType",
    "TSMappedType",
    "TSMethodSignature",
    "TSModuleBlock",
    "TSModuleDeclaration",
    "TSNamedTupleMember",
    "TSNamespaceExportDeclaration",
    "TSNeverKeyword",
    "TSNonNullExpression",
    "TSNullKeyword",
    "TSNumberKeyword",
    "TSObjectKeyword",
    "TSOptionalType",
    "TSParenthesizedType",
    "TSPropertySignature",
    "TSQualifiedName",
    "TSRestType",
    "TSSatisfiesExpression",
    "TSStringKeyword",
    "TSSymbolKeyword",
    "TSTemplateLiteralType",
    "TSThisType",
    "TSTupleType",
    "TSTypeAliasDeclaration",
    "TSTypeAnnotation",
    "TSTypeAssertion",
    "TSTypeLiteral",
    "TSTypeOperator",
    "TSTypeParameter",
    "TSTypeParameterDeclaration",
    "TSTypeParameterInstantiation",
    "TSTypePredicate",
    "TSTypeQuery",
    "TSTypeReference",
    "TSUndefinedKeyword",
    "TSUnionType",
    "TSUnknownKeyword",
    "TSVoidKeyword",
    "UnaryExpression",
    "UpdateExpression",
    "VariableDeclaration",
    "VariableDeclarator",
    "WhileStatement",
    "WithStatement",
    "YieldExpression",
];

fn collect_ast_types_from_fixtures() -> BTreeSet<String> {
    let parser = OxcParser;
    let mut seen = BTreeSet::new();
    for fixture in find_fixture_inputs(&fixtures_root()) {
        let allocator = Allocator::default();
        let Ok(parsed) = parser.parse(
            &allocator,
            &fixture.code,
            &ParseOptions {
                language: fixture.language,
                source_path: fixture.source_path.clone(),
                source_type: source_type_from_path(&fixture.source_path, fixture.language),
            },
        ) else {
            continue;
        };
        let semantic = SemanticBuilder::new().build(&parsed.program).semantic;
        for node in semantic.nodes().iter() {
            let ast_type = ast_type_of(&node.kind());
            let s = serde_json::to_value(&ast_type)
                .expect("AstType must serialize")
                .as_str()
                .expect("AstType serializes to a string")
                .to_owned();
            seen.insert(s);
        }
    }
    seen
}

#[test]
fn every_reachable_ast_type_is_exercised_by_some_fixture() {
    let seen = collect_ast_types_from_fixtures();
    let expected: BTreeSet<&str> = REACHABLE_AST_TYPES.iter().copied().collect();
    let missing: Vec<&&str> = expected.iter().filter(|t| !seen.contains(**t)).collect();
    assert!(
        missing.is_empty(),
        "the following AST_TYPE entries have no fixture coverage: {missing:?}",
    );
}

#[test]
fn reachable_plus_excluded_covers_all_ast_type_variants() {
    let parser_unreachable: &[&str] = &[
        "Hashbang",
        "TSJSDocUnknownType",
        "V8IntrinsicExpression",
        "UnknownAstType",
    ];
    let mut all_accounted: BTreeSet<&str> = BTreeSet::new();
    for &s in REACHABLE_AST_TYPES {
        all_accounted.insert(s);
    }
    for &s in WALKER_UNREACHABLE {
        all_accounted.insert(s);
    }
    for &s in parser_unreachable {
        all_accounted.insert(s);
    }

    let all_variants: BTreeSet<&str> = unsnarl_oxc_parity::AST_TYPE_ENUM_VARIANTS
        .iter()
        .copied()
        .collect();
    let missing_from_lists: Vec<&&str> = all_variants
        .iter()
        .filter(|v| !all_accounted.contains(*v))
        .collect();
    let extra_in_lists: Vec<&&str> = all_accounted
        .iter()
        .filter(|v| !all_variants.contains(*v))
        .collect();
    assert!(
        missing_from_lists.is_empty() && extra_in_lists.is_empty(),
        "variant lists out of sync with AstType enum.\n  \
         missing from REACHABLE/WALKER_UNREACHABLE/parser_unreachable: {missing_from_lists:?}\n  \
         extra (not in AstType): {extra_in_lists:?}",
    );
}
