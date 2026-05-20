use super::*;
use crate::cli::args::Args;

fn parse(argv: &[&str]) -> Args {
    Args::try_parse_from(argv).expect("argv should parse")
}

#[test]
fn stdin_true_reads_stdin_and_returns_text_with_declared_lang() {
    let args = parse(&["uns", "--stdin", "--stdin-lang", "tsx"]);
    let mut reader = "piped contents".as_bytes();
    let result = calc_source(&args, &mut reader, || "USAGE".to_string())
        .expect("calc_source should succeed");
    match result {
        ExecuteSource::Stdin { text, lang } => {
            assert_eq!(text, "piped contents");
            assert_eq!(lang, CliLanguage::Tsx);
        }
        _ => panic!("expected Stdin"),
    }
}

#[test]
fn stdin_true_ignores_any_positional_file_argument() {
    let args = parse(&["uns", "--stdin", "ignored.ts"]);
    let mut reader = "x".as_bytes();
    let result = calc_source(&args, &mut reader, || "USAGE".to_string())
        .expect("calc_source should succeed");
    match result {
        ExecuteSource::Stdin { text, lang } => {
            assert_eq!(text, "x");
            assert_eq!(lang, CliLanguage::Ts);
        }
        _ => panic!("expected Stdin"),
    }
}

#[test]
fn stdin_false_with_file_returns_path() {
    let args = parse(&["uns", "src/foo.ts"]);
    let mut reader: &[u8] = b"";
    let result = calc_source(&args, &mut reader, || "USAGE".to_string())
        .expect("calc_source should succeed");
    match result {
        ExecuteSource::File { path } => {
            assert_eq!(path.to_string_lossy(), "src/foo.ts");
        }
        _ => panic!("expected File"),
    }
}

#[test]
fn stdin_false_without_file_throws_cli_usage_error_carrying_help_text() {
    let args = parse(&["uns"]);
    let mut reader: &[u8] = b"";
    let err = calc_source(&args, &mut reader, || "USAGE".to_string()).expect_err("should reject");
    assert_eq!(err.message, "no input file (use --stdin or pass a path)");
    assert_eq!(err.help.as_deref(), Some("USAGE"));
}
