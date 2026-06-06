//! `Variable` definition variant (kind = `VariableDeclarator`).

use serde::Serialize;

use unsnarl_oxc_parity::VariableDeclarationKind;

use super::{DefinitionName, DefinitionNode};

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
