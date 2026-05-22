//! `NodeKind`: tag on `VisualNode` distinguishing every shape of node
//! the builder emits.
//!
//! The values double as the on-disk JSON tags, so they serialize to
//! the same identifiers (`"VarBinding"`,
//! `"SyntheticIfStatementTest"`, etc.).

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NodeKind {
    VarBinding,
    ConstBinding,
    LetBinding,
    FunctionDeclaration,
    ClassDeclaration,
    FormalParameter,
    CatchParameter,
    NamedImportBinding,
    DefaultImportBinding,
    NamespaceImportBinding,
    SyntheticImplicitGlobal,
    WriteReference,
    ReturnArgumentReference,
    ThrowArgumentReference,
    SyntheticIfStatementTest,
    SyntheticSwitchStatementDiscriminant,
    SyntheticWhileStatementTest,
    SyntheticDoWhileStatementTest,
    SyntheticForStatementHeader,
    SyntheticForInStatementHeader,
    SyntheticForOfStatementHeader,
    SyntheticModuleSink,
    SyntheticModuleSource,
    SyntheticImportIntermediate,
    SyntheticExpressionStatement,
    SyntheticBeyondDepth,
}

impl NodeKind {
    /// The bare variant name as it appears in the on-disk JSON
    /// (`"VarBinding"`, `"SyntheticIfStatementTest"`, ...). Used by
    /// the mermaid emitter's `--debug` mode to splice the kind name
    /// into the rendered label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VarBinding => "VarBinding",
            Self::ConstBinding => "ConstBinding",
            Self::LetBinding => "LetBinding",
            Self::FunctionDeclaration => "FunctionDeclaration",
            Self::ClassDeclaration => "ClassDeclaration",
            Self::FormalParameter => "FormalParameter",
            Self::CatchParameter => "CatchParameter",
            Self::NamedImportBinding => "NamedImportBinding",
            Self::DefaultImportBinding => "DefaultImportBinding",
            Self::NamespaceImportBinding => "NamespaceImportBinding",
            Self::SyntheticImplicitGlobal => "SyntheticImplicitGlobal",
            Self::WriteReference => "WriteReference",
            Self::ReturnArgumentReference => "ReturnArgumentReference",
            Self::ThrowArgumentReference => "ThrowArgumentReference",
            Self::SyntheticIfStatementTest => "SyntheticIfStatementTest",
            Self::SyntheticSwitchStatementDiscriminant => "SyntheticSwitchStatementDiscriminant",
            Self::SyntheticWhileStatementTest => "SyntheticWhileStatementTest",
            Self::SyntheticDoWhileStatementTest => "SyntheticDoWhileStatementTest",
            Self::SyntheticForStatementHeader => "SyntheticForStatementHeader",
            Self::SyntheticForInStatementHeader => "SyntheticForInStatementHeader",
            Self::SyntheticForOfStatementHeader => "SyntheticForOfStatementHeader",
            Self::SyntheticModuleSink => "SyntheticModuleSink",
            Self::SyntheticModuleSource => "SyntheticModuleSource",
            Self::SyntheticImportIntermediate => "SyntheticImportIntermediate",
            Self::SyntheticExpressionStatement => "SyntheticExpressionStatement",
            Self::SyntheticBeyondDepth => "SyntheticBeyondDepth",
        }
    }
}
