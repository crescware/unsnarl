//! Serialize a `Definition` into its on-disk JSON union form.
//!
//! Reads parser-owned fields off the
//! `Definition.{declaration_kind, import_source, imported_name,
//! init}` extras that the boundary materialises at declaration time.

use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::scope::definition::Definition;
use unsnarl_ir::serialized::{
    DefinitionName, DefinitionNode, ImportBindingDefaultDef, ImportBindingNamedDef,
    ImportBindingNamespaceDef, SerializedDefinition, SimpleDef,
};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::serializer::flat::span_of::{span_of_identifier, span_of_node};

pub fn serialize_definition(
    definition: &Definition,
    index: &SourceIndex<'_>,
) -> SerializedDefinition {
    let name = DefinitionName::new(
        definition.name.name().to_string(),
        span_of_identifier(&definition.name, index),
    );
    let node = DefinitionNode {
        r#type: definition.node.r#type.clone(),
        span: span_of_node(&definition.node, index),
    };
    let parent = definition.parent.as_ref().map(|p| DefinitionNode {
        r#type: p.r#type.clone(),
        span: span_of_node(p, index),
    });

    match definition.r#type {
        DefinitionType::ImportBinding => serialize_import_binding(definition, name, node, parent),
        DefinitionType::Variable => serialize_variable_def(definition, index, name, node, parent),
        DefinitionType::FunctionName => SerializedDefinition::Simple(SimpleDef {
            name,
            node,
            parent,
            r#type: unsnarl_ir::serialized::serialized_definition::SimpleDefType::FunctionName,
        }),
        DefinitionType::ClassName => SerializedDefinition::Simple(SimpleDef {
            name,
            node,
            parent,
            r#type: unsnarl_ir::serialized::serialized_definition::SimpleDefType::ClassName,
        }),
        DefinitionType::Parameter => SerializedDefinition::Simple(SimpleDef {
            name,
            node,
            parent,
            r#type: unsnarl_ir::serialized::serialized_definition::SimpleDefType::Parameter,
        }),
        DefinitionType::CatchClause => SerializedDefinition::Simple(SimpleDef {
            name,
            node,
            parent,
            r#type: unsnarl_ir::serialized::serialized_definition::SimpleDefType::CatchClause,
        }),
        DefinitionType::ImplicitGlobalVariable => SerializedDefinition::Simple(SimpleDef {
            name,
            node,
            parent,
            r#type:
                unsnarl_ir::serialized::serialized_definition::SimpleDefType::ImplicitGlobalVariable,
        }),
    }
}

fn serialize_import_binding(
    definition: &Definition,
    name: DefinitionName,
    node: DefinitionNode,
    parent: Option<DefinitionNode>,
) -> SerializedDefinition {
    let import_source = definition.import_source.clone().unwrap_or_else(|| {
        panic!(
            "ImportBinding {} missing import_source",
            definition.name.name()
        )
    });
    match definition.node.r#type {
        AstType::ImportDefaultSpecifier => SerializedDefinition::ImportBindingDefault(
            ImportBindingDefaultDef::new(name, node, parent, import_source),
        ),
        AstType::ImportNamespaceSpecifier => SerializedDefinition::ImportBindingNamespace(
            ImportBindingNamespaceDef::new(name, node, parent, import_source),
        ),
        AstType::ImportSpecifier => {
            let imported_name = definition.imported_name.clone().unwrap_or_else(|| {
                panic!(
                    "ImportSpecifier {} missing imported_name",
                    definition.name.name()
                )
            });
            SerializedDefinition::ImportBindingNamed(ImportBindingNamedDef::new(
                name,
                node,
                parent,
                imported_name,
                import_source,
            ))
        }
        _ => panic!(
            "Unexpected ImportBinding node type for {}",
            definition.name.name()
        ),
    }
}

fn serialize_variable_def(
    definition: &Definition,
    index: &SourceIndex<'_>,
    name: DefinitionName,
    node: DefinitionNode,
    parent: Option<DefinitionNode>,
) -> SerializedDefinition {
    let init = definition.init.as_ref().map(|init_node| DefinitionNode {
        r#type: init_node.r#type.clone(),
        span: span_of_node(init_node, index),
    });
    let declaration_kind = definition.declaration_kind.clone().unwrap_or_else(|| {
        panic!(
            "Variable definition {} missing declaration_kind",
            definition.name.name()
        )
    });
    SerializedDefinition::Variable(unsnarl_ir::serialized::VariableDef::new(
        name,
        node,
        parent,
        init,
        declaration_kind,
    ))
}
