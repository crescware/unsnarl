//! Numeric `value_parser`s for the non-negative-integer CLI flags.
//!
//! 1:1 port of `ts/src/cli/args/parse-generation-count.ts`: accept the
//! input only when it matches `/^\d+$/`, then parse as base-10
//! integer. TS returns `null` on rejection and lets the option's
//! `argParser` convert that into `InvalidArgumentError`; clap performs
//! the same rejection here by surfacing the `Err` arm to its parser.
//!
//! The TS side used one shared parser for `-A` / `-B` / `-C` /
//! `--depth*`. The Rust side splits into two parsers so that the
//! field types stay typed: `-A` / `-B` / `-C` produce
//! `GenerationCount` (graph-traversal distance), `--depth*` produce
//! `NestingDepth` (lexical scope depth). The two share a private
//! digit-only `u32` helper, so the validation behaviour is
//! identical.

use unsnarl_ir::NestingDepth;
use unsnarl_root_query::GenerationCount;

pub fn parse_generation_count(value: &str) -> Result<GenerationCount, String> {
    parse_non_negative_u32(value).map(GenerationCount)
}

pub fn parse_nesting_depth(value: &str) -> Result<NestingDepth, String> {
    parse_non_negative_u32(value).map(NestingDepth)
}

fn parse_non_negative_u32(value: &str) -> Result<u32, String> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!(
            "{value} is not a non-negative integer (expected /^\\d+$/)"
        ));
    }
    value
        .parse::<u32>()
        .map_err(|err| format!("{value} cannot be parsed as u32: {err}"))
}

#[cfg(test)]
#[path = "parse_generation_count_test.rs"]
mod parse_generation_count_test;
