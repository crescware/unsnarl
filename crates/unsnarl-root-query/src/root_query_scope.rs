pub struct RootQueryScope {
    pub point: bool,
    pub path: bool,
    pub direction: bool,
    pub direction_level: bool,
}

pub const ROOT_QUERY_SCOPE_POINT_ONLY: RootQueryScope = RootQueryScope {
    point: true,
    path: false,
    direction: false,
    direction_level: false,
};

/// Highlight (`-H`) opts into the path (`a..b`) and direction
/// (`a..+a` / `+b` / `+c`) shapes on top of the point grammar. The
/// `+aN` generation level stays gated off for the POC (`+a3` parses
/// structurally but is rejected here), matching issue #90's "N is
/// reserved, reject for now" decision.
pub const ROOT_QUERY_SCOPE_HIGHLIGHT: RootQueryScope = RootQueryScope {
    point: true,
    path: true,
    direction: true,
    direction_level: false,
};
