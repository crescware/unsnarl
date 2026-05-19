//! Flatten a `BindingPattern` into the list of bound identifiers.
//!
//! Mirrors `collectBindingIdentifiers` in
//! `ts/src/boundary/eslint-scope/declare/collect-binding-identifiers.ts`.
//! The TS port walks an unnormalised `AstNode` and switches on
//! `node.type` strings; the Rust port walks the typed
//! `oxc_ast::ast::BindingPattern` enum directly. The recursion shape
//! is identical: identifiers are pushed in-order, and
//! `ObjectPattern` / `ArrayPattern` / `AssignmentPattern` /
//! `RestElement` recurse into their sub-patterns. The TS implementation
//! has a `RestElement` arm at the top level even though
//! `RestElement` is not a `BindingPattern` shape in the TS AST; the
//! Rust port handles rest elements as part of the `ObjectPattern` /
//! `ArrayPattern` arms, where they appear in the oxc_ast types
//! (`ObjectPattern.rest` / `ArrayPattern.rest`).

use oxc_ast::ast::BindingPattern;

use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_oxc_parity::AstType;

pub fn collect_binding_identifiers(pattern: &BindingPattern<'_>) -> Vec<AstIdentifier> {
    let mut out = Vec::new();
    collect(pattern, &mut out);
    out
}

fn collect(pattern: &BindingPattern<'_>, out: &mut Vec<AstIdentifier>) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            out.push(AstIdentifier::new(
                AstType::Identifier,
                id.name.as_str().to_string(),
                id.span,
            ));
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect(&prop.value, out);
            }
            if let Some(rest) = obj.rest.as_deref() {
                collect(&rest.argument, out);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() {
                collect(el, out);
            }
            if let Some(rest) = arr.rest.as_deref() {
                collect(&rest.argument, out);
            }
        }
        BindingPattern::AssignmentPattern(asn) => {
            collect(&asn.left, out);
        }
    }
}

#[cfg(test)]
#[path = "collect_binding_identifiers_test.rs"]
mod collect_binding_identifiers_test;
