//! Builds a `VisualNode` from a `SerializedVariable`. The TS form
//! dispatches on `def.type` (the first definition) and assembles
//! the `common` fields once before adding the kind-specific tail;
//! the Rust port mirrors that with a single `BindingVisualNode`
//! constructed up-front and `BindingExtras` for the tail.

use unsnarl_ir::serialized::{SerializedDefinition, SerializedVariable, SimpleDefType};
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind};

use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{BindingExtras, BindingNodeKind, BindingVisualNode, VisualNode};

pub fn make_variable_node(v: &SerializedVariable) -> VisualNode {
    let def = v
        .defs
        .first()
        .expect("SerializedVariable must carry at least one def");
    // `ImplicitGlobalVariable` has no source-level definition; the
    // analyzer pins its synthetic def to the first reference, so any
    // line we read from it would lie about where the global "lives".
    // Treat it as location-less (line 0).
    let is_implicit_global = matches!(
        def,
        SerializedDefinition::Simple(s) if matches!(s.r#type, SimpleDefType::ImplicitGlobalVariable)
    );
    let line = if is_implicit_global {
        0
    } else {
        v.identifiers
            .first()
            .map(|s| s.line.0)
            .unwrap_or_else(|| name_span_line(def))
    };

    let common = |kind: BindingNodeKind, extras: BindingExtras| -> VisualNode {
        VisualNode::Binding(BindingVisualNode {
            r#type: NodeTypeTag::Node,
            id: super::node_id::node_id(v.id.value()),
            name: v.name().to_string(),
            line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            kind,
            extras,
        })
    };

    match def {
        SerializedDefinition::ImportBindingNamed(d) => common(
            BindingNodeKind::NamedImportBinding,
            BindingExtras::NamedImport {
                imported_name: d.imported_name().to_string(),
            },
        ),
        SerializedDefinition::ImportBindingDefault(_) => common(
            BindingNodeKind::DefaultImportBinding,
            BindingExtras::None {},
        ),
        SerializedDefinition::ImportBindingNamespace(_) => common(
            BindingNodeKind::NamespaceImportBinding,
            BindingExtras::None {},
        ),
        SerializedDefinition::Variable(d) => {
            let init_is_function = matches!(
                d.init().map(|n| &n.r#type),
                Some(AstType::ArrowFunctionExpression) | Some(AstType::FunctionExpression)
            );
            let kind = match d.declaration_kind() {
                VariableDeclarationKind::Const => BindingNodeKind::ConstBinding,
                VariableDeclarationKind::Let => BindingNodeKind::LetBinding,
                VariableDeclarationKind::Var => BindingNodeKind::VarBinding,
            };
            common(kind, BindingExtras::Variable { init_is_function })
        }
        SerializedDefinition::Simple(s) => {
            let kind = match s.r#type {
                SimpleDefType::FunctionName => BindingNodeKind::FunctionDeclaration,
                SimpleDefType::ClassName => BindingNodeKind::ClassDeclaration,
                SimpleDefType::Parameter => BindingNodeKind::FormalParameter,
                SimpleDefType::CatchClause => BindingNodeKind::CatchParameter,
                SimpleDefType::ImplicitGlobalVariable => BindingNodeKind::SyntheticImplicitGlobal,
            };
            common(kind, BindingExtras::None {})
        }
    }
}

fn name_span_line(def: &SerializedDefinition) -> u32 {
    let def_name = match def {
        SerializedDefinition::Variable(d) => d.name(),
        SerializedDefinition::ImportBindingNamed(d) => d.name(),
        SerializedDefinition::ImportBindingDefault(d) => d.name(),
        SerializedDefinition::ImportBindingNamespace(d) => d.name(),
        SerializedDefinition::Simple(s) => &s.name,
    };
    def_name.span().line.0
}

#[cfg(test)]
#[path = "make_variable_node_test.rs"]
mod make_variable_node_test;
