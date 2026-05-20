use super::*;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};

fn uniform(n: u32) -> NestingDepths {
    NestingDepths::uniform(NestingDepth(n))
}

#[test]
fn returns_none_when_depths_is_none() {
    assert_eq!(format_depth_query(None), None);
}

#[test]
fn returns_none_when_every_kind_is_default() {
    let depths = uniform(DEFAULT_DEPTH.0);
    assert_eq!(format_depth_query(Some(&depths)), None);
}

#[test]
fn uniform_under_default_renders_depth() {
    let depths = uniform(2);
    assert_eq!(
        format_depth_query(Some(&depths)),
        Some("--depth 2".to_string())
    );
}

#[test]
fn function_default_with_narrower_block_renders_depth_block() {
    let mut depths = uniform(DEFAULT_DEPTH.0);
    depths.r#if = NestingDepth(3);
    depths.r#for = NestingDepth(3);
    depths.r#while = NestingDepth(3);
    depths.switch = NestingDepth(3);
    depths.try_catch_finally = NestingDepth(3);
    depths.block = NestingDepth(3);
    assert_eq!(
        format_depth_query(Some(&depths)),
        Some("--depth-block 3".to_string())
    );
}

#[test]
fn block_default_with_narrower_function_renders_depth_function() {
    let mut depths = uniform(DEFAULT_DEPTH.0);
    depths.function = NestingDepth(1);
    assert_eq!(
        format_depth_query(Some(&depths)),
        Some("--depth-function 1".to_string())
    );
}

#[test]
fn non_default_split_function_and_block_renders_both_flags() {
    let mut depths = uniform(3);
    depths.function = NestingDepth(1);
    assert_eq!(
        format_depth_query(Some(&depths)),
        Some("--depth-function 1 --depth-block 3".to_string())
    );
}

#[test]
fn non_uniform_non_function_kinds_render_per_kind_comment() {
    let mut depths = uniform(DEFAULT_DEPTH.0);
    depths.r#if = NestingDepth(1);
    depths.r#for = NestingDepth(2);
    assert_eq!(
        format_depth_query(Some(&depths)),
        Some("# nesting depth: if=1 for=2".to_string())
    );
}
