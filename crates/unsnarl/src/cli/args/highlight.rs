//! Typed `-H` / `--highlight` value: absent, given without an
//! inline value (follow `-r/--roots`), or given with a parsed query
//! list.

use serde::{Serialize, Serializer};
use unsnarl_root_query::ParsedRootQuery;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Highlight {
    #[default]
    Absent,
    NoValue,
    Value(Vec<ParsedRootQuery>),
}

impl Serialize for Highlight {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Highlight::Absent => s.serialize_bool(false),
            Highlight::NoValue => s.serialize_bool(true),
            Highlight::Value(queries) => queries.serialize(s),
        }
    }
}
