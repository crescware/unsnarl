//! `NodeKind`: tag on `VisualNode` distinguishing every shape of node
//! the builder emits.
//!
//! Mirrors `ts/src/visual-graph/node-kind.ts`. The values double as
//! the on-disk JSON tags, so they serialize to the same identifiers
//! (`"VarBinding"`, `"SyntheticIfStatementTest"`, etc.).

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
