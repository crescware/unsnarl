//! Definition categorization.

use serde::Serialize;

#[derive(Serialize)]
pub enum DefinitionType {
    CatchClause,
    ClassName,
    FunctionName,
    ImplicitGlobalVariable,
    ImportBinding,
    Parameter,
    Variable,
}
