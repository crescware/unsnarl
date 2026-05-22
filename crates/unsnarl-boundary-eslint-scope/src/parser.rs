//! Wrapper around `oxc_parser::Parser`.
//!
//! `ParseOptions`, `ParsedSource`, and `SourceType` live here
//! alongside `OxcParser` because the parser's signatures reference
//! them. The wrapper itself stays in the boundary crate so the
//! `&'a Program<'a>` lifetime does not leak outside it.

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_diagnostics::{OxcDiagnostic, Severity};
use oxc_parser::Parser as OxcParserImpl;
use oxc_span::SourceType as OxcSourceType;
use unsnarl_ir::Language;

/// ECMAScript goal symbol for the parsed source.
#[derive(Clone, Copy)]
pub enum SourceType {
    Script,
    Module,
}

/// Options passed to [`OxcParser::parse`].
pub struct ParseOptions {
    pub language: Language,
    pub source_path: String,
    pub source_type: SourceType,
}

/// Successful parse result.
///
/// `language` is carried because the scope-builder normalises
/// `Program.span.start` differently for TS-flavored (`Ts`/`Tsx`)
/// versus JS-flavored (`Js`/`Jsx`) inputs.
///
/// The `program` field carries the arena lifetime `'a` so this type
/// — and any downstream consumer — must stay within the boundary
/// crate.
pub struct ParsedSource<'a> {
    pub program: Program<'a>,
    pub source_type: SourceType,
    pub language: Language,
    pub raw: &'a str,
}

/// One fatal-severity diagnostic surfaced by [`OxcParser::parse`].
#[derive(Debug)]
pub struct ParseErrorDetail {
    pub message: String,
    pub start: u32,
    pub end: u32,
}

/// Error returned by [`OxcParser::parse`] when the parser emits one or more
/// fatal-severity diagnostics.
#[derive(Debug)]
pub struct ParseError {
    message: String,
    errors: Vec<ParseErrorDetail>,
}

impl ParseError {
    pub fn new(message: String, errors: Vec<ParseErrorDetail>) -> Self {
        Self { message, errors }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn errors(&self) -> &[ParseErrorDetail] {
        &self.errors
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ParseError {}

/// Default `SourceType` for a language: `.js` files default to
/// `Script` (Node.js' default for bare `.js`); every other language
/// defaults to `Module`.
pub fn default_source_type_for(language: Language) -> SourceType {
    match language {
        Language::Js => SourceType::Script,
        Language::Ts | Language::Tsx | Language::Jsx => SourceType::Module,
    }
}

/// Wrapper around `oxc_parser::Parser`.
pub struct OxcParser;

impl OxcParser {
    pub const ID: &'static str = "oxc";

    pub fn id(&self) -> &'static str {
        Self::ID
    }

    pub fn parse<'a>(
        &self,
        allocator: &'a Allocator,
        code: &'a str,
        opts: &ParseOptions,
    ) -> Result<ParsedSource<'a>, ParseError> {
        let oxc_source_type = oxc_source_type_for(opts.language, opts.source_type);
        let ret = OxcParserImpl::new(allocator, code, oxc_source_type).parse();

        let fatal: Vec<&OxcDiagnostic> = ret
            .errors
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();

        if !fatal.is_empty() {
            let head = fatal[0].message.to_string();
            let details: Vec<ParseErrorDetail> = fatal
                .iter()
                .map(|d| {
                    let (start, end) = d
                        .labels
                        .as_ref()
                        .and_then(|labels| labels.first())
                        .map(|label| {
                            let start = u32::try_from(label.offset()).unwrap_or(0);
                            let end = start.saturating_add(u32::try_from(label.len()).unwrap_or(0));
                            (start, end)
                        })
                        .unwrap_or((0, 0));
                    ParseErrorDetail {
                        message: d.message.to_string(),
                        start,
                        end,
                    }
                })
                .collect();
            return Err(ParseError::new(
                format!("Parse error in {}: {}", opts.source_path, head),
                details,
            ));
        }

        Ok(ParsedSource {
            program: ret.program,
            source_type: opts.source_type,
            language: opts.language,
            raw: code,
        })
    }
}

fn oxc_source_type_for(language: Language, _source_type: SourceType) -> OxcSourceType {
    // Always parse with `with_module(true)` regardless of the
    // analysis-level `SourceType` -- so module-only syntax
    // (top-level `await`, `import` / `export`) parses cleanly even
    // when the surrounding analysis treats the file as a Script.
    // The analysis-level distinction is preserved on `ParsedSource`
    // and propagated downstream to `analyze`, which still picks
    // `global` vs `module` for the root scope from that field.
    match language {
        Language::Ts => OxcSourceType::ts(),
        Language::Tsx => OxcSourceType::tsx(),
        Language::Js => OxcSourceType::mjs(),
        Language::Jsx => OxcSourceType::jsx(),
    }
    .with_module(true)
}

#[cfg(test)]
#[path = "parser_test.rs"]
mod parser_test;
