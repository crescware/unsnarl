use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;

use crate::analyzer_fixtures::parse_ts;

use super::case_falls_through;

fn case_consequent<'a>(program: &'a oxc_ast::ast::Program<'a>) -> &'a [Statement<'a>] {
    let switch = match program.body.first().expect("statement") {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    let case = switch.cases.first().expect("case");
    &case.consequent
}

#[test]
fn empty_consequent_falls_through() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: }");
    assert!(case_falls_through(case_consequent(&program)));
}

#[test]
fn ends_in_break_does_not_fall_through() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: break; }");
    assert!(!case_falls_through(case_consequent(&program)));
}

#[test]
fn ends_in_return_does_not_fall_through() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { switch (x) { case 1: return; } }");
    let func = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let switch = match func
        .body
        .as_ref()
        .expect("function declaration has a body (test fixture is not abstract)")
        .statements
        .first()
        .expect("function body has at least one statement (test source)")
    {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    assert!(!case_falls_through(
        &switch
            .cases
            .first()
            .expect("switch statement in test source has at least one case")
            .consequent
    ));
}

#[test]
fn ends_in_throw_does_not_fall_through() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: throw 1; }");
    assert!(!case_falls_through(case_consequent(&program)));
}

#[test]
fn ends_in_continue_does_not_fall_through() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) switch (x) { case 1: continue; }");
    let outer = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::ForStatement(f) => f,
        _ => unreachable!(),
    };
    let switch = match &outer.body {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    assert!(!case_falls_through(
        &switch
            .cases
            .first()
            .expect("switch statement in test source has at least one case")
            .consequent
    ));
}

#[test]
fn ends_in_expression_statement_falls_through() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: y; }");
    assert!(case_falls_through(case_consequent(&program)));
}

#[test]
fn ends_in_block_that_exits_does_not_fall_through() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 1: { break; } }");
    assert!(!case_falls_through(case_consequent(&program)));
}
