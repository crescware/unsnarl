//! Pretty-print a switch-case test expression.
//!
//! Mirrors `ts/src/analyzer/format-case-test.ts`. Returns the raw
//! source slice when the expression's span is in-bounds and at most
//! [`CASE_TEST_MAX_LENGTH`] bytes long; otherwise falls back to a
//! type-specific rendering (`Identifier` name, `Literal` value) or
//! the `"<expr>"` placeholder.

use oxc_ast::ast::Expression;
use oxc_span::GetSpan;

const CASE_TEST_MAX_LENGTH: u32 = 32;

pub fn format_case_test(test_expr: &Expression<'_>, raw: &str) -> String {
    let span = test_expr.span();
    let start = span.start;
    let end = span.end;
    if end > start && (end as usize) <= raw.len() && end - start <= CASE_TEST_MAX_LENGTH {
        return raw[start as usize..end as usize].to_string();
    }
    match test_expr {
        Expression::Identifier(id) => id.name.to_string(),
        Expression::StringLiteral(s) => json_string(s.value.as_str()),
        Expression::NumericLiteral(n) => format_number(n.value),
        Expression::BooleanLiteral(b) => b.value.to_string(),
        Expression::NullLiteral(_) => "null".to_string(),
        _ => "<expr>".to_string(),
    }
}

/// JS `JSON.stringify(s)` for plain ASCII strings: wrap in double
/// quotes and escape the same characters JSON does.
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// JS `String(n)` for finite numbers — integers render without a
/// trailing `.0`; everything else uses the canonical Rust f64
/// formatting.
fn format_number(n: f64) -> String {
    if n.is_finite() && n.fract() == 0.0 && n.abs() < 1e21 {
        format!("{}", n as i64)
    } else {
        n.to_string()
    }
}

#[cfg(test)]
#[path = "format_case_test_test.rs"]
mod format_case_test_test;
