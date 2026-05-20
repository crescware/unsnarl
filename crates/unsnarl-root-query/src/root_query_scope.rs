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
