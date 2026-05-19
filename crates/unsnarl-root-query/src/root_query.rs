use crate::generation_count::GenerationCount;
use crate::parsed_root_query::ParsedRootQuery;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    After,
    Before,
    Context,
}

#[derive(Debug, PartialEq, Eq)]
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
