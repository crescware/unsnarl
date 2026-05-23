//! Sibling tests for `definition_mapping.rs`.
//!
//! Each test parses a small source string, runs `SemanticBuilder`
//! followed by [`super::super::scope_mapping::build_scopes`],
//! [`super::super::variable_mapping::build_variables`],
//! [`super::super::reference_mapping::build_references`], and finally
//! [`super::build_definitions`], then asserts properties of the
//! resulting definitions table and the cross-link onto
//! `VariableData::defs`. Characterization-style: pins the per-anchor
//! classification (six `DefinitionType` variants) plus the extras
//! (`init` / `declaration_kind` / `import_source` / `imported_name`)
//! that the IR serializer reads.

use oxc_allocator::Allocator;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::ids::{DefinitionId, ScopeId, VariableId};
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind as IrVariableDeclarationKind};

use crate::oxc_semantic_adapter::reference_mapping::build_references;
use crate::oxc_semantic_adapter::scope_mapping::build_scopes;
use crate::oxc_semantic_adapter::variable_mapping::build_variables;
use crate::parser::{OxcParser, ParseOptions, SourceType};

use super::build_definitions;

struct Built {
    scopes: IndexVec<ScopeId, ScopeData>,
    variables: IndexVec<VariableId, VariableData>,
    definitions: IndexVec<DefinitionId, DefinitionData>,
}

fn with_arena(code: &str, language: Language, source_type: SourceType, body: impl FnOnce(&Built)) {
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: format!(
                    "input.{}",
                    match language {
                        Language::Js => "js",
                        Language::Jsx => "jsx",
                        Language::Ts => "ts",
                        Language::Tsx => "tsx",
                    }
                ),
                source_type,
            },
        )
        .expect("test source must parse cleanly");
    let ret = SemanticBuilder::new().build(&parsed.program);
    let scope_mapping = build_scopes(&ret.semantic, source_type, language);
    let mut scopes = scope_mapping.scopes;
    let translation = scope_mapping.translation;
    let var_result = build_variables(&ret.semantic, &mut scopes, &translation);
    let mut variables = var_result.variables;
    let symbol_to_variable = var_result.symbol_to_variable;
    let synthetic_unresolved = var_result.synthetic_unresolved;
    let mut definitions: IndexVec<DefinitionId, DefinitionData> = IndexVec::new();
    let _references = build_references(
        &ret.semantic,
        &mut scopes,
        &mut variables,
        &mut definitions,
        &symbol_to_variable,
        &translation,
        &synthetic_unresolved,
    );
    build_definitions(
        &ret.semantic,
        &mut variables,
        &mut definitions,
        &symbol_to_variable,
    );
    body(&Built {
        scopes,
        variables,
        definitions,
    });
}

fn root() -> ScopeId {
    ScopeId::from_usize(0)
}

fn find_var<'a>(b: &'a Built, scope: ScopeId, name: &str) -> Option<&'a VariableData> {
    b.scopes[scope]
        .set()
        .get(name)
        .copied()
        .map(|id| &b.variables[id])
}

#[test]
fn empty_script_has_no_definitions() {
    with_arena("", Language::Js, SourceType::Script, |b| {
        assert!(b.definitions.is_empty());
    });
}

#[test]
fn let_declaration_emits_variable_definition_with_init_and_kind() {
    with_arena("let x = 1;", Language::Js, SourceType::Module, |b| {
        let var = find_var(b, root(), "x").expect("x");
        assert_eq!(var.defs.len(), 1);
        let def = &b.definitions[var.defs[0]];
        assert!(matches!(def.r#type, DefinitionType::Variable));
        assert_eq!(def.name.name(), "x");
        assert!(matches!(def.name.r#type, AstType::Identifier));
        assert!(matches!(def.node.r#type, AstType::VariableDeclarator));
        assert!(matches!(
            def.parent.as_ref().expect("parent").r#type,
            AstType::VariableDeclaration,
        ));
        assert!(matches!(
            def.declaration_kind,
            Some(IrVariableDeclarationKind::Let),
        ));
        let init = def.init.as_ref().expect("init present for `let x = 1`");
        assert!(matches!(init.r#type, AstType::Literal));
        assert!(def.import_source.is_none());
        assert!(def.imported_name.is_none());
    });
}

#[test]
fn const_declaration_carries_const_kind_and_no_init_when_uninitialised() {
    // `const` always requires an initializer in valid ESM, so use `let`
    // for the "no init" case and `const` for the init case to keep one
    // assertion per axis.
    with_arena(
        "let x; const y = 2;",
        Language::Js,
        SourceType::Module,
        |b| {
            let x = find_var(b, root(), "x").expect("x");
            let x_def = &b.definitions[x.defs[0]];
            assert!(matches!(
                x_def.declaration_kind,
                Some(IrVariableDeclarationKind::Let),
            ));
            assert!(
                x_def.init.is_none(),
                "let x; has no initializer, def.init must be None",
            );
            let y = find_var(b, root(), "y").expect("y");
            let y_def = &b.definitions[y.defs[0]];
            assert!(matches!(
                y_def.declaration_kind,
                Some(IrVariableDeclarationKind::Const),
            ));
            assert!(y_def.init.is_some());
        },
    );
}

#[test]
fn var_redeclaration_yields_two_definitions_on_one_variable() {
    with_arena("var x; var x;", Language::Js, SourceType::Script, |b| {
        let var = find_var(b, root(), "x").expect("x");
        assert_eq!(
            var.defs.len(),
            2,
            "`var x; var x;` collapses to one Variable with two Definitions",
        );
        for &def_id in &var.defs {
            let def = &b.definitions[def_id];
            assert!(matches!(def.r#type, DefinitionType::Variable));
            assert!(matches!(
                def.declaration_kind,
                Some(IrVariableDeclarationKind::Var),
            ));
        }
    });
}

#[test]
fn function_declaration_emits_function_name_definition() {
    with_arena(
        "function f(a) { return a; }",
        Language::Js,
        SourceType::Script,
        |b| {
            let f = find_var(b, root(), "f").expect("f");
            assert_eq!(f.defs.len(), 1);
            let def = &b.definitions[f.defs[0]];
            assert!(matches!(def.r#type, DefinitionType::FunctionName));
            assert_eq!(def.name.name(), "f");
            assert!(matches!(def.node.r#type, AstType::FunctionDeclaration));
            assert!(def.parent.is_none());
            assert!(def.declaration_kind.is_none());
            assert!(def.init.is_none());
        },
    );
}

#[test]
fn class_declaration_emits_class_name_definition() {
    with_arena("class C {}", Language::Js, SourceType::Module, |b| {
        let c = find_var(b, root(), "C").expect("C");
        // `oxc_semantic` keeps one ClassName binding in the outer scope.
        // Eslint-scope's inner-class-binding synthesis is a deferred
        // divergence (see definition_mapping.rs module header).
        let def = &b.definitions[c.defs[0]];
        assert!(matches!(def.r#type, DefinitionType::ClassName));
        assert!(matches!(def.node.r#type, AstType::ClassDeclaration));
        assert!(def.parent.is_none());
    });
}

#[test]
fn parameter_definition_uses_enclosing_function_as_node() {
    with_arena(
        "function f(a, b) { return a + b; }",
        Language::Js,
        SourceType::Script,
        |b| {
            let f_scope = b.scopes[root()].child_scopes[0];
            for name in ["a", "b"] {
                let v = find_var(b, f_scope, name).expect(name);
                let def = &b.definitions[v.defs[0]];
                assert!(matches!(def.r#type, DefinitionType::Parameter));
                assert!(
                    matches!(def.node.r#type, AstType::FunctionDeclaration),
                    "parameter def_node must be the enclosing function",
                );
                assert!(def.parent.is_none());
            }
        },
    );
}

#[test]
fn arrow_parameter_uses_arrow_function_expression_as_node() {
    with_arena(
        "const f = (a) => a;",
        Language::Js,
        SourceType::Module,
        |b| {
            let arrow_scope = b.scopes[root()].child_scopes[0];
            let a = find_var(b, arrow_scope, "a").expect("a");
            let def = &b.definitions[a.defs[0]];
            assert!(matches!(def.r#type, DefinitionType::Parameter));
            assert!(matches!(def.node.r#type, AstType::ArrowFunctionExpression));
        },
    );
}

#[test]
fn rest_parameter_emits_parameter_definition() {
    with_arena(
        "function f(...rest) { return rest; }",
        Language::Js,
        SourceType::Script,
        |b| {
            let f_scope = b.scopes[root()].child_scopes[0];
            let rest = find_var(b, f_scope, "rest").expect("rest");
            let def = &b.definitions[rest.defs[0]];
            assert!(matches!(def.r#type, DefinitionType::Parameter));
            assert!(matches!(def.node.r#type, AstType::FunctionDeclaration));
        },
    );
}

#[test]
fn catch_clause_parameter_emits_catch_clause_definition() {
    with_arena(
        "try {} catch (e) { e; }",
        Language::Js,
        SourceType::Script,
        |b| {
            // Find the `e` Variable wherever it lives. `oxc_semantic`
            // emits an extra empty-flagged BlockStatement scope for
            // the catch body in addition to the CatchClause scope, and
            // the `e` binding may land on either depending on
            // `scope_descendants_from_root`'s walk; we look it up by
            // identity rather than by scope.
            let e_var = b
                .variables
                .iter()
                .find(|v| v.name() == "e")
                .expect("`e` Variable must exist");
            let def = &b.definitions[e_var.defs[0]];
            assert!(matches!(def.r#type, DefinitionType::CatchClause));
            assert!(matches!(def.node.r#type, AstType::CatchClause));
            assert!(def.parent.is_none());
        },
    );
}

#[test]
fn named_import_emits_import_binding_with_source_and_imported_name() {
    with_arena(
        "import { foo as bar } from 'mod';",
        Language::Js,
        SourceType::Module,
        |b| {
            let bar = find_var(b, root(), "bar").expect("bar");
            let def = &b.definitions[bar.defs[0]];
            assert!(matches!(def.r#type, DefinitionType::ImportBinding));
            assert!(matches!(def.node.r#type, AstType::ImportSpecifier));
            assert!(matches!(
                def.parent.as_ref().expect("parent").r#type,
                AstType::ImportDeclaration,
            ));
            assert_eq!(def.import_source.as_deref(), Some("mod"));
            assert_eq!(def.imported_name.as_deref(), Some("foo"));
        },
    );
}

#[test]
fn default_import_emits_import_binding_without_imported_name() {
    with_arena(
        "import x from 'mod';",
        Language::Js,
        SourceType::Module,
        |b| {
            let x = find_var(b, root(), "x").expect("x");
            let def = &b.definitions[x.defs[0]];
            assert!(matches!(def.r#type, DefinitionType::ImportBinding));
            assert!(matches!(def.node.r#type, AstType::ImportDefaultSpecifier));
            assert_eq!(def.import_source.as_deref(), Some("mod"));
            assert!(
                def.imported_name.is_none(),
                "default import has no imported_name",
            );
        },
    );
}

#[test]
fn namespace_import_emits_import_binding_without_imported_name() {
    with_arena(
        "import * as ns from 'mod';",
        Language::Js,
        SourceType::Module,
        |b| {
            let ns = find_var(b, root(), "ns").expect("ns");
            let def = &b.definitions[ns.defs[0]];
            assert!(matches!(def.r#type, DefinitionType::ImportBinding));
            assert!(matches!(def.node.r#type, AstType::ImportNamespaceSpecifier));
            assert_eq!(def.import_source.as_deref(), Some("mod"));
            assert!(def.imported_name.is_none());
        },
    );
}

#[test]
fn implicit_global_definition_from_reference_mapping_survives_definition_mapping() {
    // `reference_mapping` emits ImplicitGlobalVariable definitions
    // before `definition_mapping` runs. `definition_mapping` must
    // leave those untouched and append the six other variants.
    with_arena(
        "function f() { return missing; } let y = 1;",
        Language::Js,
        SourceType::Script,
        |b| {
            // 1 implicit global (`missing`) + 1 Variable (y) + 1 FunctionName (f).
            // (`arguments` synthesises a Variable but no Definition.)
            assert!(b.definitions.iter().any(|d| matches!(
                d.r#type,
                DefinitionType::ImplicitGlobalVariable
            ) && d.name.name() == "missing"));
            assert!(b
                .definitions
                .iter()
                .any(|d| matches!(d.r#type, DefinitionType::Variable) && d.name.name() == "y"));
            assert!(
                b.definitions
                    .iter()
                    .any(|d| matches!(d.r#type, DefinitionType::FunctionName,)
                        && d.name.name() == "f")
            );
        },
    );
}
