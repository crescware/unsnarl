//! Definition categorization.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DefinitionType {
    CatchClause,
    ClassName,
    FunctionName,
    ImplicitGlobalVariable,
    ImportBinding,
    Parameter,
    Variable,
}
