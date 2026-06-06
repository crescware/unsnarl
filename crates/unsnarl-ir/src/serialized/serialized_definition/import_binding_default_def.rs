//! Default-import binding definition variant.

use serde::Serialize;

use crate::import_kind::ImportKind;
use crate::non_empty::assert_non_empty;

use super::{DefinitionName, DefinitionNode, ImportBindingTag};

#[derive(Serialize)]
enum DefaultImportKind {
    #[serde(rename = "default")]
    Default,
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
