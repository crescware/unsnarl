//! Constructors for [`SyntheticVisualNode`].

use crate::visual_element_type::NodeTypeTag;

use super::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode};

impl SyntheticVisualNode {
    fn base(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
        kind: SyntheticNodeKind,
        extras: SyntheticExtras,
    ) -> Self {
        Self {
            r#type: NodeTypeTag::Node,
            id: id.into(),
            kind,
            name: name.into(),
            line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            extras,
        }
    }

    pub fn write_reference(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::WriteReference,
            SyntheticExtras::WriteOp {
                declaration_kind: None,
            },
        )
    }

    pub fn return_argument_reference(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::ReturnArgumentReference,
            SyntheticExtras::None {},
        )
    }

    pub fn throw_argument_reference(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::ThrowArgumentReference,
            SyntheticExtras::None {},
        )
    }

    pub fn module_sink(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticModuleSink,
            SyntheticExtras::None {},
        )
    }

    pub fn import_intermediate(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticImportIntermediate,
            SyntheticExtras::None {},
        )
    }

    pub fn expression_statement(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticExpressionStatement,
            SyntheticExtras::None {},
        )
    }

    pub fn if_statement_test(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "if-test",
            line,
            SyntheticNodeKind::SyntheticIfStatementTest,
            SyntheticExtras::None {},
        )
    }

    pub fn switch_discriminant(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "switch-discriminant",
            line,
            SyntheticNodeKind::SyntheticSwitchStatementDiscriminant,
            SyntheticExtras::None {},
        )
    }

    pub fn while_statement_test(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "while-test",
            line,
            SyntheticNodeKind::SyntheticWhileStatementTest,
            SyntheticExtras::None {},
        )
    }

    pub fn do_while_statement_test(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "do-while-test",
            line,
            SyntheticNodeKind::SyntheticDoWhileStatementTest,
            SyntheticExtras::None {},
        )
    }

    pub fn for_statement_header(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "for-test",
            line,
            SyntheticNodeKind::SyntheticForStatementHeader,
            SyntheticExtras::None {},
        )
    }

    pub fn for_in_statement_header(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "for-test",
            line,
            SyntheticNodeKind::SyntheticForInStatementHeader,
            SyntheticExtras::None {},
        )
    }

    pub fn for_of_statement_header(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "for-test",
            line,
            SyntheticNodeKind::SyntheticForOfStatementHeader,
            SyntheticExtras::None {},
        )
    }

    pub fn beyond_depth(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "...",
            line,
            SyntheticNodeKind::SyntheticBeyondDepth,
            SyntheticExtras::None {},
        )
    }

    /// `break` or `break <label>` synthetic node. `name` is the
    /// pre-formatted label (`"break"` or `"break outer"`); the
    /// emitter writes it verbatim plus the trailing line stamp.
    pub fn break_statement(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticBreakStatement,
            SyntheticExtras::None {},
        )
    }

    /// `continue` or `continue <label>` synthetic node. See
    /// [`Self::break_statement`] for the `name` convention.
    pub fn continue_statement(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticContinueStatement,
            SyntheticExtras::None {},
        )
    }
}
