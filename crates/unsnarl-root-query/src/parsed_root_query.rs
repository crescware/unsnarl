use serde::Serialize;
use unsnarl_ir::SourceLine;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ParsedRootQuery {
    Line {
        line: SourceLine,
        raw: String,
    },
    LineName {
        line: SourceLine,
        name: String,
        raw: String,
    },
    Range {
        start: SourceLine,
        end: SourceLine,
        raw: String,
    },
    RangeName {
        start: SourceLine,
        end: SourceLine,
        name: String,
        raw: String,
    },
    Name {
        name: String,
        raw: String,
    },
    LineOrName {
        line: SourceLine,
        name: String,
        raw: String,
    },
}
