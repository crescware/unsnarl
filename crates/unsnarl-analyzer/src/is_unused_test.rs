use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;
use oxc_span::Span;

use unsnarl_ir::ids::{DefinitionId, VariableId};
use unsnarl_ir::IrArena;

use crate::testing::parse_and_analyze_ts;

use super::is_unused;

fn find_var(arena: &IrArena, name: &str) -> VariableId {
    arena
        .variables
        .iter_enumerated()
        .find(|(_, v)| v.name() == name)
        .map(|(id, _)| id)
        .unwrap_or_else(|| panic!("variable {name} not found"))
}

/// Body-span lookup that treats every def whose node is a
/// `VariableDeclarator` as having no functionlike init. Sufficient
/// for cases that exercise the read/write logic without
/// destructuring through a functionlike init.
fn no_init_bodies(_: DefinitionId) -> Option<Span> {
    None
}

#[test]
fn returns_true_when_no_references_exist() {
    // `let a;` declares `a` with no init (no Write either): no references.
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "let a;");
    let a = find_var(&result.arena, "a");
    assert!(is_unused(a, &result.arena, no_init_bodies));
}

#[test]
fn returns_true_when_only_init_write_reference_exists() {
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "const a = 1;");
    let a = find_var(&result.arena, "a");
    assert!(is_unused(a, &result.arena, no_init_bodies));
}

#[test]
fn returns_false_when_a_non_init_read_reference_is_present() {
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "const a = 1; const b = a;");
    let a = find_var(&result.arena, "a");
    assert!(!is_unused(a, &result.arena, no_init_bodies));
}

#[test]
fn returns_true_when_only_write_only_reassignments_are_present() {
    // After init, `x = 2;` is a Write-only reference. No Read → considered unused.
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "let x = 1; x = 2;");
    let x = find_var(&result.arena, "x");
    assert!(is_unused(x, &result.arena, no_init_bodies));
}

#[test]
fn returns_false_when_a_read_write_reference_is_present() {
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "let x = 1; x += 1;");
    let x = find_var(&result.arena, "x");
    assert!(!is_unused(x, &result.arena, no_init_bodies));
}

#[test]
fn function_self_recursion_does_not_count_as_external_read() {
    // `foo` reads itself from inside its own body — that's a self-internal
    // read, which `is_unused` should NOT treat as usage. (#68)
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "function foo() { foo(); }");
    let foo = find_var(&result.arena, "foo");
    // For function defs the body span equals the def's own AstNode span,
    // which `is_unused` derives without consulting the lookup; passing
    // `no_init_bodies` is therefore fine.
    assert!(is_unused(foo, &result.arena, no_init_bodies));
}

#[test]
fn external_read_marks_function_used() {
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "function foo() { return 1; } foo();");
    let foo = find_var(&result.arena, "foo");
    assert!(!is_unused(foo, &result.arena, no_init_bodies));
}

#[test]
fn mutual_recursion_keeps_both_used() {
    // `f` and `g` reference each other. Each read originates from the
    // other's body, which counts as external for the readee.
    let alloc = Allocator::default();
    let (_, result) = parse_and_analyze_ts(&alloc, "function f() { g(); } function g() { f(); }");
    let f = find_var(&result.arena, "f");
    let g = find_var(&result.arena, "g");
    assert!(!is_unused(f, &result.arena, no_init_bodies));
    assert!(!is_unused(g, &result.arena, no_init_bodies));
}

#[test]
fn arrow_initializer_self_recursion_via_body_span_lookup() {
    // `const a = () => a;` — the only Read of `a` lives inside the arrow
    // expression body. With a body-span lookup that resolves the
    // VariableDeclarator def to the arrow's span, `is_unused` should
    // treat the read as self-internal and return true.
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "const a = () => a;");
    let a = find_var(&result.arena, "a");
    // Locate the arrow's span from the AST: walk to the VariableDeclarator
    // and read its init's span.
    let arrow_span = {
        let decl = match program.body.first().unwrap() {
            Statement::VariableDeclaration(d) => d,
            _ => unreachable!(),
        };
        let init = decl.declarations[0].init.as_ref().expect("init");
        // The init is the ArrowFunctionExpression.
        match init {
            oxc_ast::ast::Expression::ArrowFunctionExpression(a) => a.span,
            _ => unreachable!(),
        }
    };
    // Build a lookup keyed by the (single) def of `a`.
    let a_def = result.arena.variables[a].defs[0];
    let lookup = move |id: DefinitionId| -> Option<Span> {
        if id == a_def {
            Some(arrow_span)
        } else {
            None
        }
    };
    assert!(is_unused(a, &result.arena, lookup));
}

#[test]
fn external_read_through_variable_declarator_init_marks_used() {
    // `const a = () => null; a();` — the read of `a` sits outside the
    // arrow body, so `is_unused` must return false.
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "const a = () => null; a();");
    let a = find_var(&result.arena, "a");
    let arrow_span = {
        let decl = match program.body.first().unwrap() {
            Statement::VariableDeclaration(d) => d,
            _ => unreachable!(),
        };
        let init = decl.declarations[0].init.as_ref().expect("init");
        match init {
            oxc_ast::ast::Expression::ArrowFunctionExpression(a) => a.span,
            _ => unreachable!(),
        }
    };
    let a_def = result.arena.variables[a].defs[0];
    let lookup = move |id: DefinitionId| -> Option<Span> {
        if id == a_def {
            Some(arrow_span)
        } else {
            None
        }
    };
    assert!(!is_unused(a, &result.arena, lookup));
}
