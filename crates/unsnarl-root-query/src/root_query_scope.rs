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
/// `+aN` generation level stays gated off (`+a3` parses structurally
/// but is rejected here): issue #90 reserves the radius for a future
/// extension and rejects it for now.
pub const ROOT_QUERY_SCOPE_HIGHLIGHT: RootQueryScope = RootQueryScope {
    point: true,
    path: true,
    direction: true,
    direction_level: false,
};
