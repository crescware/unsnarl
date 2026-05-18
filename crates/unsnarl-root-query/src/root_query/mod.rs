use crate::parsed_root_query::ParsedRootQuery;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    A,
    B,
    C,
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
        level: Option<u32>,
        raw: String,
    },
}
