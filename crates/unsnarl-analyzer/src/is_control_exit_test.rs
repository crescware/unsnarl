use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;

use crate::testing::parse_ts;

use super::is_control_exit;

fn first_stmt<'a>(program: &'a oxc_ast::ast::Program<'a>) -> &'a Statement<'a> {
    program
        .body
        .iter()
        .find(|s| !matches!(s, Statement::ExpressionStatement(es) if matches!(es.expression, oxc_ast::ast::Expression::StringLiteral(_))))
        .expect("program has at least one non-directive statement")
}

#[test]
fn break_continue_return_throw_yield_true() {
    for (src, _label) in [
        ("for (;;) { break; }", "break"),
        ("for (;;) { continue; }", "continue"),
        ("function f() { return; }", "return"),
        ("function f() { throw 1; }", "throw"),
    ] {
        let alloc = Allocator::default();
        let program = parse_ts(&alloc, src);
        let stmt = first_stmt(&program);
        // For each shape, the relevant statement is nested; navigate to it.
        let exit = match stmt {
            Statement::ForStatement(f) => match &f.body {
                Statement::BlockStatement(b) => is_control_exit(b.body.last().unwrap()),
                other => is_control_exit(other),
            },
            Statement::FunctionDeclaration(f) => {
                let body = f.body.as_ref().expect("body");
                is_control_exit(body.statements.last().unwrap())
            }
            other => is_control_exit(other),
        };
        assert!(exit, "expected control-exit for source: {src}");
    }
}

#[test]
fn expression_statement_does_not_exit() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "x;");
    let stmt = first_stmt(&program);
    assert!(!is_control_exit(stmt));
}

#[test]
fn variable_declaration_does_not_exit() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "let x = 1;");
    let stmt = first_stmt(&program);
    assert!(!is_control_exit(stmt));
}

#[test]
fn block_ending_in_break_exits() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) { x; break; }");
    let stmt = first_stmt(&program);
    let for_stmt = match stmt {
        Statement::ForStatement(f) => f,
        _ => unreachable!(),
    };
    assert!(is_control_exit(&for_stmt.body));
}

#[test]
fn empty_block_does_not_exit() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "{}");
    let stmt = first_stmt(&program);
    assert!(!is_control_exit(stmt));
}

#[test]
fn if_statement_both_branches_exit_returns_true() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) { if (x) break; else throw 1; }");
    let stmt = first_stmt(&program);
    let for_stmt = match stmt {
        Statement::ForStatement(f) => f,
        _ => unreachable!(),
    };
    let inner = match &for_stmt.body {
        Statement::BlockStatement(b) => b.body.last().unwrap(),
        _ => unreachable!(),
    };
    assert!(is_control_exit(inner));
}

#[test]
fn if_statement_only_one_branch_exits_returns_false() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) { if (x) break; else y; }");
    let stmt = first_stmt(&program);
    let for_stmt = match stmt {
        Statement::ForStatement(f) => f,
        _ => unreachable!(),
    };
    let inner = match &for_stmt.body {
        Statement::BlockStatement(b) => b.body.last().unwrap(),
        _ => unreachable!(),
    };
    assert!(!is_control_exit(inner));
}

#[test]
fn if_statement_without_alternate_returns_false() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "for (;;) { if (x) break; }");
    let stmt = first_stmt(&program);
    let for_stmt = match stmt {
        Statement::ForStatement(f) => f,
        _ => unreachable!(),
    };
    let inner = match &for_stmt.body {
        Statement::BlockStatement(b) => b.body.last().unwrap(),
        _ => unreachable!(),
    };
    assert!(!is_control_exit(inner));
}
