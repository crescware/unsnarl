use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement};

use crate::analyzer_fixtures::parse_and_analyze_ts;

use super::{all_binding_variables, assignment_target_variables};

#[test]
fn simple_identifier_binding_resolves_to_single_variable() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let a = 1;");
    let var_decl = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::VariableDeclaration(d) => d,
        _ => unreachable!(),
    };
    let declarator = var_decl
        .declarations
        .first()
        .expect("test variable declaration has at least one declarator");
    let vars = all_binding_variables(&declarator.id, result.global_scope, &result.arena);
    assert_eq!(vars.len(), 1);
    assert_eq!(result.arena.variables[vars[0]].name(), "a");
}

#[test]
fn array_pattern_binding_resolves_each_element() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let [a, b, c] = [1, 2, 3];");
    let var_decl = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::VariableDeclaration(d) => d,
        _ => unreachable!(),
    };
    let declarator = var_decl
        .declarations
        .first()
        .expect("test variable declaration has at least one declarator");
    let vars = all_binding_variables(&declarator.id, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["a", "b", "c"]);
}

#[test]
fn object_pattern_binding_resolves_each_property() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let { a, b } = obj;");
    let var_decl = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::VariableDeclaration(d) => d,
        _ => unreachable!(),
    };
    let declarator = var_decl
        .declarations
        .first()
        .expect("test variable declaration has at least one declarator");
    let vars = all_binding_variables(&declarator.id, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["a", "b"]);
}

#[test]
fn nested_destructuring_collects_all_leaves() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let { a: [x, y], b: { z } } = obj;");
    let var_decl = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::VariableDeclaration(d) => d,
        _ => unreachable!(),
    };
    let declarator = var_decl
        .declarations
        .first()
        .expect("test variable declaration has at least one declarator");
    let vars = all_binding_variables(&declarator.id, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["x", "y", "z"]);
}

#[test]
fn rest_element_is_included() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let [a, ...rest] = arr;");
    let var_decl = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::VariableDeclaration(d) => d,
        _ => unreachable!(),
    };
    let declarator = var_decl
        .declarations
        .first()
        .expect("test variable declaration has at least one declarator");
    let vars = all_binding_variables(&declarator.id, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["a", "rest"]);
}

#[test]
fn unresolved_names_are_skipped() {
    // `a` is declared, `b` is not. `all_binding_variables` walks the
    // identifier list but only emits IDs for names that resolve.
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let a = 1;");
    // Use a manually-constructed BindingPattern by parsing a destructuring
    // line that binds an undeclared name. Since the boundary still records
    // them as bindings inside the BindingPattern, the better approach is to
    // exercise the resolve-side guarantee by querying the global scope
    // with a synthetic pattern. We re-parse a different source to obtain
    // the pattern with a non-existent name.
    let alloc2 = Allocator::default();
    let (program2, _) = parse_and_analyze_ts(&alloc2, "let q = 1;");
    let var_decl = match program2
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::VariableDeclaration(d) => d,
        _ => unreachable!(),
    };
    let declarator = var_decl
        .declarations
        .first()
        .expect("test variable declaration has at least one declarator");
    // Use the first program's arena/scope so `q` doesn't resolve.
    let vars = all_binding_variables(&declarator.id, result.global_scope, &result.arena);
    assert!(vars.is_empty());
    // Sanity: `a` does resolve in the first arena.
    let var_decl_a = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::VariableDeclaration(d) => d,
        _ => unreachable!(),
    };
    let resolved = all_binding_variables(
        &var_decl_a.declarations[0].id,
        result.global_scope,
        &result.arena,
    );
    assert_eq!(resolved.len(), 1);
}

#[test]
fn assignment_target_identifier_resolves_to_single_variable() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let a = 1; a = 2;");
    let assign = match program
        .body
        .get(1)
        .expect("test source has at least N+1 top-level statements")
    {
        Statement::ExpressionStatement(es) => match &es.expression {
            Expression::AssignmentExpression(a) => a,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let vars = assignment_target_variables(&assign.left, result.global_scope, &result.arena);
    assert_eq!(vars.len(), 1);
    assert_eq!(result.arena.variables[vars[0]].name(), "a");
}

#[test]
fn assignment_target_array_destructuring_resolves_each_element() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let a; let b; [a, b] = arr;");
    let assign = match program
        .body
        .get(2)
        .expect("test source has at least N+1 top-level statements")
    {
        Statement::ExpressionStatement(es) => match &es.expression {
            Expression::AssignmentExpression(a) => a,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let vars = assignment_target_variables(&assign.left, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["a", "b"]);
}

#[test]
fn assignment_target_object_destructuring_resolves_each_property() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let a; let b; ({ a, b } = obj);");
    let assign = match program
        .body
        .get(2)
        .expect("test source has at least N+1 top-level statements")
    {
        Statement::ExpressionStatement(es) => match &es.expression {
            Expression::ParenthesizedExpression(p) => match &p.expression {
                Expression::AssignmentExpression(a) => a,
                _ => unreachable!(),
            },
            Expression::AssignmentExpression(a) => a,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let vars = assignment_target_variables(&assign.left, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["a", "b"]);
}

#[test]
fn assignment_target_member_expression_is_skipped() {
    // `obj.prop = ...` does not introduce a binding; the target is a
    // member access that resolves to the existing `obj` binding only
    // indirectly. `assignment_target_variables` returns an empty list
    // for member-target assignments.
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let obj = {}; obj.prop = 1;");
    let assign = match program
        .body
        .get(1)
        .expect("test source has at least N+1 top-level statements")
    {
        Statement::ExpressionStatement(es) => match &es.expression {
            Expression::AssignmentExpression(a) => a,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let vars = assignment_target_variables(&assign.left, result.global_scope, &result.arena);
    assert!(vars.is_empty());
}

#[test]
fn assignment_target_array_rest_is_collected() {
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let a; let rest; [a, ...rest] = arr;");
    let assign = match program
        .body
        .get(2)
        .expect("test source has at least N+1 top-level statements")
    {
        Statement::ExpressionStatement(es) => match &es.expression {
            Expression::AssignmentExpression(a) => a,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let vars = assignment_target_variables(&assign.left, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["a", "rest"]);
}

#[test]
fn assignment_target_object_rest_is_collected() {
    let alloc = Allocator::default();
    let (program, result) =
        parse_and_analyze_ts(&alloc, "let a; let rest; ({ a, ...rest } = obj);");
    let assign = match program
        .body
        .get(2)
        .expect("test source has at least N+1 top-level statements")
    {
        Statement::ExpressionStatement(es) => match &es.expression {
            Expression::ParenthesizedExpression(p) => match &p.expression {
                Expression::AssignmentExpression(a) => a,
                _ => unreachable!(),
            },
            Expression::AssignmentExpression(a) => a,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let vars = assignment_target_variables(&assign.left, result.global_scope, &result.arena);
    let names: Vec<&str> = vars
        .iter()
        .map(|&id| result.arena.variables[id].name())
        .collect();
    assert_eq!(names, vec!["a", "rest"]);
}

#[test]
fn duplicate_names_are_deduped() {
    // `assignment_target_variables` de-dupes via `Vec::contains`.
    // Build a pattern that binds two declarations with the same name
    // — `let [a, a]` is not valid syntax, so exercise the dedupe with
    // a doubly-bound destructuring target.
    let alloc = Allocator::default();
    let (program, result) = parse_and_analyze_ts(&alloc, "let a; let a2; [a, a] = [1, 2];");
    let assign = match program
        .body
        .get(2)
        .expect("test source has at least N+1 top-level statements")
    {
        Statement::ExpressionStatement(es) => match &es.expression {
            Expression::AssignmentExpression(a) => a,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let vars = assignment_target_variables(&assign.left, result.global_scope, &result.arena);
    assert_eq!(vars.len(), 1);
    assert_eq!(result.arena.variables[vars[0]].name(), "a");
}
