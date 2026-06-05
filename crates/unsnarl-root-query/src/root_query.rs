use serde::{Serialize, Serializer};

use crate::generation_count::GenerationCount;
use crate::parsed_root_query::ParsedRootQuery;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    After,
    Before,
    Context,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootQuery {
    Single {
        query: ParsedRootQuery,
        raw: String,
    },
    Path {
        lhs: ParsedRootQuery,
        rhs: ParsedRootQuery,
        raw: String,
    },
    Direction {
        lhs: ParsedRootQuery,
        dir: Direction,
        level: Option<GenerationCount>,
        raw: String,
    },
}

impl Serialize for RootQuery {
    /// Backward-compatible serialization: a point query serializes
    /// exactly as its inner [`ParsedRootQuery`] did before the
    /// highlight carrier widened to `RootQuery`, so the `-H foo` debug
    /// JSON shape is unchanged. The new path / direction shapes have no
    /// prior JSON contract, so they serialize as their raw token.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RootQuery::Single { query, .. } => query.serialize(serializer),
            RootQuery::Path { raw, .. } | RootQuery::Direction { raw, .. } => {
                serializer.serialize_str(raw)
            }
        }
    }
}
