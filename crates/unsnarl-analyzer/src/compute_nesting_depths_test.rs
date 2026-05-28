use oxc_allocator::Allocator;

use unsnarl_ir::nesting_kind::NestingDepth;

use crate::analyzer_fixtures::parse_ts;

use super::compute_nesting_depths;

fn function_at(source: &str, offset: u32) -> u32 {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, source);
    let depths = compute_nesting_depths(&program);
    let depth = depths
        .get(&offset)
        .unwrap_or_else(|| panic!("no depth snapshot at offset {offset}"));
    depth.function.0
}

fn at(source: &str, offset: u32) -> unsnarl_ir::nesting_kind::NestingDepths {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, source);
    let depths = compute_nesting_depths(&program);
    let depth = depths
        .get(&offset)
        .unwrap_or_else(|| panic!("no depth snapshot at offset {offset}"));
    unsnarl_ir::nesting_kind::NestingDepths {
        function: depth.function,
        r#if: depth.r#if,
        r#for: depth.r#for,
        r#while: depth.r#while,
        switch: depth.switch,
        try_catch_finally: depth.try_catch_finally,
        block: depth.block,
    }
}

#[test]
fn function_body_increments_function_counter() {
    let code = "function f() { let x = 1; }";
    let fn_idx = code
        .find("function")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    assert_eq!(function_at(code, fn_idx), 1);
}

#[test]
fn if_body_increments_if_independently_of_function() {
    let code = "function f() { if (a) { let y = 1; } }";
    let inner_idx = code
        .find("{ let y")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d = at(code, inner_idx);
    assert_eq!(d.function, NestingDepth(1));
    assert_eq!(d.r#if, NestingDepth(1));
}

#[test]
fn for_init_binding_lives_at_parent_depth() {
    let code = "for (let i = 0; i < 1; i++) { i; }";
    let for_idx = code
        .find("for ")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d_for = at(code, for_idx);
    assert_eq!(d_for.r#for, NestingDepth(0));
    let body_idx = code
        .find("{ i;")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d_body = at(code, body_idx);
    assert_eq!(d_body.r#for, NestingDepth(1));
}

#[test]
fn nested_if_inside_if_increments_if_to_two() {
    let code = "if (a) { if (b) { 1; } }";
    let inner_idx = code
        .find("{ 1;")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d = at(code, inner_idx);
    assert_eq!(d.r#if, NestingDepth(2));
}

#[test]
fn each_nesting_kind_counts_independently() {
    let code = "for (;;) { if (b) { 1; } }";
    let inner_idx = code
        .find("{ 1;")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d = at(code, inner_idx);
    assert_eq!(d.r#for, NestingDepth(1));
    assert_eq!(d.r#if, NestingDepth(1));
}

#[test]
fn empty_if_body_still_counts() {
    let code = "if (a) { foo(); }";
    let inner_idx = code
        .find("{ foo")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d = at(code, inner_idx);
    assert_eq!(d.r#if, NestingDepth(1));
}

#[test]
fn arrow_function_increments_function_counter() {
    let code = "const f = () => { 1; };";
    let arrow_idx = code
        .find("() =>")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d = at(code, arrow_idx);
    assert_eq!(d.function, NestingDepth(1));
}

#[test]
fn switch_statement_increments_switch_counter() {
    let code = "switch (x) { case 1: y; }";
    let switch_idx = code
        .find("switch")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d = at(code, switch_idx);
    assert_eq!(d.switch, NestingDepth(1));
}

#[test]
fn while_body_increments_while_counter() {
    let code = "while (cond) { foo(); }";
    let body_idx = code
        .find("{ foo")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let d = at(code, body_idx);
    assert_eq!(d.r#while, NestingDepth(1));
}

#[test]
fn try_catch_finally_increments_try_catch_finally() {
    let code = "try { a; } catch (e) { b; } finally { c; }";
    let try_block_idx = code
        .find("{ a;")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let catch_body_idx = code
        .find("{ b;")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    let finally_body_idx = code
        .find("{ c;")
        .expect("test source string literally contains this substring at construction time")
        as u32;
    assert_eq!(at(code, try_block_idx).try_catch_finally, NestingDepth(1));
    assert_eq!(at(code, catch_body_idx).try_catch_finally, NestingDepth(1));
    assert_eq!(
        at(code, finally_body_idx).try_catch_finally,
        NestingDepth(1)
    );
}

#[test]
fn plain_block_statement_increments_block_counter() {
    let code = "{ 1; }";
    let block_idx = 0u32;
    let d = at(code, block_idx);
    assert_eq!(d.block, NestingDepth(1));
}
