use std::io::{self, Write};

use crate::cli::args::{Args, CliFormat};

pub fn run(args: &Args) {
    let mut stderr = io::stderr().lock();
    run_to(args, &mut stderr);
}

pub(crate) fn run_to(args: &Args, out: &mut dyn Write) {
    let handler = select_handler(&args.format);
    handler(args, out);
}

type Handler = fn(&Args, &mut dyn Write);

fn select_handler(format: &CliFormat) -> Handler {
    match format {
        CliFormat::Mermaid => emit_mermaid,
        CliFormat::Ir => emit_ir,
        CliFormat::Json => emit_json,
        CliFormat::Markdown => emit_markdown,
        CliFormat::Stats => emit_stats,
    }
}

fn emit_mermaid(args: &Args, out: &mut dyn Write) {
    emit_stub("mermaid emitter", args, out);
}

fn emit_ir(args: &Args, out: &mut dyn Write) {
    emit_stub("ir emitter", args, out);
}

fn emit_json(args: &Args, out: &mut dyn Write) {
    emit_stub("json emitter", args, out);
}

fn emit_markdown(args: &Args, out: &mut dyn Write) {
    emit_stub("markdown emitter", args, out);
}

fn emit_stats(args: &Args, out: &mut dyn Write) {
    emit_stub("stats emitter", args, out);
}

fn emit_stub(label: &str, args: &Args, out: &mut dyn Write) {
    let json = serde_json::to_string_pretty(args).expect("serialize CLI args");
    writeln!(out, "Not implemented yet: {label}").expect("write stub label");
    writeln!(out, "{json}").expect("write CLI args JSON");
}

#[cfg(test)]
mod test;
