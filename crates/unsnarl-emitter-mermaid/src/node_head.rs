//! Builds the leading text fragment of a node's mermaid label
//! (before the trailing `<br/>L<line>` range stamp).
//!
//! Mirrors `ts/src/emitter/mermaid/node-head.ts`.

use unsnarl_oxc_parity::VariableDeclarationKind;
use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

use crate::escape::escape;

pub fn node_head(n: &VisualNode) -> String {
    let name = escape(n.name());
    if n.is_jsx_element() {
        // Mermaid `["..."]` labels require HTML-escaped angle
        // brackets so the parser does not mistake them for syntax;
        // the renderer surfaces them as literal `<` / `>` in the
        // output.
        return format!("&lt;{name}&gt;");
    }
    match n.kind() {
        NodeKind::FunctionDeclaration => format!("{name}()"),
        NodeKind::ClassDeclaration => format!("class {name}"),
        NodeKind::NamedImportBinding => match n.imported_name() {
            Some(imported) if imported != n.name() => name,
            _ => format!("import {name}"),
        },
        NodeKind::DefaultImportBinding | NodeKind::NamespaceImportBinding => {
            format!("import {name}")
        }
        NodeKind::CatchParameter => format!("catch {name}"),
        NodeKind::SyntheticImplicitGlobal => format!("global {name}"),
        NodeKind::WriteReference => match n.declaration_kind() {
            Some(VariableDeclarationKind::Let) => format!("let {name}"),
            _ => name,
        },
        NodeKind::SyntheticModuleSource => format!("module {name}"),
        NodeKind::SyntheticImportIntermediate => format!("import {name}"),
        NodeKind::VarBinding => {
            if n.init_is_function() {
                format!("{name}()")
            } else {
                format!("var {name}")
            }
        }
        NodeKind::ConstBinding => {
            if n.init_is_function() {
                format!("{name}()")
            } else {
                name
            }
        }
        NodeKind::LetBinding => {
            if n.init_is_function() {
                format!("{name}()")
            } else {
                format!("let {name}")
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
        | NodeKind::SyntheticExpressionStatement => name,
        NodeKind::SyntheticBeyondDepth => "...".to_string(),
    }
}
