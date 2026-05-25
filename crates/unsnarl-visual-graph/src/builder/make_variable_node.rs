//! Builds a `VisualNode` from a `SerializedVariable`. Dispatches on
//! `def.type` (the first definition); the `common` fields are
//! assembled once into a [`BindingVisualNode`] before the
//! kind-specific tail in [`BindingExtras`] is attached.

use unsnarl_ir::serialized::{SerializedDefinition, SerializedVariable, SimpleDefType};
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind};

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

    let id = super::node_id::node_id(v.id.value());
    let name = v.name().to_string();

    let node = match def {
        SerializedDefinition::ImportBindingNamed(d) => {
            BindingVisualNode::named_import_binding(id, name, d.imported_name().to_string(), line)
        }
        SerializedDefinition::ImportBindingDefault(_) => {
            BindingVisualNode::default_import_binding(id, name, line)
        }
        SerializedDefinition::ImportBindingNamespace(_) => {
            BindingVisualNode::namespace_import_binding(id, name, line)
        }
        SerializedDefinition::Variable(d) => {
            let init_is_function = matches!(
                d.init().map(|n| &n.r#type),
                Some(AstType::ArrowFunctionExpression) | Some(AstType::FunctionExpression)
            );
            let mut n = match d.declaration_kind() {
                VariableDeclarationKind::Const => {
                    BindingVisualNode::const_binding(&id, &name, line)
                }
                VariableDeclarationKind::Let => BindingVisualNode::let_binding(&id, &name, line),
                VariableDeclarationKind::Var => BindingVisualNode::var_binding(&id, &name, line),
            };
            n.extras = BindingExtras::Variable { init_is_function };
            n
        }
        SerializedDefinition::Simple(s) => match s.r#type {
            SimpleDefType::FunctionName => BindingVisualNode::function_declaration(id, name, line),
            SimpleDefType::ClassName => BindingVisualNode::class_declaration(id, name, line),
            SimpleDefType::Parameter => BindingVisualNode::formal_parameter(id, name, line),
            SimpleDefType::CatchClause => BindingVisualNode::catch_parameter(id, name, line),
            SimpleDefType::ImplicitGlobalVariable => BindingVisualNode {
                kind: BindingNodeKind::SyntheticImplicitGlobal,
                ..BindingVisualNode::formal_parameter(id, name, line)
            },
        },
    };
    node.into()
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
