//! Parse + analyse + serialize the source into a [`SerializedIR`].

use oxc_allocator::Allocator;

use unsnarl_analyzer::run_analysis;
use unsnarl_emitter::{IRSerializer, SerializeContext, SerializeSourceMeta};
use unsnarl_emitter_ir::FlatSerializer;
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::{OxcParser, ParseError, ParseOptions};

use super::language_for_path::source_type_from_path;

pub(super) fn serialize_ir(
    code: &str,
    source_path: &str,
    language: Language,
) -> Result<SerializedIR, ParseError> {
    let source_type = source_type_from_path(source_path, language);
    let allocator = Allocator::default();
    let parser = OxcParser;
    let parsed = {
        let _span = unsnarl_instrumentation::span!("parse", bytes = code.len());
        parser.parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: source_path.to_string(),
                source_type,
            },
        )?
    };
    let analyzed = {
        let _span = unsnarl_instrumentation::span!("analyze");
        run_analysis(&parsed.program, parsed.source_type, language, parsed.raw)
    };
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
    let _span = unsnarl_instrumentation::span!("serialize");
    let ir = serializer.serialize(&ctx);
    tracing::info!(
        scopes = ir.scopes.len(),
        variables = ir.variables.len(),
        references = ir.references.len(),
        diagnostics = ir.diagnostics.len(),
        "ir counts",
    );
    Ok(ir)
}
