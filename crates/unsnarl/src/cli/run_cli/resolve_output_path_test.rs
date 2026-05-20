use super::*;
use crate::cli::args::Args;
use unsnarl_emitter::Emitter;
use unsnarl_emitter_ir::IrEmitter;
use unsnarl_emitter_json::JsonEmitter;
use unsnarl_emitter_stats::StatsEmitter;

fn parse(argv: &[&str]) -> Args {
    Args::try_parse_from(argv).expect("argv should parse")
}

#[test]
fn returns_none_when_neither_out_file_nor_out_dir_is_set() {
    let args = parse(&["uns", "x.ts"]);
    assert!(resolve_output_path(&args, &IrEmitter).is_none());
}

#[test]
fn returns_the_out_file_path_verbatim() {
    let args = parse(&["uns", "--out-file", "/tmp/out.txt", "x.ts"]);
    let path = resolve_output_path(&args, &IrEmitter).expect("path expected");
    assert_eq!(path.to_string_lossy(), "/tmp/out.txt");
}

#[test]
fn joins_out_dir_with_derived_basename_and_emitter_extension_for_input_file() {
    let args = parse(&["uns", "-o", "build", "src/foo.ts"]);
    let path = resolve_output_path(&args, &IrEmitter).expect("path expected");
    assert_eq!(
        path.to_string_lossy(),
        format!("build/foo.{}", IrEmitter.extension())
    );
}

#[test]
fn joins_out_dir_with_derived_basename_for_roots() {
    let args = parse(&["uns", "-o", "build", "-r", "render", "x.ts"]);
    let path = resolve_output_path(&args, &JsonEmitter).expect("path expected");
    assert_eq!(
        path.to_string_lossy(),
        format!("build/render.{}", JsonEmitter.extension())
    );
}

#[test]
fn picks_the_emitter_extension_per_format() {
    let args = parse(&["uns", "-o", "build", "src/x.ts"]);
    let stats_path = resolve_output_path(&args, &StatsEmitter).expect("path expected");
    assert!(
        stats_path
            .to_string_lossy()
            .ends_with(&format!(".{}", StatsEmitter.extension())),
        "expected stats extension, got: {}",
        stats_path.display()
    );
}
