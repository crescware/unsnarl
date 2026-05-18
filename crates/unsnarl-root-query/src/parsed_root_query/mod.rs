use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ParsedRootQuery {
    Line {
        line: u32,
        raw: String,
    },
    LineName {
        line: u32,
        name: String,
        raw: String,
    },
    Range {
        start: u32,
        end: u32,
        raw: String,
    },
    RangeName {
        start: u32,
        end: u32,
        name: String,
        raw: String,
    },
    Name {
        name: String,
        raw: String,
    },
    LineOrName {
        line: u32,
        name: String,
        raw: String,
    },
}
