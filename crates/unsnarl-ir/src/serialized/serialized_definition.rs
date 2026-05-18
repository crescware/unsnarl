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

use unsnarl_ast_type::AstType;

use crate::import_kind::ImportKind;
use crate::primitive::Span;
use crate::variable_declaration_kind::VariableDeclarationKind;

#[derive(Serialize)]
pub struct DefinitionName {
    name: String,
    span: Span,
}

impl DefinitionName {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        let name = name.into();
        assert!(!name.is_empty(), "DefinitionName.name must be non-empty");
        Self { name, span }
    }
}

#[derive(Serialize)]
pub struct DefinitionNode {
    pub r#type: AstType,
    pub span: Span,
}

#[derive(Serialize)]
enum VariableTag {
    Variable,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableDef {
    name: DefinitionName,
    node: DefinitionNode,
    parent: Option<DefinitionNode>,
    r#type: VariableTag,
    init: Option<DefinitionNode>,
    declaration_kind: VariableDeclarationKind,
}

impl VariableDef {
    pub fn new(
        name: DefinitionName,
        node: DefinitionNode,
        parent: Option<DefinitionNode>,
        init: Option<DefinitionNode>,
        declaration_kind: VariableDeclarationKind,
    ) -> Self {
        Self {
            name,
            node,
            parent,
            r#type: VariableTag::Variable,
            init,
            declaration_kind,
        }
    }
}

#[derive(Serialize)]
enum ImportBindingTag {
    ImportBinding,
}

#[derive(Serialize)]
enum NamedImportKind {
    #[serde(rename = "named")]
    Named,
}

#[derive(Serialize)]
enum DefaultImportKind {
    #[serde(rename = "default")]
    Default,
}

#[derive(Serialize)]
enum NamespaceImportKind {
    #[serde(rename = "namespace")]
    Namespace,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportBindingNamedDef {
    name: DefinitionName,
    node: DefinitionNode,
    parent: Option<DefinitionNode>,
    r#type: ImportBindingTag,
    import_kind: NamedImportKind,
    imported_name: String,
    import_source: String,
}

impl ImportBindingNamedDef {
    pub fn new(
        name: DefinitionName,
        node: DefinitionNode,
        parent: Option<DefinitionNode>,
        imported_name: impl Into<String>,
        import_source: impl Into<String>,
    ) -> Self {
        let imported_name = imported_name.into();
        let import_source = import_source.into();
        assert!(
            !imported_name.is_empty(),
            "ImportBindingNamedDef.imported_name must be non-empty"
        );
        assert!(
            !import_source.is_empty(),
            "ImportBindingNamedDef.import_source must be non-empty"
        );
        Self {
            name,
            node,
            parent,
            r#type: ImportBindingTag::ImportBinding,
            import_kind: NamedImportKind::Named,
            imported_name,
            import_source,
        }
    }

    pub fn import_kind(&self) -> ImportKind {
        ImportKind::Named
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportBindingDefaultDef {
    name: DefinitionName,
    node: DefinitionNode,
    parent: Option<DefinitionNode>,
    r#type: ImportBindingTag,
    import_kind: DefaultImportKind,
    import_source: String,
}

impl ImportBindingDefaultDef {
    pub fn new(
        name: DefinitionName,
        node: DefinitionNode,
        parent: Option<DefinitionNode>,
        import_source: impl Into<String>,
    ) -> Self {
        let import_source = import_source.into();
        assert!(
            !import_source.is_empty(),
            "ImportBindingDefaultDef.import_source must be non-empty"
        );
        Self {
            name,
            node,
            parent,
            r#type: ImportBindingTag::ImportBinding,
            import_kind: DefaultImportKind::Default,
            import_source,
        }
    }

    pub fn import_kind(&self) -> ImportKind {
        ImportKind::Default
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportBindingNamespaceDef {
    name: DefinitionName,
    node: DefinitionNode,
    parent: Option<DefinitionNode>,
    r#type: ImportBindingTag,
    import_kind: NamespaceImportKind,
    import_source: String,
}

impl ImportBindingNamespaceDef {
    pub fn new(
        name: DefinitionName,
        node: DefinitionNode,
        parent: Option<DefinitionNode>,
        import_source: impl Into<String>,
    ) -> Self {
        let import_source = import_source.into();
        assert!(
            !import_source.is_empty(),
            "ImportBindingNamespaceDef.import_source must be non-empty"
        );
        Self {
            name,
            node,
            parent,
            r#type: ImportBindingTag::ImportBinding,
            import_kind: NamespaceImportKind::Namespace,
            import_source,
        }
    }

    pub fn import_kind(&self) -> ImportKind {
        ImportKind::Namespace
    }
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
