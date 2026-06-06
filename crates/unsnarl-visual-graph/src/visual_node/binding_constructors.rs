//! Constructors for [`BindingVisualNode`].

use crate::visual_element_type::NodeTypeTag;

use super::{BindingExtras, BindingNodeKind, BindingVisualNode};

impl BindingVisualNode {
    fn base(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
        kind: BindingNodeKind,
        extras: BindingExtras,
    ) -> Self {
        Self {
            r#type: NodeTypeTag::Node,
            id: id.into(),
            name: name.into(),
            line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            kind,
            extras,
        }
    }

    pub fn const_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::ConstBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn var_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::VarBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn let_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::LetBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn using_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::UsingBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn await_using_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::AwaitUsingBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn function_declaration(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::FunctionDeclaration,
            BindingExtras::None {},
        )
    }

    pub fn class_declaration(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::ClassDeclaration,
            BindingExtras::None {},
        )
    }

    pub fn formal_parameter(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::FormalParameter,
            BindingExtras::None {},
        )
    }

    pub fn catch_parameter(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::CatchParameter,
            BindingExtras::None {},
        )
    }

    pub fn named_import_binding(
        id: impl Into<String>,
        name: impl Into<String>,
        imported_name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::NamedImportBinding,
            BindingExtras::NamedImport {
                imported_name: imported_name.into(),
            },
        )
    }

    pub fn default_import_binding(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::DefaultImportBinding,
            BindingExtras::None {},
        )
    }

    pub fn namespace_import_binding(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::NamespaceImportBinding,
            BindingExtras::None {},
        )
    }
}
