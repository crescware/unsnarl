export type RootQueryScope = Readonly<{
  point: boolean;
  path: boolean;
  direction: boolean;
  directionLevel: boolean;
}>;

export const ROOT_QUERY_SCOPE_POINT_ONLY: RootQueryScope = {
  point: true,
  path: false,
  direction: false,
  directionLevel: false,
};
