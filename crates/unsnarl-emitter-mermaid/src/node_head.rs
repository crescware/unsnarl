//! Builds the leading text fragment of a node's mermaid label
//! (before the trailing `<br/>L<line>` range stamp).

use unsnarl_oxc_parity::VariableDeclarationKind;
use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

use crate::escape::escape_into;

pub fn node_head(n: &VisualNode) -> String {
    let mut out = String::new();
    node_head_into(&mut out, n);
    out
}

/// Destination-arg variant of [`node_head`]: writes the head fragment
/// straight into `out`. Used by `emit_node` so the per-node label
/// build does not bounce through an intermediate `String` for the
/// "name", a second `String` for the head shell, and a third one for
/// the outer line.
pub fn node_head_into(out: &mut String, n: &VisualNode) {
    if n.is_jsx_element() {
        // Mermaid `["..."]` labels require HTML-escaped angle
        // brackets so the parser does not mistake them for syntax;
        // the renderer surfaces them as literal `<` / `>` in the
        // output.
        out.push_str("&lt;");
        escape_into(out, n.name());
        out.push_str("&gt;");
        return;
    }
    match n.kind() {
        NodeKind::FunctionDeclaration => {
            escape_into(out, n.name());
            out.push_str("()");
        }
        NodeKind::ClassDeclaration => {
            out.push_str("class ");
            escape_into(out, n.name());
        }
        NodeKind::NamedImportBinding => match n.imported_name() {
            Some(imported) if imported != n.name() => {
                escape_into(out, n.name());
            }
            _ => {
                out.push_str("import ");
                escape_into(out, n.name());
            }
        },
        NodeKind::DefaultImportBinding | NodeKind::NamespaceImportBinding => {
            out.push_str("import ");
            escape_into(out, n.name());
        }
        NodeKind::CatchParameter => {
            out.push_str("catch ");
            escape_into(out, n.name());
        }
        NodeKind::SyntheticImplicitGlobal => {
            out.push_str("global ");
            escape_into(out, n.name());
        }
        NodeKind::WriteReference => match n.declaration_kind() {
            Some(VariableDeclarationKind::Let) => {
                out.push_str("let ");
                escape_into(out, n.name());
            }
            _ => {
                escape_into(out, n.name());
            }
        },
        NodeKind::SyntheticModuleSource => {
            out.push_str("module ");
            escape_into(out, n.name());
        }
        NodeKind::SyntheticImportIntermediate => {
            out.push_str("import ");
            escape_into(out, n.name());
        }
        NodeKind::VarBinding => {
            if n.init_is_function() {
                escape_into(out, n.name());
                out.push_str("()");
            } else {
                out.push_str("var ");
                escape_into(out, n.name());
            }
        }
        NodeKind::ConstBinding => {
            if n.init_is_function() {
                escape_into(out, n.name());
                out.push_str("()");
            } else {
                escape_into(out, n.name());
            }
        }
        NodeKind::LetBinding => {
            if n.init_is_function() {
                escape_into(out, n.name());
                out.push_str("()");
            } else {
                out.push_str("let ");
                escape_into(out, n.name());
            }
        }
        NodeKind::UsingBinding => {
            if n.init_is_function() {
                escape_into(out, n.name());
                out.push_str("()");
            } else {
                out.push_str("using ");
                escape_into(out, n.name());
            }
        }
        NodeKind::AwaitUsingBinding => {
            if n.init_is_function() {
                escape_into(out, n.name());
                out.push_str("()");
            } else {
                out.push_str("await using ");
                escape_into(out, n.name());
            }
        }
        NodeKind::FormalParameter
        | NodeKind::ReturnArgumentReference
        | NodeKind::ThrowArgumentReference
        | NodeKind::SyntheticIfStatementTest
        | NodeKind::SyntheticSwitchStatementDiscriminant
        | NodeKind::SyntheticWhileStatementTest
        | NodeKind::SyntheticDoWhileStatementTest
        | NodeKind::SyntheticForStatementHeader
        | NodeKind::SyntheticForInStatementHeader
        | NodeKind::SyntheticForOfStatementHeader
        | NodeKind::SyntheticModuleSink
        | NodeKind::SyntheticExpressionStatement
        | NodeKind::SyntheticBreakStatement
        | NodeKind::SyntheticContinueStatement => {
            escape_into(out, n.name());
        }
        NodeKind::SyntheticBeyondDepth => {
            out.push_str("...");
        }
    }
}

#[cfg(test)]
#[path = "node_head_test.rs"]
mod node_head_test;
