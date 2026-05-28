//! Tests for [`OxcParser`].

use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;
use unsnarl_ir::Language;

use super::{default_source_type_for, OxcParser, ParseError, ParseOptions, SourceType};

fn parser() -> OxcParser {
    OxcParser
}

fn is_variable_declaration(stmt: &Statement) -> bool {
    matches!(stmt, Statement::VariableDeclaration(_))
}

fn is_import_declaration(stmt: &Statement) -> bool {
    matches!(stmt, Statement::ImportDeclaration(_))
}

fn is_export_named_declaration(stmt: &Statement) -> bool {
    matches!(stmt, Statement::ExportNamedDeclaration(_))
}

#[test]
fn identifies_itself_as_oxc() {
    assert_eq!(parser().id(), "oxc");
}

#[test]
fn parses_a_simple_ts_program_into_an_estree_compatible_program_node() {
    let allocator = Allocator::default();
    let code = "const greeting: string = 'hi';\nconst length = greeting.length;\n";
    let parsed = parser()
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .expect("test source is hand-written valid TS and must parse");

    assert!(matches!(parsed.source_type, SourceType::Module));
    assert_eq!(parsed.raw, code);

    let program = &parsed.program;
    assert_eq!(program.body.len(), 2);
    assert!(is_variable_declaration(&program.body[0]));
    assert!(is_variable_declaration(&program.body[1]));
}

#[test]
fn parses_tsx_with_jsx_elements() {
    let allocator = Allocator::default();
    let code = "const Hello = () => <div className=\"x\"><span>{\"hi\"}</span></div>;\n";
    let parsed = parser()
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language: Language::Tsx,
                source_path: "input.tsx".to_string(),
                source_type: default_source_type_for(Language::Tsx),
            },
        )
        .expect("test source is hand-written valid TS and must parse");

    let program = &parsed.program;
    assert_eq!(program.body.len(), 1);
    assert!(is_variable_declaration(&program.body[0]));
}

#[test]
fn parses_js_with_esm_import() {
    let allocator = Allocator::default();
    let code = "import { join } from 'node:path';\nexport const sep = join('a', 'b');\n";
    let parsed = parser()
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language: Language::Js,
                source_path: "input.js".to_string(),
                source_type: SourceType::Module,
            },
        )
        .expect("test source is hand-written valid TS and must parse");

    let program = &parsed.program;
    assert!(is_import_declaration(&program.body[0]));
    assert!(is_export_named_declaration(&program.body[1]));
}

#[test]
fn preserves_an_explicitly_requested_source_type_regardless_of_the_language_extension() {
    let allocator = Allocator::default();
    let code = "var legacy = 1;\n";
    let parsed = parser()
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language: Language::Js,
                source_path: "input.js".to_string(),
                source_type: SourceType::Script,
            },
        )
        .expect("test source is hand-written valid TS and must parse");
    assert!(matches!(parsed.source_type, SourceType::Script));
}

#[test]
fn parses_js_with_top_level_await_when_analysis_source_type_is_script() {
    // The parser passes `sourceType: "module"` to oxc unconditionally
    // so top-level `await` parses cleanly even when the analysis-level
    // SourceType is Script. CLI shebang files like `vite/bin/vite.js`
    // and `oxlint/dist/cli.js` rely on this -- they ship as `.js`
    // (analysis-level Script) but use top-level `await` and ES module
    // syntax, and must still parse without error.
    let allocator = Allocator::default();
    let code = "import { x } from 'node:fs';\nconst y = await x();\n";
    let parsed = parser()
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language: Language::Js,
                source_path: "input.js".to_string(),
                source_type: SourceType::Script,
            },
        )
        .expect("top-level await must parse cleanly even when SourceType is Script");
    assert!(matches!(parsed.source_type, SourceType::Script));
    assert_eq!(parsed.program.body.len(), 2);
}

#[test]
fn synthesizes_a_filename_with_the_correct_extension_when_source_path_has_none() {
    let allocator = Allocator::default();
    let code = "const x = 1;\n";
    let result = parser().parse(
        &allocator,
        code,
        &ParseOptions {
            language: Language::Ts,
            source_path: String::new(),
            source_type: default_source_type_for(Language::Ts),
        },
    );
    assert!(result.is_ok());
}

#[test]
fn throws_parse_error_on_syntactically_invalid_source() {
    let allocator = Allocator::default();
    let code = "const = 1;\n";
    let result = parser().parse(
        &allocator,
        code,
        &ParseOptions {
            language: Language::Ts,
            source_path: "broken.ts".to_string(),
            source_type: default_source_type_for(Language::Ts),
        },
    );
    let captured: ParseError = match result {
        Ok(_) => panic!("expected ParseError"),
        Err(e) => e,
    };
    assert!(!captured.errors().is_empty());
    assert!(captured.message().contains("broken.ts"));
}

// `default_source_type_for` lives next to the parser in the
// boundary crate (not under `pipeline/parse/`) because it pairs
// directly with `OxcParser::parse`; the tests are pinned here so
// the function's sibling test file owns its behaviour.
#[test]
fn default_source_type_for_maps_js_to_script_nodejs_default() {
    assert!(matches!(
        default_source_type_for(Language::Js),
        SourceType::Script
    ));
}

#[test]
fn default_source_type_for_maps_ts_to_module() {
    assert!(matches!(
        default_source_type_for(Language::Ts),
        SourceType::Module
    ));
}

#[test]
fn default_source_type_for_maps_tsx_to_module() {
    assert!(matches!(
        default_source_type_for(Language::Tsx),
        SourceType::Module
    ));
}

#[test]
fn default_source_type_for_maps_jsx_to_module() {
    assert!(matches!(
        default_source_type_for(Language::Jsx),
        SourceType::Module
    ));
}
