use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;

use crate::testing::parse_ts;

use super::case_exits_function;

fn case_consequent_from_function<'a>(
    program: &'a oxc_ast::ast::Program<'a>,
) -> &'a [Statement<'a>] {
    let func = match program.body.first().unwrap() {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let switch = match func.body.as_ref().unwrap().statements.first().unwrap() {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    &switch.cases.first().unwrap().consequent
}

fn case_consequent_top_level<'a>(program: &'a oxc_ast::ast::Program<'a>) -> &'a [Statement<'a>] {
    let switch = match program.body.first().unwrap() {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    &switch.cases.first().unwrap().consequent
}

#[test]
fn empty_consequent_does_not_exit() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: }");
    assert!(!case_exits_function(case_consequent_top_level(&program)));
}

#[test]
fn ends_in_return_exits() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { switch (x) { case 1: return; } }");
    assert!(case_exits_function(case_consequent_from_function(&program)));
}

#[test]
fn ends_in_throw_exits() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: throw 1; }");
    assert!(case_exits_function(case_consequent_top_level(&program)));
}

#[test]
fn ends_in_break_does_not_exit_function() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: break; }");
    assert!(!case_exits_function(case_consequent_top_level(&program)));
}

#[test]
fn ends_in_continue_does_not_exit_function() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) switch (x) { case 1: continue; }");
    let outer = match program.body.first().unwrap() {
        Statement::ForStatement(f) => f,
        _ => unreachable!(),
    };
    let switch = match &outer.body {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    assert!(!case_exits_function(
        &switch.cases.first().unwrap().consequent
    ));
}

#[test]
fn ends_in_expression_statement_does_not_exit() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: y; }");
    assert!(!case_exits_function(case_consequent_top_level(&program)));
}

#[test]
fn ends_in_if_break_throw_does_not_exit_function() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: if (y) break; else throw 1; }");
    assert!(!case_exits_function(case_consequent_top_level(&program)));
}

#[test]
fn ends_in_if_return_break_does_not_exit_function() {
    let alloc = Allocator::default();
    let program = parse_ts(
        &alloc,
        "function f() { switch (x) { case 1: if (y) return; else break; } }",
    );
    assert!(!case_exits_function(case_consequent_from_function(
        &program
    )));
}

#[test]
fn ends_in_if_return_continue_does_not_exit_function() {
    let alloc = Allocator::default();
    let program = parse_ts(
        &alloc,
        "function f() { for (;;) switch (x) { case 1: if (y) return; else continue; } }",
    );
    let func = match program.body.first().unwrap() {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let for_stmt = match func.body.as_ref().unwrap().statements.first().unwrap() {
        Statement::ForStatement(f) => f,
        _ => unreachable!(),
    };
    let switch = match &for_stmt.body {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    assert!(!case_exits_function(
        &switch.cases.first().unwrap().consequent
    ));
}

#[test]
fn ends_in_if_return_throw_exits_function() {
    let alloc = Allocator::default();
    let program = parse_ts(
        &alloc,
        "function f() { switch (x) { case 1: if (y) return; else throw 1; } }",
    );
    assert!(case_exits_function(case_consequent_from_function(&program)));
}
