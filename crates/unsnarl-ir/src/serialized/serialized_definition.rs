//! Serialized counterpart of `Definition`.
//!
//! The JSON shape is a 9-way union: 5 "no-extra-fields" definition
//! kinds plus 3 `ImportBinding` sub-shapes keyed by `importKind`,
//! plus `Variable`. `name` / `node` / `parent` are declared BEFORE
//! the `type` discriminator (so the tag appears in the middle of the
//! object). Serde's tagged-enum modes always put the tag first, so
//! each variant is its own struct with explicit field order and the
//! wrapper enum delegates `Serialize` to the variant's struct.

use serde::Serialize;

use unsnarl_oxc_parity::AstType;

use crate::non_empty::assert_non_empty;
use crate::primitive::Span;

mod import_binding_default_def;
mod import_binding_named_def;
mod import_binding_namespace_def;
mod variable_def;

pub use import_binding_default_def::ImportBindingDefaultDef;
pub use import_binding_named_def::ImportBindingNamedDef;
pub use import_binding_namespace_def::ImportBindingNamespaceDef;
pub use variable_def::VariableDef;

#[derive(Serialize)]
pub struct DefinitionName {
    name: String,
    span: Span,
}

impl DefinitionName {
    pub fn new(name: String, span: Span) -> Self {
        assert_non_empty(&name, "DefinitionName.name");
        Self { name, span }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Serialize)]
pub struct DefinitionNode {
    pub r#type: AstType,
    pub span: Span,
}

#[derive(Serialize)]
enum ImportBindingTag {
    ImportBinding,
}

/// The 5 "no-extra-fields" variants share one struct: `FunctionName`,
/// `ClassName`, `Parameter`, `CatchClause`, `ImplicitGlobalVariable`.
#[derive(Serialize)]
pub struct SimpleDef {
    pub name: DefinitionName,
    pub node: DefinitionNode,
    pub parent: Option<DefinitionNode>,
    pub r#type: SimpleDefType,
}

#[derive(Serialize)]
pub enum SimpleDefType {
    FunctionName,
    ClassName,
    Parameter,
    CatchClause,
    ImplicitGlobalVariable,
}

pub enum SerializedDefinition {
    Variable(VariableDef),
    ImportBindingNamed(ImportBindingNamedDef),
    ImportBindingDefault(ImportBindingDefaultDef),
    ImportBindingNamespace(ImportBindingNamespaceDef),
    Simple(SimpleDef),
}

impl Serialize for SerializedDefinition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Variable(d) => d.serialize(serializer),
            Self::ImportBindingNamed(d) => d.serialize(serializer),
            Self::ImportBindingDefault(d) => d.serialize(serializer),
            Self::ImportBindingNamespace(d) => d.serialize(serializer),
            Self::Simple(d) => d.serialize(serializer),
        }
    }
}

#[cfg(test)]
#[path = "serialized_definition_test.rs"]
mod serialized_definition_test;
