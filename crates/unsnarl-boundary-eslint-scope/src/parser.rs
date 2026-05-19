//! Wrapper around `oxc_parser::Parser`, mirroring `ts/src/parser/oxc-parser.ts`.
//!
//! `ParseOptions`, `ParsedSource`, and `SourceType` originate from
//! `ts/src/pipeline/parse/` on the TS side. They are colocated here for now
//! because `OxcParser`'s signatures reference them, and the pipeline crate
//! does not yet exist in the Rust workspace. When `ts/src/pipeline/parse/`
//! is migrated, the contract types may move out of this module; the
//! `OxcParser` wrapper itself stays in the boundary crate so that the
//! `&'a Program<'a>` lifetime does not leak outside it.

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_diagnostics::{OxcDiagnostic, Severity};
use oxc_parser::Parser as OxcParserImpl;
use oxc_span::SourceType as OxcSourceType;
use unsnarl_ir::Language;

/// ECMAScript goal symbol for the parsed source.
///
/// Mirrors `SOURCE_TYPE` in `ts/src/pipeline/parse/source-type.ts`.
#[derive(Clone, Copy)]
pub enum SourceType {
    Script,
    Module,
}

/// Options passed to [`OxcParser::parse`].
///
/// Mirrors `ParseOptions` in `ts/src/pipeline/parse/parse-options.ts`.
pub struct ParseOptions {
    pub language: Language,
    pub source_path: String,
    pub source_type: SourceType,
}

/// Successful parse result.
///
/// Mirrors `ParsedSource` in `ts/src/pipeline/parse/parsed-source.ts`. The
/// `program` field carries the arena lifetime `'a` so this type — and any
/// downstream consumer — must stay within the boundary crate.
pub struct ParsedSource<'a> {
    pub program: Program<'a>,
    pub language: Language,
    pub source_path: String,
    pub source_type: SourceType,
    pub raw: &'a str,
}

/// One fatal-severity diagnostic surfaced by [`OxcParser::parse`].
///
/// Mirrors the anonymous detail object in `ts/src/parser/parse-error.ts`.
#[derive(Debug)]
pub struct ParseErrorDetail {
    pub message: String,
    pub start: u32,
    pub end: u32,
}

/// Error returned by [`OxcParser::parse`] when the parser emits one or more
/// fatal-severity diagnostics.
///
/// Mirrors `ParseError` in `ts/src/parser/parse-error.ts`.
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

/// Default `SourceType` for a language, mirroring `defaultSourceTypeFor` in
/// `ts/src/pipeline/parse/default-source-type-for.ts`. `.js` files default to
/// `Script` (Node.js' default for bare `.js`); every other language defaults
/// to `Module`.
pub fn default_source_type_for(language: Language) -> SourceType {
    match language {
        Language::Js => SourceType::Script,
        Language::Ts | Language::Tsx | Language::Jsx => SourceType::Module,
    }
}

/// Wrapper around `oxc_parser::Parser` that mirrors the TS `OxcParser`
/// (`ts/src/parser/oxc-parser.ts`).
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
            language: opts.language,
            source_path: opts.source_path.clone(),
            source_type: opts.source_type,
            raw: code,
        })
    }
}

fn oxc_source_type_for(language: Language, source_type: SourceType) -> OxcSourceType {
    let base = match language {
        Language::Ts => OxcSourceType::ts(),
        Language::Tsx => OxcSourceType::tsx(),
        Language::Js => OxcSourceType::mjs(),
        Language::Jsx => OxcSourceType::jsx(),
    };
    match source_type {
        SourceType::Module => base.with_module(true),
        SourceType::Script => base.with_script(true),
    }
}

#[cfg(test)]
#[path = "parser_test.rs"]
mod parser_test;
