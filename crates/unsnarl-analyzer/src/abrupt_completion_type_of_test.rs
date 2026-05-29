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
fn labeled_statement_around_return_inherits_return_completion() {
    // ECMA §14.13.4 step 2: the wrapper inherits its body's
    // completion record. `outer: return;` therefore propagates the
    // Return completion up through the labelled wrapper.
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Return]));
}

#[test]
fn labeled_statement_around_throw_inherits_throw_completion() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: throw 1; }");
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Throw]));
}

#[test]
fn labeled_statement_around_block_with_return_inherits_return_completion() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: { return; } }");
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Return]));
}

#[test]
fn labeled_statement_with_matching_break_target_collapses_to_normal() {
    // ECMA §14.13.4 step 3: a Break completion whose
    // `[[Target]]` matches the labelled statement's label
    // collapses to Normal. `outer: { break outer; }` therefore has
    // a normal completion (the labelled wrapper "absorbs" the
    // break) and `abrupt_completion_type_of` returns `None`.
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: { break outer; } }");
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

#[test]
fn labeled_statement_with_matching_continue_target_collapses_to_normal() {
    let alloc = Allocator::default();
    let program = parse_ts(
        &alloc,
        "function f() { outer: for (;;) { continue outer; } }",
    );
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

#[test]
fn labeled_statement_with_non_matching_break_target_propagates() {
    // ECMA §14.13.4 step 3 only collapses when `[[Target]]` matches
    // this LabelledStatement's label. A break targeting some other
    // label propagates the Break completion through unchanged.
    // (oxc's parser is permissive about label semantic validity;
    // the analyzer just walks what it gets.)
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: break inner; }");
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Break]));
}

#[test]
fn nested_labeled_outer_collapses_outer_break() {
    // `outer: { inner: { break outer; } }`. The inner label does
    // not match `outer`, so the break propagates up to the outer
    // labelled wrapper, which absorbs it.
    let alloc = Allocator::default();
    let program = parse_ts(
        &alloc,
        "function f() { outer: { inner: { break outer; } } }",
    );
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

#[test]
fn nested_labeled_inner_collapses_inner_break() {
    // `outer: { inner: { break inner; } }`. The inner labelled
    // wrapper absorbs the matching break, so the outer wrapper
    // sees a normal completion from its body and also returns
    // None.
    let alloc = Allocator::default();
    let program = parse_ts(
        &alloc,
        "function f() { outer: { inner: { break inner; } } }",
    );
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

#[test]
fn labeled_statement_with_bare_break_propagates() {
    // ECMA §14.13.4 step 3 only collapses Break / Continue with a
    // `[[Target]]`. A bare `break;` carries no target, so
    // `matches_label` never matches it and the Break completion
    // propagates through the labelled wrapper unchanged.
    // (oxc's parser is permissive about a bare break outside an
    // iteration / switch; the analyzer just walks what it gets.)
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: { break; } }");
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Break]));
}

#[test]
fn labeled_statement_with_non_matching_continue_target_propagates() {
    // Symmetric to the non-matching break case: a continue whose
    // `[[Target]]` is some other label does not match this
    // LabelledStatement's label and propagates the Continue
    // completion through unchanged.
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: { continue inner; } }");
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Continue]));
}

#[test]
fn labeled_statement_with_bare_continue_propagates() {
    // A bare `continue;` carries no `[[Target]]`, so it never
    // matches the label and the Continue completion propagates.
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: { continue; } }");
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Continue]));
}

#[test]
fn labeled_statement_around_if_without_alternate_yields_none() {
    // Combination of the IfStatement alternate-required branch with a
    // labelled wrapper. `outer: if (x) break outer;` has no
    // `alternate`, so `compute_outcomes` returns `None` for the body
    // (a path falls through), and the labelled wrapper inherits that
    // normal completion — the matching break never gets a chance to
    // collapse because the body completion is already normal.
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "function f() { outer: if (x) break outer; }");
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

#[test]
fn labeled_statement_collapses_only_matching_branch_of_if() {
    // `outer: if (x) break outer; else return;`. Both branches
    // are abrupt; the matching break collapses but the return
    // propagates, so the wrapper returns Some([Return]).
    let alloc = Allocator::default();
    let program = parse_ts(
        &alloc,
        "function f() { outer: if (x) break outer; else return; }",
    );
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
    let res = abrupt_completion_type_of(labeled).expect("Some");
    assert!(types_match(&res, &[CompletionType::Return]));
}
