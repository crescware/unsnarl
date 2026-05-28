use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;

use unsnarl_ir::completion::CompletionType;

use crate::analyzer_fixtures::parse_ts;

use super::abrupt_completion_type_of;

fn first_stmt<'a>(program: &'a oxc_ast::ast::Program<'a>) -> &'a Statement<'a> {
    program.body.first().expect("program has a statement")
}

fn types_match(actual: &[CompletionType], expected: &[CompletionType]) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    for e in expected {
        if !actual
            .iter()
            .any(|a| std::mem::discriminant(a) == std::mem::discriminant(e))
        {
            return false;
        }
    }
    true
}

#[test]
fn return_statement_inside_function_yields_return() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { return; }");
    let func = match first_stmt(&program) {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let stmt = func
        .body
        .as_ref()
        .expect("function declaration has a body (test fixture is not abstract)")
        .statements
        .last()
        .expect("test source has at least one statement in the function body");
    let res = abrupt_completion_type_of(stmt).expect("Some");
    assert!(types_match(&res, &[CompletionType::Return]));
}

#[test]
fn throw_statement_yields_throw() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "throw 1;");
    let stmt = first_stmt(&program);
    let res = abrupt_completion_type_of(stmt).expect("Some");
    assert!(types_match(&res, &[CompletionType::Throw]));
}

#[test]
fn break_statement_yields_break() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) { break; }");
    let outer = first_stmt(&program);
    let body = match outer {
        Statement::ForStatement(f) => match &f.body {
            Statement::BlockStatement(b) => b
                .body
                .last()
                .expect("test source has at least one statement in the block body"),
            other => other,
        },
        _ => unreachable!(),
    };
    let res = abrupt_completion_type_of(body).expect("Some");
    assert!(types_match(&res, &[CompletionType::Break]));
}

#[test]
fn continue_statement_yields_continue() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) { continue; }");
    let outer = first_stmt(&program);
    let body = match outer {
        Statement::ForStatement(f) => match &f.body {
            Statement::BlockStatement(b) => b
                .body
                .last()
                .expect("test source has at least one statement in the block body"),
            other => other,
        },
        _ => unreachable!(),
    };
    let res = abrupt_completion_type_of(body).expect("Some");
    assert!(types_match(&res, &[CompletionType::Continue]));
}

#[test]
fn expression_statement_yields_none() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "x;");
    let stmt = first_stmt(&program);
    assert!(abrupt_completion_type_of(stmt).is_none());
}

#[test]
fn block_ending_in_throw_yields_throw() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "{ x; throw 1; }");
    let stmt = first_stmt(&program);
    let res = abrupt_completion_type_of(stmt).expect("Some");
    assert!(types_match(&res, &[CompletionType::Throw]));
}

#[test]
fn block_ending_in_non_abrupt_yields_none() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "{ throw 1; x; }");
    let stmt = first_stmt(&program);
    assert!(abrupt_completion_type_of(stmt).is_none());
}

#[test]
fn empty_block_yields_none() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "{}");
    let stmt = first_stmt(&program);
    assert!(abrupt_completion_type_of(stmt).is_none());
}

#[test]
fn if_with_return_and_throw_yields_both() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { if (x) return; else throw 1; }");
    let func = match first_stmt(&program) {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let if_stmt = func
        .body
        .as_ref()
        .expect("function declaration has a body (test fixture is not abstract)")
        .statements
        .last()
        .expect("test source has at least one statement in the function body");
    let res = abrupt_completion_type_of(if_stmt).expect("Some");
    assert!(types_match(
        &res,
        &[CompletionType::Return, CompletionType::Throw]
    ));
}

#[test]
fn if_with_only_consequent_abrupt_yields_none() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { if (x) return; else y; }");
    let func = match first_stmt(&program) {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let if_stmt = func
        .body
        .as_ref()
        .expect("function declaration has a body (test fixture is not abstract)")
        .statements
        .last()
        .expect("test source has at least one statement in the function body");
    assert!(abrupt_completion_type_of(if_stmt).is_none());
}

#[test]
fn if_without_alternate_yields_none() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { if (x) return; }");
    let func = match first_stmt(&program) {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let if_stmt = func
        .body
        .as_ref()
        .expect("function declaration has a body (test fixture is not abstract)")
        .statements
        .last()
        .expect("test source has at least one statement in the function body");
    assert!(abrupt_completion_type_of(if_stmt).is_none());
}

#[test]
fn labeled_statement_pre_existing_limitation_yields_none() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: return; }");
    let func = match first_stmt(&program) {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let labeled = func
        .body
        .as_ref()
        .expect("function declaration has a body (test fixture is not abstract)")
        .statements
        .last()
        .expect("test source has at least one statement in the function body");
    assert!(abrupt_completion_type_of(labeled).is_none());
}
