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

use unsnarl_oxc_parity::{AstType, VariableDeclarationKind};

use crate::import_kind::ImportKind;
use crate::non_empty::assert_non_empty;
use crate::primitive::Span;

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

    pub fn name(&self) -> &DefinitionName {
        &self.name
    }

    pub fn node(&self) -> &DefinitionNode {
        &self.node
    }

    pub fn parent(&self) -> Option<&DefinitionNode> {
        self.parent.as_ref()
    }

    pub fn init(&self) -> Option<&DefinitionNode> {
        self.init.as_ref()
    }

    /// Replace the `init` field. Used by `unsnarl-plugin-react` to
    /// peel `useCallback(...)` so the variable's init points at the
    /// inner function expression instead of the call.
    pub fn set_init(&mut self, init: Option<DefinitionNode>) {
        self.init = init;
    }

    pub fn declaration_kind(&self) -> &VariableDeclarationKind {
        &self.declaration_kind
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
        imported_name: String,
        import_source: String,
    ) -> Self {
        assert_non_empty(&imported_name, "ImportBindingNamedDef.imported_name");
        assert_non_empty(&import_source, "ImportBindingNamedDef.import_source");
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

    pub fn name(&self) -> &DefinitionName {
        &self.name
    }

    pub fn node(&self) -> &DefinitionNode {
        &self.node
    }

    pub fn parent(&self) -> Option<&DefinitionNode> {
        self.parent.as_ref()
    }

    pub fn imported_name(&self) -> &str {
        &self.imported_name
    }

    pub fn import_source(&self) -> &str {
        &self.import_source
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
        import_source: String,
    ) -> Self {
        assert_non_empty(&import_source, "ImportBindingDefaultDef.import_source");
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

    pub fn name(&self) -> &DefinitionName {
        &self.name
    }

    pub fn node(&self) -> &DefinitionNode {
        &self.node
    }

    pub fn parent(&self) -> Option<&DefinitionNode> {
        self.parent.as_ref()
    }

    pub fn import_source(&self) -> &str {
        &self.import_source
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
        import_source: String,
    ) -> Self {
        assert_non_empty(&import_source, "ImportBindingNamespaceDef.import_source");
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

    pub fn name(&self) -> &DefinitionName {
        &self.name
    }

    pub fn node(&self) -> &DefinitionNode {
        &self.node
    }

    pub fn parent(&self) -> Option<&DefinitionNode> {
        self.parent.as_ref()
    }

    pub fn import_source(&self) -> &str {
        &self.import_source
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
