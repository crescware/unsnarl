//! Numeric `value_parser` for `-A` / `-B` / `-C` / `--depth` /
//! `--depth-function` / `--depth-block`.
//!
//! 1:1 port of `ts/src/cli/args/parse-generation-count.ts`: accept the
//! input only when it matches `/^\d+$/`, then parse as base-10 integer.
//! TS returns `null` on rejection and lets the option's `argParser`
//! convert that into `InvalidArgumentError`; clap performs the same
//! rejection here by surfacing the `Err` arm to its parser.

pub fn parse_generation_count(value: &str) -> Result<u32, String> {
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
mod test;
