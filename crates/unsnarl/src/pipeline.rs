//! End-to-end pipeline helpers for the CLI and the parity harness.
//!
//! Mirrors the slice of `ts/src/pipeline/` that Step 12 exercises: parse
//! the source with `OxcParser`, analyse it with [`run_analysis`], hand
//! the analysed IR + annotations to [`FlatSerializer`], and render the
//! result with [`IrEmitter`]. Visual-graph / pruning / highlight
//! plumbing comes in later steps (#122 onward).

use std::path::Path;

use oxc_allocator::Allocator;
use unsnarl_analyzer::run_analysis;
use unsnarl_boundary_eslint_scope::parser::{
    default_source_type_for, OxcParser, ParseError, ParseOptions, SourceType,
};
use unsnarl_emitter::{EmitOptions, Emitter, IRSerializer, SerializeContext, SerializeSourceMeta};
use unsnarl_emitter_ir::{FlatSerializer, IrEmitter};
use unsnarl_emitter_json::JsonEmitter;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::ColorTheme;
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_ir::Language;

/// Map a path's extension to a [`Language`]. Mirrors
/// `fixtureLanguageFromExt` in `ts/integration/fixture-snapshot.ts`.
/// `.mjs` / `.cjs` map to `Js` because they are JavaScript at the
/// parser level; module-vs-script is resolved separately via
/// [`source_type_from_path`].
pub fn language_for_path(path: &str) -> Option<Language> {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match ext {
        "ts" => Some(Language::Ts),
        "tsx" => Some(Language::Tsx),
        "jsx" => Some(Language::Jsx),
        "js" | "mjs" | "cjs" => Some(Language::Js),
        _ => None,
    }
}

/// Mirrors `sourceTypeFromPath` in
/// `ts/src/pipeline/parse/source-type-from-path.ts`: `.mjs` / `.cjs`
/// are spec-pinned to module / script, every other extension falls
/// back to the language-level default.
pub fn source_type_from_path(path: &str, language: Language) -> SourceType {
    if path.ends_with(".mjs") {
        return SourceType::Module;
    }
    if path.ends_with(".cjs") {
        return SourceType::Script;
    }
    default_source_type_for(language)
}

/// Run the full parse -> analyse -> serialize -> emit pipeline for the
/// `ir` format and return the rendered text.
///
/// The `code` slice is owned by the caller; the AST and IR are built
/// inside this function and dropped before it returns, so no oxc
/// lifetime escapes to the caller.
pub fn emit_ir_text(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
) -> Result<String, ParseError> {
    emit_text_with(
        code,
        source_path,
        language,
        &IrEmitter,
        &EmitOptions {
            pretty_json,
            debug: false,
        },
    )
}

/// Same as [`emit_ir_text`] but routes the parsed IR through
/// [`JsonEmitter`], which builds a `VisualGraph` and serialises it
/// as JSON. Used by the `-f json` CLI handler and the parity
/// harness's `expected.json` comparison.
pub fn emit_json_text(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
) -> Result<String, ParseError> {
    emit_text_with(
        code,
        source_path,
        language,
        &JsonEmitter,
        &EmitOptions {
            pretty_json,
            debug: false,
        },
    )
}

/// Same as [`emit_ir_text`] but routes the parsed IR through
/// [`MermaidEmitter`]. The strategy / theme decisions are made by
/// the caller (CLI flags `--mermaid-renderer` / `--color-theme`)
/// rather than baked into the pipeline.
pub fn emit_mermaid_text(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
) -> Result<String, ParseError> {
    let emitter = MermaidEmitter::new(strategy, theme);
    emit_text_with(
        code,
        source_path,
        language,
        &emitter,
        &EmitOptions {
            pretty_json: false,
            debug,
        },
    )
}

fn emit_text_with(
    code: &str,
    source_path: &str,
    language: Language,
    emitter: &dyn Emitter,
    options: &EmitOptions,
) -> Result<String, ParseError> {
    let source_type = source_type_from_path(source_path, language);
    let allocator = Allocator::default();
    let parser = OxcParser;
    let parsed = parser.parse(
        &allocator,
        code,
        &ParseOptions {
            language,
            source_path: source_path.to_string(),
            source_type,
        },
    )?;
    let analyzed = run_analysis(&parsed.program, parsed.source_type, parsed.raw);
    let serializer = FlatSerializer;
    let ctx = SerializeContext {
        arena: &analyzed.arena,
        root_scope: analyzed.root_scope,
        annotations: &analyzed.annotations,
        source: SerializeSourceMeta {
            path: source_path.to_string(),
            language,
        },
        diagnostics: &analyzed.diagnostics,
        raw: analyzed.raw,
    };
    let serialized = serializer.serialize(&ctx);
    Ok(emitter.emit(&serialized, options))
}
