//! Definition categorization. Ports `ts/src/analyzer/definition-type.ts`.

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
