//! Named-import binding definition variant.

use serde::Serialize;

use crate::import_kind::ImportKind;
use crate::non_empty::assert_non_empty;

use super::{DefinitionName, DefinitionNode, ImportBindingTag};

#[derive(Serialize)]
enum NamedImportKind {
    #[serde(rename = "named")]
    Named,
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
